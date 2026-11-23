//! Idle detection and the counters/full measurement regimes.
//!
//! Wall-clock time is only a regression signal when no other process is
//! competing for the CPU. Before a run and around every timed window this
//! module measures how much CPU time every process outside this tool's own
//! process tree consumed, plus the load average. A window is idle-qualified
//! when that is below a threshold (default 10% of one core).
//!
//! The process tree is excluded because the benchmark itself is the only busy
//! process it is allowed to be: Wild children, Wasmtime compilation threads and
//! the guest all belong to this run. On Linux the same accounting is available
//! from `/proc/stat` minus this tree; on macOS `proc_listpids` plus
//! `proc_pid_rusage` gives per-process user+system time without root.

use crate::counters;
use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Default idle threshold: other processes may use at most 10% of one core.
pub const DEFAULT_IDLE_THRESHOLD_FRACTION: f64 = 0.10;
/// How long the startup probe watches before deciding auto mode.
pub const STARTUP_PROBE: Duration = Duration::from_millis(400);
/// How often auto mode re-probes while waiting for the machine to become idle.
pub const WAIT_POLL: Duration = Duration::from_millis(500);

/// The requested measurement regime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum BenchMode {
    /// Full mode when the machine is idle at start; counters mode when it is
    /// still busy after `--wait-idle` seconds.
    #[default]
    Auto,
    /// Skip the wall-clock timing loops; use exact and counter metrics only.
    Counters,
    /// Always time; mark every wall metric idle-qualified or not.
    Full,
}

impl BenchMode {
    /// The lowercase name used on the command line and in result.json.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Counters => "counters",
            Self::Full => "full",
        }
    }
}

/// The regime a run actually used.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Regime {
    Counters,
    /// Also the meaning of a schema-1 result, which had no regime field.
    #[default]
    Full,
}

impl Regime {
    /// The lowercase name used in result.json and the summary.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counters => "counters",
            Self::Full => "full",
        }
    }
}

/// One idle observation: the CPU time other processes used over a window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdleSample {
    /// The measurement this window covered, e.g. `warm` or `edit`.
    pub label: String,
    pub window_ms: f64,
    /// CPU time consumed by processes outside this tool's tree.
    pub other_cpu_ms: f64,
    /// `other_cpu_ms / window_ms`: fraction of one core.
    pub other_cpu_fraction: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_average: Option<f64>,
    pub qualified: bool,
    /// Other processes whose CPU time was read in both snapshots.
    pub observed_processes: usize,
    /// This tool's own processes, excluded from the sum.
    pub excluded_processes: usize,
    /// Processes the kernel refused to describe; their CPU time is missing.
    pub unreadable_processes: usize,
    pub method: String,
}

/// The evidence behind a run's or a program's idle qualification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdleEvidence {
    #[serde(default)]
    pub method: String,
    #[serde(default = "default_threshold")]
    pub threshold_fraction: f64,
    #[serde(default)]
    pub samples: Vec<IdleSample>,
}

fn default_threshold() -> f64 {
    DEFAULT_IDLE_THRESHOLD_FRACTION
}

impl Default for IdleEvidence {
    fn default() -> Self {
        Self {
            method: String::new(),
            threshold_fraction: DEFAULT_IDLE_THRESHOLD_FRACTION,
            samples: Vec::new(),
        }
    }
}

impl IdleEvidence {
    /// A run is idle-qualified only when it has evidence and every timed
    /// window stayed below the threshold. No evidence is not idle.
    #[must_use]
    pub fn qualified(&self) -> bool {
        !self.samples.is_empty() && self.samples.iter().all(|sample| sample.qualified)
    }

    /// The busiest window's fraction of one core, when there is evidence.
    #[must_use]
    pub fn worst_fraction(&self) -> Option<f64> {
        self.samples
            .iter()
            .map(|sample| sample.other_cpu_fraction)
            .max_by(f64::total_cmp)
    }
}

/// Classify one window from its measured fraction of one core.
///
/// This is the single rule the monitor and the unit tests use. A window with
/// no observed other process is not qualified: that means the accounting
/// failed, not that the machine was idle.
#[must_use]
pub fn classify_fraction(fraction: f64, threshold_fraction: f64) -> bool {
    fraction.is_finite() && fraction >= 0.0 && fraction < threshold_fraction
}

struct ProcessSnapshot {
    cpu_ns: BTreeMap<u32, u64>,
    excluded: usize,
    unreadable: usize,
}

/// A started timed window; pass it to [`IdleMonitor::finish`].
pub struct IdleWindow {
    label: String,
    started: Instant,
    before: ProcessSnapshot,
}

/// The process-wide idle monitor. Timed measurement groups call
/// [`IdleMonitor::window`] before the work and [`IdleMonitor::finish`] after.
pub struct IdleMonitor {
    threshold_fraction: f64,
    root_pid: u32,
    samples: Mutex<Vec<IdleSample>>,
}

impl IdleMonitor {
    /// Create a monitor for this process and its descendants.
    #[must_use]
    pub fn new(threshold_fraction: f64) -> Self {
        Self {
            threshold_fraction,
            root_pid: std::process::id(),
            samples: Mutex::new(Vec::new()),
        }
    }

    /// The threshold this monitor applies.
    #[must_use]
    pub fn threshold_fraction(&self) -> f64 {
        self.threshold_fraction
    }

    /// Start a timed window named `label`.
    #[must_use]
    pub fn window(&self, label: impl Into<String>) -> IdleWindow {
        IdleWindow {
            label: label.into(),
            started: Instant::now(),
            before: snapshot(self.root_pid),
        }
    }

    /// Finish a timed window, classify it, and keep the evidence.
    #[must_use]
    pub fn finish(&self, window: IdleWindow) -> IdleSample {
        let after = snapshot(self.root_pid);
        let elapsed = window.started.elapsed();
        let window_ms = elapsed.as_secs_f64() * 1e3;
        let elapsed_ns = elapsed.as_nanos();
        // Only processes present in both snapshots can be differenced. A
        // process that started or exited inside the window is not counted;
        // this undercounts rather than inventing time.
        let mut other_ns = 0_u64;
        let mut observed = 0_usize;
        for (pid, after_ns) in &after.cpu_ns {
            if let Some(before_ns) = window.before.cpu_ns.get(pid) {
                other_ns = other_ns.saturating_add(after_ns.saturating_sub(*before_ns));
                observed += 1;
            }
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "nanosecond and millisecond magnitudes are far below 2^53"
        )]
        let other_cpu_ms = other_ns as f64 / 1e6;
        #[expect(
            clippy::cast_precision_loss,
            reason = "nanosecond magnitudes are far below 2^53"
        )]
        let window_ns = elapsed_ns as f64;
        let fraction = if window_ns > 0.0 {
            #[expect(
                clippy::cast_precision_loss,
                reason = "nanosecond magnitudes are far below 2^53"
            )]
            let other_ns = other_ns as f64;
            other_ns / window_ns
        } else {
            0.0
        };
        let sample = IdleSample {
            label: window.label,
            window_ms,
            other_cpu_ms,
            other_cpu_fraction: fraction,
            load_average: util::load_average(),
            qualified: observed > 0 && classify_fraction(fraction, self.threshold_fraction),
            observed_processes: observed,
            excluded_processes: after.excluded,
            unreadable_processes: after.unreadable,
            method: counters::method().to_owned(),
        };
        lock(&self.samples).push(sample.clone());
        sample
    }

    /// Observe the machine for `duration` and return the sample.
    #[must_use]
    pub fn probe(&self, label: impl Into<String>, duration: Duration) -> IdleSample {
        let window = self.window(label);
        std::thread::sleep(duration);
        self.finish(window)
    }

    /// Number of samples recorded so far, for per-program slices.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        lock(&self.samples).len()
    }

    /// All evidence recorded so far.
    #[must_use]
    pub fn evidence(&self) -> IdleEvidence {
        self.evidence_since(0)
    }

    /// Evidence recorded at or after sample index `start`.
    #[must_use]
    pub fn evidence_since(&self, start: usize) -> IdleEvidence {
        IdleEvidence {
            method: counters::method().to_owned(),
            threshold_fraction: self.threshold_fraction,
            samples: lock(&self.samples).iter().skip(start).cloned().collect(),
        }
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn snapshot(root_pid: u32) -> ProcessSnapshot {
    let pids = counters::list_pids();
    let mut parents = BTreeMap::new();
    for pid in &pids {
        if let Some(parent) = counters::process_parent(*pid) {
            parents.insert(*pid, parent);
        }
    }
    let mut cpu_ns = BTreeMap::new();
    let mut excluded = 0_usize;
    let mut unreadable = 0_usize;
    for pid in pids {
        if in_own_tree(pid, root_pid, &parents) {
            excluded += 1;
            continue;
        }
        match counters::process_cpu_ns(pid) {
            Some(ns) => {
                cpu_ns.insert(pid, ns);
            }
            None => unreadable += 1,
        }
    }
    ProcessSnapshot {
        cpu_ns,
        excluded,
        unreadable,
    }
}

fn in_own_tree(pid: u32, root_pid: u32, parents: &BTreeMap<u32, u32>) -> bool {
    if pid == root_pid {
        return true;
    }
    let mut current = pid;
    // A process table is a forest; the cap turns a malformed cycle into "not
    // ours" instead of an infinite loop.
    for _ in 0..64 {
        match parents.get(&current) {
            Some(parent) if *parent == root_pid => return true,
            Some(parent) if *parent > 1 => current = *parent,
            _ => return false,
        }
    }
    false
}

/// Decide the regime from the requested mode and the startup probe.
///
/// Returns the regime and a one-line reason for the summary and result.json.
#[must_use]
pub fn decide(mode: BenchMode, monitor: &IdleMonitor, wait_idle_seconds: u64) -> (Regime, String) {
    match mode {
        BenchMode::Counters => (Regime::Counters, "requested --mode counters".to_owned()),
        BenchMode::Full => (Regime::Full, "requested --mode full".to_owned()),
        BenchMode::Auto => {
            let first = monitor.probe("startup", STARTUP_PROBE);
            if first.qualified {
                return (
                    Regime::Full,
                    format!(
                        "idle at start: other processes used {:.1}% of one core (threshold {:.1}%)",
                        first.other_cpu_fraction * 100.0,
                        monitor.threshold_fraction() * 100.0
                    ),
                );
            }
            if first.observed_processes == 0 {
                return (
                    Regime::Counters,
                    "idle detection unavailable; using counters".to_owned(),
                );
            }
            let deadline = Instant::now() + Duration::from_secs(wait_idle_seconds);
            let mut last = first;
            while wait_idle_seconds > 0 && Instant::now() < deadline {
                let remaining = deadline.saturating_duration_since(Instant::now());
                let duration = WAIT_POLL.min(remaining);
                if duration.is_zero() {
                    break;
                }
                last = monitor.probe("wait-idle", duration);
                if last.qualified {
                    return (
                        Regime::Full,
                        format!(
                            "idle after waiting: other processes used {:.1}% of one core",
                            last.other_cpu_fraction * 100.0
                        ),
                    );
                }
            }
            (
                Regime::Counters,
                format!(
                    "busy at start: other processes used {:.1}% of one core (threshold {:.1}%, \
                     load {}); using counters",
                    last.other_cpu_fraction * 100.0,
                    monitor.threshold_fraction() * 100.0,
                    last.load_average
                        .map_or_else(|| "unknown".to_owned(), |load| format!("{load:.2}"))
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(fraction: f64) -> IdleSample {
        IdleSample {
            label: "fixture".to_owned(),
            window_ms: 100.0,
            other_cpu_ms: fraction * 100.0,
            other_cpu_fraction: fraction,
            load_average: Some(1.0),
            qualified: classify_fraction(fraction, DEFAULT_IDLE_THRESHOLD_FRACTION),
            observed_processes: 10,
            excluded_processes: 1,
            unreadable_processes: 0,
            method: counters::method().to_owned(),
        }
    }

    #[test]
    fn idle_classification_uses_a_strict_threshold() {
        assert!(classify_fraction(0.0, 0.10));
        assert!(classify_fraction(0.099, 0.10));
        assert!(!classify_fraction(0.10, 0.10));
        assert!(!classify_fraction(0.25, 0.10));
        assert!(!classify_fraction(f64::NAN, 0.10));
        assert!(!classify_fraction(-1.0, 0.10));
    }

    #[test]
    fn evidence_requires_at_least_one_qualified_window() {
        let mut evidence = IdleEvidence {
            method: counters::method().to_owned(),
            threshold_fraction: 0.10,
            samples: Vec::new(),
        };
        assert!(!evidence.qualified(), "no evidence must not be idle");
        evidence.samples.push(sample(0.02));
        assert!(evidence.qualified());
        evidence.samples.push(sample(0.50));
        assert!(!evidence.qualified());
        assert_eq!(evidence.worst_fraction(), Some(0.50));
    }

    #[test]
    fn fixture_samples_below_the_threshold_are_idle() {
        let fixtures = [0.001, 0.02, 0.099];
        for fraction in fixtures {
            assert!(
                sample(fraction).qualified,
                "fraction {fraction} should be idle"
            );
        }
        for fraction in [0.10, 0.101, 0.5, 3.0] {
            assert!(
                !sample(fraction).qualified,
                "fraction {fraction} should be busy"
            );
        }
    }

    #[test]
    fn own_process_tree_is_excluded() {
        let monitor = IdleMonitor::new(DEFAULT_IDLE_THRESHOLD_FRACTION);
        let window = monitor.window("fixture");
        std::thread::sleep(Duration::from_millis(5));
        let sample = monitor.finish(window);
        assert!(
            sample.excluded_processes >= 1,
            "this process was not excluded"
        );
        assert!(
            sample.observed_processes > 0,
            "no other process was readable"
        );
    }

    #[test]
    fn evidence_since_slices_per_program_windows() {
        let monitor = IdleMonitor::new(DEFAULT_IDLE_THRESHOLD_FRACTION);
        let first = monitor.window("first");
        let first = monitor.finish(first);
        let start = monitor.sample_count();
        let second = monitor.window("second");
        let second = monitor.finish(second);
        assert_eq!(first.label, "first");
        assert_eq!(second.label, "second");
        let slice = monitor.evidence_since(start);
        assert_eq!(slice.samples.len(), 1);
        assert_eq!(slice.samples[0].label, "second");
    }

    #[test]
    fn auto_mode_honours_an_explicit_choice() {
        let monitor = IdleMonitor::new(DEFAULT_IDLE_THRESHOLD_FRACTION);
        assert_eq!(decide(BenchMode::Counters, &monitor, 0).0, Regime::Counters);
        assert_eq!(decide(BenchMode::Full, &monitor, 0).0, Regime::Full);
    }
}
