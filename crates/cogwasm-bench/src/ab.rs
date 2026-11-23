//! Interleaved A/B wall-clock comparison.
//!
//! Wall clock drifts over a long run (thermal throttling, a background build
//! starting halfway through). Measuring all of A and then all of B lets that
//! drift land on one side. This command alternates A, B, A, B per program and
//! reports the minimum of N rounds for each side plus the ratio and its spread.
//!
//! A non-idle run is marked non-definitive instead of pretending the ratio is
//! a code difference.

use crate::build::{BuildRequest, Config, ServeCompiler, measure_build};
use crate::idle::{IdleEvidence, IdleMonitor};
use crate::programs::{Suite, ensure_dependencies, load_manifest, prepare_programs};
use crate::toolchain::Toolchain;
use crate::util;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Which side of an A/B round is being measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    A,
    B,
}

/// The A, B, A, B sequence one program is measured in.
#[must_use]
pub fn interleaving(rounds: usize) -> Vec<Side> {
    let mut order = Vec::with_capacity(rounds.saturating_mul(2));
    for _ in 0..rounds {
        order.push(Side::A);
        order.push(Side::B);
    }
    order
}

/// One metric's per-program A/B summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbMetric {
    /// The smallest A sample (min-of-N).
    pub a_min_ms: f64,
    /// The smallest B sample (min-of-N).
    pub b_min_ms: f64,
    /// `a_min / b_min`; a value below 1 means A was faster.
    pub ratio: f64,
    /// Smallest per-round `A / B` ratio.
    pub ratio_min: f64,
    /// Largest per-round `A / B` ratio.
    pub ratio_max: f64,
}

/// Aggregate interleaved `(A, B)` rounds. A missing/zero denominator is not a
/// ratio, so an empty or zero sample set is `None`.
#[must_use]
pub fn aggregate(samples: &[(f64, f64)]) -> Option<AbMetric> {
    let a_min = samples
        .iter()
        .map(|(a, _)| *a)
        .filter(|value| value.is_finite() && *value > 0.0)
        .min_by(f64::total_cmp)?;
    let b_min = samples
        .iter()
        .map(|(_, b)| *b)
        .filter(|value| value.is_finite() && *value > 0.0)
        .min_by(f64::total_cmp)?;
    let ratios: Vec<f64> = samples
        .iter()
        .filter(|(a, b)| *a > 0.0 && *b > 0.0)
        .map(|(a, b)| a / b)
        .collect();
    if ratios.is_empty() {
        return None;
    }
    Some(AbMetric {
        a_min_ms: a_min,
        b_min_ms: b_min,
        ratio: a_min / b_min,
        ratio_min: ratios.iter().copied().fold(f64::INFINITY, f64::min),
        ratio_max: ratios.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    })
}

/// One program's A/B result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbProgram {
    pub name: String,
    pub class: String,
    pub warm_compile: AbMetric,
    pub edit_to_result: AbMetric,
    pub cold_compile: AbMetric,
    /// True only when every round on both sides was idle-qualified.
    pub definitive: bool,
}

/// The `ab.json` document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbDocument {
    pub schema: u32,
    pub generated_at: String,
    pub suite: String,
    pub rounds: usize,
    pub a: String,
    pub b: String,
    pub idle: IdleEvidence,
    /// True only when every program's rounds were idle-qualified.
    pub definitive: bool,
    pub programs: Vec<AbProgram>,
}

/// One side's compiler and configuration.
pub struct AbSide {
    pub label: String,
    pub config: Config,
    pub rustc: Option<PathBuf>,
}

/// Everything the `ab` command needs.
pub struct AbOptions {
    pub suite: Suite,
    pub programs: Vec<String>,
    pub a: AbSide,
    pub b: AbSide,
    pub rounds: usize,
    pub out_dir: PathBuf,
    pub idle_threshold_fraction: f64,
}

/// Run the interleaved comparison and write `ab.json` + `ab.md`.
///
/// # Errors
///
/// Returns an error on a missing toolchain, a failed build, or an A/B
/// configuration that names the same build twice.
#[expect(
    clippy::too_many_lines,
    reason = "the A/B driver is one interleaved measurement sequence plus output"
)]
pub fn run(toolchain: &Toolchain, options: &AbOptions) -> Result<PathBuf> {
    if options.rounds == 0 {
        bail!("--runs must be at least 1");
    }
    if options.a.label == options.b.label {
        bail!(
            "A and B are both {:?}; give two different configs or rustc.wasm builds",
            options.a.label
        );
    }
    toolchain.validate()?;
    let programs = prepare_programs(toolchain, options.suite, &options.programs)?;
    let dependencies = ensure_dependencies(toolchain, &programs)?;
    let seeds = load_manifest(toolchain)?.seeds;
    let out_dir = util::absolute(&options.out_dir)?;
    std::fs::create_dir_all(&out_dir).with_context(|| format!("creating {}", out_dir.display()))?;
    let work = util::absolute(&toolchain.work_root.join("bench/.ab-work"))?;
    crate::build::clear_directory(&work)?;
    std::fs::create_dir_all(&work)?;

    let mut sides = [
        prepare_side(toolchain, &options.a, &work, "a")?,
        prepare_side(toolchain, &options.b, &work, "b")?,
    ];
    let monitor = IdleMonitor::new(options.idle_threshold_fraction);
    let mut programs_out = Vec::with_capacity(programs.len());
    for program in &programs {
        eprintln!("[cogwasm-ab] {} ({})", program.def.name, program.def.class);
        let mut warm: Vec<(f64, f64)> = Vec::with_capacity(options.rounds);
        let mut edit: Vec<(f64, f64)> = Vec::with_capacity(options.rounds);
        let mut cold: Vec<(f64, f64)> = Vec::with_capacity(options.rounds);
        let mut round_values: BTreeMap<(usize, Side), (f64, f64, f64)> = BTreeMap::new();
        let start = monitor.sample_count();
        for (index, side) in interleaving(options.rounds).into_iter().enumerate() {
            let side_index = match side {
                Side::A => 0,
                Side::B => 1,
            };
            let (toolchain, compiler, serve) = sides[side_index].parts();
            let mut request = BuildRequest {
                toolchain,
                compiler,
                serve,
                fuel: None,
                program,
                config: &options_side(&options.a, &options.b, side_index).config,
                dependencies: &dependencies,
                work: work.join(if side == Side::A { "side-a" } else { "side-b" }),
                runs: 1,
                seeds: &seeds,
                idle: &monitor,
                timed: true,
            };
            let built = measure_build(&mut request)?;
            round_values.insert(
                (index / 2, side),
                (
                    built.measurement.warm_compile_ms.median,
                    built.measurement.edit.p50_ms,
                    built.measurement.cold_compile_ms.median,
                ),
            );
        }
        for round in 0..options.rounds {
            let a = round_values
                .get(&(round, Side::A))
                .copied()
                .context("the A round is missing")?;
            let b = round_values
                .get(&(round, Side::B))
                .copied()
                .context("the B round is missing")?;
            cold.push((a.2, b.2));
            warm.push((a.0, b.0));
            edit.push((a.1, b.1));
        }
        let idle = monitor.evidence_since(start);
        programs_out.push(AbProgram {
            name: program.def.name.clone(),
            class: program.def.class.clone(),
            warm_compile: aggregate(&warm).context("no warm-compile A/B samples")?,
            edit_to_result: aggregate(&edit).context("no edit A/B samples")?,
            cold_compile: aggregate(&cold).context("no cold-compile A/B samples")?,
            definitive: idle.qualified(),
        });
    }
    let idle = monitor.evidence();
    let document = AbDocument {
        schema: 1,
        generated_at: util::utc_rfc3339(),
        suite: options.suite.as_str().to_owned(),
        rounds: options.rounds,
        a: options.a.label.clone(),
        b: options.b.label.clone(),
        definitive: idle.qualified() && programs_out.iter().all(|program| program.definitive),
        idle,
        programs: programs_out,
    };
    let path = out_dir.join("ab.json");
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&document).context("serialising the A/B document")?,
    )
    .with_context(|| format!("writing {}", path.display()))?;
    let markdown_path = out_dir.join("ab.md");
    std::fs::write(&markdown_path, render(&document))
        .with_context(|| format!("writing {}", markdown_path.display()))?;
    Ok(path)
}

fn options_side<'a>(a: &'a AbSide, b: &'a AbSide, index: usize) -> &'a AbSide {
    if index == 0 { a } else { b }
}

struct PreparedSide {
    toolchain: Toolchain,
    compiler: cogwasm_host::Compiler,
    serve: ServeCompiler,
}

impl PreparedSide {
    fn parts(&mut self) -> (&Toolchain, &cogwasm_host::Compiler, &mut ServeCompiler) {
        (&self.toolchain, &self.compiler, &mut self.serve)
    }
}

fn prepare_side(
    toolchain: &Toolchain,
    side: &AbSide,
    work: &Path,
    directory: &str,
) -> Result<PreparedSide> {
    let mut toolchain = toolchain.clone();
    if let Some(rustc) = &side.rustc {
        toolchain.rustc_wasm = rustc.clone();
    }
    let compiler = cogwasm_host::Compiler::load(
        &toolchain.rustc_wasm,
        toolchain.precompiled.as_deref(),
        false,
        Some(&toolchain.cache_dir),
    )
    .with_context(|| {
        format!(
            "loading {} for side {directory}",
            toolchain.rustc_wasm.display()
        )
    })?;
    let serve = ServeCompiler::new(
        compiler.engine(),
        compiler.module(),
        &toolchain.wild,
        vec![work.to_path_buf(), toolchain.sysroot.clone()],
    )?;
    Ok(PreparedSide {
        toolchain,
        compiler,
        serve,
    })
}

/// Render the A/B document as Markdown.
#[must_use]
pub fn render(document: &AbDocument) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let _ = writeln!(out, "# CogWasm interleaved A/B");
    let _ = writeln!(out);
    let _ = writeln!(out, "- A: `{}`", document.a);
    let _ = writeln!(out, "- B: `{}`", document.b);
    let _ = writeln!(
        out,
        "- rounds: {} (A, B, A, B per program)",
        document.rounds
    );
    let _ = writeln!(
        out,
        "- definitive: **{}** ({}; threshold {:.1}%)",
        if document.definitive { "yes" } else { "no" },
        if document.idle.qualified() {
            "idle-qualified"
        } else {
            "not idle-qualified: wall-clock A/B is not definitive"
        },
        document.idle.threshold_fraction * 100.0
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| program | class | metric | A min ms | B min ms | A/B | ratio spread | definitive |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | --- | ---: | ---: | ---: | --- | :---: |"
    );
    for program in &document.programs {
        for (label, metric) in [
            ("warm compile", &program.warm_compile),
            ("edit to result", &program.edit_to_result),
            ("cold compile", &program.cold_compile),
        ] {
            let _ = writeln!(
                out,
                "| {} | {} | {label} | {:.3} | {:.3} | {:.3}x | {:.3}..{:.3}x | {} |",
                program.name,
                program.class,
                metric.a_min_ms,
                metric.b_min_ms,
                metric.ratio,
                metric.ratio_min,
                metric.ratio_max,
                if program.definitive { "yes" } else { "no" }
            );
        }
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Ratios use the minimum of N rounds on each side; the spread is the min..max of the \\
         per-round A/B ratio. Below 1 means A was faster. An unqualified run is marked \\
         non-definitive and must not be read as a regression."
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::MetricClass;

    #[test]
    fn interleaving_is_a_b_a_b() {
        assert_eq!(
            interleaving(3),
            [Side::A, Side::B, Side::A, Side::B, Side::A, Side::B]
        );
        assert!(interleaving(0).is_empty());
    }

    #[test]
    fn aggregation_uses_min_of_n_and_reports_the_ratio_spread() {
        let metric = aggregate(&[(10.0, 20.0), (8.0, 18.0), (12.0, 16.0)]).expect("metric");
        assert_eq!(metric.a_min_ms, 8.0);
        assert_eq!(metric.b_min_ms, 16.0);
        assert!((metric.ratio - 0.5).abs() < 1e-12);
        assert!((metric.ratio_min - 8.0 / 18.0).abs() < 1e-12);
        assert!((metric.ratio_max - 0.75).abs() < 1e-12);
    }

    #[test]
    fn aggregation_rejects_empty_or_zero_samples() {
        assert!(aggregate(&[]).is_none());
        assert!(aggregate(&[(0.0, 1.0)]).is_none());
        assert!(aggregate(&[(1.0, 0.0)]).is_none());
    }

    #[test]
    fn a_non_idle_document_is_not_definitive() {
        let document = AbDocument {
            schema: 1,
            generated_at: "now".to_owned(),
            suite: "quick".to_owned(),
            rounds: 1,
            a: "a".to_owned(),
            b: "b".to_owned(),
            idle: IdleEvidence::default(),
            definitive: false,
            programs: Vec::new(),
        };
        let text = render(&document);
        assert!(text.contains("definitive: **no**"));
        assert!(text.contains("not idle-qualified"));
    }

    #[test]
    fn metric_class_is_available_for_ab_samples() {
        assert_eq!(MetricClass::Wall.as_str(), "wall");
    }
}
