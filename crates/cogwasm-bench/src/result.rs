//! The versioned result document, the run orchestration and the derived
//! suite geomeans / V2 target table.

use crate::build::{
    BuildMeasurement, BuildRequest, Config, FuelCompiler, ServeCompiler, measure_build,
};
use crate::exec::{BenchRuntime, ExecMeasurement, InstructionMeasurement, Outcome};
use crate::llvm::{LlvmBuild, LlvmMeasurement, measure_llvm};
use crate::programs::{Program, Suite, ensure_dependencies, prepare_programs};
use crate::report;
use crate::size::SizeBreakdown;
use crate::stats::{geomean, median};
use crate::toolchain::{Toolchain, ToolchainFootprint, measure_footprint};
use crate::util;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Version of the result-document layout.
pub const SCHEMA: u32 = 1;

/// Machine facts recorded before and after a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MachineInfo {
    pub os: String,
    pub arch: String,
    pub cores: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_before: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_after: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_warning: Option<String>,
}

/// Compiler identity and load facts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilerInfo {
    pub rustc_wasm_sha256: String,
    pub rustc_wasm_bytes: u64,
    pub git_commit: String,
    pub git_subject: String,
    pub rustc_version: String,
    pub wasmtime_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precompiled: Option<String>,
    pub precompiled_source: String,
    pub precompiled_load_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precompiled_fallback_reason: Option<String>,
    pub module_load_ms: f64,
    pub serve_start_ms: f64,
}

/// Run settings, so a result explains how to reproduce itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub runs: usize,
    pub configs: Vec<String>,
    pub programs: Vec<String>,
    pub seeds: Vec<u64>,
    pub fuel: bool,
    pub out_dir: String,
}

/// One config's measurements for one program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigResult {
    pub build: BuildMeasurement,
    pub size: SizeBreakdown,
    pub exec: ExecMeasurement,
    pub instructions: InstructionMeasurement,
}

/// LLVM baselines, shared by every config of a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LlvmResult {
    pub o0: LlvmBuild,
    pub o2: LlvmBuild,
    pub oz: LlvmBuild,
    pub exec: BTreeMap<String, ExecMeasurement>,
    pub instructions: BTreeMap<String, InstructionMeasurement>,
}

/// One program's measurements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramResult {
    pub name: String,
    pub class: String,
    pub source: String,
    pub source_lines: usize,
    pub source_bytes: u64,
    pub configs: BTreeMap<String, ConfigResult>,
    pub llvm: LlvmResult,
    pub correct: bool,
    pub outcomes: BTreeMap<String, Outcome>,
}

/// Suite geomeans for one config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeomeanSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_ratio_o2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_ratio_oz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_compile_median_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_p50_median_ms: Option<f64>,
    pub programs: usize,
    pub exec_programs: usize,
}

/// Suite geomeans, one set per config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geomeans {
    pub by_config: BTreeMap<String, GeomeanSet>,
}

/// One row of the V2 target table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2Target {
    pub axis: String,
    pub v1: String,
    pub target: String,
    pub measured: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub met: Option<bool>,
}

/// The complete versioned result document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchResult {
    pub schema: u32,
    pub tool: String,
    pub version: String,
    pub generated_at: String,
    pub suite: String,
    pub machine: MachineInfo,
    pub compiler: CompilerInfo,
    pub toolchain: ToolchainFootprint,
    pub settings: Settings,
    pub programs: BTreeMap<String, ProgramResult>,
    pub geomeans: Geomeans,
    pub v2_targets: Vec<V2Target>,
    pub ok: bool,
}

/// What `run` wrote.
#[derive(Debug, Clone)]
pub struct RunArtifact {
    pub result: BenchResult,
    pub result_path: PathBuf,
    pub report_path: PathBuf,
}

/// `run` settings.
#[derive(Debug, Clone)]
pub struct RunOptions {
    pub suite: Suite,
    pub programs: Vec<String>,
    pub configs: Vec<String>,
    pub runs: usize,
    pub out_dir: PathBuf,
    pub fuel: bool,
}

/// Run the requested suite and write `result.json` + `report.md`.
///
/// # Errors
///
/// Returns an error on a missing toolchain, a failed measurement, or a
/// correctness mismatch. A failed run never writes a result document.
#[expect(
    clippy::too_many_lines,
    reason = "the run is a flat sequence of setup, per-program measurement and output"
)]
pub fn run(toolchain: &Toolchain, options: &RunOptions) -> Result<RunArtifact> {
    if options.runs == 0 {
        bail!("--runs must be at least 1");
    }
    toolchain.validate()?;
    let load_before = util::load_average();
    let configs = parse_configs(&options.configs)?;
    let programs = prepare_programs(toolchain, options.suite, &options.programs)?;
    let seeds = crate::programs::load_manifest(toolchain)?.seeds;
    let dependencies = ensure_dependencies(toolchain, &programs)?;
    let footprint = measure_footprint(toolchain)?;
    let out_dir = util::absolute(&options.out_dir)?;
    std::fs::create_dir_all(&out_dir).with_context(|| format!("creating {}", out_dir.display()))?;
    // Builds run in one fixed directory, not under `out_dir`: rustc's work
    // (and so its fuel) depends on the length of the paths it handles, so a
    // per-run directory would make fuel differ between otherwise equal runs.
    let work = util::absolute(&toolchain.work_root.join("bench/.work"))?;
    crate::build::clear_directory(&work)?;
    std::fs::create_dir_all(&work)?;
    let compiler = cogwasm_host::Compiler::load(
        &toolchain.rustc_wasm,
        toolchain.precompiled.as_deref(),
        false,
        Some(&toolchain.cache_dir),
    )
    .with_context(|| format!("loading {}", toolchain.rustc_wasm.display()))?;
    let serve = ServeCompiler::new(
        compiler.engine(),
        compiler.module(),
        &toolchain.wild,
        vec![work.clone(), toolchain.sysroot.clone()],
    )?;
    let mut serve = serve;
    let fuel = if options.fuel {
        Some(FuelCompiler::load(toolchain)?)
    } else {
        None
    };
    let runtime = BenchRuntime::new("bench")?;
    let mut measured = BTreeMap::new();
    for program in &programs {
        eprintln!(
            "[cogwasm-bench] {} ({}, {} lines)",
            program.def.name, program.def.class, program.lines
        );
        let llvm = measure_llvm(
            toolchain,
            program,
            &dependencies,
            &work,
            options.runs,
            &seeds,
            &runtime,
        )?;
        let llvm = measure_llvm_instructions(&runtime, program, &work, &seeds, llvm)?;
        let mut config_results = BTreeMap::new();
        for config in &configs {
            eprintln!("[cogwasm-bench]   config {}", config.name);
            let mut request = BuildRequest {
                toolchain,
                compiler: &compiler,
                serve: &mut serve,
                fuel: fuel.as_ref(),
                program,
                config,
                dependencies: &dependencies,
                work: work.clone(),
                runs: options.runs,
                seeds: &seeds,
            };
            let built = measure_build(&mut request)?;
            let bytes = std::fs::read(&built.wasm)
                .with_context(|| format!("reading {}", built.wasm.display()))?;
            let exec = runtime
                .execute(&bytes, &seeds, options.runs)
                .with_context(|| format!("executing {} / {}", program.def.name, config.name))?;
            let instructions = runtime
                .measure_instructions(&bytes, &seeds, &exec.outcomes)
                .with_context(|| {
                    format!(
                        "counting instructions for {} / {}",
                        program.def.name, config.name
                    )
                })?;
            check_outcomes(&program.def.name, &exec.outcomes, &llvm.exec)?;
            config_results.insert(
                config.name.clone(),
                ConfigResult {
                    build: built.measurement,
                    size: crate::size::analyze_module(&bytes)?,
                    exec,
                    instructions,
                },
            );
        }
        let outcomes = config_results
            .values()
            .next()
            .map(|result| result.exec.outcomes.clone())
            .unwrap_or_default();
        measured.insert(
            program.def.name.clone(),
            ProgramResult {
                name: program.def.name.clone(),
                class: program.def.class.clone(),
                source: program.def.source.clone(),
                source_lines: program.lines,
                source_bytes: program.bytes,
                configs: config_results,
                llvm,
                correct: true,
                outcomes,
            },
        );
    }
    let load_after = util::load_average();
    let cores = util::cores();
    let load_warning = load_after.filter(|load| *load > f64::from(u32::try_from(cores).unwrap_or(u32::MAX)) / 2.0).map(|load| {
        format!(
            "load average {load:.2} is above half the {cores} cores; wall-clock numbers may be noisy"
        )
    });
    let machine = MachineInfo {
        os: format!("{} {}", std::env::consts::OS, std::env::consts::ARCH),
        arch: std::env::consts::ARCH.to_owned(),
        cores,
        cpu: cpu_brand(),
        load_before,
        load_after,
        load_warning,
    };
    let (git_commit, git_subject) = util::git_identity(&toolchain.root);
    let compiler_info = CompilerInfo {
        rustc_wasm_sha256: footprint.rustc_wasm_sha256.clone(),
        rustc_wasm_bytes: footprint.rustc_wasm_bytes,
        git_commit,
        git_subject,
        rustc_version: toolchain
            .native_rustc_version()
            .unwrap_or_else(|_| "unknown".to_owned()),
        wasmtime_version: cogwasm_host::wasmtime_version(),
        precompiled: toolchain
            .precompiled
            .as_ref()
            .map(|path| path.display().to_string()),
        precompiled_source: compiler.source.as_str().to_owned(),
        precompiled_load_ms: compiler.module_load_ms,
        precompiled_fallback_reason: compiler.fallback_reason.clone(),
        module_load_ms: compiler.module_load_ms,
        serve_start_ms: serve.start_ms,
    };
    let config_names = configs.iter().map(|config| config.name.clone()).collect();
    let program_names = measured.keys().cloned().collect();
    let settings = Settings {
        runs: options.runs,
        configs: config_names,
        programs: program_names,
        seeds,
        fuel: options.fuel,
        out_dir: out_dir.display().to_string(),
    };
    let geomeans = geomeans(&measured);
    let v2_targets = v2_targets(&measured, &geomeans);
    let result = BenchResult {
        schema: SCHEMA,
        tool: "cogwasm-bench".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        generated_at: util::format_utc(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |duration| duration.as_secs()),
        ),
        suite: options.suite.as_str().to_owned(),
        machine,
        compiler: compiler_info,
        toolchain: footprint,
        settings,
        programs: measured,
        geomeans,
        v2_targets,
        ok: true,
    };
    let result_path = out_dir.join("result.json");
    let report_path = out_dir.join("report.md");
    let encoded = serde_json::to_vec_pretty(&result).context("serialising the result")?;
    std::fs::write(&result_path, encoded)
        .with_context(|| format!("writing {}", result_path.display()))?;
    let markdown = report::render(&result);
    std::fs::write(&report_path, markdown)
        .with_context(|| format!("writing {}", report_path.display()))?;
    Ok(RunArtifact {
        result,
        result_path,
        report_path,
    })
}

fn parse_configs(names: &[String]) -> Result<Vec<Config>> {
    let names: Vec<String> = if names.is_empty() {
        vec!["default".to_owned()]
    } else {
        names.to_vec()
    };
    let mut configs = Vec::with_capacity(names.len());
    let mut seen = std::collections::BTreeSet::new();
    for name in names {
        if !seen.insert(name.clone()) {
            bail!("config {name:?} was given more than once");
        }
        configs.push(Config::parse(&name)?);
    }
    Ok(configs)
}

fn measure_llvm_instructions(
    runtime: &BenchRuntime,
    program: &Program,
    work: &Path,
    seeds: &[u64],
    mut llvm: LlvmMeasurement,
) -> Result<LlvmResult> {
    let mut instructions = BTreeMap::new();
    for name in ["o0", "o2"] {
        let path = work
            .join("llvm")
            .join(&program.def.name)
            .join(format!("{name}.wasm"));
        let bytes = std::fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
        let expected = llvm
            .exec
            .get(name)
            .with_context(|| format!("missing the LLVM {name} outcomes"))?;
        instructions.insert(
            name.to_owned(),
            runtime
                .measure_instructions(&bytes, seeds, &expected.outcomes)
                .with_context(|| format!("counting LLVM {name} instructions"))?,
        );
    }
    Ok(LlvmResult {
        o0: llvm.o0,
        o2: llvm.o2,
        oz: llvm.oz,
        exec: std::mem::take(&mut llvm.exec),
        instructions,
    })
}

fn check_outcomes(
    program: &str,
    clif: &BTreeMap<String, Outcome>,
    llvm: &BTreeMap<String, ExecMeasurement>,
) -> Result<()> {
    for (name, measurement) in llvm {
        if clif != &measurement.outcomes {
            bail!(
                "{program}: outputs differ between the CogWasm pipeline and LLVM {name}\n\
                 clif:    {clif:?}\n\
                 llvm {name}: {:?}",
                measurement.outcomes
            );
        }
    }
    Ok(())
}

/// Compute the suite geomeans, one set per config.
#[must_use]
#[expect(
    clippy::similar_names,
    clippy::excessive_nesting,
    clippy::cast_precision_loss,
    reason = "the geomean table is one flat walk over programs and configs; sizes are far below 2^53"
)]
pub fn geomeans(programs: &BTreeMap<String, ProgramResult>) -> Geomeans {
    let mut by_config = BTreeMap::new();
    let config_names: Vec<String> = programs
        .values()
        .next()
        .map(|program| program.configs.keys().cloned().collect())
        .unwrap_or_default();
    for config in config_names {
        let mut exec_ratios = Vec::new();
        let mut instruction_ratios = Vec::new();
        let mut size_o2 = Vec::new();
        let mut size_oz = Vec::new();
        let mut warm = Vec::new();
        let mut edit = Vec::new();
        let mut exec_programs = 0usize;
        for program in programs.values() {
            let Some(result) = program.configs.get(&config) else {
                continue;
            };
            let Some(llvm_exec) = program.llvm.exec.get("o2") else {
                continue;
            };
            let clif_ms = result.exec.exec_ms.median;
            let llvm_ms = llvm_exec.exec_ms.median;
            if llvm_ms >= 0.1 && clif_ms > 0.0 {
                exec_ratios.push(clif_ms / llvm_ms);
                exec_programs += 1;
                if let Some(llvm_instr) = program.llvm.instructions.get("o2") {
                    let ours = result.instructions.instructions.median;
                    let theirs = llvm_instr.instructions.median;
                    if ours > 0.0 && theirs > 0.0 {
                        instruction_ratios.push(ours / theirs);
                    }
                }
            }
            if result.size.total_bytes > 0 && program.llvm.o2.size.total_bytes > 0 {
                size_o2
                    .push(result.size.total_bytes as f64 / program.llvm.o2.size.total_bytes as f64);
            }
            if result.size.total_bytes > 0 && program.llvm.oz.size.total_bytes > 0 {
                size_oz
                    .push(result.size.total_bytes as f64 / program.llvm.oz.size.total_bytes as f64);
            }
            warm.push(result.build.warm_compile_ms.median);
            edit.push(result.build.edit.p50_ms);
        }
        by_config.insert(
            config,
            GeomeanSet {
                exec_ratio: geomean(exec_ratios),
                instructions_ratio: geomean(instruction_ratios),
                size_ratio_o2: geomean(size_o2),
                size_ratio_oz: geomean(size_oz),
                warm_compile_median_ms: median(&warm).ok(),
                edit_p50_median_ms: median(&edit).ok(),
                programs: programs.len(),
                exec_programs,
            },
        );
    }
    Geomeans { by_config }
}

#[expect(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    reason = "the target table is a flat list; module sizes are far below 2^53"
)]
fn v2_targets(programs: &BTreeMap<String, ProgramResult>, geomeans: &Geomeans) -> Vec<V2Target> {
    let default = geomeans.by_config.get("default");
    let large = programs.get("large").and_then(|program| {
        program
            .configs
            .get("default")
            .or_else(|| program.configs.values().next())
    });
    let small_edit = median(
        &programs
            .values()
            .filter(|program| program.class == "small")
            .filter_map(|program| {
                program
                    .configs
                    .get("default")
                    .or_else(|| program.configs.values().next())
                    .map(|result| result.build.edit.p50_ms)
            })
            .collect::<Vec<_>>(),
    )
    .ok();
    let size_median = median(
        &programs
            .values()
            .filter_map(|program| {
                program
                    .configs
                    .get("default")
                    .or_else(|| program.configs.values().next())
                    .map(|result| result.size.total_bytes as f64)
            })
            .collect::<Vec<_>>(),
    )
    .ok();
    let oz_median = median(
        &programs
            .values()
            .map(|program| program.llvm.oz.size.total_bytes as f64)
            .collect::<Vec<_>>(),
    )
    .ok();
    let edit_large = large.map(|result| result.build.edit.p50_ms);
    let cold_large = large.map(|result| result.build.cold_wall_ms.median);
    let targets = vec![
        V2Target {
            axis: "Edit -> result, one function changed, large program".to_owned(),
            v1: "2,191 ms p50, 2,224 ms p90 (98k-line generated program)".to_owned(),
            target: "<= 20 ms p50, <= 50 ms p99".to_owned(),
            measured: edit_large.map_or_else(
                || "not measured (run --suite large)".to_owned(),
                |value| format!("{value:.1} ms p50 (p90 in the edit table)"),
            ),
            met: edit_large.map(|value| value <= 20.0),
        },
        V2Target {
            axis: "Edit -> result, small program".to_owned(),
            v1: "25 ms (CI)".to_owned(),
            target: "<= 5 ms".to_owned(),
            measured: small_edit.map_or_else(
                || "not measured".to_owned(),
                |value| format!("{value:.1} ms median p50"),
            ),
            met: small_edit.map(|value| value <= 5.0),
        },
        V2Target {
            axis: "Clean checkout -> running, large program".to_owned(),
            v1: "18.7 s cold program build (toolchain prebuilt)".to_owned(),
            target: "<= 1 s warm store, <= 20 s cold".to_owned(),
            measured: cold_large.map_or_else(
                || "not measured (run --suite large)".to_owned(),
                |value| format!("{value:.0} ms cold program build"),
            ),
            met: cold_large.map(|value| value <= 20_000.0),
        },
        V2Target {
            axis: "Steady-state speed of the output".to_owned(),
            v1: "about 2.8x LLVM -O2 (executed wasm instructions)".to_owned(),
            target: "<= 1.0x LLVM -O2 on the suite".to_owned(),
            measured: default.and_then(|set| set.instructions_ratio).map_or_else(
                || "not measured".to_owned(),
                |value| format!("{value:.2}x instructions (geomean)"),
            ),
            met: default
                .and_then(|set| set.instructions_ratio)
                .map(|value| value <= 1.0),
        },
        V2Target {
            axis: "Language fidelity".to_owned(),
            v1: "real rustc; difftest on every push".to_owned(),
            target: "zero known wrong-result bugs".to_owned(),
            measured: format!(
                "{} programs matched LLVM -O0/-O2 on every seed",
                programs.len()
            ),
            met: Some(programs.values().all(|program| program.correct)),
        },
        V2Target {
            axis: "Same compiler for release".to_owned(),
            v1: "yes".to_owned(),
            target: "yes, deterministic and reproducible".to_owned(),
            measured: "the measured compiler is the checked-in pipeline".to_owned(),
            met: None,
        },
        V2Target {
            axis: "Where it runs".to_owned(),
            v1: "anywhere wasm runs, including the browser".to_owned(),
            target: "same, plus native via Wasmtime AOT".to_owned(),
            measured: "not measured by this tool".to_owned(),
            met: None,
        },
        V2Target {
            axis: "Binary size".to_owned(),
            v1: "median 46 KB on the suite".to_owned(),
            target: "<= LLVM -Oz after the size tier".to_owned(),
            measured: match (size_median, oz_median) {
                (Some(size), Some(oz)) => format!("{size:.0} B median vs LLVM -Oz {oz:.0} B"),
                _ => "not measured".to_owned(),
            },
            met: match (size_median, oz_median) {
                (Some(size), Some(oz)) if oz > 0.0 => Some(size <= oz),
                _ => None,
            },
        },
    ];
    targets
}

fn cpu_brand() -> Option<String> {
    if cfg!(target_os = "macos") {
        return std::process::Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
            .filter(|value| !value.is_empty());
    }
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines()
                .find(|line| line.starts_with("model name"))
                .and_then(|line| line.split_once(':'))
                .map(|(_, value)| value.trim().to_owned())
        })
}

/// Load a result document written by `run`.
///
/// # Errors
///
/// Returns an error when the file cannot be read, is not JSON, or has an
/// unsupported schema.
pub fn load_result(path: &Path) -> Result<BenchResult> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let result: BenchResult =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    if result.schema != SCHEMA {
        bail!(
            "{} has result schema {} (this tool reads {SCHEMA})",
            path.display(),
            result.schema
        );
    }
    Ok(result)
}

/// The path of the sibling Markdown report for a JSON result.
#[must_use]
pub fn report_path_for(result_path: &Path) -> PathBuf {
    result_path.with_extension("md")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::build::{EditSummary, FuelMeasurement, PhaseBreakdown};
    use crate::exec::{ExecMeasurement, InstructionMeasurement, Outcome};
    use crate::size::SizeBreakdown;
    use crate::stats::Metric;

    fn metric(value: f64, deterministic: bool) -> Metric {
        Metric::one(value, deterministic).expect("metric")
    }

    fn size(total: u64) -> SizeBreakdown {
        SizeBreakdown {
            total_bytes: total,
            code_bytes: total / 2,
            data_bytes: total / 4,
            custom_bytes: total / 8,
            name_bytes: total / 16,
            import_bytes: 10,
            export_bytes: 20,
            functions: 3,
            data_segments: 1,
            sections: Vec::new(),
        }
    }

    fn exec(value: f64) -> ExecMeasurement {
        ExecMeasurement {
            exec_ms: metric(value, false),
            instantiate_ms: metric(0.1, false),
            outcomes: BTreeMap::from([(
                "0".to_owned(),
                Outcome {
                    result: 1,
                    out_len: 8,
                    out_sha256: "00".to_owned(),
                },
            )]),
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "test values are small positive integers"
    )]
    fn instructions(value: f64) -> InstructionMeasurement {
        InstructionMeasurement {
            instructions: metric(value, true),
            blocks: 1,
            static_instructions: 1,
            per_seed: BTreeMap::from([("0".to_owned(), value as u64)]),
        }
    }

    /// A small but structurally complete result for report/compare tests.
    #[expect(
        clippy::too_many_lines,
        reason = "the fixture builder spells out every field so the shape is visible"
    )]
    pub(crate) fn minimal_result() -> BenchResult {
        let edit = EditSummary {
            p50_ms: 10.0,
            p90_ms: 11.0,
            min_ms: 9.0,
            max_ms: 11.0,
            build_ms: metric(8.0, false),
            link_ms: metric(1.0, false),
            build_no_link_ms: metric(7.0, false),
            swap_ms: metric(1.0, false),
            call_ms: metric(1.0, false),
            edits: Vec::new(),
            byte_identical_to_cold: true,
        };
        let phases = PhaseBreakdown {
            front_end: metric(2.0, false),
            expansion: metric(0.5, false),
            type_borrow_check: metric(0.5, false),
            monomorphisation: metric(1.0, false),
            codegen: metric(3.0, false),
            incremental: metric(1.0, false),
            link: metric(1.0, false),
            total: metric(9.0, false),
            raw: BTreeMap::new(),
        };
        let build = BuildMeasurement {
            cold_compile_ms: metric(9.0, false),
            cold_wall_ms: metric(10.0, false),
            warm_compile_ms: metric(10.0, false),
            warm_wall_ms: metric(11.0, false),
            peak_guest_memory_bytes: metric(1024.0 * 1024.0, false),
            host_rss_peak_bytes: Some(metric(2.0 * 1024.0 * 1024.0, false)),
            edit,
            phases,
            fuel: Some(FuelMeasurement {
                fuel: metric(1000.0, true),
                samples: vec![1000, 1000],
            }),
            incremental_dir_bytes: 4096,
            incremental_files: 4,
            intermediate_object_files: 2,
            intermediate_objects_bytes: 2048,
        };
        let config = ConfigResult {
            build,
            size: size(1000),
            exec: exec(1.0),
            instructions: instructions(100.0),
        };
        let llvm = LlvmResult {
            o0: LlvmBuild {
                compile_ms: metric(20.0, false),
                size: size(1200),
            },
            o2: LlvmBuild {
                compile_ms: metric(25.0, false),
                size: size(900),
            },
            oz: LlvmBuild {
                compile_ms: metric(26.0, false),
                size: size(800),
            },
            exec: BTreeMap::from([("o0".to_owned(), exec(2.0)), ("o2".to_owned(), exec(1.2))]),
            instructions: BTreeMap::from([
                ("o0".to_owned(), instructions(200.0)),
                ("o2".to_owned(), instructions(90.0)),
            ]),
        };
        let program = ProgramResult {
            name: "a_add".to_owned(),
            class: "small".to_owned(),
            source: "bench/programs/small/a_add.rs".to_owned(),
            source_lines: 26,
            source_bytes: 500,
            configs: BTreeMap::from([("default".to_owned(), config)]),
            llvm,
            correct: true,
            outcomes: BTreeMap::new(),
        };
        let set = GeomeanSet {
            exec_ratio: Some(0.83),
            instructions_ratio: Some(1.11),
            size_ratio_o2: Some(1.11),
            size_ratio_oz: Some(1.25),
            warm_compile_median_ms: Some(10.0),
            edit_p50_median_ms: Some(10.0),
            programs: 1,
            exec_programs: 1,
        };
        BenchResult {
            schema: SCHEMA,
            tool: "cogwasm-bench".to_owned(),
            version: "0.1.0".to_owned(),
            generated_at: "20260101T000000Z".to_owned(),
            suite: "quick".to_owned(),
            machine: MachineInfo {
                os: "macos aarch64".to_owned(),
                arch: "aarch64".to_owned(),
                cores: 14,
                cpu: Some("Apple M4 Pro".to_owned()),
                load_before: Some(1.0),
                load_after: Some(1.1),
                load_warning: None,
            },
            compiler: CompilerInfo {
                rustc_wasm_sha256: "0123456789abcdef".to_owned(),
                rustc_wasm_bytes: 100,
                git_commit: "0123456789abcdef".to_owned(),
                git_subject: "test".to_owned(),
                rustc_version: "rustc test".to_owned(),
                wasmtime_version: "50.0.2".to_owned(),
                precompiled: None,
                precompiled_source: "wasm".to_owned(),
                precompiled_load_ms: 1.0,
                precompiled_fallback_reason: None,
                module_load_ms: 1.0,
                serve_start_ms: 1.0,
            },
            toolchain: ToolchainFootprint {
                rustc_wasm_bytes: 100,
                rustc_wasm_sha256: "0123456789abcdef".to_owned(),
                rustc_wasm_stripped_bytes: 80,
                rustc_wasm_gzip_bytes: 50,
                rustc_wasm_stripped_gzip_bytes: 40,
                cwasm_bytes: Some(60),
                sysroot: crate::toolchain::AssetSize {
                    bytes: 1000,
                    files: Some(2),
                },
                repl_assets: None,
            },
            settings: Settings {
                runs: 3,
                configs: vec!["default".to_owned()],
                programs: vec!["a_add".to_owned()],
                seeds: vec![0, 1, 42, u64::MAX],
                fuel: true,
                out_dir: "toolchain/work/bench/test".to_owned(),
            },
            programs: BTreeMap::from([("a_add".to_owned(), program)]),
            geomeans: Geomeans {
                by_config: BTreeMap::from([("default".to_owned(), set)]),
            },
            v2_targets: vec![V2Target {
                axis: "test".to_owned(),
                v1: "old".to_owned(),
                target: "new".to_owned(),
                measured: "measured".to_owned(),
                met: Some(true),
            }],
            ok: true,
        }
    }
}
