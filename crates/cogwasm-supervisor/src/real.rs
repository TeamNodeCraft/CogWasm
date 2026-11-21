//! The real compiler backend (M2): drives the CogWasm host's `raw` mode.
//!
//! The supervisor stays agnostic: it hands the host the toolchain paths and the
//! rustc arguments for the job's tier, and reads the linked `.wasm` back. The
//! host owns instantiation, the WASI sandbox and the Wild link.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use crate::compiler::{CompileError, CompileOutput, Compiler};
use crate::job::JobSpec;

/// Everything the host needs to run one build.
#[derive(Debug, Clone)]
pub struct RealCompilerConfig {
    /// The `cogwasm-host` binary.
    pub host: PathBuf,
    /// The `rustc.wasm` to run.
    pub rustc: PathBuf,
    /// The pure sysroot.
    pub sysroot: PathBuf,
    /// The Wild linker binary the guest spawns.
    pub linker: PathBuf,
    /// Wasmtime's compiled-module cache, if any.
    pub cache_dir: Option<PathBuf>,
    /// Precompiled `rustc.wasm` (`.cwasm`) to skip the module load.
    pub precompiled: Option<PathBuf>,
    /// Root under which each build gets a work directory.
    pub work_root: PathBuf,
    /// Identity of the compiler for cache keys (a hash or version string).
    pub compiler_id: String,
}

/// A [`Compiler`] that runs the real pipeline.
#[derive(Debug)]
pub struct RealCompiler {
    config: RealCompilerConfig,
}

impl RealCompiler {
    /// A real backend over `config`.
    #[must_use]
    pub fn new(config: RealCompilerConfig) -> Self {
        Self { config }
    }
}

impl Compiler for RealCompiler {
    fn id(&self) -> &str {
        &self.config.compiler_id
    }

    fn compile(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError> {
        let work = self.config.work_root.join(key);
        fs::create_dir_all(&work).map_err(|error| {
            CompileError::Unavailable(format!("creating {}: {error}", work.display()))
        })?;
        let out = work.join("artifact.wasm");

        let mut command = Command::new(&self.config.host);
        command
            .arg("raw")
            .arg("--rustc")
            .arg(&self.config.rustc)
            .arg("--sysroot")
            .arg(&self.config.sysroot)
            .arg("--linker")
            .arg(&self.config.linker);
        if let Some(precompiled) = &self.config.precompiled {
            command.arg("--precompiled").arg(precompiled);
        }
        if let Some(cache) = &self.config.cache_dir {
            command.arg("--cache-dir").arg(cache);
        }
        command
            .arg("--")
            .args(raw_rustc_args(&self.config, job, &out));

        let started = Instant::now();
        let result = command.output().map_err(|error| {
            CompileError::Unavailable(format!("running {}: {error}", self.config.host.display()))
        })?;
        let compile_ms = u64::try_from(started.elapsed().as_millis()).ok();
        if !result.status.success() {
            return Err(CompileError::Rejected(tail(&String::from_utf8_lossy(
                &result.stderr,
            ))));
        }
        let bytes = fs::read(&out).map_err(|error| {
            CompileError::Unavailable(format!("reading {}: {error}", out.display()))
        })?;
        Ok(CompileOutput {
            bytes,
            compile_ms,
            link_ms: None,
        })
    }
}

/// The rustc arguments after the host's own flags and the `--` separator.
///
/// Both the subprocess backend and the resident library backend build their
/// argv from this, so the two can never drift.
#[must_use]
pub fn raw_rustc_args(config: &RealCompilerConfig, job: &JobSpec, out: &Path) -> Vec<String> {
    let mut args = vec![
        "--target".to_owned(),
        job.target.clone(),
        "--sysroot".to_owned(),
        config.sysroot.display().to_string(),
        "--edition".to_owned(),
        job.source.edition.clone(),
        "--crate-name".to_owned(),
        job.source.crate_name.clone(),
        "--crate-type".to_owned(),
        job.kind.as_str().to_owned(),
        "-C".to_owned(),
        "panic=abort".to_owned(),
        "-Zcodegen-backend=cranelift".to_owned(),
        format!("-Clinker={}", config.linker.display()),
        "-Clinker-flavor=wasm-ld".to_owned(),
    ];
    args.extend(job.compiler_flags());
    for extern_spec in &job.externs {
        args.push("--extern".to_owned());
        args.push(format!(
            "{}={}",
            extern_spec.name,
            extern_spec.path.display()
        ));
    }
    args.push(job.source.path.display().to_string());
    args.push("-o".to_owned());
    args.push(out.display().to_string());
    args
}

/// The last few lines of a compiler's stderr, for the error message.
fn tail(stderr: &str) -> String {
    const LIMIT: usize = 2000;
    if stderr.len() <= LIMIT {
        return stderr.trim().to_owned();
    }
    let start = stderr.len() - LIMIT;
    format!("...{}", &stderr[start..])
}
