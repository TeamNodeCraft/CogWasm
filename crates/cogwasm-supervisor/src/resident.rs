//! The resident compiler backend: one Wasmtime engine and one loaded
//! `rustc.wasm` module for the process lifetime.
//!
//! When the module was built with the serve patch, one guest serves every job
//! through [`ServeSession`], keeping its metadata cache and allocator warm;
//! otherwise each job gets a fresh guest. [`ResidentCompiler::compile_cold`]
//! always takes the fresh-guest path, so the edit loop can compare the two.
//!
//! This is the in-process replacement for spawning `cogwasm-host raw` for
//! every build. It uses the same argument list and the same work-directory
//! layout as [`RealCompiler`](crate::real::RealCompiler), so the two backends
//! produce identical artifacts; the difference is that the multi-second
//! rustc.wasm module load happens once, in [`ResidentCompiler::new`].

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::time::Instant;

use cogwasm_host::Compiler as HostCompiler;
use cogwasm_host::ServeSession;
use cogwasm_host::compile::{CompileOutcome, Teardown};
use cogwasm_host::config::Allocation;
use cogwasm_host::module_supports_serve;

use crate::compiler::{CompileError, CompileOutput, Compiler};
use crate::job::JobSpec;
use crate::real::{RealCompilerConfig, raw_rustc_args};

/// A [`Compiler`] that keeps one engine and one rustc.wasm module alive.
///
/// [`RealCompilerConfig::host`] is unused: this backend calls the host library
/// directly instead of spawning the `cogwasm-host` binary.
pub struct ResidentCompiler {
    config: RealCompilerConfig,
    compiler: HostCompiler,
    /// Whether the loaded module can serve and `COGWASM_SERVE` did not disable
    /// it. The session itself is created lazily on the first compile.
    serving: bool,
    serve: Mutex<Option<ServeSession>>,
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
        let serving = module_supports_serve(compiler.module())
            && !matches!(std::env::var("COGWASM_SERVE").as_deref(), Ok("0"));
        Ok(Self {
            config,
            compiler,
            serving,
            serve: Mutex::new(None),
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

    /// Whether this backend is using one long-lived guest.
    #[must_use]
    pub fn is_serving(&self) -> bool {
        self.serving
    }

    /// Compile one job with a fresh per-job guest, even when the module can
    /// serve. The work directory is `key-cold`, so the warm and cold artifacts
    /// never share output or incremental state.
    ///
    /// # Errors
    ///
    /// Returns [`CompileError`] for the same reasons as [`Compiler::compile`].
    pub fn compile_cold(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError> {
        self.compile_with(job, key, true)
    }

    fn compile_with(
        &self,
        job: &JobSpec,
        key: &str,
        cold: bool,
    ) -> Result<CompileOutput, CompileError> {
        let started = Instant::now();
        let work_key = if cold {
            format!("{key}-cold")
        } else {
            key.to_owned()
        };
        let work = self.config.work_root.join(&work_key);
        fs::create_dir_all(&work).map_err(|error| {
            CompileError::Unavailable(format!("creating {}: {error}", work.display()))
        })?;
        let out = work.join("artifact.wasm");

        // A cold compile must not read any incremental state: not the warm
        // compile's, and not an earlier cold compile's, since the directory
        // is shared by every edit of one source.
        let mut job = job.clone();
        if cold && let Some(directory) = &job.incremental_dir {
            let directory = cold_incremental(directory);
            match fs::remove_dir_all(&directory) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(CompileError::Unavailable(format!(
                        "clearing {}: {error}",
                        directory.display()
                    )));
                }
            }
            job.incremental_dir = Some(directory);
        }

        let mut args = vec!["rustc".to_owned()];
        args.extend(raw_rustc_args(&self.config, &job, &out));
        let outcome = if cold {
            self.compiler
                .compile_args(&args, &self.config.linker, Teardown::Drop)
                .map_err(|error| CompileError::Rejected(format!("{error:#}")))?
        } else if self.serving {
            self.compile_served(&args)?
        } else {
            self.compiler
                .compile_args(&args, &self.config.linker, Teardown::Drop)
                .map_err(|error| CompileError::Rejected(format!("{error:#}")))?
        };

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

    fn compile_served(&self, args: &[String]) -> Result<CompileOutcome, CompileError> {
        let mut session = self.serve.lock().unwrap_or_else(PoisonError::into_inner);
        if session.as_ref().is_some_and(ServeSession::is_dead) {
            *session = None;
        }
        if session.is_none() {
            *session = Some(
                ServeSession::new(
                    self.compiler.engine(),
                    self.compiler.module(),
                    &self.config.linker,
                )
                .map_err(|error| {
                    CompileError::Unavailable(format!(
                        "starting the rustc.wasm serve session: {error:#}"
                    ))
                })?,
            );
        }
        let result = session
            .as_ref()
            .expect("the serve session was just created")
            .compile_args(args);
        if result.is_err() && session.as_ref().is_some_and(ServeSession::is_dead) {
            // A trap or exit cannot be retried; the next call starts a new
            // session. A plain rustc rejection keeps this one warm.
            *session = None;
        }
        result.map_err(|error| CompileError::Rejected(format!("{error:#}")))
    }
}

fn cold_incremental(directory: &Path) -> PathBuf {
    let mut name = directory.file_name().map_or_else(
        || std::ffi::OsString::from("incremental"),
        |name| name.to_os_string(),
    );
    name.push("-cold");
    directory.with_file_name(name)
}

impl Compiler for ResidentCompiler {
    fn id(&self) -> &str {
        &self.config.compiler_id
    }

    fn compile(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError> {
        self.compile_with(job, key, false)
    }

    fn compile_cold(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError> {
        ResidentCompiler::compile_cold(self, job, key)
    }
}

// The supervisor may compile several tiers from more than one thread; Wasmtime's
// Engine and Module are Send + Sync, and the serve session is behind a mutex.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ResidentCompiler>();
};
