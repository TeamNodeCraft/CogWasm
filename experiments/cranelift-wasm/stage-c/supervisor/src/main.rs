//! CLI for the reference supervisor.
//!
//! `build` uses the deterministic fake compiler (M1). `matrix` drives the real
//! pipeline through `stage-c-host raw` and prints compile time, artifact size
//! and cache state per tier (M2).

use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use cogwasm_supervisor::{
    ArtifactStore, FakeCompiler, JobSpec, RealCompiler, RealCompilerConfig, SourceSpec, Supervisor,
    Tier,
};
use serde::Serialize;

#[derive(Parser)]
#[command(
    name = "cogwasm-supervisor",
    about = "Reference host-side compile supervisor"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[expect(
    clippy::large_enum_variant,
    reason = "clap subcommand argument structs are parsed once per process"
)]
#[derive(Subcommand)]
enum Command {
    /// Print the cache key a job would be stored under.
    Key {
        #[arg(long)]
        job: PathBuf,
        /// Compiler identity used in the key.
        #[arg(long, default_value = "fake-0")]
        compiler: String,
    },
    /// Cache-check or build a job with the reference (fake) compiler.
    Build {
        #[arg(long)]
        job: PathBuf,
        #[arg(long)]
        store: PathBuf,
        /// Compiler identity used in the key.
        #[arg(long, default_value = "fake-0")]
        compiler: String,
    },
    /// Compile programs at every tier through the real pipeline.
    Matrix {
        #[arg(long)]
        host: PathBuf,
        #[arg(long)]
        rustc: PathBuf,
        #[arg(long)]
        sysroot: PathBuf,
        #[arg(long)]
        linker: PathBuf,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        /// Precompiled rustc.wasm, so the module load is not part of the time.
        #[arg(long)]
        precompiled: Option<PathBuf>,
        /// Root for per-build work directories.
        #[arg(long)]
        work: PathBuf,
        #[arg(long)]
        store: PathBuf,
        /// Compiler identity used in the keys (a rustc.wasm hash or version).
        #[arg(long)]
        compiler_id: String,
        /// Comma-separated tiers to build.
        #[arg(long, value_delimiter = ',', default_value = "fast,opt,small")]
        tiers: Vec<String>,
        /// Prepared program sources (each includes the benchmark prelude).
        #[arg(long, required = true, num_args = 1..)]
        programs: Vec<PathBuf>,
        /// Write the rows as JSON here.
        #[arg(long)]
        json: Option<PathBuf>,
    },
}

#[derive(Serialize)]
struct MatrixRow {
    program: String,
    tier: String,
    key: String,
    bytes: u64,
    compile_ms: Option<u64>,
    hit: bool,
}

#[derive(Serialize)]
struct MatrixReport {
    rows: Vec<MatrixRow>,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Key { job, compiler } => {
            let job = load_job(&job)?;
            println!("{}", ArtifactStore::key_for(&job, &compiler)?);
        }
        Command::Build {
            job,
            store,
            compiler,
        } => build_fake(&load_job(&job)?, &store, &compiler)?,
        Command::Matrix {
            host,
            rustc,
            sysroot,
            linker,
            cache_dir,
            precompiled,
            work,
            store,
            compiler_id,
            tiers,
            programs,
            json,
        } => {
            let config = RealCompilerConfig {
                host,
                rustc,
                sysroot,
                linker,
                cache_dir,
                precompiled,
                work_root: work,
                compiler_id,
            };
            let report = matrix(&config, &store, &tiers, &programs)?;
            if let Some(path) = json {
                let encoded =
                    serde_json::to_vec_pretty(&report).context("serialising the matrix")?;
                std::fs::write(&path, encoded)
                    .with_context(|| format!("writing {}", path.display()))?;
            }
        }
    }
    Ok(())
}

fn load_job(path: &Path) -> Result<JobSpec> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    serde_json::from_reader(file).with_context(|| format!("parsing {}", path.display()))
}

fn build_fake(job: &JobSpec, store: &Path, compiler: &str) -> Result<()> {
    let supervisor = Supervisor::new(ArtifactStore::new(store), FakeCompiler::with_id(compiler));
    let outcome = supervisor.build(job)?;
    let artifact = outcome.artifact();
    println!(
        "{} {} ({} bytes)",
        if outcome.was_hit() { "hit" } else { "built" },
        artifact.key,
        artifact.bytes.len()
    );
    Ok(())
}

fn matrix(
    config: &RealCompilerConfig,
    store: &Path,
    tiers: &[String],
    programs: &[PathBuf],
) -> Result<MatrixReport> {
    if programs.is_empty() {
        bail!("matrix needs at least one --programs entry");
    }
    let tiers = tiers
        .iter()
        .map(|name| Tier::from_str(name).map_err(anyhow::Error::msg))
        .collect::<Result<Vec<_>>>()?;
    let supervisor = Supervisor::new(ArtifactStore::new(store), RealCompiler::new(config.clone()));
    let mut rows = Vec::new();
    println!(
        "{:<22} {:<6} {:<7} {:>9} {:>10}",
        "program", "tier", "state", "bytes", "compile_ms"
    );
    for program in programs {
        let source = SourceSpec::new(program, crate_name(program)?);
        for tier in &tiers {
            let mut job = JobSpec::new(source.clone(), *tier);
            if *tier == Tier::Fast {
                job.incremental_dir = Some(
                    config
                        .work_root
                        .join("incremental")
                        .join(&source.crate_name),
                );
            }
            let outcome = supervisor.build(&job)?;
            let artifact = outcome.artifact();
            println!(
                "{:<22} {:<6} {:<7} {:>9} {:>10}",
                source.crate_name,
                tier.as_str(),
                if outcome.was_hit() { "hit" } else { "built" },
                artifact.bytes.len(),
                artifact
                    .meta
                    .compile_ms
                    .map_or_else(|| "-".to_owned(), |ms| ms.to_string())
            );
            rows.push(MatrixRow {
                program: source.crate_name.clone(),
                tier: tier.as_str().to_owned(),
                key: artifact.key.clone(),
                bytes: artifact.meta.bytes,
                compile_ms: artifact.meta.compile_ms,
                hit: outcome.was_hit(),
            });
        }
    }
    Ok(MatrixReport { rows })
}

fn crate_name(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_owned)
        .with_context(|| format!("{} has no file stem", path.display()))
}
