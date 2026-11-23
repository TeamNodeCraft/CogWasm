//! Class-aware comparison of two result documents.
//!
//! Three metric classes have three rules:
//!
//! - exact (fuel, executed wasm instructions, bytes): compare the medians
//!   exactly; any difference is a change.
//! - counter (native instructions retired, cycles): compare with a small
//!   relative tolerance, because the OS and the compiler emit a few
//!   instructions more or less between builds.
//! - wall (milliseconds): compare with the run-to-run spread rule, and only
//!   when both runs are idle-qualified. A busy run is reported as "not
//!   comparable: busy run" and can never be a regression.

use crate::result::BenchResult;
use crate::stats::{Metric, MetricClass};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Default relative tolerance for [`MetricClass::Counter`] metrics.
///
/// Two `--mode counters` quick runs on a busy machine spread native counter
/// values by up to 3.5% (the Wild link is the noisiest step), so the default
/// is 5%. The provisional 0.5% was far inside the measured spread and would
/// have reported run-to-run noise as regressions. `--counter-tolerance`
/// overrides it for a quieter machine.
pub const DEFAULT_COUNTER_TOLERANCE: f64 = 0.05;

/// One metric's comparison outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Unchanged,
    Changed,
    Regression,
    NotComparable,
}

/// One comparable metric from a result document.
#[derive(Debug, Clone)]
struct Point {
    metric: Metric,
    lower_is_better: bool,
}

/// A single row of the comparison.
#[derive(Debug, Clone)]
pub struct ComparisonRow {
    pub metric: String,
    pub baseline: Metric,
    pub current: Metric,
    pub change: Option<f64>,
    pub status: Status,
    /// Why the row could not be compared, or any other caveat.
    pub note: Option<String>,
}

impl ComparisonRow {
    /// The metric class, or `None` when either side is unavailable.
    #[must_use]
    pub fn class(&self) -> Option<MetricClass> {
        (self.baseline.class == self.current.class).then_some(self.baseline.class)
    }
}

/// The whole comparison.
#[derive(Debug, Clone)]
pub struct Comparison {
    pub rows: Vec<ComparisonRow>,
    pub regressions: Vec<ComparisonRow>,
    pub changed: Vec<ComparisonRow>,
    pub not_comparable: Vec<ComparisonRow>,
    pub unchanged: usize,
}

impl Comparison {
    #[must_use]
    pub fn has_regressions(&self) -> bool {
        !self.regressions.is_empty()
    }
}

/// Compare two results.
#[must_use]
pub fn compare_with_tolerance(
    baseline: &BenchResult,
    current: &BenchResult,
    counter_tolerance: f64,
) -> Comparison {
    let before = collect(baseline);
    let after = collect(current);
    let mut rows = Vec::new();
    for (path, old) in &before {
        let Some(new) = after.get(path) else {
            continue;
        };
        let (status, note) = classify(old, new, counter_tolerance);
        let change = if old.metric.median == 0.0 {
            None
        } else {
            Some((new.metric.median - old.metric.median) / old.metric.median)
        };
        rows.push(ComparisonRow {
            metric: path.clone(),
            baseline: old.metric.clone(),
            current: new.metric.clone(),
            change,
            status,
            note,
        });
    }
    rows.sort_by(|left, right| left.metric.cmp(&right.metric));
    let filter = |status: Status| {
        rows.iter()
            .filter(|row| row.status == status)
            .cloned()
            .collect::<Vec<_>>()
    };
    let unchanged = rows
        .iter()
        .filter(|row| row.status == Status::Unchanged)
        .count();
    Comparison {
        regressions: filter(Status::Regression),
        changed: filter(Status::Changed),
        not_comparable: filter(Status::NotComparable),
        rows,
        unchanged,
    }
}

/// Relative change below which two single-sample wall-clock or memory
/// readings are treated as the same.
const SINGLE_SAMPLE_NOISE: f64 = 0.05;

fn classify(old: &Point, new: &Point, counter_tolerance: f64) -> (Status, Option<String>) {
    if !old.metric.comparable_with(&new.metric) {
        let reason = old
            .metric
            .unavailable
            .as_deref()
            .or(new.metric.unavailable.as_deref())
            .unwrap_or("not comparable: busy run");
        return (Status::NotComparable, Some(reason.to_owned()));
    }
    match old.metric.class {
        MetricClass::Exact => {
            let different =
                old.metric.median.total_cmp(&new.metric.median) != std::cmp::Ordering::Equal;
            verdict(different, old, new, None)
        }
        MetricClass::Counter => {
            // Use the minimum of N repeats: parallel Wasmtime compilation
            // spin-waits differently under load, so the median of a swap
            // compile can move by tens of percent while its cheapest compile
            // stays within a couple of percent. The minimum is the stable
            // lower bound on the work.
            let scale = old
                .metric
                .min
                .abs()
                .max(new.metric.min.abs())
                .max(f64::MIN_POSITIVE);
            let relative = (new.metric.min - old.metric.min).abs() / scale;
            let different = relative > counter_tolerance;
            verdict(
                different,
                old,
                new,
                Some(format!(
                    "min-of-N, ±{:.2}% tolerance",
                    counter_tolerance * 100.0
                )),
            )
        }
        MetricClass::Wall => {
            let different = if old.metric.runs < 2 && new.metric.runs < 2 {
                // A derived geomean, p50 or host RSS has no spread to compare:
                // only a change beyond the noise floor counts.
                let scale = old
                    .metric
                    .median
                    .abs()
                    .max(new.metric.median.abs())
                    .max(f64::MIN_POSITIVE);
                (new.metric.median - old.metric.median).abs() / scale > SINGLE_SAMPLE_NOISE
            } else {
                new.metric.outside_spread_of(&old.metric)
                    || old.metric.outside_spread_of(&new.metric)
            };
            verdict(different, old, new, None)
        }
    }
}

fn verdict(
    different: bool,
    old: &Point,
    new: &Point,
    note: Option<String>,
) -> (Status, Option<String>) {
    if !different {
        return (Status::Unchanged, note);
    }
    if old.lower_is_better && new.metric.median > old.metric.median {
        (Status::Regression, note)
    } else {
        (Status::Changed, note)
    }
}

#[expect(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    reason = "the metric walk is one flat list; byte counts are far below 2^53"
)]
fn collect(result: &BenchResult) -> BTreeMap<String, Point> {
    let run_idle = result.idle.qualified();
    let mut points = BTreeMap::new();
    insert(
        &mut points,
        "toolchain.rustc_wasm_bytes",
        one(result.toolchain.rustc_wasm_bytes as f64, MetricClass::Exact),
        true,
    );
    insert(
        &mut points,
        "toolchain.rustc_wasm_stripped_bytes",
        one(
            result.toolchain.rustc_wasm_stripped_bytes as f64,
            MetricClass::Exact,
        ),
        true,
    );
    insert(
        &mut points,
        "toolchain.rustc_wasm_gzip_bytes",
        one(
            result.toolchain.rustc_wasm_gzip_bytes as f64,
            MetricClass::Exact,
        ),
        true,
    );
    insert(
        &mut points,
        "toolchain.sysroot_bytes",
        one(result.toolchain.sysroot.bytes as f64, MetricClass::Exact),
        true,
    );
    for (config, set) in &result.geomeans.by_config {
        if let Some(value) = set.exec_ratio {
            insert(
                &mut points,
                &format!("geomeans.{config}.exec_ratio"),
                derived_wall(value, run_idle),
                true,
            );
        }
        if let Some(value) = set.instructions_ratio {
            insert(
                &mut points,
                &format!("geomeans.{config}.instructions_ratio"),
                one(value, MetricClass::Exact),
                true,
            );
        }
        if let Some(value) = set.size_ratio_o2 {
            insert(
                &mut points,
                &format!("geomeans.{config}.size_ratio_o2"),
                one(value, MetricClass::Exact),
                true,
            );
        }
        if let Some(value) = set.size_ratio_oz {
            insert(
                &mut points,
                &format!("geomeans.{config}.size_ratio_oz"),
                one(value, MetricClass::Exact),
                true,
            );
        }
        if let Some(value) = set.warm_compile_median_ms {
            insert(
                &mut points,
                &format!("geomeans.{config}.warm_compile_ms"),
                derived_wall(value, run_idle),
                true,
            );
        }
        if let Some(value) = set.edit_p50_median_ms {
            insert(
                &mut points,
                &format!("geomeans.{config}.edit_p50_ms"),
                derived_wall(value, run_idle),
                true,
            );
        }
    }
    for (name, program) in &result.programs {
        let prefix = format!("programs.{name}");
        insert_metric(
            &mut points,
            &format!("{prefix}.llvm.o0.compile_ms"),
            &program.llvm.o0.compile_ms,
            true,
        );
        insert_metric(
            &mut points,
            &format!("{prefix}.llvm.o2.compile_ms"),
            &program.llvm.o2.compile_ms,
            true,
        );
        insert_metric(
            &mut points,
            &format!("{prefix}.llvm.oz.compile_ms"),
            &program.llvm.oz.compile_ms,
            true,
        );
        insert_size(
            &mut points,
            &format!("{prefix}.llvm.o0.size"),
            program.llvm.o0.size.total_bytes,
            true,
        );
        insert_size(
            &mut points,
            &format!("{prefix}.llvm.o2.size"),
            program.llvm.o2.size.total_bytes,
            true,
        );
        insert_size(
            &mut points,
            &format!("{prefix}.llvm.oz.size"),
            program.llvm.oz.size.total_bytes,
            true,
        );
        for (level, measurement) in &program.llvm.exec {
            insert_metric(
                &mut points,
                &format!("{prefix}.llvm.{level}.exec_ms"),
                &measurement.exec_ms,
                true,
            );
        }
        for (level, measurement) in &program.llvm.instructions {
            insert_metric(
                &mut points,
                &format!("{prefix}.llvm.{level}.instructions"),
                &measurement.instructions,
                true,
            );
        }
        for (config, measurement) in &program.configs {
            let base = format!("{prefix}.configs.{config}");
            for (label, metric) in [
                ("cold_compile_ms", &measurement.build.cold_compile_ms),
                ("cold_wall_ms", &measurement.build.cold_wall_ms),
                ("warm_compile_ms", &measurement.build.warm_compile_ms),
                ("warm_wall_ms", &measurement.build.warm_wall_ms),
                (
                    "peak_guest_memory_bytes",
                    &measurement.build.peak_guest_memory_bytes,
                ),
            ] {
                insert_metric(&mut points, &format!("{base}.build.{label}"), metric, true);
            }
            if let Some(metric) = &measurement.build.host_rss_peak_bytes {
                insert_metric(
                    &mut points,
                    &format!("{base}.build.host_rss_peak_bytes"),
                    metric,
                    true,
                );
            }
            insert_metric(
                &mut points,
                &format!("{base}.build.edit.p50_ms"),
                &derived_wall(measurement.build.edit.p50_ms, run_idle),
                true,
            );
            insert_metric(
                &mut points,
                &format!("{base}.build.edit.p90_ms"),
                &derived_wall(measurement.build.edit.p90_ms, run_idle),
                true,
            );
            for (label, metric) in [
                ("build_ms", &measurement.build.edit.build_ms),
                ("link_ms", &measurement.build.edit.link_ms),
                ("build_no_link_ms", &measurement.build.edit.build_no_link_ms),
                ("swap_ms", &measurement.build.edit.swap_ms),
                ("call_ms", &measurement.build.edit.call_ms),
            ] {
                insert_metric(
                    &mut points,
                    &format!("{base}.build.edit.{label}"),
                    metric,
                    true,
                );
            }
            for (label, metric) in [
                ("front_end", &measurement.build.phases.front_end),
                ("expansion", &measurement.build.phases.expansion),
                (
                    "type_borrow_check",
                    &measurement.build.phases.type_borrow_check,
                ),
                (
                    "monomorphisation",
                    &measurement.build.phases.monomorphisation,
                ),
                ("codegen", &measurement.build.phases.codegen),
                ("incremental", &measurement.build.phases.incremental),
                ("link", &measurement.build.phases.link),
                ("total", &measurement.build.phases.total),
            ] {
                insert_metric(
                    &mut points,
                    &format!("{base}.build.phases.{label}"),
                    metric,
                    true,
                );
            }
            if let Some(fuel) = &measurement.build.fuel {
                insert_metric(&mut points, &format!("{base}.build.fuel"), &fuel.fuel, true);
            }
            if let Some(fuel) = &measurement.build.warm_fuel {
                insert_metric(
                    &mut points,
                    &format!("{base}.build.warm_fuel"),
                    &fuel.fuel,
                    true,
                );
            }
            if let Some(fuel) = &measurement.build.edit.fuel {
                insert_metric(
                    &mut points,
                    &format!("{base}.build.edit.fuel"),
                    &fuel.fuel,
                    true,
                );
            }
            insert_metric(
                &mut points,
                &format!("{base}.build.edit.link_instructions"),
                &measurement.build.edit.link_instructions,
                true,
            );
            insert_metric(
                &mut points,
                &format!("{base}.build.edit.swap_instructions"),
                &measurement.build.edit.swap_instructions,
                true,
            );
            for (label, metric) in measurement.build.native.iter() {
                insert_metric(
                    &mut points,
                    &format!("{base}.build.native.{label}"),
                    metric,
                    true,
                );
            }
            // Incremental workspace sizes are reported but not compared:
            // they count cache files, not compiler output, and a one-byte
            // difference between runs is not a code change.
            insert_size(
                &mut points,
                &format!("{base}.size.total_bytes"),
                measurement.size.total_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.size.code_bytes"),
                measurement.size.code_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.size.data_bytes"),
                measurement.size.data_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.size.custom_bytes"),
                measurement.size.custom_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.size.import_bytes"),
                measurement.size.import_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.size.export_bytes"),
                measurement.size.export_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.size.functions"),
                u64::from(measurement.size.functions),
                true,
            );
            insert_metric(
                &mut points,
                &format!("{base}.exec.exec_ms"),
                &measurement.exec.exec_ms,
                true,
            );
            insert_metric(
                &mut points,
                &format!("{base}.instructions"),
                &measurement.instructions.instructions,
                true,
            );
        }
    }
    points
}

fn one(value: f64, class: MetricClass) -> Metric {
    Metric {
        min: value,
        median: value,
        max: value,
        runs: 1,
        class,
        idle_qualified: None,
        unavailable: None,
    }
}

fn derived_wall(value: f64, idle_qualified: bool) -> Metric {
    one(value, MetricClass::Wall).with_idle_qualified(idle_qualified)
}

fn insert(points: &mut BTreeMap<String, Point>, path: &str, metric: Metric, lower_is_better: bool) {
    points.insert(
        path.to_owned(),
        Point {
            metric,
            lower_is_better,
        },
    );
}

fn insert_metric(
    points: &mut BTreeMap<String, Point>,
    path: &str,
    metric: &Metric,
    lower_is_better: bool,
) {
    insert(points, path, metric.clone(), lower_is_better);
}

#[expect(clippy::cast_precision_loss, reason = "byte counts are far below 2^53")]
fn insert_size(
    points: &mut BTreeMap<String, Point>,
    path: &str,
    bytes: u64,
    lower_is_better: bool,
) {
    insert(
        points,
        path,
        one(bytes as f64, MetricClass::Exact),
        lower_is_better,
    );
}

/// Render the comparison and the regimes/idle evidence of both results.
#[must_use]
pub fn render_with_results(
    comparison: &Comparison,
    baseline_path: &str,
    current_path: &str,
    baseline: Option<&BenchResult>,
    current: Option<&BenchResult>,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# CogWasm benchmark comparison");
    let _ = writeln!(out);
    let _ = writeln!(out, "- baseline: `{baseline_path}`");
    let _ = writeln!(out, "- current: `{current_path}`");
    if let Some(baseline) = baseline {
        let _ = writeln!(
            out,
            "- baseline regime: **{}** ({})",
            baseline.regime.as_str(),
            baseline.regime_reason
        );
        idle_evidence(&mut out, "baseline", &baseline.idle);
    }
    if let Some(current) = current {
        let _ = writeln!(
            out,
            "- current regime: **{}** ({})",
            current.regime.as_str(),
            current.regime_reason
        );
        idle_evidence(&mut out, "current", &current.idle);
    }
    let _ = writeln!(
        out,
        "- {} metric(s): {} regression(s), {} other change(s), {} not comparable, {} unchanged",
        comparison.rows.len(),
        comparison.regressions.len(),
        comparison.changed.len(),
        comparison.not_comparable.len(),
        comparison.unchanged
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "## Regressions");
    let _ = writeln!(out);
    if comparison.regressions.is_empty() {
        let _ = writeln!(out, "None.");
    } else {
        table(&comparison.regressions, &mut out);
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Other changes");
    let _ = writeln!(out);
    if comparison.changed.is_empty() {
        let _ = writeln!(out, "None outside the compare rule.");
    } else {
        table(&comparison.changed, &mut out);
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Not comparable");
    let _ = writeln!(out);
    if comparison.not_comparable.is_empty() {
        let _ = writeln!(out, "None.");
    } else {
        table(&comparison.not_comparable, &mut out);
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Method");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Exact metrics (fuel, executed wasm instructions, sizes) are compared exactly. Counter \
         metrics (native instructions retired, cycles) are compared with a relative tolerance: \
         the default is {:.2}%, set from the measured run-to-run spread on a busy machine. \
         Wall-clock metrics are flagged only when both runs are idle-qualified and one median \
         lies outside the other run's min..max spread. A busy run reports wall-clock rows as \
         \"not comparable: busy run\" and they can never be regressions.",
        DEFAULT_COUNTER_TOLERANCE * 100.0
    );
    let _ = writeln!(out);
    out
}

fn idle_evidence(out: &mut String, label: &str, evidence: &crate::idle::IdleEvidence) {
    let worst = evidence.worst_fraction().map_or_else(
        || "no samples".to_owned(),
        |fraction| format!("{:.1}% of one core at the busiest window", fraction * 100.0),
    );
    let _ = writeln!(
        out,
        "- {label} idle: **{}** ({worst}; threshold {:.1}%; {} sample(s), method `{}`)",
        if evidence.qualified() {
            "qualified"
        } else {
            "not qualified"
        },
        evidence.threshold_fraction * 100.0,
        evidence.samples.len(),
        evidence.method
    );
}

fn table(rows: &[ComparisonRow], out: &mut String) {
    let _ = writeln!(
        out,
        "| metric | class | baseline | current | change | rule | baseline spread | current spread |"
    );
    let _ = writeln!(out, "| --- | --- | ---: | ---: | ---: | --- | --- | --- |");
    for row in rows {
        let change = row
            .change
            .map_or_else(|| "—".to_owned(), |value| format!("{:+.1}%", value * 100.0));
        let class = row
            .class()
            .map_or_else(|| "—".to_owned(), |c| c.as_str().to_owned());
        let _ = writeln!(
            out,
            "| {} | {class} | {:.6} | {:.6} | {change} | {} | {:.6}..{:.6} | {:.6}..{:.6} |",
            row.metric,
            row.baseline.median,
            row.current.median,
            row.note.as_deref().unwrap_or("spread / exact"),
            row.baseline.min,
            row.baseline.max,
            row.current.min,
            row.current.max
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::{
        BenchResult, CompilerInfo, Geomeans, MachineInfo, ProgramResult, Settings,
    };

    fn metric(median: f64, min: f64, max: f64, class: MetricClass) -> Metric {
        Metric {
            min,
            median,
            max,
            runs: 3,
            class,
            idle_qualified: (class == MetricClass::Wall).then_some(true),
            unavailable: None,
        }
    }

    fn results(baseline: Metric, current: Metric) -> (BenchResult, BenchResult) {
        let mut first = base_result();
        let mut second = base_result();
        first
            .programs
            .get_mut("a_add")
            .expect("program")
            .configs
            .get_mut("default")
            .expect("config")
            .build
            .warm_compile_ms = baseline;
        second
            .programs
            .get_mut("a_add")
            .expect("program")
            .configs
            .get_mut("default")
            .expect("config")
            .build
            .warm_compile_ms = current;
        (first, second)
    }

    fn base_result() -> BenchResult {
        crate::result::tests::minimal_result()
    }

    #[test]
    fn identical_results_have_no_regressions() {
        let result = base_result();
        let comparison = compare_with_tolerance(&result, &result, DEFAULT_COUNTER_TOLERANCE);
        assert!(!comparison.has_regressions());
        assert_eq!(comparison.changed.len(), 0);
    }

    #[test]
    fn timing_change_inside_the_spread_is_not_flagged() {
        let (baseline, current) = results(
            metric(10.0, 9.0, 11.0, MetricClass::Wall),
            metric(10.5, 10.0, 11.0, MetricClass::Wall),
        );
        let comparison = compare_with_tolerance(&baseline, &current, DEFAULT_COUNTER_TOLERANCE);
        assert!(!comparison.has_regressions());
        assert!(
            !comparison
                .rows
                .iter()
                .any(|row| row.metric.ends_with("warm_compile_ms")
                    && row.status != Status::Unchanged)
        );
    }

    fn point(median: f64, class: MetricClass, idle: Option<bool>) -> Point {
        Point {
            metric: Metric {
                min: median,
                median,
                max: median,
                runs: 1,
                class,
                idle_qualified: idle,
                unavailable: None,
            },
            lower_is_better: true,
        }
    }

    #[test]
    fn exact_metrics_compare_exactly() {
        let (status, _) = classify(
            &point(1000.0, MetricClass::Exact, None),
            &point(1000.0, MetricClass::Exact, None),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(status, Status::Unchanged);
        let (status, _) = classify(
            &point(1000.0, MetricClass::Exact, None),
            &point(1001.0, MetricClass::Exact, None),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(status, Status::Regression);
    }

    #[test]
    fn counter_metrics_use_the_configured_tolerance() {
        let (status, note) = classify(
            &point(1_000_000.0, MetricClass::Counter, None),
            &point(1_002_000.0, MetricClass::Counter, None),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(status, Status::Unchanged);
        assert!(note.expect("tolerance note").contains("5.00%"));
        let (status, _) = classify(
            &point(1_000_000.0, MetricClass::Counter, None),
            &point(1_060_000.0, MetricClass::Counter, None),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(status, Status::Regression);
        // A tighter tolerance makes the same change visible.
        let (status, _) = classify(
            &point(1_000_000.0, MetricClass::Counter, None),
            &point(1_002_000.0, MetricClass::Counter, None),
            0.001,
        );
        assert_eq!(status, Status::Regression);
    }

    #[test]
    fn wall_metrics_need_both_runs_idle() {
        let busy = classify(
            &point(10.0, MetricClass::Wall, Some(false)),
            &point(20.0, MetricClass::Wall, Some(true)),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(busy.0, Status::NotComparable);
        assert!(busy.1.expect("reason").contains("busy run"));
        let unknown = classify(
            &point(10.0, MetricClass::Wall, None),
            &point(20.0, MetricClass::Wall, None),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(unknown.0, Status::NotComparable);
        let idle = classify(
            &point(10.0, MetricClass::Wall, Some(true)),
            &point(20.0, MetricClass::Wall, Some(true)),
            DEFAULT_COUNTER_TOLERANCE,
        );
        assert_eq!(idle.0, Status::Regression);
    }

    #[test]
    fn timing_change_outside_both_spreads_is_a_regression() {
        let (baseline, current) = results(
            metric(10.0, 9.0, 11.0, MetricClass::Wall),
            metric(12.0, 11.5, 12.5, MetricClass::Wall),
        );
        let comparison = compare_with_tolerance(&baseline, &current, DEFAULT_COUNTER_TOLERANCE);
        assert!(comparison.has_regressions());
    }

    #[test]
    fn busy_wall_change_is_never_a_regression() {
        let mut baseline = metric(10.0, 9.0, 11.0, MetricClass::Wall);
        let mut current = metric(20.0, 19.0, 21.0, MetricClass::Wall);
        baseline.idle_qualified = Some(false);
        current.idle_qualified = Some(false);
        let (baseline, current) = results(baseline, current);
        let comparison = compare_with_tolerance(&baseline, &current, DEFAULT_COUNTER_TOLERANCE);
        assert!(!comparison.has_regressions());
        assert!(
            comparison
                .not_comparable
                .iter()
                .any(|row| row.note.as_deref() == Some("not comparable: busy run"))
        );
    }

    #[test]
    fn deterministic_metrics_compare_exactly() {
        let mut first = base_result();
        let mut second = base_result();
        first
            .programs
            .get_mut("a_add")
            .expect("program")
            .configs
            .get_mut("default")
            .expect("config")
            .size
            .total_bytes = 100;
        second
            .programs
            .get_mut("a_add")
            .expect("program")
            .configs
            .get_mut("default")
            .expect("config")
            .size
            .total_bytes = 101;
        let comparison = compare_with_tolerance(&first, &second, DEFAULT_COUNTER_TOLERANCE);
        assert!(comparison.rows.iter().any(
            |row| row.metric.ends_with("size.total_bytes") && row.status == Status::Regression
        ));
    }

    #[test]
    fn rendering_names_both_inputs_and_the_regression_section() {
        let result = base_result();
        let comparison = compare_with_tolerance(&result, &result, DEFAULT_COUNTER_TOLERANCE);
        let text = render_with_results(&comparison, "base.json", "current.json", None, None);
        assert!(text.contains("base.json"));
        assert!(text.contains("## Regressions"));
        assert!(text.contains("None."));
        assert!(text.contains("## Not comparable"));
    }

    #[test]
    fn rendering_includes_regime_and_idle_evidence() {
        let result = base_result();
        let comparison = compare_with_tolerance(&result, &result, DEFAULT_COUNTER_TOLERANCE);
        let text = render_with_results(
            &comparison,
            "base.json",
            "current.json",
            Some(&result),
            Some(&result),
        );
        assert!(text.contains("baseline regime"));
        assert!(text.contains("idle:"));
        assert!(text.contains("threshold"));
    }

    #[test]
    fn minimal_fixture_has_the_expected_fields() {
        let result = base_result();
        assert_eq!(result.schema, crate::result::SCHEMA);
        assert!(result.programs.contains_key("a_add"));
        let _: &MachineInfo = &result.machine;
        let _: &CompilerInfo = &result.compiler;
        let _: &Settings = &result.settings;
        let _: &Geomeans = &result.geomeans;
        let _: &ProgramResult = result.programs.get("a_add").expect("program");
    }
}
