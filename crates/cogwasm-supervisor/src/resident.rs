//! The resident compiler backend: one Wasmtime engine and one loaded
//! `rustc.wasm` module for the process lifetime, with a fresh guest store per
//! job.
//!
//! This is the in-process replacement for spawning `cogwasm-host raw` for
//! every build. It uses the same argument list and the same work-directory
//! layout as [`RealCompiler`](crate::real::RealCompiler), so the two backends
//! produce identical artifacts; the difference is that the multi-second
//! rustc.wasm module load happens once, in [`ResidentCompiler::new`].

use std::fs;
use std::time::Instant;

use cogwasm_host::Compiler as HostCompiler;
use cogwasm_host::compile::Teardown;
use cogwasm_host::config::Allocation;

use crate::compiler::{CompileError, CompileOutput, Compiler};
use crate::job::JobSpec;
use crate::real::{RealCompilerConfig, raw_rustc_args};

/// A [`Compiler`] that keeps one engine and one rustc.wasm module alive and
/// instantiates a fresh guest per job.
///
/// [`RealCompilerConfig::host`] is unused: this backend calls the host library
/// directly instead of spawning the `cogwasm-host` binary.
pub struct ResidentCompiler {
    config: RealCompilerConfig,
    compiler: HostCompiler,
    loads: usize,
}

impl ResidentCompiler {
    /// Load rustc.wasm once into a fresh engine.
    ///
    /// # Errors
    ///
    /// Returns [`CompileError::Unavailable`] when the engine cannot be created
    /// or rustc.wasm cannot be loaded.
    pub fn new(config: RealCompilerConfig) -> Result<Self, CompileError> {
        // One instance per job: the pooling allocator reuses a resident memory
        // slot instead of mapping and faulting in fresh memory every time
        // (−25% edit → result on the Linux CI runner; see Allocation).
        let compiler = HostCompiler::load_with(
            &config.rustc,
            config.precompiled.as_deref(),
            false,
            config.cache_dir.as_deref(),
            Allocation::from_env_or(Allocation::RESIDENT_DEFAULT),
        )
        .map_err(|error| {
            CompileError::Unavailable(format!("loading {}: {error:#}", config.rustc.display()))
        })?;
        Ok(Self {
            config,
            compiler,
            loads: 1,
        })
    }

    /// How long the one module load took, in milliseconds.
    #[must_use]
    pub fn module_load_ms(&self) -> f64 {
        self.compiler.module_load_ms
    }

    /// How many times rustc.wasm has been loaded. Always one for this backend;
    /// the resident test asserts it does not grow per job.
    #[must_use]
    pub fn loads(&self) -> usize {
        self.loads
    }
}

impl Compiler for ResidentCompiler {
    fn id(&self) -> &str {
        &self.config.compiler_id
    }

    fn compile(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError> {
        let started = Instant::now();
        let work = self.config.work_root.join(key);
        fs::create_dir_all(&work).map_err(|error| {
            CompileError::Unavailable(format!("creating {}: {error}", work.display()))
        })?;
        let out = work.join("artifact.wasm");

        let mut args = vec!["rustc".to_owned()];
        args.extend(raw_rustc_args(&self.config, job, &out));
        let outcome = self
            .compiler
            .compile_args(&args, &self.config.linker, Teardown::Drop)
            .map_err(|error| CompileError::Rejected(format!("{error:#}")))?;

        let bytes = fs::read(&out).map_err(|error| {
            CompileError::Unavailable(format!("reading {}: {error}", out.display()))
        })?;
        let compile_ms = u64::try_from(started.elapsed().as_millis()).ok();
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "link time is a non-negative wall-clock duration in whole milliseconds"
        )]
        let link_ms = outcome.link_ms.round() as u64;
        Ok(CompileOutput {
            bytes,
            compile_ms,
            link_ms: Some(link_ms),
        })
    }
}

// The supervisor may compile several tiers from more than one thread; Wasmtime's
// Engine and Module are Send + Sync, and this backend keeps no mutable state.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ResidentCompiler>();
};
