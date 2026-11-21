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
}

/// Policy hook called after every completed spawn.
pub trait SpawnRecorder: Send + 'static {
    /// Record one invocation. `argv` is the complete argument vector,
    /// including the program name the guest supplied first.
    fn record(&self, argv: &[String], ms: f64, status: i32);

    /// Whether [`add_playground_spawn`] should print the one-line per-spawn
    /// summary. The product host does; the benchmark host keeps its stderr
    /// quiet so timings and captured compiler output are unchanged.
    fn log_summary(&self) -> bool {
        false
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
    fn record(&self, argv: &[String], ms: f64, status: i32) {
        locked(&self.records).push(ArgvRecord {
            argv: argv.to_vec(),
            ms,
            status,
        });
    }

    fn log_summary(&self) -> bool {
        true
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
    fn record(&self, argv: &[String], ms: f64, status: i32) {
        locked(&self.summaries).push(SpawnSummary {
            program: argv.first().cloned().unwrap_or_default(),
            args: argv.len().saturating_sub(1),
            ms,
            status,
        });
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
    let status = Command::new(linker)
        .args(args.iter().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    let ms = started.elapsed().as_secs_f64() * 1e3;
    let code = match status {
        Ok(status) if status.success() => 0,
        Ok(status) => status.code().unwrap_or(-1),
        Err(error) => {
            eprintln!("{log_prefix} cannot run {}: {error}", linker.display());
            -1
        }
    };
    let recorder = caller.data().recorder();
    if recorder.log_summary() {
        eprintln!(
            "{log_prefix} spawn: {} ({} args) -> {code} in {ms:.1} ms",
            args.first().map_or("<empty>", String::as_str),
            args.len().saturating_sub(1)
        );
    }
    recorder.record(&args, ms, code);
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv_recorder_keeps_every_argument() {
        let recorder = ArgvRecorder::default();
        recorder.record(&["wild".to_owned(), "-o".to_owned()], 1.5, 0);
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
        recorder.record(&["wild".to_owned(), "-o".to_owned()], 1.5, 0);
        let summaries = recorder.summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].program, "wild");
        assert_eq!(summaries[0].args, 1);
        assert_eq!(summaries[0].ms, 1.5);
        assert_eq!(summaries[0].status, 0);
        assert!(!recorder.log_summary());
    }

    #[test]
    fn poisoned_spawn_log_is_recovered() {
        let recorder = ArgvRecorder::default();
        let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = recorder.records.lock().expect("lock");
            panic!("poison the spawn log");
        }));
        assert!(poisoned.is_err());
        recorder.record(&["wild".to_owned()], 0.0, 0);
        assert_eq!(recorder.records().len(), 1);
    }
}
