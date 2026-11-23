//! Chart export: turn one or more `result.json` documents into the
//! `data.json` the site's interactive chart (`bench/chart/chart.js`) renders.
//!
//! Each result is one point on the x axis (ordered by commit date, then by
//! generation time). The document carries its own panels and series, so the
//! page draws whatever this module lists; nothing in `chart.js` is specific to
//! these keys.

use crate::result::{BenchResult, load_result};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Chart document schema, matching the one `chart.js` reads.
pub const CHART_SCHEMA: u32 = 1;
/// LLVM -O2 time floor for the execution geomean (the run applies it).
const EXEC_MIN_LLVM_MS: f64 = 0.1;
const SHORT_SHA_LENGTH: usize = 7;

/// One plotted series.
#[derive(Debug, Clone, Serialize)]
pub struct Series {
    pub key: &'static str,
    pub panel: u32,
    pub label: &'static str,
    pub unit: &'static str,
    pub digits: u32,
    pub color: &'static str,
    pub marker: &'static str,
    pub axis: &'static str,
    pub deterministic: bool,
}

/// The series every chart carries, Okabe-Ito colours with distinct markers.
pub const SERIES: &[Series] = &[
    Series {
        key: "exec_ratio",
        panel: 1,
        label: "Execution",
        unit: "x LLVM -O2",
        digits: 2,
        color: "#0072B2",
        marker: "circle",
        axis: "left",
        deterministic: false,
    },
    Series {
        key: "exec_instructions_ratio",
        panel: 1,
        label: "Executed instructions",
        unit: "x LLVM -O2",
        digits: 2,
        color: "#56B4E9",
        marker: "cross",
        axis: "left",
        deterministic: true,
    },
    Series {
        key: "size_ratio",
        panel: 1,
        label: "Module size",
        unit: "x LLVM -O2",
        digits: 2,
        color: "#D55E00",
        marker: "square",
        axis: "left",
        deterministic: true,
    },
    Series {
        key: "size_ratio_oz",
        panel: 1,
        label: "Module size vs -Oz",
        unit: "x LLVM -Oz",
        digits: 2,
        color: "#E69F00",
        marker: "diamond",
        axis: "left",
        deterministic: true,
    },
    Series {
        key: "warm_compile_ms",
        panel: 2,
        label: "Warm compile",
        unit: "ms",
        digits: 1,
        color: "#009E73",
        marker: "triangle",
        axis: "left",
        deterministic: false,
    },
    Series {
        key: "edit_ms",
        panel: 2,
        label: "Edit -> result",
        unit: "ms",
        digits: 1,
        color: "#1F1F1F",
        marker: "circle",
        axis: "left",
        deterministic: false,
    },
    Series {
        key: "precompiled_load_ms",
        panel: 2,
        label: "Precompiled load",
        unit: "ms",
        digits: 1,
        color: "#CC79A7",
        marker: "plus",
        axis: "left",
        deterministic: false,
    },
    Series {
        key: "compiler_instructions",
        panel: 2,
        label: "Compiler instructions",
        unit: "instructions",
        digits: 0,
        color: "#5B3A8E",
        marker: "star",
        axis: "right",
        deterministic: true,
    },
];

/// Chart export options.
#[derive(Debug, Clone)]
pub struct ChartOptions {
    /// The config whose numbers are plotted.
    pub config: String,
    pub repo: String,
    pub page_url: String,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            config: "default".to_owned(),
            repo: "TeamNodeCraft/CogWasm".to_owned(),
            page_url: "https://teamnodecraft.github.io/CogWasm/#benchmarks".to_owned(),
        }
    }
}

/// Expand the inputs: files are taken as given, directories contribute every
/// `*.json` directly inside them, in name order.
///
/// # Errors
///
/// Returns an error when an input cannot be read or no input remains.
pub fn collect_inputs(inputs: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for input in inputs {
        if input.is_dir() {
            let mut found: Vec<PathBuf> = std::fs::read_dir(input)
                .with_context(|| format!("listing {}", input.display()))?
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
                .collect();
            found.sort();
            files.extend(found);
        } else {
            files.push(input.clone());
        }
    }
    if files.is_empty() {
        bail!("no result documents to chart");
    }
    Ok(files)
}

/// Load and chart result documents.
///
/// # Errors
///
/// Returns an error when a document cannot be loaded or lacks the config.
pub fn chart_files(files: &[PathBuf], options: &ChartOptions) -> Result<Value> {
    let results = files
        .iter()
        .map(|path| load_result(path))
        .collect::<Result<Vec<_>>>()?;
    build(&results, options)
}

/// Build the chart document from results.
///
/// # Errors
///
/// Returns an error when no result measured the requested config.
pub fn build(results: &[BenchResult], options: &ChartOptions) -> Result<Value> {
    let mut ordered: Vec<&BenchResult> = results
        .iter()
        .filter(|result| result.geomeans.by_config.contains_key(&options.config))
        .collect();
    if ordered.is_empty() {
        bail!("no result measured the `{}` config", options.config);
    }
    ordered.sort_by(|a, b| {
        point_date(a)
            .cmp(&point_date(b))
            .then_with(|| a.generated_at.cmp(&b.generated_at))
    });
    let mut previous: Option<BTreeSet<String>> = None;
    let mut points = Vec::with_capacity(ordered.len());
    for (index, result) in ordered.iter().enumerate() {
        let programs: BTreeSet<String> = result.programs.keys().cloned().collect();
        let (added, removed) = previous.as_ref().map_or_else(
            || (Vec::new(), Vec::new()),
            |before| {
                (
                    programs.difference(before).cloned().collect::<Vec<_>>(),
                    before.difference(&programs).cloned().collect::<Vec<_>>(),
                )
            },
        );
        points.push(point(index, result, options, &programs, &added, &removed));
        previous = Some(programs);
    }
    Ok(json!({
        "schema": CHART_SCHEMA,
        "generated_at": crate::util::utc_rfc3339(),
        "repo": options.repo,
        "page_url": options.page_url,
        "config": options.config,
        "exec_min_llvm_ms": EXEC_MIN_LLVM_MS,
        "panels": [
            {"id": 1, "title": "Against LLVM (lower is better, 1.0 = parity)"},
            {"id": 2, "title": "Compile speed (ms, lower is better)"},
        ],
        "series": SERIES,
        "reference": {"panel": 1, "value": 1.0, "label": "LLVM"},
        "points": points,
    }))
}

fn point(
    index: usize,
    result: &BenchResult,
    options: &ChartOptions,
    programs: &BTreeSet<String>,
    added: &[String],
    removed: &[String],
) -> Value {
    let set = &result.geomeans.by_config[&options.config];
    let mut metrics = Map::new();
    let mut put = |key: &str, value: Option<f64>| {
        if let Some(value) = value.filter(|value| value.is_finite()) {
            metrics.insert(key.to_owned(), json!(value));
        }
    };
    put("exec_ratio", set.exec_ratio);
    put("exec_instructions_ratio", set.instructions_ratio);
    put("size_ratio", set.size_ratio_o2);
    put("size_ratio_oz", set.size_ratio_oz);
    put("warm_compile_ms", set.warm_compile_median_ms);
    put("edit_ms", set.edit_p50_median_ms);
    let load = result.compiler.precompiled_load_ms;
    put("precompiled_load_ms", (load > 0.0).then_some(load));
    let fuel: Vec<f64> = result
        .programs
        .values()
        .filter_map(|program| program.configs.get(&options.config))
        .filter_map(|config| config.build.fuel.as_ref())
        .map(|fuel| fuel.fuel.median)
        .collect();
    let compiler_programs = fuel.len();
    put("compiler_instructions", median(fuel));
    let sha = result.compiler.git_commit.clone();
    let short: String = sha.chars().take(SHORT_SHA_LENGTH).collect();
    let date = point_date(result);
    // Results written before `os` and `arch` were separate fields carry the
    // arch in `os` too.
    let os = result
        .machine
        .os
        .strip_suffix(&format!(" {}", result.machine.arch))
        .unwrap_or(&result.machine.os);
    let runner = format!(
        "{} {} ({} cores{})",
        os,
        result.machine.arch,
        result.machine.cores,
        result
            .machine
            .cpu
            .as_deref()
            .map_or_else(String::new, |cpu| format!(", {cpu}")),
    );
    json!({
        "index": index,
        "sha": sha,
        "short_sha": short,
        "subject": result.compiler.git_subject,
        "date": date,
        "generated_at": crate::util::compact_to_rfc3339(&result.generated_at),
        "run_id": Value::Null,
        "runner": runner,
        "suite": result.suite,
        "runs": result.settings.runs,
        "programs": programs.iter().collect::<Vec<_>>(),
        "program_count": programs.len(),
        "subset_changed": !added.is_empty() || !removed.is_empty(),
        "subset_added": added,
        "subset_removed": removed,
        "metrics": metrics,
        "exec_programs": set.exec_programs,
        "size_programs": set.programs,
        "instruction_programs": set.programs,
        "compiler_programs": compiler_programs,
        "load_warning": result.machine.load_warning,
        "source": "cogwasm-bench",
    })
}

/// The commit date (RFC 3339 UTC), or the run's own time for results made
/// before the commit date was recorded.
fn point_date(result: &BenchResult) -> String {
    if result.compiler.git_commit_date.is_empty() {
        crate::util::compact_to_rfc3339(&result.generated_at)
    } else {
        result.compiler.git_commit_date.clone()
    }
}

fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    Some(if values.len().is_multiple_of(2) {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    })
}

/// Write the chart document as `<out>/data.json`, returning its path.
///
/// # Errors
///
/// Returns an error when the directory or file cannot be written.
pub fn write(document: &Value, out_dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(out_dir).with_context(|| format!("creating {}", out_dir.display()))?;
    let path = out_dir.join("data.json");
    let text = serde_json::to_string_pretty(document).context("serialising the chart")?;
    std::fs::write(&path, text + "\n").with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::tests::minimal_result;

    fn result_at(sha: &str, date: &str, warm: f64) -> BenchResult {
        let mut result = minimal_result();
        result.compiler.git_commit = sha.to_owned();
        result.compiler.git_commit_date = date.to_owned();
        if let Some(set) = result.geomeans.by_config.get_mut("default") {
            set.warm_compile_median_ms = Some(warm);
        }
        result
    }

    #[test]
    fn points_follow_commit_date_and_carry_the_series_values() {
        let results = [
            result_at("bbbbbbbbbb", "2026-02-01T00:00:00Z", 20.0),
            result_at("aaaaaaaaaa", "2026-01-01T00:00:00Z", 30.0),
        ];
        let document = build(&results, &ChartOptions::default()).expect("chart builds");
        assert_eq!(document["schema"], CHART_SCHEMA);
        let points = document["points"].as_array().expect("points");
        assert_eq!(points.len(), 2);
        assert_eq!(points[0]["short_sha"], "aaaaaaa");
        assert_eq!(points[0]["index"], 0);
        assert_eq!(points[1]["metrics"]["warm_compile_ms"], 20.0);
        assert_eq!(points[0]["subset_changed"], false);
        let keys: Vec<&str> = document["series"]
            .as_array()
            .expect("series")
            .iter()
            .filter_map(|series| series["key"].as_str())
            .collect();
        assert!(keys.contains(&"exec_ratio") && keys.contains(&"compiler_instructions"));
    }

    #[test]
    fn a_changed_program_set_is_marked() {
        let first = result_at("a", "2026-01-01T00:00:00Z", 1.0);
        let mut second = result_at("b", "2026-01-02T00:00:00Z", 1.0);
        let program = second.programs.values().next().expect("program").clone();
        second.programs.insert("zz_new".to_owned(), program);
        let document = build(&[first, second], &ChartOptions::default()).expect("chart builds");
        assert_eq!(document["points"][1]["subset_changed"], true);
        assert_eq!(document["points"][1]["subset_added"][0], "zz_new");
    }

    #[test]
    fn a_missing_config_is_an_error() {
        let options = ChartOptions {
            config: "capture".to_owned(),
            ..ChartOptions::default()
        };
        assert!(build(&[minimal_result()], &options).is_err());
    }

    #[test]
    fn median_handles_even_and_odd_counts() {
        assert_eq!(median(vec![]), None);
        assert_eq!(median(vec![3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(vec![4.0, 1.0, 2.0, 3.0]), Some(2.5));
    }
}
