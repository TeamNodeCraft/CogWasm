//! LLVM reference builds made with the toolchain's native rustc.
//!
//! The flags are the same ones `bench/run.py` uses: `--edition=2021`,
//! `wasm32-unknown-unknown`, `cdylib`, `panic=abort`, `strip=debuginfo`, and
//! `-Copt-level=0/2/z`. The output is the only baseline this tool trusts; the
//! `CogWasm` artifact is compared against it before any timing is accepted.

use crate::exec::{BenchRuntime, ExecMeasurement};
use crate::idle::IdleMonitor;
use crate::programs::{DependencySet, Program, externs_for, search_paths};
use crate::size::{SizeBreakdown, analyze_module};
use crate::stats::{Metric, MetricClass};
use crate::toolchain::Toolchain;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

/// One LLVM optimisation level's result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LlvmBuild {
    pub compile_ms: Metric,
    pub size: SizeBreakdown,
}

/// All LLVM baselines for one program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LlvmMeasurement {
    pub o0: LlvmBuild,
    pub o2: LlvmBuild,
    pub oz: LlvmBuild,
    pub exec: BTreeMap<String, ExecMeasurement>,
}

/// Compile and measure the LLVM `-O0`, `-O2` and `-Oz` baselines.
///
/// # Errors
///
/// Returns an error when the native rustc cannot compile the program, writes
/// no module, or a module cannot be executed.
#[expect(
    clippy::too_many_arguments,
    reason = "the LLVM driver needs the toolchain, program, dependencies, work path, sample count, seeds, runtime and idle gate"
)]
pub fn measure_llvm(
    toolchain: &Toolchain,
    program: &Program,
    dependencies: &DependencySet,
    work: &Path,
    runs: usize,
    seeds: &[u64],
    runtime: &BenchRuntime,
    idle: &IdleMonitor,
    timed: bool,
) -> Result<LlvmMeasurement> {
    let externs = externs_for(&program.def, dependencies, "llvm")?;
    let search = search_paths(&dependencies.llvm);
    let mut builds = BTreeMap::new();
    let mut outputs = BTreeMap::new();
    let samples = if timed { runs } else { 1 };
    let window = timed.then(|| idle.window("llvm"));
    for (name, level) in [("o0", "0"), ("o2", "2"), ("oz", "z")] {
        let out = work
            .join("llvm")
            .join(&program.def.name)
            .join(format!("{name}.wasm"));
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let compile_ms = compile_llvm(toolchain, program, &externs, &search, level, &out, samples)?;
        let bytes = std::fs::read(&out).with_context(|| format!("reading {}", out.display()))?;
        let size = analyze_module(&bytes)?;
        builds.insert(name.to_owned(), LlvmBuild { compile_ms, size });
        outputs.insert(name.to_owned(), out);
    }
    let sample = window.map(|window| idle.finish(window));
    let qualified = sample.as_ref().is_some_and(|sample| sample.qualified);
    for build in builds.values_mut() {
        build.compile_ms = build.compile_ms.clone().with_idle_qualified(qualified);
    }
    let mut exec = BTreeMap::new();
    for name in ["o0", "o2"] {
        let out = outputs
            .get(name)
            .with_context(|| format!("missing the LLVM {name} output"))?;
        let bytes = std::fs::read(out).with_context(|| format!("reading {}", out.display()))?;
        exec.insert(
            name.to_owned(),
            runtime
                .execute(&bytes, seeds, samples, idle, timed)
                .with_context(|| format!("executing the LLVM {name} build"))?,
        );
    }
    let o0 = builds.remove("o0").context("missing the LLVM -O0 build")?;
    let o2 = builds.remove("o2").context("missing the LLVM -O2 build")?;
    let oz = builds.remove("oz").context("missing the LLVM -Oz build")?;
    Ok(LlvmMeasurement { o0, o2, oz, exec })
}

fn compile_llvm(
    toolchain: &Toolchain,
    program: &Program,
    externs: &[String],
    search: &[String],
    level: &str,
    out: &Path,
    runs: usize,
) -> Result<Metric> {
    remove_if_exists(out)?;
    let mut command = llvm_command(toolchain, program, externs, search, level, out);
    run_llvm(&mut command)?;
    let mut values = Vec::with_capacity(runs);
    for index in 0..runs {
        remove_if_exists(out)?;
        let mut command = llvm_command(toolchain, program, externs, search, level, out);
        let started = Instant::now();
        run_llvm(&mut command)
            .with_context(|| format!("LLVM -O{level} compile {index} of {}", program.def.name))?;
        values.push(started.elapsed().as_secs_f64() * 1e3);
    }
    if !out.is_file() {
        bail!(
            "LLVM -O{level} compile of {} wrote no {}",
            program.def.name,
            out.display()
        );
    }
    Metric::new(&values, MetricClass::Wall)
}

fn llvm_command(
    toolchain: &Toolchain,
    program: &Program,
    externs: &[String],
    search: &[String],
    level: &str,
    out: &Path,
) -> Command {
    let mut command = Command::new(&toolchain.native_rustc);
    command
        .arg("--edition=2021")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--crate-type")
        .arg("cdylib")
        .arg("-C")
        .arg("panic=abort")
        .arg("-C")
        .arg(format!("opt-level={level}"))
        .arg("-C")
        .arg("strip=debuginfo")
        .arg("--crate-name")
        .arg(&program.def.name);
    for external in externs {
        command.arg("--extern").arg(external);
    }
    for directory in search {
        command.arg("-L").arg(format!("dependency={directory}"));
    }
    command
        .arg(&program.source)
        .arg("-o")
        .arg(out)
        .env_remove("RUSTC");
    command
}

fn run_llvm(command: &mut Command) -> Result<()> {
    let output = command
        .output()
        .with_context(|| format!("running {command:?}"))?;
    if !output.status.success() {
        bail!(
            "LLVM compile failed ({}): {command:?}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("removing {}", path.display())),
    }
}
