//! The rustc.wasm compile path, shared by the CLI and future resident hosts.
//!
//! This module owns the arguments rustc sees, the per-job guest instantiation,
//! the `playground.spawn` linker calls, and the timings the CLI reports. It
//! deliberately does no printing: the CLI keeps its byte-identical messages,
//! and a resident service (G2.1 step 2) can reuse the same path without a
//! terminal attached.

use anyhow::{anyhow, bail, Result};
use cogwasm_runtime::{prepare_instance, ArgvRecorder, SpawnSummary, StoreOptions, Wt};
use std::path::{Path, PathBuf};
use std::time::Instant;
use wasmtime::{Engine, Module};
use wasmtime_wasi::I32Exit;

use crate::precompiled::ModuleSource;

/// One rustc.wasm invocation: the source to compile, where to put the linked
/// output, and the compiler flags that come from the CLI.
#[derive(Clone, Debug)]
pub struct CompileRequest {
    /// The Rust source file rustc compiles.
    pub src: PathBuf,
    /// The linked `.wasm` rustc writes.
    pub out: PathBuf,
    /// The wasm32-unknown-unknown sysroot.
    pub sysroot: PathBuf,
    /// Native Wild, used by the guest's `playground.spawn` import.
    pub linker: PathBuf,
    /// Crate name passed as `--crate-name`.
    pub crate_name: String,
    /// `-Cincremental=DIR`, when incremental compilation is requested.
    pub incremental: Option<PathBuf>,
    /// `-Ccodegen-units=N`, when set.
    pub codegen_units: Option<u32>,
    /// `--extern NAME=PATH` arguments, in order.
    pub externs: Vec<String>,
}

/// The argv rustc.wasm receives, in the order the guest expects it.
#[must_use]
pub fn rustc_args(req: &CompileRequest) -> Vec<String> {
    let mut args = vec![
        "rustc".into(),
        req.src.display().to_string(),
        "--crate-name".into(),
        req.crate_name.clone(),
        "--crate-type=cdylib".into(),
        "--edition=2021".into(),
        "--target=wasm32-unknown-unknown".into(),
        "--sysroot".into(),
        req.sysroot.display().to_string(),
        "-Copt-level=2".into(),
        "-Cpanic=abort".into(),
        "-Zcodegen-backend=cranelift".into(),
        "-Clinker=wild".into(),
        "-Clinker-flavor=wasm-ld".into(),
        "-o".into(),
        req.out.display().to_string(),
    ];
    if let Some(dir) = &req.incremental {
        args.push(format!("-Cincremental={}", dir.display()));
    }
    if let Some(units) = req.codegen_units {
        args.push(format!("-Ccodegen-units={units}"));
    }
    for external in &req.externs {
        args.push("--extern".into());
        args.push(external.clone());
    }
    args
}

/// Timings and linker invocations from one rustc.wasm job.
#[derive(Clone, Debug)]
pub struct CompileOutcome {
    /// Time to build the store, linker and guest instance.
    pub instantiate_ms: f64,
    /// Time spent inside rustc.wasm's `_start`.
    pub rustc_total_ms: f64,
    /// Time spent in native linker subprocesses during the job.
    pub link_ms: f64,
    /// `rustc_total_ms - link_ms`, clamped at zero.
    pub compile_ms: f64,
    /// One entry per `playground.spawn` call the guest made.
    pub spawns: Vec<SpawnSummary>,
}

/// Run one rustc.wasm job on an already-loaded module.
///
/// A zero `_start` exit is success, matching rustc's WASI convention. A
/// non-zero linker status fails the job before any plugin is evaluated.
pub fn compile(engine: &Engine, module: &Module, req: &CompileRequest) -> Result<CompileOutcome> {
    let args = rustc_args(req);
    let t = Instant::now();
    let prepared = prepare_instance(
        engine,
        module,
        &args,
        &req.linker,
        "[cogwasm-host]",
        StoreOptions::default(),
        ArgvRecorder::default(),
    )?;
    let mut store = prepared.store;
    let instance = prepared.instance;
    let instantiate_ms = t.elapsed().as_secs_f64() * 1e3;
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .wt("rustc.wasm _start")?;

    let t = Instant::now();
    let call = start.call(&mut store, ());
    let rustc_total_ms = t.elapsed().as_secs_f64() * 1e3;
    match call {
        Ok(()) => {}
        Err(e) if e.downcast_ref::<I32Exit>().is_some_and(|x| x.0 == 0) => {}
        Err(e) => return Err(anyhow!("rustc.wasm exited with an error: {e}")),
    }
    let spawns = store
        .data()
        .recorder()
        .records()
        .iter()
        .map(|record| SpawnSummary {
            program: record.argv.first().cloned().unwrap_or_default(),
            args: record.argv.len().saturating_sub(1),
            ms: record.ms,
            status: record.status,
        })
        .collect::<Vec<_>>();
    let link_ms: f64 = spawns.iter().map(|s| s.ms).sum();
    if spawns.iter().any(|s| s.status != 0) {
        bail!("linker failed: {:?}", spawns);
    }

    // A 137 MiB compiled module + 1 GiB shared memory can take seconds to
    // tear down. A hot-reload host would keep one engine/module alive, so the
    // measured edit-to-result path deliberately excludes teardown.
    //
    // G2.1 step 2: the resident service must decide teardown explicitly
    // instead of leaking one store per job. Leaking is only acceptable for
    // this one-shot CLI path.
    std::mem::forget(store);

    Ok(CompileOutcome {
        instantiate_ms,
        rustc_total_ms,
        link_ms,
        compile_ms: (rustc_total_ms - link_ms).max(0.0),
        spawns,
    })
}

/// A resident Wasmtime engine plus one loaded rustc.wasm module.
///
/// [`Compiler::load`] does all the expensive setup once; [`Compiler::compile`]
/// then pays only for a fresh guest instantiation per job. The CLI uses the
/// accessors so it can keep its historical leak-until-exit behaviour.
pub struct Compiler {
    engine: Engine,
    module: Module,
    /// Wall time spent loading rustc.wasm.
    pub module_load_ms: f64,
    /// Whether the module came from the `.wasm` or a `.cwasm`.
    pub source: ModuleSource,
    /// Why a supplied precompiled artifact was not used.
    pub fallback_reason: Option<String>,
}

impl Compiler {
    /// Create the engine and load rustc.wasm, optionally from a precompiled
    /// artifact.
    ///
    /// The load time measures `load_rustc_module` only, exactly like the CLI's
    /// historical "rustc.wasm loaded ..." number; engine creation is cheap and
    /// happens before the timer.
    pub fn load(
        rustc: &Path,
        precompiled: Option<&Path>,
        verify_precompiled: bool,
        cache_dir: Option<&Path>,
    ) -> Result<Self> {
        let engine = crate::config::engine(cache_dir)?;
        let t = Instant::now();
        let loaded =
            crate::precompiled::load_rustc_module(&engine, rustc, precompiled, verify_precompiled)?;
        let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
        Ok(Self {
            engine,
            module: loaded.module,
            module_load_ms,
            source: loaded.source,
            fallback_reason: loaded.fallback_reason,
        })
    }

    /// Run one job on the resident module.
    pub fn compile(&self, req: &CompileRequest) -> Result<CompileOutcome> {
        compile(&self.engine, &self.module, req)
    }

    /// The resident Wasmtime engine.
    #[must_use]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// The loaded rustc.wasm module.
    #[must_use]
    pub fn module(&self) -> &Module {
        &self.module
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> CompileRequest {
        CompileRequest {
            src: PathBuf::from("work/lib.rs"),
            out: PathBuf::from("work/c_plugin.wasm"),
            sysroot: PathBuf::from("toolchain/sysroot"),
            linker: PathBuf::from("toolchain/target/wild/release/wild"),
            crate_name: "c_plugin".to_owned(),
            incremental: None,
            codegen_units: None,
            externs: Vec::new(),
        }
    }

    #[test]
    fn rustc_args_without_optional_fields_is_the_historical_list() {
        assert_eq!(
            rustc_args(&request()),
            vec![
                "rustc",
                "work/lib.rs",
                "--crate-name",
                "c_plugin",
                "--crate-type=cdylib",
                "--edition=2021",
                "--target=wasm32-unknown-unknown",
                "--sysroot",
                "toolchain/sysroot",
                "-Copt-level=2",
                "-Cpanic=abort",
                "-Zcodegen-backend=cranelift",
                "-Clinker=wild",
                "-Clinker-flavor=wasm-ld",
                "-o",
                "work/c_plugin.wasm",
            ]
        );
    }

    #[test]
    fn rustc_args_append_incremental_codegen_units_and_externs_in_order() {
        let req = CompileRequest {
            incremental: Some(PathBuf::from("/tmp/inc")),
            codegen_units: Some(16),
            externs: vec![
                "libm=/deps/liblibm.rlib".to_owned(),
                "other=/deps/libother.rlib".to_owned(),
            ],
            ..request()
        };
        assert_eq!(
            rustc_args(&req),
            vec![
                "rustc",
                "work/lib.rs",
                "--crate-name",
                "c_plugin",
                "--crate-type=cdylib",
                "--edition=2021",
                "--target=wasm32-unknown-unknown",
                "--sysroot",
                "toolchain/sysroot",
                "-Copt-level=2",
                "-Cpanic=abort",
                "-Zcodegen-backend=cranelift",
                "-Clinker=wild",
                "-Clinker-flavor=wasm-ld",
                "-o",
                "work/c_plugin.wasm",
                "-Cincremental=/tmp/inc",
                "-Ccodegen-units=16",
                "--extern",
                "libm=/deps/liblibm.rlib",
                "--extern",
                "other=/deps/libother.rlib",
            ]
        );
    }
}
