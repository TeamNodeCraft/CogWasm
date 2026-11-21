//! CLI for the reference supervisor.
//!
//! `build` uses the deterministic fake compiler (M1). `matrix` drives the real
//! pipeline through `stage-c-host raw` and prints compile time, artifact size
//! and cache state per tier (M2).

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand};
use cogwasm_supervisor::{
    Artifact, ArtifactStore, CompileOptions, FakeCompiler, JobSpec, Measurement, Objective, Preset,
    RealCompiler, RealCompilerConfig, Runtime, Scoreboard, SourceSpec, Supervisor, Swapper,
    WasmtimeRuntime,
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
        /// Option specs: a preset (`fast`, `opt`, `release`) or a
        /// comma-separated key=value list. Repeat the flag for several specs.
        #[arg(long = "spec", value_name = "SPEC")]
        specs: Vec<String>,
        /// Prepared program sources (each includes the benchmark prelude).
        #[arg(long, required = true, num_args = 1..)]
        programs: Vec<PathBuf>,
        /// Run each built artifact and record execution time.
        #[arg(long)]
        exec: bool,
        /// Execution samples per artifact (the first is a warm-up).
        #[arg(long, default_value_t = 5)]
        samples: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        /// Append measurements to this scoreboard JSON.
        #[arg(long)]
        record: Option<PathBuf>,
        /// Write the rows as JSON here.
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Print the best measured spec for a program.
    Select {
        #[arg(long)]
        profiles: PathBuf,
        #[arg(long)]
        program: String,
        /// exec, size or balanced.
        #[arg(long, default_value = "exec")]
        objective: String,
        #[arg(long, default_value_t = 3)]
        min_samples: usize,
        /// Fractional improvement required to switch away from --current.
        #[arg(long, default_value_t = 0.05)]
        margin: f64,
        #[arg(long)]
        current: Option<String>,
    },
    /// Swap demo (M3): serve T0, build T1 in the background, install it, verify.
    Demo {
        #[arg(long)]
        host: PathBuf,
        #[arg(long)]
        rustc: PathBuf,
        #[arg(long)]
        precompiled: Option<PathBuf>,
        #[arg(long)]
        sysroot: PathBuf,
        #[arg(long)]
        linker: PathBuf,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        work: PathBuf,
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        compiler_id: String,
        /// Bench source that exports `run(u64) -> u64` with no imports.
        #[arg(long)]
        source: PathBuf,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        /// Calls to serve on the live instance while T1 compiles.
        #[arg(long, default_value_t = 200)]
        serve_calls: usize,
        /// Toggle the benchmark edit probe before the T1 build.
        #[arg(long)]
        probe: bool,
    },
}

#[derive(Serialize)]
struct MatrixRow {
    program: String,
    spec: String,
    options: CompileOptions,
    key: String,
    bytes: u64,
    compile_ms: Option<u64>,
    exec_ms: Option<f64>,
    hit: bool,
}

#[derive(Clone, Copy)]
struct ExecConfig {
    samples: usize,
    seed: u64,
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
            specs,
            programs,
            exec,
            samples,
            seed,
            record,
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
            let exec = exec.then_some(ExecConfig { samples, seed });
            let report = matrix(&config, &store, &specs, &programs, exec, record.as_deref())?;
            if let Some(path) = json {
                let encoded =
                    serde_json::to_vec_pretty(&report).context("serialising the matrix")?;
                std::fs::write(&path, encoded)
                    .with_context(|| format!("writing {}", path.display()))?;
            }
        }
        Command::Select {
            profiles,
            program,
            objective,
            min_samples,
            margin,
            current,
        } => {
            let objective = match objective.as_str() {
                "exec" => Objective::Exec,
                "size" => Objective::Size,
                "balanced" => Objective::Balanced,
                other => bail!("unknown objective {other:?} (exec, size or balanced)"),
            };
            let current = match current {
                Some(text) => Some(CompileOptions::from_spec(&text).map_err(anyhow::Error::msg)?),
                None => None,
            };
            let board = Scoreboard::load(&profiles)?;
            match board.choose(&program, objective, min_samples, current.as_ref(), margin) {
                Some(spec) => println!("{}", spec.to_spec()),
                None => println!("no spec has {min_samples} samples for {program}"),
            }
        }
        Command::Demo {
            host,
            rustc,
            precompiled,
            sysroot,
            linker,
            cache_dir,
            work,
            store,
            compiler_id,
            source,
            seed,
            serve_calls,
            probe,
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
            demo(&config, &store, &source, seed, serve_calls, probe)?;
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
    specs: &[String],
    programs: &[PathBuf],
    exec: Option<ExecConfig>,
    record: Option<&Path>,
) -> Result<MatrixReport> {
    if programs.is_empty() {
        bail!("matrix needs at least one --programs entry");
    }
    let specs = if specs.is_empty() {
        vec!["fast".to_owned(), "opt".to_owned(), "release".to_owned()]
    } else {
        specs.to_vec()
    };
    let specs = specs
        .iter()
        .map(|text| {
            CompileOptions::from_spec(text)
                .map(|options| (text.clone(), options))
                .map_err(anyhow::Error::msg)
        })
        .collect::<Result<Vec<_>>>()?;
    let supervisor = Supervisor::new(ArtifactStore::new(store), RealCompiler::new(config.clone()));
    let runtime = exec.map(|_| WasmtimeRuntime::new());
    let mut board = match record {
        Some(path) if path.exists() => Scoreboard::load(path)?,
        Some(_) | None => Scoreboard::new(),
    };
    let mut rows = Vec::new();
    println!(
        "{:<22} {:<24} {:<7} {:>9} {:>10} {:>10}",
        "program", "spec", "state", "bytes", "compile_ms", "exec_ms"
    );
    for program in programs {
        let source = SourceSpec::new(program, crate_name(program)?);
        for (text, options) in &specs {
            let mut job = JobSpec::new(source.clone(), options.clone());
            if options.incremental {
                let key = ArtifactStore::key_for(&job, &config.compiler_id)?;
                job.incremental_dir = Some(config.work_root.join("incremental").join(key));
            }
            let outcome = supervisor.build(&job)?;
            let artifact = outcome.artifact();
            let exec_ms = match (exec, runtime.as_ref()) {
                (Some(settings), Some(runtime)) => {
                    match measure_exec(runtime, artifact, settings.seed, settings.samples) {
                        Ok(ms) => Some(ms),
                        Err(error) => {
                            eprintln!("exec {} {text} failed: {error}", source.crate_name);
                            None
                        }
                    }
                }
                _ => None,
            };
            if record.is_some() {
                board.record(
                    &source.crate_name,
                    Measurement {
                        spec: options.clone(),
                        compile_ms: artifact.meta.compile_ms,
                        exec_ms,
                        size_bytes: artifact.meta.bytes,
                    },
                );
            }
            println!(
                "{:<22} {:<24} {:<7} {:>9} {:>10} {:>10}",
                source.crate_name,
                text,
                if outcome.was_hit() { "hit" } else { "built" },
                artifact.bytes.len(),
                artifact
                    .meta
                    .compile_ms
                    .map_or_else(|| "-".to_owned(), |ms| ms.to_string()),
                exec_ms.map_or_else(|| "-".to_owned(), |ms| format!("{ms:.3}"))
            );
            rows.push(MatrixRow {
                program: source.crate_name.clone(),
                spec: text.clone(),
                options: options.clone(),
                key: artifact.key.clone(),
                bytes: artifact.meta.bytes,
                compile_ms: artifact.meta.compile_ms,
                exec_ms,
                hit: outcome.was_hit(),
            });
        }
    }
    if let Some(path) = record {
        board.save(path)?;
        println!("recorded measurements in {}", path.display());
    }
    Ok(MatrixReport { rows })
}

fn measure_exec(
    runtime: &WasmtimeRuntime,
    artifact: &Artifact,
    seed: u64,
    samples: usize,
) -> Result<f64> {
    let mut instance = runtime.instantiate(artifact, 0)?;
    let samples = samples.max(2);
    let mut times = Vec::with_capacity(samples - 1);
    for index in 0..samples {
        let started = Instant::now();
        instance.call(seed)?;
        if index > 0 {
            times.push(started.elapsed().as_secs_f64() * 1000.0);
        }
    }
    times.sort_by(f64::total_cmp);
    Ok(times[times.len() / 2])
}

fn demo(
    config: &RealCompilerConfig,
    store: &Path,
    source: &Path,
    seed: u64,
    serve_calls: usize,
    probe: bool,
) -> Result<()> {
    let supervisor = Supervisor::new(ArtifactStore::new(store), RealCompiler::new(config.clone()));
    let name = crate_name(source)?;
    let make_job = |options: CompileOptions| -> Result<JobSpec> {
        let mut job = JobSpec::new(SourceSpec::new(source.to_path_buf(), name.clone()), options);
        if job.options.incremental {
            let key = ArtifactStore::key_for(&job, &config.compiler_id)?;
            job.incremental_dir = Some(config.work_root.join("incremental").join(key));
        }
        Ok(job)
    };
    let runtime = Arc::new(WasmtimeRuntime::new());
    let mut swapper = Swapper::new(runtime.clone());

    let cold = supervisor.build(&make_job(Preset::Fast.options())?)?;
    let first = swapper.install(cold.artifact())?;
    let baseline = swapper.call(seed)?;
    println!(
        "T0 cold   gen {first}  {} bytes  {:>6} ms",
        cold.artifact().bytes.len(),
        ms(cold.artifact().meta.compile_ms)
    );

    if probe {
        toggle_probe(source)?;
    }
    let edit = Instant::now();
    let fast = supervisor.build(&make_job(Preset::Fast.options())?)?;
    let edit_to_t0 = edit.elapsed();
    let swap = Instant::now();
    let second = swapper.install(fast.artifact())?;
    let swap_t0 = swap.elapsed();
    let after_t0 = swapper.call(seed)?;

    let background = spawn_opt(config, store, source, &name);
    let mut served = 0;
    while served < serve_calls && !background.is_finished() {
        swapper.call(seed)?;
        served += 1;
    }
    let opt = background
        .join()
        .map_err(|_| anyhow!("the background build panicked"))??;
    let edit_to_t1 = edit.elapsed();
    let precompile = Instant::now();
    runtime.precompile(&opt)?;
    let precompile_t1 = precompile.elapsed();
    let swap = Instant::now();
    let third = swapper.install(&opt)?;
    let swap_t1 = swap.elapsed();
    let after_t1 = swapper.call(seed)?;

    println!(
        "T0 edit   gen {second}  {:>9}  swap {}",
        fmt_ms(edit_to_t0),
        fmt_ms(swap_t0)
    );
    println!(
        "T1 build  gen {third}  {:>9}  precompile {}  swap {}  served {served} calls on T0",
        fmt_ms(edit_to_t1),
        fmt_ms(precompile_t1),
        fmt_ms(swap_t1)
    );
    println!("results: cold {baseline}, T0 {after_t0}, T1 {after_t1}");
    if baseline != after_t0 || baseline != after_t1 {
        bail!("tiers disagree on run({seed})");
    }
    Ok(())
}

fn spawn_opt(
    config: &RealCompilerConfig,
    store: &Path,
    source: &Path,
    name: &str,
) -> std::thread::JoinHandle<Result<Artifact>> {
    let config = config.clone();
    let store = store.to_path_buf();
    let source = source.to_path_buf();
    let name = name.to_owned();
    std::thread::spawn(move || {
        let supervisor = Supervisor::new(ArtifactStore::new(store), RealCompiler::new(config));
        let mut job = JobSpec::preset(SourceSpec::new(source, name), Preset::Opt);
        job.request_id = Some("background-opt".to_owned());
        Ok(supervisor.build(&job)?.artifact().clone())
    })
}

fn toggle_probe(source: &Path) -> Result<()> {
    const FROM: &str = "const BENCH_EDIT_TWEAK: u64 = 0u64;";
    const TO: &str = "const BENCH_EDIT_TWEAK: u64 = 1u64;";
    let text =
        std::fs::read_to_string(source).with_context(|| format!("reading {}", source.display()))?;
    if !text.contains(FROM) {
        bail!("{} has no benchmark edit probe to toggle", source.display());
    }
    std::fs::write(source, text.replace(FROM, TO))
        .with_context(|| format!("writing {}", source.display()))
}

fn ms(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_owned(), |ms| ms.to_string())
}

fn fmt_ms(elapsed: std::time::Duration) -> String {
    format!("{:.1} ms", elapsed.as_secs_f64() * 1000.0)
}

fn crate_name(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_owned)
        .with_context(|| format!("{} has no file stem", path.display()))
}
