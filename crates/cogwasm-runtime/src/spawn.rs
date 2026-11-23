//! Reporting and execution of `playground.spawn` calls.
//!
//! Both hosts expose the same guest import and run the same native linker
//! subprocess. What they keep afterwards differs: the product host reports the
//! full argv in its JSON, while the benchmark host reports only the program
//! and argument count. A [`SpawnRecorder`] is the per-host policy hook.

use crate::Wt;
use crate::env::env_var_compat;
use crate::mem::guest_memory;
use crate::store::HostState;
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Instant;
use wasmtime::{Caller, Linker};

/// Native counters read from a child process just before it is reaped.
#[derive(Serialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChildCounters {
    /// Instructions retired by the child, when the platform reports them.
    pub instructions: Option<u64>,
    /// CPU cycles, informational.
    pub cycles: Option<u64>,
}

/// One `playground.spawn` invocation, as reported in host JSON output.
#[derive(Serialize, Clone, Debug)]
pub struct SpawnSummary {
    /// Program name: the first argv element.
    pub program: String,
    /// Number of arguments after the program name.
    pub args: usize,
    /// Wall-clock time spent in the linker subprocess, in milliseconds.
    pub ms: f64,
    /// Exit status, or -1 when the linker could not be started.
    pub status: i32,
    /// Instructions retired by the child, when the platform reports them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<u64>,
    /// CPU cycles, informational.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycles: Option<u64>,
}

/// Policy hook called after every completed spawn.
pub trait SpawnRecorder: Send + 'static {
    /// Record one invocation. `argv` is the complete argument vector,
    /// including the program name the guest supplied first.
    fn record(&self, argv: &[String], ms: f64, status: i32, counters: ChildCounters);

    /// A child process was started. The default opens the Linux perf events
    /// that must exist before the child exits; macOS needs no setup.
    fn child_started(&self, pid: u32) {
        observe_child(pid);
    }

    /// Read a child's native counters. Called after the child has exited but
    /// before it is reaped, so `proc_pid_rusage` can still see it, or so the
    /// Linux perf descriptors can be read and closed.
    fn child_counters(&self, pid: u32) -> ChildCounters {
        read_child(pid)
    }

    /// Whether [`add_playground_spawn`] should print the one-line per-spawn
    /// summary. The product host does; the benchmark host keeps its stderr
    /// quiet so timings and captured compiler output are unchanged.
    fn log_summary(&self) -> bool {
        false
    }

    /// Drop every record. A serving store calls this before each request so a
    /// request's report contains only its own spawns.
    fn reset(&self) {}

    /// Snapshot the records in the summarized form the host reports.
    fn summaries(&self) -> Vec<SpawnSummary> {
        Vec::new()
    }
}

/// One recorded invocation with its full argv.
#[derive(Clone, Debug)]
pub struct ArgvRecord {
    /// Complete argument vector, program name first.
    pub argv: Vec<String>,
    /// Wall-clock time spent in the linker subprocess, in milliseconds.
    pub ms: f64,
    /// Exit status, or -1 when the linker could not be started.
    pub status: i32,
    /// Native counters sampled from the child before it was reaped.
    pub counters: ChildCounters,
}

/// The product host's recorder: keeps the full argv of every spawn.
#[derive(Default)]
pub struct ArgvRecorder {
    records: Mutex<Vec<ArgvRecord>>,
}

impl ArgvRecorder {
    /// Snapshot the recorded invocations.
    pub fn records(&self) -> Vec<ArgvRecord> {
        locked(&self.records).clone()
    }
}

impl SpawnRecorder for ArgvRecorder {
    fn record(&self, argv: &[String], ms: f64, status: i32, counters: ChildCounters) {
        locked(&self.records).push(ArgvRecord {
            argv: argv.to_vec(),
            ms,
            status,
            counters,
        });
    }

    fn log_summary(&self) -> bool {
        true
    }

    fn reset(&self) {
        locked(&self.records).clear();
    }

    fn summaries(&self) -> Vec<SpawnSummary> {
        self.records()
            .iter()
            .map(|record| SpawnSummary {
                program: record.argv.first().cloned().unwrap_or_default(),
                args: record.argv.len().saturating_sub(1),
                ms: record.ms,
                status: record.status,
                instructions: record.counters.instructions,
                cycles: record.counters.cycles,
            })
            .collect()
    }
}

/// The benchmark host's recorder: keeps only the reported summary.
#[derive(Default)]
pub struct SummaryRecorder {
    summaries: Mutex<Vec<SpawnSummary>>,
}

impl SummaryRecorder {
    /// Snapshot the reported summaries.
    pub fn summaries(&self) -> Vec<SpawnSummary> {
        locked(&self.summaries).clone()
    }
}

impl SpawnRecorder for SummaryRecorder {
    fn record(&self, argv: &[String], ms: f64, status: i32, counters: ChildCounters) {
        locked(&self.summaries).push(SpawnSummary {
            program: argv.first().cloned().unwrap_or_default(),
            args: argv.len().saturating_sub(1),
            ms,
            status,
            instructions: counters.instructions,
            cycles: counters.cycles,
        });
    }

    fn reset(&self) {
        locked(&self.summaries).clear();
    }

    fn summaries(&self) -> Vec<SpawnSummary> {
        SummaryRecorder::summaries(self)
    }
}

/// Lock a spawn log, recovering from a poisoned mutex.
///
/// A panic elsewhere while one of these short-lived locks was held leaves the
/// `Vec` structurally valid; the alternative is a second panic inside a guest
/// import, which hides the original failure and traps rustc.wasm. Recovery is
/// safe because entries are only appended, never removed or partially
/// rewritten.
fn locked<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Define the `playground.spawn` import on `linker`.
///
/// `log_prefix` is the host's bracketed log tag, for example
/// `[cogwasm-host]`. `linker_path` is the native Wild binary the guest's
/// argument vector is meant for.
pub fn add_playground_spawn<R: SpawnRecorder>(
    linker: &mut Linker<HostState<R>>,
    linker_path: &Path,
    log_prefix: &'static str,
) -> Result<()> {
    let linker_path = linker_path.to_path_buf();
    linker
        .func_wrap(
            "playground",
            "spawn",
            move |mut caller: Caller<'_, HostState<R>>, argv: u32, argc: u32| -> i32 {
                match spawn_impl(&mut caller, argv, argc, &linker_path, log_prefix) {
                    Ok(code) => code,
                    Err(error) => {
                        eprintln!("{log_prefix} spawn failed: {error:#}");
                        -1
                    }
                }
            },
        )
        .wt("defining playground::spawn")?;
    Ok(())
}

/// Read the guest's argv and run the native linker on everything after the
/// program name. The guest keeps the exit status; spawn details go to the
/// store's recorder.
fn spawn_impl<R: SpawnRecorder>(
    caller: &mut Caller<'_, HostState<R>>,
    pointer_array: u32,
    argument_count: u32,
    linker: &Path,
    log_prefix: &str,
) -> Result<i32> {
    let memory = guest_memory(caller)?;
    let count = usize::try_from(argument_count).context("spawn argument count")?;
    let bytes = count.checked_mul(4).context("spawn argument array size")?;
    let mut raw = vec![0u8; bytes];
    let pointer_array = usize::try_from(pointer_array).context("spawn pointer array")?;
    memory.read(caller, pointer_array, &mut raw)?;
    let mut args = Vec::with_capacity(count);
    for chunk in raw.as_chunks::<4>().0 {
        let pointer = u32::from_le_bytes(*chunk);
        args.push(memory.read_cstr(caller, pointer)?);
    }
    if env_var_compat("SPAWN_DEBUG").is_some() {
        eprintln!("{log_prefix} spawn argv: {args:?}");
    }
    // The rustc patch puts the linker program name first. Run native Wild on
    // the remaining arguments, inheriting stdio and the filesystem view.
    let started = Instant::now();
    let mut counters = ChildCounters::default();
    let status = match Command::new(linker)
        .args(args.iter().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            let pid = child.id();
            let recorder = caller.data().recorder();
            recorder.child_started(pid);
            // Wait for the child to exit without reaping it, so its rusage
            // (instructions and cycles) can still be read. If `waitid` is not
            // available, `child.wait` below still reaps it and the counters
            // stay unavailable rather than becoming invented values.
            let waited = wait_without_reaping(pid);
            counters = recorder.child_counters(pid);
            if let Err(error) = waited {
                eprintln!("{log_prefix} cannot wait for the linker without reaping it: {error}");
            }
            child.wait()
        }
        Err(error) => {
            eprintln!("{log_prefix} cannot run {}: {error}", linker.display());
            Err(error)
        }
    };
    let ms = started.elapsed().as_secs_f64() * 1e3;
    let code = match status {
        Ok(status) if status.success() => 0,
        Ok(status) => status.code().unwrap_or(-1),
        Err(_) => -1,
    };
    let recorder = caller.data().recorder();
    if recorder.log_summary() {
        eprintln!(
            "{log_prefix} spawn: {} ({} args) -> {code} in {ms:.1} ms",
            args.first().map_or("<empty>", String::as_str),
            args.len().saturating_sub(1)
        );
    }
    recorder.record(&args, ms, code, counters);
    Ok(code)
}

/// Wait for `pid` to exit without reaping it.
#[cfg(unix)]
fn wait_without_reaping(pid: u32) -> std::io::Result<()> {
    let mut info = std::mem::MaybeUninit::<libc::siginfo_t>::zeroed();
    let id = pid as libc::id_t;
    // SAFETY: `info` is a writable `siginfo_t`; `waitid` initialises it and
    // does not retain it. `WNOWAIT` leaves the child waitable for the later
    // `Child::wait` that reaps it.
    let status = unsafe {
        libc::waitid(
            libc::P_PID,
            id,
            info.as_mut_ptr(),
            libc::WEXITED | libc::WNOWAIT,
        )
    };
    if status == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(not(unix))]
fn wait_without_reaping(_pid: u32) -> std::io::Result<()> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn observe_child(_pid: u32) {}

/// Read `ri_instructions`/`ri_cycles` from a child that has exited but not yet
/// been reaped.
#[cfg(target_os = "macos")]
fn read_child(pid: u32) -> ChildCounters {
    let pid = pid as libc::pid_t;
    let mut info = std::mem::MaybeUninit::<libc::rusage_info_v4>::zeroed();
    // SAFETY: `info` is a writable, correctly sized `rusage_info_v4` buffer.
    // `proc_pid_rusage` writes at most `sizeof(rusage_info_v4)` bytes for
    // `RUSAGE_INFO_V4`, does not retain the pointer, and returns a non-zero
    // status instead of filling the buffer on failure.
    let status = unsafe {
        libc::proc_pid_rusage(
            pid,
            libc::RUSAGE_INFO_V4,
            info.as_mut_ptr().cast::<libc::rusage_info_t>(),
        )
    };
    if status != 0 {
        return ChildCounters::default();
    }
    // SAFETY: a zero status means the buffer was initialised.
    let info = unsafe { info.assume_init() };
    ChildCounters {
        instructions: Some(info.ri_instructions),
        cycles: Some(info.ri_cycles),
    }
}

#[cfg(target_os = "linux")]
static PERF_FDS: Mutex<Vec<(u32, i32)>> = Mutex::new(Vec::new());

#[cfg(target_os = "linux")]
#[repr(C)]
struct PerfEventAttr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
    wakeup_events: u32,
    bp_type: u32,
    config1: u64,
    config2: u64,
}

#[cfg(target_os = "linux")]
fn perf_open(pid: u32, config: u64) -> Option<i32> {
    const TYPE_HARDWARE: u32 = 0;
    // inherit (bit 1), exclude_hv (bit 6).
    const FLAGS: u64 = (1 << 1) | (1 << 6);
    let mut attr = PerfEventAttr {
        type_: TYPE_HARDWARE,
        size: u32::try_from(std::mem::size_of::<PerfEventAttr>()).ok()?,
        config,
        sample_period: 0,
        sample_type: 0,
        read_format: 0,
        flags: FLAGS,
        wakeup_events: 0,
        bp_type: 0,
        config1: 0,
        config2: 0,
    };
    #[expect(
        clippy::cast_possible_wrap,
        reason = "perf_event_open takes a pid_t and Linux process ids fit"
    )]
    let raw_pid = pid as libc::pid_t;
    // SAFETY: `attr` is a valid `perf_event_attr` whose `size` is its own byte
    // length; the other arguments are integers. A negative return is an error
    // reported through errno.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_perf_event_open,
            &raw mut attr,
            raw_pid,
            -1_i32,
            -1_i32,
            0_u64,
        )
    };
    i32::try_from(fd).ok().filter(|fd| *fd >= 0)
}

#[cfg(target_os = "linux")]
fn observe_child(pid: u32) {
    for config in [1_u64, 0_u64] {
        if let Some(fd) = perf_open(pid, config) {
            locked(&PERF_FDS).push((pid, fd));
        }
    }
}

#[cfg(target_os = "linux")]
fn read_child(pid: u32) -> ChildCounters {
    let mut fds = Vec::new();
    {
        let mut observed = locked(&PERF_FDS);
        let mut index = 0;
        while index < observed.len() {
            if observed[index].0 == pid {
                fds.push(observed.swap_remove(index).1);
            } else {
                index += 1;
            }
        }
    }
    let mut instructions = None;
    let mut cycles = None;
    for fd in fds {
        let mut value = 0_u64;
        // SAFETY: `value` is a writable u64 and `read` writes at most its size.
        let count = unsafe {
            libc::read(
                fd,
                (&raw mut value).cast::<libc::c_void>(),
                std::mem::size_of::<u64>(),
            )
        };
        // SAFETY: the descriptor came from `perf_event_open` above and is
        // closed exactly once here.
        unsafe { libc::close(fd) };
        if count == isize::try_from(std::mem::size_of::<u64>()).unwrap_or(0) {
            // The first descriptor is instructions, the second cycles; the
            // order comes from `observe_child`.
            if instructions.is_none() {
                instructions = Some(value);
            } else {
                cycles = Some(value);
            }
        }
    }
    ChildCounters {
        instructions,
        cycles,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn observe_child(_pid: u32) {}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn read_child(_pid: u32) -> ChildCounters {
    ChildCounters::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv_recorder_keeps_every_argument() {
        let recorder = ArgvRecorder::default();
        recorder.record(
            &["wild".to_owned(), "-o".to_owned()],
            1.5,
            0,
            ChildCounters::default(),
        );
        let records = recorder.records();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].argv, ["wild", "-o"]);
        assert_eq!(records[0].ms, 1.5);
        assert_eq!(records[0].status, 0);
        assert!(recorder.log_summary());
    }

    #[test]
    fn summary_recorder_keeps_program_and_count() {
        let recorder = SummaryRecorder::default();
        recorder.record(
            &["wild".to_owned(), "-o".to_owned()],
            1.5,
            0,
            ChildCounters::default(),
        );
        let summaries = recorder.summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].program, "wild");
        assert_eq!(summaries[0].args, 1);
        assert_eq!(summaries[0].ms, 1.5);
        assert_eq!(summaries[0].status, 0);
        assert!(!recorder.log_summary());
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn child_counters_are_read_before_the_child_is_reaped() {
        let recorder = SummaryRecorder::default();
        let mut child = std::process::Command::new("/bin/sh")
            .args(["-c", "for i in $(seq 1 2000); do :; done"])
            .spawn()
            .expect("spawn a child");
        let pid = child.id();
        recorder.child_started(pid);
        wait_without_reaping(pid).expect("waitid WNOWAIT");
        let counters = recorder.child_counters(pid);
        assert!(child.wait().expect("reap").success());
        if cfg!(target_os = "macos") {
            assert!(
                counters.instructions.is_some(),
                "child instructions missing"
            );
        }
        // Linux may deny perf_event_open; both fields are then None, which is
        // the documented "unavailable" case rather than a zero.
    }

    #[test]
    fn poisoned_spawn_log_is_recovered() {
        let recorder = ArgvRecorder::default();
        let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = recorder.records.lock().expect("lock");
            panic!("poison the spawn log");
        }));
        assert!(poisoned.is_err());
        recorder.record(&["wild".to_owned()], 0.0, 0, ChildCounters::default());
        assert_eq!(recorder.records().len(), 1);
    }
}
