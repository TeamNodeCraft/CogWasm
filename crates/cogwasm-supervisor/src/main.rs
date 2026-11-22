//! CLI for the reference supervisor.
//!
//! `build` uses the deterministic fake compiler (M1). `matrix` drives the real
//! pipeline through `cogwasm-host raw` (the default `--backend host`) or the
//! in-process `--backend resident` and prints compile time, artifact size and
//! cache state per tier (M2).

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use cogwasm_supervisor::{
    Artifact, ArtifactStore, CompileOptions, Compiler, FakeCompiler, JobSpec, Measurement,
    Objective, Preset, RealCompiler, RealCompilerConfig, ResidentCompiler, Runtime, Scoreboard,
    SourceSpec, Supervisor, Swapper, WasmtimeRuntime, Workspace, apply_edit, copy_source,
    has_edit_probe, probe_value, set_probe_value,
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

/// Which real compiler backend the subcommand drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum Backend {
    /// Spawn `cogwasm-host raw` for each job (the historical behaviour).
    Host,
    /// Keep one loaded rustc.wasm in this process and instantiate per job.
    Resident,
}

impl Backend {
    fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::Resident => "resident",
        }
    }
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
        /// Which compiler backend to use.
        #[arg(long, value_enum, default_value = "host")]
        backend: Backend,
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
        /// Which compiler backend to use.
        #[arg(long, value_enum, default_value = "host")]
        backend: Backend,
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
    /// Measure edit -> result through the selected compiler backend.
    EditLoop {
        /// Which compiler backend to use.
        #[arg(long, value_enum, default_value = "resident")]
        backend: Backend,
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
        /// Source that exports `run(u64) -> u64` and contains the edit probe.
        #[arg(long)]
        source: PathBuf,
        /// Number of measured edits.
        #[arg(long, default_value_t = 10)]
        edits: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        /// Write the measurement as JSON here.
        #[arg(long)]
        json: Option<PathBuf>,
        /// Goal 2 target, in milliseconds.
        #[arg(long, default_value_t = 100.0)]
        target_ms: f64,
        /// After each edit, compile the same source in a fresh guest and
        /// require byte-identical output.
        #[arg(long)]
        verify_cold: bool,
    },
    /// Build a multi-crate workspace in dependency order.
    Workspace {
        #[arg(long)]
        manifest: PathBuf,
        /// Which compiler backend to use.
        #[arg(long, value_enum, default_value = "host")]
        backend: Backend,
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
        /// Spec for every crate.
        #[arg(long, default_value = "opt")]
        spec: String,
        #[arg(long)]
        json: Option<PathBuf>,
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

#[derive(Serialize)]
struct EditRow {
    edit: usize,
    probe: u64,
    build_ms: f64,
    swap_ms: f64,
    call_ms: f64,
    edit_to_result_ms: f64,
    link_ms: Option<u64>,
}

#[derive(Serialize)]
struct EditLoopReport {
    backend: &'static str,
    module_load_ms: Option<f64>,
    edits: Vec<EditRow>,
    median: f64,
    p90: f64,
    min: f64,
    max: f64,
    target_ms: f64,
    met: bool,
}

struct Stats {
    median: f64,
    p90: f64,
    min: f64,
    max: f64,
}

#[expect(
    clippy::too_many_lines,
    reason = "flat dispatch over the clap subcommands"
)]
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
            backend,
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
            let compiler = backend_compiler(&config, backend)?;
            let exec = exec.then_some(ExecConfig { samples, seed });
            let report = matrix(
                &config,
                compiler,
                &store,
                &specs,
                &programs,
                exec,
                record.as_deref(),
            )?;
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
            backend,
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
            let compiler = backend_compiler(&config, backend)?;
            demo(&config, compiler, &store, &source, seed, serve_calls, probe)?;
        }
        Command::EditLoop {
            backend,
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
            edits,
            seed,
            json,
            target_ms,
            verify_cold,
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
            let loaded = load_backend(&config, backend)?;
            edit_loop(
                &config,
                backend,
                loaded,
                &store,
                &source,
                edits,
                seed,
                json.as_deref(),
                target_ms,
                verify_cold,
            )?;
        }
        Command::Workspace {
            manifest,
            backend,
            host,
            rustc,
            precompiled,
            sysroot,
            linker,
            cache_dir,
            work,
            store,
            compiler_id,
            spec,
            json,
        } => {
            let config = RealCompilerConfig {
                host,
                rustc,
                sysroot,
                linker,
                cache_dir,
                precompiled,
                work_root: work.clone(),
                compiler_id,
            };
            let compiler = backend_compiler(&config, backend)?;
            build_workspace(&manifest, &spec, compiler, &store, &work, json.as_deref())?;
        }
    }
    Ok(())
}

fn load_job(path: &Path) -> Result<JobSpec> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    serde_json::from_reader(file).with_context(|| format!("parsing {}", path.display()))
}

/// A compiler backend with the resident module load timed separately.
struct LoadedBackend {
    compiler: Arc<dyn Compiler>,
    /// `None` for the host backend, which loads rustc.wasm per job.
    module_load_ms: Option<f64>,
    /// Whether the resident backend is using one long-lived guest.
    serving: bool,
}

/// Build the selected backend once, so a resident module load is reused by
/// every job (and every worker) in the subcommand.
fn load_backend(config: &RealCompilerConfig, backend: Backend) -> Result<LoadedBackend> {
    match backend {
        Backend::Host => Ok(LoadedBackend {
            compiler: Arc::new(RealCompiler::new(config.clone())),
            module_load_ms: None,
            serving: false,
        }),
        Backend::Resident => {
            let resident = ResidentCompiler::new(config.clone())?;
            let module_load_ms = resident.module_load_ms();
            let serving = resident.is_serving();
            Ok(LoadedBackend {
                compiler: Arc::new(resident),
                module_load_ms: Some(module_load_ms),
                serving,
            })
        }
    }
}

fn backend_compiler(config: &RealCompilerConfig, backend: Backend) -> Result<Arc<dyn Compiler>> {
    let loaded = load_backend(config, backend)?;
    if let Some(module_load_ms) = loaded.module_load_ms {
        eprintln!(
            "[cogwasm-supervisor] resident backend: rustc.wasm loaded in {module_load_ms:.1} ms"
        );
    }
    Ok(loaded.compiler)
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
    compiler: Arc<dyn Compiler>,
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
    let supervisor = Supervisor::new(ArtifactStore::new(store), compiler);
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
                let key = ArtifactStore::incremental_key_for(&job, &config.compiler_id)?;
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
    compiler: Arc<dyn Compiler>,
    store: &Path,
    source: &Path,
    seed: u64,
    serve_calls: usize,
    probe: bool,
) -> Result<()> {
    let supervisor = Supervisor::new(ArtifactStore::new(store), compiler.clone());
    let name = crate_name(source)?;
    let make_job = |options: CompileOptions| -> Result<JobSpec> {
        let mut job = JobSpec::new(SourceSpec::new(source.to_path_buf(), name.clone()), options);
        if job.options.incremental {
            let key = ArtifactStore::incremental_key_for(&job, &config.compiler_id)?;
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

    let background = spawn_opt(compiler, store, source, &name);
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

#[expect(
    clippy::too_many_arguments,
    reason = "the edit-loop command passes the compiler config and its measurement settings"
)]
fn edit_loop(
    config: &RealCompilerConfig,
    backend: Backend,
    loaded: LoadedBackend,
    store: &Path,
    source: &Path,
    edits: usize,
    seed: u64,
    json: Option<&Path>,
    target_ms: f64,
    verify_cold: bool,
) -> Result<()> {
    if edits == 0 {
        bail!("--edits must be at least 1");
    }
    if !target_ms.is_finite() || target_ms <= 0.0 {
        bail!("--target-ms must be a positive number");
    }

    let original =
        std::fs::read_to_string(source).with_context(|| format!("reading {}", source.display()))?;
    if !has_edit_probe(&original) {
        bail!(
            "{} has no `const BENCH_EDIT_TWEAK: u64 = ...;` benchmark edit probe",
            source.display()
        );
    }
    let initial_probe = probe_value(&original).with_context(|| {
        format!(
            "{} has a malformed `BENCH_EDIT_TWEAK` declaration",
            source.display()
        )
    })?;

    // Never edit the caller's file: the copy lives under the work root and
    // gets a fresh path per run, so old artifacts cannot turn an edit into a
    // cache hit.
    let copy = copy_source(source, &config.work_root)?;
    eprintln!("[cogwasm-supervisor] edit-loop source: {}", copy.display());
    match loaded.module_load_ms {
        Some(module_load_ms) => eprintln!(
            "[cogwasm-supervisor] {} backend: module load {module_load_ms:.1} ms",
            backend.as_str()
        ),
        None => eprintln!(
            "[cogwasm-supervisor] host backend: module load happens per job (not measured)"
        ),
    }
    let mode = if loaded.serving {
        "serving (one guest)"
    } else {
        "one instance per job"
    };
    if backend == Backend::Resident {
        eprintln!("[cogwasm-supervisor] resident backend: {mode}");
    } else {
        eprintln!("[cogwasm-supervisor] host backend: {mode}");
    }

    let LoadedBackend {
        compiler,
        module_load_ms,
        serving: _,
    } = loaded;
    let supervisor = Supervisor::new(ArtifactStore::new(store), compiler);
    let runtime = Arc::new(WasmtimeRuntime::new());
    let mut swapper = Swapper::new(runtime);
    let name = crate_name(&copy)?;
    let make_job = |options: CompileOptions| -> Result<JobSpec> {
        let mut job = JobSpec::new(SourceSpec::new(copy.clone(), name.clone()), options);
        if job.options.incremental {
            let key = ArtifactStore::incremental_key_for(&job, &config.compiler_id)?;
            job.incremental_dir = Some(config.work_root.join("incremental").join(key));
        }
        Ok(job)
    };

    let cold = supervisor.build(&make_job(Preset::Fast.options())?)?;
    let first = swapper.install(cold.artifact())?;
    let baseline = swapper.call(seed)?;
    println!(
        "cold      gen {first}  {} bytes  build {} ms  {}",
        cold.artifact().bytes.len(),
        ms(cold.artifact().meta.compile_ms),
        if cold.was_hit() { "hit" } else { "built" }
    );

    let mut text = std::fs::read_to_string(&copy)
        .with_context(|| format!("reading the edit-loop copy {}", copy.display()))?;
    let mut expected: HashMap<u64, u64> = HashMap::new();
    expected.insert(initial_probe, baseline);
    let mut rows = Vec::with_capacity(edits);
    let mut totals = Vec::with_capacity(edits);

    for offset in 0..edits {
        let index = offset + 1;
        let probe = if offset % 2 == 0 { 0 } else { 1 };
        let next = apply_edit(&text, index, probe)?;
        if next.as_bytes() == text.as_bytes() {
            bail!("edit {index} did not change the source bytes");
        }
        std::fs::write(&copy, &next)
            .with_context(|| format!("writing {} for edit {index}", copy.display()))?;
        text = next;

        let started = Instant::now();
        let job = make_job(Preset::Fast.options())?;
        let outcome = supervisor.build(&job)?;
        let build_ms = wall_ms(started);
        if outcome.was_hit() {
            bail!("edit {index} was a cache hit; every edit must be a cache miss");
        }
        if verify_cold {
            let cold = supervisor
                .compiler()
                .compile_cold(&job, &outcome.artifact().key)
                .map_err(anyhow::Error::new)
                .with_context(|| format!("compiling edit {index} in a fresh guest"))?;
            if cold.bytes != outcome.artifact().bytes {
                bail!(
                    "edit {index}: warm artifact is {} bytes but a cold compile is {} bytes; \
                     the serving guest did not produce a byte-identical wasm",
                    outcome.artifact().bytes.len(),
                    cold.bytes.len()
                );
            }
        }
        let link_ms = outcome.artifact().meta.link_ms;

        let started = Instant::now();
        swapper.install(outcome.artifact())?;
        let swap_ms = wall_ms(started);

        let started = Instant::now();
        let result = swapper.call(seed)?;
        let call_ms = wall_ms(started);

        if let Some(want) = expected.get(&probe) {
            if *want != result {
                bail!(
                    "edit {index}: run({seed}) = {result} with probe {probe}u64, expected {want}"
                );
            }
        } else {
            expected.insert(probe, result);
        }

        let edit_to_result_ms = build_ms + swap_ms + call_ms;
        println!(
            "edit {index:>2}  probe {probe}  build {build_ms:>7.1} ms  swap {swap_ms:>6.1} ms  \
             call {call_ms:>6.2} ms  edit-to-result {edit_to_result_ms:>7.1} ms  link {} ms",
            ms(link_ms)
        );
        rows.push(EditRow {
            edit: index,
            probe,
            build_ms,
            swap_ms,
            call_ms,
            edit_to_result_ms,
            link_ms,
        });
        totals.push(edit_to_result_ms);
    }

    let stats = Stats::new(&totals)?;
    let met = stats.median <= target_ms;
    let target = if target_ms.fract() == 0.0 {
        format!("{target_ms:.0}")
    } else {
        format!("{target_ms:.1}")
    };
    println!(
        "edit-to-result median {:.1} ms (p90 {:.1} ms, min {:.1} ms) over {edits} edits; \
         target {target} ms: {}",
        stats.median,
        stats.p90,
        stats.min,
        if met { "MET" } else { "MISSED" }
    );

    if let Some(path) = json {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let report = EditLoopReport {
            backend: backend.as_str(),
            module_load_ms,
            edits: rows,
            median: stats.median,
            p90: stats.p90,
            min: stats.min,
            max: stats.max,
            target_ms,
            met,
        };
        let encoded =
            serde_json::to_vec_pretty(&report).context("serialising the edit-loop report")?;
        std::fs::write(path, encoded).with_context(|| format!("writing {}", path.display()))?;
        eprintln!("[cogwasm-supervisor] edit-loop report: {}", path.display());
    }
    Ok(())
}

impl Stats {
    fn new(values: &[f64]) -> Result<Self> {
        if values.is_empty() {
            bail!("no edit measurements");
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(f64::total_cmp);
        let middle = sorted.len() / 2;
        let median = if sorted.len().is_multiple_of(2) {
            (sorted[middle - 1] + sorted[middle]) / 2.0
        } else {
            sorted[middle]
        };
        // Nearest-rank p90: the smallest sample at or above 90% of the run.
        let rank = (sorted.len() * 9).div_ceil(10).max(1);
        Ok(Self {
            median,
            p90: sorted[rank - 1],
            min: sorted[0],
            max: sorted[sorted.len() - 1],
        })
    }
}

fn wall_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

fn spawn_opt(
    compiler: Arc<dyn Compiler>,
    store: &Path,
    source: &Path,
    name: &str,
) -> std::thread::JoinHandle<Result<Artifact>> {
    let store = store.to_path_buf();
    let source = source.to_path_buf();
    let name = name.to_owned();
    std::thread::spawn(move || {
        let supervisor = Supervisor::new(ArtifactStore::new(store), compiler);
        let mut job = JobSpec::preset(SourceSpec::new(source, name), Preset::Opt);
        job.request_id = Some("background-opt".to_owned());
        Ok(supervisor.build(&job)?.artifact().clone())
    })
}

fn print_workspace(builds: &[cogwasm_supervisor::CrateBuild]) {
    println!(
        "{:<18} {:<7} {:<7} {:>9} {:>10}",
        "crate", "kind", "state", "bytes", "compile_ms"
    );
    for build in builds {
        println!(
            "{:<18} {:<7} {:<7} {:>9} {:>10}",
            build.name,
            build.kind.as_str(),
            if build.hit { "hit" } else { "built" },
            build.bytes,
            build
                .compile_ms
                .map_or_else(|| "-".to_owned(), |ms| ms.to_string())
        );
    }
}

fn build_workspace(
    manifest: &Path,
    spec: &str,
    compiler: Arc<dyn Compiler>,
    store: &Path,
    work_root: &Path,
    json: Option<&Path>,
) -> Result<()> {
    let workspace = Workspace::from_json(manifest)?;
    let options = CompileOptions::from_spec(spec).map_err(anyhow::Error::msg)?;
    let supervisor = Supervisor::new(ArtifactStore::new(store), compiler);
    let builds = workspace.build(&supervisor, &options, work_root)?;
    print_workspace(&builds);
    if let Some(path) = json {
        let encoded = serde_json::to_vec_pretty(&builds).context("serialising the workspace")?;
        std::fs::write(path, encoded).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}

fn toggle_probe(source: &Path) -> Result<()> {
    const FROM: &str = "const BENCH_EDIT_TWEAK: u64 = 0u64;";
    let text =
        std::fs::read_to_string(source).with_context(|| format!("reading {}", source.display()))?;
    if !text.contains(FROM) {
        bail!("{} has no benchmark edit probe to toggle", source.display());
    }
    let toggled = set_probe_value(&text, 1)?;
    std::fs::write(source, toggled).with_context(|| format!("writing {}", source.display()))
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
