//! Native CPU counters for steps outside the wasm interpreter.
//!
//! The benchmark uses these counters to decide regressions on a busy machine,
//! where wall-clock time is dominated by other processes. A counter is never
//! invented: a platform or process the kernel refuses to describe produces
//! [`CounterReading::default`] (both fields `None`) and the report says
//! "unavailable".
//!
//! - macOS: `proc_pid_rusage` with `RUSAGE_INFO_V4`, which reports
//!   `ri_instructions` and `ri_cycles` without root. The same call also gives
//!   the user+system time the idle detector uses.
//! - Linux: `perf_event_open(PERF_COUNT_HW_INSTRUCTIONS | PERF_COUNT_HW_CPU_CYCLES)`
//!   with `inherit = 1`, read through the returned file descriptor.
//!
//! Counters are read as a delta around a step. Child processes (the Wild
//! linker) are counted by `cogwasm-runtime`, which owns the spawn and reads
//! their counters after `waitid(WNOWAIT)` but before `waitpid`.

use serde::{Deserialize, Serialize};

/// A pair of native CPU counters. `None` means the platform or kernel refused
/// to report that counter; it is not zero.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CounterReading {
    pub instructions: Option<u64>,
    pub cycles: Option<u64>,
}

impl CounterReading {
    /// The work done between `self` (taken earlier) and `later`.
    #[must_use]
    pub fn delta(self, later: Self) -> Self {
        Self {
            instructions: subtract(self.instructions, later.instructions),
            cycles: subtract(self.cycles, later.cycles),
        }
    }

    /// True when at least one counter was reported.
    #[must_use]
    pub fn is_available(self) -> bool {
        self.instructions.is_some() || self.cycles.is_some()
    }

    /// Sum of several readings; a counter is available only when every reading
    /// has it.
    #[must_use]
    pub fn sum(readings: &[Self]) -> Self {
        let instructions = if readings.iter().all(|r| r.instructions.is_some()) {
            Some(readings.iter().filter_map(|r| r.instructions).sum())
        } else {
            None
        };
        let cycles = if readings.iter().all(|r| r.cycles.is_some()) {
            Some(readings.iter().filter_map(|r| r.cycles).sum())
        } else {
            None
        };
        Self {
            instructions,
            cycles,
        }
    }
}

fn subtract(before: Option<u64>, after: Option<u64>) -> Option<u64> {
    after?.checked_sub(before?)
}

/// The method this platform uses, for result.json and reports.
#[must_use]
pub const fn method() -> &'static str {
    if cfg!(target_os = "macos") {
        "proc_pid_rusage(RUSAGE_INFO_V4)"
    } else if cfg!(target_os = "linux") {
        "perf_event_open(PERF_COUNT_HW_INSTRUCTIONS)"
    } else {
        "unavailable"
    }
}

/// A per-step counter. Start it before the step and [`ProcessCounter::finish`]
/// after it; the difference is the step's CPU work.
#[derive(Debug)]
pub struct ProcessCounter {
    start: CounterReading,
    #[cfg(target_os = "linux")]
    fd_instructions: Option<i32>,
    #[cfg(target_os = "linux")]
    fd_cycles: Option<i32>,
}

impl ProcessCounter {
    /// Start counting this process (including its children on Linux).
    #[must_use]
    pub fn start() -> Self {
        Self {
            start: sample_self(),
            #[cfg(target_os = "linux")]
            fd_instructions: perf_open(0, perf_config::INSTRUCTIONS),
            #[cfg(target_os = "linux")]
            fd_cycles: perf_open(0, perf_config::CYCLES),
        }
    }

    /// Finish counting and return the delta since [`ProcessCounter::start`].
    #[must_use]
    pub fn finish(self) -> CounterReading {
        #[cfg(target_os = "linux")]
        {
            // A working perf descriptor counts from `start`, so its value is
            // already the delta. Without one, fall back to the `/proc`-free
            // lifetime reading (which will usually be unavailable too).
            let reading = if self.fd_instructions.is_some() || self.fd_cycles.is_some() {
                CounterReading {
                    instructions: self.fd_instructions.and_then(read_perf),
                    cycles: self.fd_cycles.and_then(read_perf),
                }
            } else {
                self.start.delta(sample_self())
            };
            close_fd(self.fd_instructions);
            close_fd(self.fd_cycles);
            return reading;
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.start.delta(sample_self())
        }
    }
}

/// A single lifetime reading for this process. Used for the overall per-build
/// counter, which is a start/finish pair.
#[must_use]
pub fn sample_self() -> CounterReading {
    #[cfg(target_os = "macos")]
    {
        sample_pid(
            i32::try_from(std::process::id()).expect("the process id fits in pid_t on macOS"),
        )
    }
    #[cfg(target_os = "linux")]
    {
        let fd_instructions = perf_open(0, perf_config::INSTRUCTIONS);
        let fd_cycles = perf_open(0, perf_config::CYCLES);
        let reading = CounterReading {
            instructions: fd_instructions.and_then(read_perf),
            cycles: fd_cycles.and_then(read_perf),
        };
        close_fd(fd_instructions);
        close_fd(fd_cycles);
        reading
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        CounterReading::default()
    }
}

#[cfg(target_os = "macos")]
fn rusage_info(pid: libc::pid_t) -> Option<libc::rusage_info_v4> {
    let mut info = std::mem::MaybeUninit::<libc::rusage_info_v4>::zeroed();
    // SAFETY: `info` is a writable, correctly sized `rusage_info_v4` buffer.
    // `proc_pid_rusage` writes at most `sizeof(rusage_info_v4)` bytes for
    // `RUSAGE_INFO_V4` and does not retain the pointer. Reading a process the
    // caller does not own fails with a negative status and is reported as an
    // unavailable counter instead of being trusted.
    let status = unsafe {
        libc::proc_pid_rusage(
            pid,
            libc::RUSAGE_INFO_V4,
            info.as_mut_ptr().cast::<libc::rusage_info_t>(),
        )
    };
    if status != 0 {
        return None;
    }
    // SAFETY: a zero status means `proc_pid_rusage` initialised the buffer.
    Some(unsafe { info.assume_init() })
}

#[cfg(target_os = "macos")]
fn sample_pid(pid: libc::pid_t) -> CounterReading {
    let Some(info) = rusage_info(pid) else {
        return CounterReading::default();
    };
    CounterReading {
        instructions: Some(info.ri_instructions),
        cycles: Some(info.ri_cycles),
    }
}

/// Linux `perf_event_attr`, truncated to the fields every supported kernel
/// accepts (size 72 is `PERF_ATTR_SIZE_VER2`).
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
mod perf_config {
    pub const INSTRUCTIONS: u64 = 1; // PERF_COUNT_HW_INSTRUCTIONS
    pub const CYCLES: u64 = 0; // PERF_COUNT_HW_CPU_CYCLES
    pub const TYPE_HARDWARE: u32 = 0;
    /// inherit (bit 1) so children are included, and exclude_hv (bit 6).
    pub const FLAGS: u64 = (1 << 1) | (1 << 6);
}

#[cfg(target_os = "linux")]
fn perf_open(pid: u32, config: u64) -> Option<i32> {
    let mut attr = PerfEventAttr {
        type_: perf_config::TYPE_HARDWARE,
        size: u32::try_from(std::mem::size_of::<PerfEventAttr>()).ok()?,
        config,
        sample_period: 0,
        sample_type: 0,
        read_format: 0,
        flags: perf_config::FLAGS,
        wakeup_events: 0,
        bp_type: 0,
        config1: 0,
        config2: 0,
    };
    #[expect(
        clippy::cast_possible_wrap,
        reason = "perf_event_open takes a pid_t and Rust process ids fit on Linux"
    )]
    let pid = pid as libc::pid_t;
    // SAFETY: `attr` is a valid `perf_event_attr` whose `size` is its own
    // byte length. The remaining arguments are plain integers. A negative
    // return is an error the kernel reports through errno; the descriptor is
    // closed by `close_fd`.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_perf_event_open,
            &raw mut attr,
            pid,
            -1_i32,
            -1_i32,
            0_u64,
        )
    };
    i32::try_from(fd).ok().filter(|fd| *fd >= 0)
}

#[cfg(target_os = "linux")]
fn read_perf(fd: i32) -> Option<u64> {
    let mut value = 0_u64;
    // SAFETY: `value` is a writable `u64`; `read` writes at most eight bytes.
    let count = unsafe {
        libc::read(
            fd,
            (&raw mut value).cast::<libc::c_void>(),
            std::mem::size_of::<u64>(),
        )
    };
    (count == i64::try_from(std::mem::size_of::<u64>()).ok()?).then_some(value)
}

#[cfg(target_os = "linux")]
fn close_fd(fd: Option<i32>) {
    if let Some(fd) = fd {
        // SAFETY: the descriptor was returned by `perf_event_open` above and is
        // closed exactly once.
        let _ = unsafe { libc::close(fd) };
    }
}

/// Every process id the OS will list. The idle detector compares consecutive
/// snapshots of these processes.
#[must_use]
pub fn list_pids() -> Vec<u32> {
    #[cfg(target_os = "macos")]
    {
        // PROC_ALL_PIDS is from <sys/proc_info.h>; libc does not export it.
        const PROC_ALL_PIDS: u32 = 1;
        // SAFETY: a null buffer with size zero asks `proc_listpids` only for
        // the required byte count; it writes nothing.
        let bytes = unsafe { libc::proc_listpids(PROC_ALL_PIDS, 0, std::ptr::null_mut(), 0) };
        if bytes <= 0 {
            return Vec::new();
        }
        #[expect(clippy::cast_sign_loss, reason = "the byte count was checked positive")]
        let capacity = bytes as usize / std::mem::size_of::<libc::pid_t>();
        let mut pids = vec![0_i32; capacity];
        // SAFETY: `pids` is a writable buffer of `capacity` pid_t values, and
        // the byte length passed is exactly its size, so `proc_listpids`
        // cannot write out of bounds. It does not retain the pointer.
        let written = unsafe {
            libc::proc_listpids(
                PROC_ALL_PIDS,
                0,
                pids.as_mut_ptr().cast::<libc::c_void>(),
                bytes,
            )
        };
        if written <= 0 {
            return Vec::new();
        }
        let count = usize::try_from(written)
            .unwrap_or(0)
            .checked_div(std::mem::size_of::<libc::pid_t>())
            .unwrap_or(0)
            .min(pids.len());
        pids.truncate(count);
        pids.into_iter()
            .filter(|pid| *pid > 0)
            .filter_map(|pid| u32::try_from(pid).ok())
            .collect()
    }
    #[cfg(target_os = "linux")]
    {
        let Ok(entries) = std::fs::read_dir("/proc") else {
            return Vec::new();
        };
        entries
            .filter_map(Result::ok)
            .filter_map(|entry| entry.file_name().to_str()?.parse::<u32>().ok())
            .collect()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Vec::new()
    }
}

/// The parent process id, when the platform reports it.
#[must_use]
pub fn process_parent(pid: u32) -> Option<u32> {
    #[cfg(target_os = "macos")]
    {
        let mut info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::zeroed();
        let size = std::mem::size_of::<libc::proc_bsdinfo>();
        let raw_pid = pid as libc::pid_t;
        // SAFETY: `info` is a writable `proc_bsdinfo` buffer and `size` is its
        // exact byte length. `proc_pidinfo` writes at most `size` bytes and
        // does not retain the pointer.
        let written = unsafe {
            libc::proc_pidinfo(
                raw_pid,
                libc::PROC_PIDTBSDINFO,
                0,
                info.as_mut_ptr().cast::<libc::c_void>(),
                i32::try_from(size).ok()?,
            )
        };
        if written != i32::try_from(size).ok()? {
            return None;
        }
        // SAFETY: the exact expected byte count was written.
        let info = unsafe { info.assume_init() };
        (info.pbi_ppid > 0).then_some(info.pbi_ppid)
    }
    #[cfg(target_os = "linux")]
    {
        let text = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
        parse_proc_pid_stat(&text).map(|stat| stat.ppid)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = pid;
        None
    }
}

/// User+system CPU time this process has consumed, in nanoseconds.
#[must_use]
pub fn process_cpu_ns(pid: u32) -> Option<u64> {
    #[cfg(target_os = "macos")]
    {
        let raw_pid = pid as libc::pid_t;
        let info = rusage_info(raw_pid)?;
        Some(info.ri_user_time.saturating_add(info.ri_system_time))
    }
    #[cfg(target_os = "linux")]
    {
        let text = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
        let stat = parse_proc_pid_stat(&text)?;
        let ticks = stat.utime.saturating_add(stat.stime);
        let ticks_per_second = ticks_per_second()?;
        ticks
            .checked_mul(1_000_000_000)?
            .checked_div(ticks_per_second)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = pid;
        None
    }
}

/// The fields the idle detector needs from `/proc/<pid>/stat`.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcPidStat {
    pub ppid: u32,
    /// User-mode CPU time in clock ticks.
    pub utime: u64,
    /// Kernel-mode CPU time in clock ticks.
    pub stime: u64,
}

/// Parse `/proc/<pid>/stat`. The command name is in parentheses and may itself
/// contain spaces or parentheses, so the fixed fields start after the last
/// `)`.
#[cfg(target_os = "linux")]
#[must_use]
pub fn parse_proc_pid_stat(text: &str) -> Option<ProcPidStat> {
    let (_, rest) = text.rsplit_once(')')?;
    let fields: Vec<&str> = rest.split_whitespace().collect();
    // After `)`: state is field 3, ppid field 4, utime field 14, stime 15.
    Some(ProcPidStat {
        ppid: fields.get(1)?.parse().ok()?,
        utime: fields.get(11)?.parse().ok()?,
        stime: fields.get(12)?.parse().ok()?,
    })
}

#[cfg(target_os = "linux")]
fn ticks_per_second() -> Option<u64> {
    // SAFETY: `sysconf` has no pointer arguments and only reads the named
    // configuration value.
    let ticks = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    u64::try_from(ticks).ok().filter(|ticks| *ticks > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_subtracts_only_when_both_sides_are_present() {
        let before = CounterReading {
            instructions: Some(10),
            cycles: None,
        };
        let after = CounterReading {
            instructions: Some(17),
            cycles: Some(3),
        };
        assert_eq!(
            before.delta(after),
            CounterReading {
                instructions: Some(7),
                cycles: None,
            }
        );
    }

    #[test]
    fn sum_is_available_only_when_every_part_is() {
        let parts = [
            CounterReading {
                instructions: Some(2),
                cycles: Some(5),
            },
            CounterReading {
                instructions: Some(3),
                cycles: None,
            },
        ];
        let sum = CounterReading::sum(&parts);
        assert_eq!(sum.instructions, Some(5));
        assert_eq!(sum.cycles, None);
    }

    #[test]
    fn method_is_named_on_supported_platforms() {
        if cfg!(any(target_os = "macos", target_os = "linux")) {
            assert_ne!(method(), "unavailable");
        }
    }

    /// Exercises the `proc_pid_rusage` / `perf_event_open` FFI in
    /// `ProcessCounter::start`/`finish`.
    #[test]
    fn process_counter_measures_work_in_this_process() {
        let counter = ProcessCounter::start();
        let mut sum = 0_u64;
        for value in 0..2_000_000_u64 {
            sum = sum.wrapping_add(value.rotate_left(7));
        }
        assert_ne!(sum, u64::MAX);
        let reading = counter.finish();
        if cfg!(target_os = "macos") {
            let instructions = reading.instructions.expect("macOS reports instructions");
            assert!(instructions > 0, "instructions did not increase");
        }
        // A sandboxed Linux kernel may refuse perf_event_open; then both fields
        // are None, which is the documented "unavailable" case.
        assert!(reading.is_available() || cfg!(target_os = "linux"));
    }

    /// Exercises the `proc_listpids` / `/proc` enumeration and the CPU-time
    /// path (`proc_pid_rusage` or `/proc/<pid>/stat`).
    #[test]
    fn process_listing_contains_this_process_and_cpu_time_is_monotonic() {
        let pids = list_pids();
        assert!(pids.contains(&std::process::id()));
        let before = process_cpu_ns(std::process::id());
        let mut sum = 0_u64;
        for value in 0..500_000_u64 {
            sum = sum.wrapping_add(value.rotate_left(3));
        }
        assert_ne!(sum, u64::MAX);
        let after = process_cpu_ns(std::process::id());
        if let (Some(before), Some(after)) = (before, after) {
            assert!(after >= before);
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parses_proc_pid_stat_after_a_command_name_with_spaces() {
        let text = "4242 (a (weird) name) S 1 4242 4242 0 -1 4194560 100 0 0 0 12 34 0 0 20 0 1 0 99 1000 200 0";
        let stat = parse_proc_pid_stat(text).expect("parse");
        assert_eq!(stat.ppid, 1);
        assert_eq!(stat.utime, 12);
        assert_eq!(stat.stime, 34);
    }
}
