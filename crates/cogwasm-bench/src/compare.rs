//! Spread-aware comparison of two result documents.

use crate::result::BenchResult;
use crate::stats::Metric;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// One metric's comparison outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Unchanged,
    Changed,
    Regression,
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
}

/// The whole comparison.
#[derive(Debug, Clone)]
pub struct Comparison {
    pub rows: Vec<ComparisonRow>,
    pub regressions: Vec<ComparisonRow>,
    pub changed: Vec<ComparisonRow>,
    pub unchanged: usize,
}

impl Comparison {
    #[must_use]
    pub fn has_regressions(&self) -> bool {
        !self.regressions.is_empty()
    }
}

/// Compare two results. Deterministic metrics must match exactly; timing
/// metrics are flagged only when the current median is outside the baseline's
/// `[min, max]` spread (or the baseline median is outside the current spread).
#[must_use]
pub fn compare(baseline: &BenchResult, current: &BenchResult) -> Comparison {
    let before = collect(baseline);
    let after = collect(current);
    let mut rows = Vec::new();
    for (path, old) in &before {
        let Some(new) = after.get(path) else {
            continue;
        };
        let status = classify(old, new);
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
        });
    }
    rows.sort_by(|left, right| left.metric.cmp(&right.metric));
    let regressions: Vec<ComparisonRow> = rows
        .iter()
        .filter(|row| row.status == Status::Regression)
        .cloned()
        .collect();
    let changed: Vec<ComparisonRow> = rows
        .iter()
        .filter(|row| row.status == Status::Changed)
        .cloned()
        .collect();
    let unchanged = rows
        .iter()
        .filter(|row| row.status == Status::Unchanged)
        .count();
    Comparison {
        rows,
        regressions,
        changed,
        unchanged,
    }
}

/// Relative change below which two single-sample wall-clock or memory
/// readings are treated as the same.
const SINGLE_SAMPLE_NOISE: f64 = 0.05;

fn classify(old: &Point, new: &Point) -> Status {
    let different = if old.metric.deterministic {
        old.metric.median.total_cmp(&new.metric.median) != std::cmp::Ordering::Equal
    } else if old.metric.runs < 2 && new.metric.runs < 2 {
        // A single sample on both sides has no spread to compare against
        // (host RSS, peak memory): only a change beyond the noise floor counts.
        let scale = old.metric.median.abs().max(f64::MIN_POSITIVE);
        (new.metric.median - old.metric.median).abs() / scale > SINGLE_SAMPLE_NOISE
    } else {
        new.metric.outside_spread_of(&old.metric) || old.metric.outside_spread_of(&new.metric)
    };
    if !different {
        return Status::Unchanged;
    }
    if old.lower_is_better && new.metric.median > old.metric.median {
        Status::Regression
    } else {
        Status::Changed
    }
}

#[expect(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    reason = "the metric walk is one flat list; byte counts are far below 2^53"
)]
fn collect(result: &BenchResult) -> BTreeMap<String, Point> {
    let mut points = BTreeMap::new();
    insert(
        &mut points,
        "toolchain.rustc_wasm_bytes",
        one(result.toolchain.rustc_wasm_bytes as f64, true),
        true,
    );
    insert(
        &mut points,
        "toolchain.rustc_wasm_stripped_bytes",
        one(result.toolchain.rustc_wasm_stripped_bytes as f64, true),
        true,
    );
    insert(
        &mut points,
        "toolchain.rustc_wasm_gzip_bytes",
        one(result.toolchain.rustc_wasm_gzip_bytes as f64, true),
        true,
    );
    insert(
        &mut points,
        "toolchain.sysroot_bytes",
        one(result.toolchain.sysroot.bytes as f64, true),
        true,
    );
    for (config, set) in &result.geomeans.by_config {
        if let Some(value) = set.exec_ratio {
            insert(
                &mut points,
                &format!("geomeans.{config}.exec_ratio"),
                one(value, false),
                true,
            );
        }
        if let Some(value) = set.instructions_ratio {
            insert(
                &mut points,
                &format!("geomeans.{config}.instructions_ratio"),
                one(value, false),
                true,
            );
        }
        if let Some(value) = set.size_ratio_o2 {
            insert(
                &mut points,
                &format!("geomeans.{config}.size_ratio_o2"),
                one(value, false),
                true,
            );
        }
        if let Some(value) = set.size_ratio_oz {
            insert(
                &mut points,
                &format!("geomeans.{config}.size_ratio_oz"),
                one(value, false),
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
                &one(measurement.build.edit.p50_ms, false),
                true,
            );
            insert_metric(
                &mut points,
                &format!("{base}.build.edit.p90_ms"),
                &one(measurement.build.edit.p90_ms, false),
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
            insert_size(
                &mut points,
                &format!("{base}.build.incremental_dir_bytes"),
                measurement.build.incremental_dir_bytes,
                true,
            );
            insert_size(
                &mut points,
                &format!("{base}.build.intermediate_objects_bytes"),
                measurement.build.intermediate_objects_bytes,
                true,
            );
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

fn one(value: f64, deterministic: bool) -> Metric {
    Metric {
        min: value,
        median: value,
        max: value,
        runs: 1,
        deterministic,
    }
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
    insert(points, path, one(bytes as f64, true), lower_is_better);
}

/// Render the comparison as Markdown.
#[must_use]
pub fn render(comparison: &Comparison, baseline: &str, current: &str) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# CogWasm benchmark comparison");
    let _ = writeln!(out);
    let _ = writeln!(out, "- baseline: `{baseline}`");
    let _ = writeln!(out, "- current: `{current}`");
    let _ = writeln!(
        out,
        "- {} metric(s): {} regression(s), {} other change(s), {} unchanged",
        comparison.rows.len(),
        comparison.regressions.len(),
        comparison.changed.len(),
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
        let _ = writeln!(out, "None outside the run-to-run spread.");
    } else {
        table(&comparison.changed, &mut out);
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Method");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Deterministic metrics (instruction counts, fuel, sizes) are compared exactly. Wall-clock \
         metrics are flagged only when one run's median lies outside the other run's min..max \
         spread; a smaller median inside the spread is noise, not a change."
    );
    let _ = writeln!(out);
    out
}

fn table(rows: &[ComparisonRow], out: &mut String) {
    let _ = writeln!(
        out,
        "| metric | baseline | current | change | baseline spread | current spread |"
    );
    let _ = writeln!(out, "| --- | ---: | ---: | ---: | --- | --- |");
    for row in rows {
        let change = row
            .change
            .map_or_else(|| "—".to_owned(), |value| format!("{:+.1}%", value * 100.0));
        let _ = writeln!(
            out,
            "| {} | {:.6} | {:.6} | {change} | {:.6}..{:.6} | {:.6}..{:.6} |",
            row.metric,
            row.baseline.median,
            row.current.median,
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

    fn metric(median: f64, min: f64, max: f64, deterministic: bool) -> Metric {
        Metric {
            min,
            median,
            max,
            runs: 3,
            deterministic,
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
        let comparison = compare(&result, &result);
        assert!(!comparison.has_regressions());
        assert_eq!(comparison.changed.len(), 0);
    }

    #[test]
    fn timing_change_inside_the_spread_is_not_flagged() {
        let (baseline, current) = results(
            metric(10.0, 9.0, 11.0, false),
            metric(10.5, 10.0, 11.0, false),
        );
        let comparison = compare(&baseline, &current);
        assert!(!comparison.has_regressions());
        assert!(
            !comparison
                .rows
                .iter()
                .any(|row| row.metric.ends_with("warm_compile_ms")
                    && row.status != Status::Unchanged)
        );
    }

    fn single(median: f64) -> Point {
        Point {
            metric: Metric {
                min: median,
                median,
                max: median,
                runs: 1,
                deterministic: false,
            },
            lower_is_better: true,
        }
    }

    #[test]
    fn single_samples_change_only_beyond_the_noise_floor() {
        assert_eq!(
            classify(&single(1000.0), &single(1010.0)),
            Status::Unchanged
        );
        assert_eq!(
            classify(&single(1000.0), &single(1100.0)),
            Status::Regression
        );
        assert_eq!(classify(&single(1000.0), &single(900.0)), Status::Changed);
    }

    #[test]
    fn timing_change_outside_both_spreads_is_a_regression() {
        let (baseline, current) = results(
            metric(10.0, 9.0, 11.0, false),
            metric(12.0, 11.5, 12.5, false),
        );
        let comparison = compare(&baseline, &current);
        assert!(comparison.has_regressions());
    }

    #[test]
    fn deterministic_metrics_compare_exactly() {
        let (baseline, current) = results(
            metric(10.0, 10.0, 10.0, false),
            metric(10.0, 10.0, 10.0, false),
        );
        let mut first = baseline;
        let mut second = current;
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
        let comparison = compare(&first, &second);
        assert!(comparison.rows.iter().any(
            |row| row.metric.ends_with("size.total_bytes") && row.status == Status::Regression
        ));
    }

    #[test]
    fn rendering_names_both_inputs_and_the_regression_section() {
        let result = base_result();
        let comparison = compare(&result, &result);
        let text = render(&comparison, "base.json", "current.json");
        assert!(text.contains("base.json"));
        assert!(text.contains("## Regressions"));
        assert!(text.contains("None."));
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
