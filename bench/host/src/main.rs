//! Benchmark host for the `CogWasm` performance suite.
//!
//! The host measures the product, not an experiment: it loads the stage-C
//! `rustc.wasm` through the same Wasmtime engine configuration and precompiled
//! artifact loader as the shipped host, compiles benchmark programs with the
//! compiler inside the module, and runs the linked modules with the difftest
//! corpus ABI (`run(seed) -> u64` plus the output buffer).
//!
//! Subcommands:
//!   startup       module compile (cold), cached load, precompiled load, instantiate
//!   compile       cold/warm/incremental compiles inside rustc.wasm, with phases
//!   run           execute one linked module and record the result and output
//!   sizes         total/code/data sizes of a linked module
//!
//! Every timing is a wall-clock measurement on the host. Compile time excludes
//! instantiation and linker subprocess time; those are reported separately.

use anyhow::{Context, Result, anyhow, bail};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use stage_c_host::precompiled::load_rustc_module;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wasmtime::{Caller, Engine, Extern, Instance, Linker, Memory, Module, ResourceLimiter, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::pipe::MemoryOutputPipe;
use wasmtime_wasi::{FsPerms, I32Exit, WasiCtxBuilder};

const PAGE_SIZE: u64 = 65_536;
const STDERR_CAPACITY: usize = 8 << 20;
/// Enough fuel for a compile so it never traps; rustc.wasm uses far less.
const FUEL_LIMIT: u64 = 1_000_000_000_000_000;

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(name = "bench-host", about = "CogWasm performance-suite host")]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
#[expect(
    clippy::large_enum_variant,
    reason = "clap subcommand argument structs are parsed once per process"
)]
enum CommandKind {
    /// Measure module load and instantiation in child processes.
    Startup(StartupArgs),
    /// One child measurement used by `startup`.
    StartupOnce(StartupOnceArgs),
    /// Compile a program inside rustc.wasm and measure the phases.
    Compile(CompileArgs),
    /// Execute a linked module through the difftest corpus ABI.
    Run(RunArgs),
    /// Report total, code and data sizes of a linked module.
    Sizes(SizesArgs),
}

#[derive(Args)]
struct StartupArgs {
    #[arg(long)]
    rustc: PathBuf,
    #[arg(long)]
    precompiled: Option<PathBuf>,
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    #[arg(long, default_value_t = 5)]
    runs: usize,
    /// `cold`, `cached`, `precompiled` or `all`.
    #[arg(long, default_value = "all")]
    mode: String,
    #[arg(long)]
    json: Option<PathBuf>,
}

#[derive(Args)]
struct StartupOnceArgs {
    #[arg(long)]
    rustc: PathBuf,
    #[arg(long)]
    precompiled: Option<PathBuf>,
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    #[arg(long)]
    mode: String,
}

#[derive(Args)]
struct CompileArgs {
    #[arg(long)]
    rustc: PathBuf,
    #[arg(long)]
    sysroot: PathBuf,
    #[arg(long)]
    linker: PathBuf,
    #[arg(long)]
    src: PathBuf,
    /// Shared prelude included by the benchmark program.
    #[arg(long)]
    prelude: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long)]
    work: PathBuf,
    #[arg(long)]
    crate_name: String,
    #[arg(long, default_value_t = 5)]
    runs: usize,
    /// Extra compiles with `-Ztime-passes`, for the phase table.
    #[arg(long, default_value_t = 1)]
    phase_runs: usize,
    /// `cold`, `cached` or `precompiled`.
    #[arg(long, default_value = "cached")]
    load: String,
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    #[arg(long)]
    precompiled: Option<PathBuf>,
    /// `NAME=PATH` (`--extern`), repeated for rlib and rmeta.
    #[arg(long = "extern")]
    externs: Vec<String>,
    /// Extra `-L dependency=DIR` search directory.
    #[arg(long = "search-path")]
    search_paths: Vec<String>,
    /// Extra argument passed to the compiler inside rustc.wasm. Used by the
    /// gap-attribution experiment to raise rustc's MIR optimisation level
    /// without changing the product's default.
    #[arg(long = "rustc-arg", allow_hyphen_values = true)]
    rustc_args: Vec<String>,
    /// Also measure incremental compiles with a one-line edit between runs.
    #[arg(long)]
    incremental: bool,
    #[arg(long)]
    incremental_dir: Option<PathBuf>,
    /// Measure deterministic compiler work with Wasmtime fuel after the timed
    /// runs. Fuel runs use a separate engine, so they never affect the wall
    /// clock samples above.
    #[arg(long)]
    fuel: bool,
    /// Fuel measurement compiles per program. Values are compared with each
    /// other, so fewer than two cannot show determinism.
    #[arg(long, default_value_t = 2)]
    fuel_runs: usize,
    #[arg(long)]
    json: Option<PathBuf>,
}

#[derive(Args)]
struct RunArgs {
    #[arg(long)]
    wasm: PathBuf,
    #[arg(long, default_value_t = 5)]
    runs: usize,
    #[arg(long, default_value = "0,1,42,170141183460469231731687303715884105727")]
    seeds: String,
    #[arg(long)]
    json: Option<PathBuf>,
}

#[derive(Args)]
struct SizesArgs {
    #[arg(long)]
    wasm: PathBuf,
    #[arg(long)]
    json: Option<PathBuf>,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        CommandKind::Startup(args) => startup(&args),
        CommandKind::StartupOnce(args) => startup_once(&args),
        CommandKind::Compile(args) => compile_command(&args),
        CommandKind::Run(args) => run_command(&args),
        CommandKind::Sizes(args) => sizes_command(&args),
    }
}

// ---------------------------------------------------------------------------
// Shared host state
// ---------------------------------------------------------------------------

struct Limits {
    max_pages: u64,
}

impl ResourceLimiter for Limits {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        let pages = u64::try_from(desired).unwrap_or(u64::MAX) / PAGE_SIZE;
        self.max_pages = self.max_pages.max(pages);
        Ok(true)
    }

    fn table_growing(
        &mut self,
        _current: usize,
        _desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        Ok(true)
    }
}

struct HostState {
    wasi: Option<WasiP1Ctx>,
    spawns: Arc<Mutex<Vec<SpawnRecord>>>,
    limits: Limits,
}

type HostStore = Store<HostState>;

/// Convert a `wasmtime::Result` (its own error type) into `anyhow::Result`
/// with a readable context, like the stage-C host does.
trait Wt<T> {
    fn wt(self, context: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, context: &'static str) -> Result<T> {
        self.map_err(|error| anyhow!("{context}: {error}"))
    }
}

#[derive(Clone)]
struct SpawnRecord {
    program: String,
    args: usize,
    ms: f64,
    status: i32,
}

#[derive(Serialize, Clone, Debug)]
struct SpawnSummary {
    program: String,
    args: usize,
    ms: f64,
    status: i32,
}

fn new_store(
    engine: &Engine,
    args: &[String],
    stderr: &MemoryOutputPipe,
    deterministic_random: bool,
) -> Result<HostStore> {
    let mut builder = WasiCtxBuilder::new();
    builder
        .inherit_stdout()
        .stderr(stderr.clone())
        .args(args)
        .env("RUST_MIN_STACK", "16777216")
        .env("RUST_BACKTRACE", "1");
    if deterministic_random {
        // rustc's std hash seeds come from WASI `random_get`. A fresh random
        // seed per compile changes a few hash-table probes (and therefore the
        // fuel count), so the fuel samples pin the generator. The timed runs
        // keep the product's real random source.
        builder
            .secure_random(wasmtime_wasi::Deterministic::new((0..=255_u8).collect()))
            .insecure_random(wasmtime_wasi::Deterministic::new(
                (0..=255_u8).rev().collect(),
            ))
            .insecure_random_seed(0);
    }
    if let Ok(value) = std::env::var("CG_CLIF_WASM_STRICT") {
        builder.env("CG_CLIF_WASM_STRICT", value);
    }
    // rustc sees the host filesystem at the same absolute paths so the native
    // Wild process can consume the linker arguments unchanged.
    builder
        .preopened_dir(Path::new("/"), "/", FsPerms::ReadWrite)
        .wt("preopening the host root as WASI /")?;
    Ok(Store::new(
        engine,
        HostState {
            wasi: Some(builder.build_p1()),
            spawns: Arc::new(Mutex::new(Vec::new())),
            limits: Limits { max_pages: 0 },
        },
    ))
}

fn add_playground_spawn(linker: &mut Linker<HostState>, linker_path: &Path) -> Result<()> {
    let linker_path = linker_path.to_path_buf();
    linker
        .func_wrap(
            "playground",
            "spawn",
            move |mut caller: Caller<'_, HostState>, argv: u32, argc: u32| -> i32 {
                match spawn_impl(&mut caller, argv, argc, &linker_path) {
                    Ok(code) => code,
                    Err(error) => {
                        eprintln!("[bench-host] spawn failed: {error:#}");
                        -1
                    }
                }
            },
        )
        .wt("defining playground::spawn")?;
    Ok(())
}

fn guest_memory(caller: &mut Caller<'_, HostState>) -> Result<Memory> {
    match caller.get_export("memory") {
        Some(Extern::Memory(memory)) => Ok(memory),
        Some(_) => bail!("rustc.wasm export `memory` is not a memory"),
        None => bail!("rustc.wasm does not export memory"),
    }
}

fn read_cstr(caller: &Caller<'_, HostState>, memory: &Memory, pointer: u32) -> Result<String> {
    let mut bytes = Vec::new();
    let mut offset = usize::try_from(pointer).context("guest string pointer")?;
    loop {
        let mut byte = [0u8; 1];
        memory
            .read(caller, offset, &mut byte)
            .with_context(|| format!("reading guest string at {pointer:#x}"))?;
        if byte[0] == 0 {
            break;
        }
        bytes.push(byte[0]);
        offset += 1;
        if bytes.len() > (1 << 20) {
            bail!("guest string at {pointer:#x} is longer than 1 MiB");
        }
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn spawn_impl(
    caller: &mut Caller<'_, HostState>,
    pointer_array: u32,
    argument_count: u32,
    linker: &Path,
) -> Result<i32> {
    let memory = guest_memory(&mut *caller)?;
    let mut raw = vec![0u8; usize::try_from(argument_count).context("spawn argument count")? * 4];
    memory
        .read(
            &*caller,
            usize::try_from(pointer_array).context("spawn pointer array")?,
            &mut raw,
        )
        .context("reading spawn pointer array")?;
    let mut linker_args = Vec::with_capacity(raw.len() / 4);
    for chunk in raw.as_chunks::<4>().0 {
        let pointer = u32::from_le_bytes(*chunk);
        linker_args.push(read_cstr(&*caller, &memory, pointer)?);
    }
    let started = Instant::now();
    let status = Command::new(linker)
        .args(linker_args.iter().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    let ms = elapsed_ms(started);
    let code = match status {
        Ok(status) if status.success() => 0,
        Ok(status) => status.code().unwrap_or(-1),
        Err(error) => {
            eprintln!("[bench-host] cannot run {}: {error}", linker.display());
            -1
        }
    };
    caller
        .data()
        .spawns
        .lock()
        .expect("spawn log lock")
        .push(SpawnRecord {
            program: linker_args.first().cloned().unwrap_or_default(),
            args: linker_args.len().saturating_sub(1),
            ms,
            status: code,
        });
    Ok(code)
}

fn prepare_instance(
    engine: &Engine,
    module: &Module,
    args: &[String],
    linker_path: &Path,
    fuel_limit: Option<u64>,
    deterministic_random: bool,
) -> Result<(HostStore, Instance, MemoryOutputPipe)> {
    let stderr = MemoryOutputPipe::new(STDERR_CAPACITY);
    let mut linker: Linker<HostState> = Linker::new(engine);
    p1::add_to_linker_sync(&mut linker, |state: &mut HostState| {
        state.wasi.as_mut().expect("WASI context present")
    })
    .wt("adding WASI p1 imports")?;
    add_playground_spawn(&mut linker, linker_path)?;
    let mut store = new_store(engine, args, &stderr, deterministic_random)?;
    if let Some(fuel) = fuel_limit {
        store.set_fuel(fuel).wt("setting fuel")?;
    }
    store.limiter(|state| &mut state.limits);
    let instance = linker
        .instantiate(&mut store, module)
        .wt("instantiating rustc.wasm")?;
    Ok((store, instance, stderr))
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1e3
}

#[expect(
    clippy::cast_precision_loss,
    reason = "peak memory is far below 2^53 bytes, so the median is exact"
)]
fn as_f64(value: u64) -> f64 {
    value as f64
}

fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).expect("timings are finite"));
    let middle = sorted.len().midpoint(0);
    if sorted.len().is_multiple_of(2) {
        sorted[middle - 1] + (sorted[middle] - sorted[middle - 1]) / 2.0
    } else {
        sorted[middle]
    }
}

fn stats(values: &[f64]) -> Stats {
    Stats {
        median_ms: median(values),
        min_ms: values.iter().copied().fold(f64::INFINITY, f64::min),
        max_ms: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        runs: values.len(),
    }
}

#[derive(Serialize, Clone)]
struct Stats {
    median_ms: f64,
    min_ms: f64,
    max_ms: f64,
    runs: usize,
}

fn abs(path: &Path) -> Result<PathBuf> {
    std::path::absolute(path)
        .map_err(|error| anyhow!("making {} absolute: {error}", path.display()))
}

fn absolute_arg(path: &Path) -> Result<PathBuf> {
    let absolute = abs(path)?;
    if !absolute.is_absolute() {
        bail!("{} did not become an absolute path", absolute.display());
    }
    Ok(absolute)
}

// ---------------------------------------------------------------------------
// startup: module load and instantiation
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct StartupOnceResult {
    mode: String,
    module_load_ms: f64,
    instantiate_ms: f64,
    source: String,
    fallback_reason: Option<String>,
    rustc_bytes: u64,
}

#[derive(Serialize)]
struct StartupResult {
    rustc: String,
    rustc_bytes: u64,
    rustc_sha256: String,
    wasmtime_version: String,
    modes: BTreeMap<String, ModeResult>,
}

#[derive(Serialize)]
struct ModeResult {
    module_load: Stats,
    instantiate: Stats,
    source: String,
    fallback_reason: Option<String>,
}

fn startup_once(args: &StartupOnceArgs) -> Result<()> {
    let rustc = absolute_arg(&args.rustc)?;
    let precompiled = args.precompiled.as_deref().map(absolute_arg).transpose()?;
    let cache_dir = args.cache_dir.as_deref().map(absolute_arg).transpose()?;
    let engine = match args.mode.as_str() {
        "cold" => stage_c_host::engine(None)?,
        "cached" => {
            let directory = cache_dir
                .as_deref()
                .ok_or_else(|| anyhow!("--mode cached needs --cache-dir"))?;
            std::fs::create_dir_all(directory)?;
            stage_c_host::engine(Some(directory))?
        }
        "precompiled" => stage_c_host::engine(cache_dir.as_deref())?,
        other => bail!("unknown startup mode {other}; use cold, cached or precompiled"),
    };
    let precompiled_arg = match args.mode.as_str() {
        "precompiled" => Some(
            precompiled
                .as_deref()
                .ok_or_else(|| anyhow!("--mode precompiled needs --precompiled"))?,
        ),
        _ => None,
    };
    let started = Instant::now();
    let loaded = load_rustc_module(&engine, &rustc, precompiled_arg, false)?;
    let module_load_ms = elapsed_ms(started);
    let module = loaded.module;
    let bytes = std::fs::metadata(&rustc)
        .with_context(|| format!("reading size of {}", rustc.display()))?
        .len();
    let started = Instant::now();
    let (store, _instance, _stderr) = prepare_instance(
        &engine,
        &module,
        &["rustc".to_owned()],
        Path::new("/bin/false"),
        None,
        false,
    )?;
    let instantiate_ms = elapsed_ms(started);
    println!(
        "{}",
        serde_json::to_string(&StartupOnceResult {
            mode: args.mode.clone(),
            module_load_ms,
            instantiate_ms,
            source: loaded.source.as_str().to_owned(),
            fallback_reason: loaded.fallback_reason,
            rustc_bytes: bytes,
        })?
    );
    // A compiled 137 MiB module takes seconds to tear down; a long-lived host
    // keeps it. Skip the destructors so teardown is not part of the sample.
    std::mem::forget(store);
    std::mem::forget(module);
    std::mem::forget(engine);
    std::process::exit(0);
}

fn run_startup_child(args: &StartupArgs, mode: &str) -> Result<StartupOnceResult> {
    let exe = std::env::current_exe().context("current executable")?;
    let mut command = Command::new(exe);
    command
        .arg("startup-once")
        .arg("--rustc")
        .arg(&args.rustc)
        .arg("--mode")
        .arg(mode);
    if let Some(cache_dir) = &args.cache_dir {
        command.arg("--cache-dir").arg(cache_dir);
    }
    if let Some(precompiled) = &args.precompiled {
        command.arg("--precompiled").arg(precompiled);
    }
    let output = command
        .output()
        .with_context(|| format!("running startup-once --mode {mode}"))?;
    if !output.status.success() {
        bail!(
            "startup-once --mode {mode} failed: {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout
        .lines()
        .rev()
        .find(|line| line.trim_start().starts_with('{'))
        .ok_or_else(|| anyhow!("startup-once printed no JSON: {stdout}"))?;
    serde_json::from_str(line).context("parsing startup-once JSON")
}

fn startup(args: &StartupArgs) -> Result<()> {
    let rustc = absolute_arg(&args.rustc)?;
    let precompiled = args.precompiled.as_deref().map(absolute_arg).transpose()?;
    let cache_dir = args.cache_dir.as_deref().map(absolute_arg).transpose()?;
    let modes: Vec<&str> = match args.mode.as_str() {
        "all" => {
            if precompiled.is_some() {
                vec!["cold", "cached", "precompiled"]
            } else {
                vec!["cold", "cached"]
            }
        }
        "cold" => vec!["cold"],
        "cached" => vec!["cached"],
        "precompiled" => vec!["precompiled"],
        other => bail!("unknown startup mode {other}"),
    };
    let owned = StartupArgs {
        rustc: rustc.clone(),
        precompiled,
        cache_dir,
        runs: args.runs,
        mode: args.mode.clone(),
        json: args.json.clone(),
    };
    // Prime the Wasmtime cache once so measured cached loads are hits. The
    // cold mode must not touch the cache directory at all.
    if modes.contains(&"cached") {
        let _ = run_startup_child(&owned, "cached");
    }
    let mut results = BTreeMap::new();
    for mode in modes {
        let mut loads = Vec::with_capacity(args.runs);
        let mut instantiates = Vec::with_capacity(args.runs);
        let mut source = String::new();
        let mut fallback_reason = None;
        for _ in 0..args.runs {
            let child = run_startup_child(&owned, mode)?;
            loads.push(child.module_load_ms);
            instantiates.push(child.instantiate_ms);
            source = child.source;
            fallback_reason = child.fallback_reason;
        }
        results.insert(
            mode.to_owned(),
            ModeResult {
                module_load: stats(&loads),
                instantiate: stats(&instantiates),
                source,
                fallback_reason,
            },
        );
    }
    let bytes = std::fs::read(&rustc).with_context(|| format!("reading {}", rustc.display()))?;
    let result = StartupResult {
        rustc: rustc.display().to_string(),
        rustc_bytes: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
        rustc_sha256: hex(&Sha256::digest(&bytes)),
        wasmtime_version: stage_c_host::wasmtime_version(),
        modes: results,
    };
    let text = serde_json::to_string_pretty(&result)?;
    if let Some(path) = &args.json {
        std::fs::write(path, &text)?;
    }
    println!("{text}");
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from(DIGITS[usize::from(byte >> 4)]));
        out.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    out
}

// ---------------------------------------------------------------------------
// compile: cold, warm and incremental inside rustc.wasm
// ---------------------------------------------------------------------------

struct CompileOptions {
    sysroot: PathBuf,
    linker: PathBuf,
    src: PathBuf,
    out: PathBuf,
    crate_name: String,
    externs: Vec<String>,
    search_paths: Vec<String>,
    incremental: Option<PathBuf>,
    time_passes: bool,
    extra_args: Vec<String>,
}

impl CompileOptions {
    fn rustc_args(&self) -> Vec<String> {
        let mut args = vec![
            "rustc".to_owned(),
            self.src.display().to_string(),
            "--crate-name".to_owned(),
            self.crate_name.clone(),
            "--crate-type=cdylib".to_owned(),
            "--edition=2021".to_owned(),
            "--target=wasm32-unknown-unknown".to_owned(),
            "--sysroot".to_owned(),
            self.sysroot.display().to_string(),
            "-Copt-level=2".to_owned(),
            "-Cpanic=abort".to_owned(),
            "-Cstrip=debuginfo".to_owned(),
            "-Zcodegen-backend=cranelift".to_owned(),
            "-Clinker=wild".to_owned(),
            "-Clinker-flavor=wasm-ld".to_owned(),
            "-o".to_owned(),
            self.out.display().to_string(),
        ];
        if let Some(directory) = &self.incremental {
            args.push(format!("-Cincremental={}", directory.display()));
        }
        for external in &self.externs {
            args.push("--extern".to_owned());
            args.push(external.clone());
        }
        for directory in &self.search_paths {
            args.push(format!("-Ldependency={directory}"));
        }
        if self.time_passes {
            args.push("-Ztime-passes".to_owned());
        }
        args.extend(self.extra_args.iter().cloned());
        args
    }
}

#[derive(Serialize, Clone)]
struct CompileRun {
    run: usize,
    instantiate_ms: f64,
    rustc_total_ms: f64,
    link_ms: f64,
    compile_ms: f64,
    peak_memory_bytes: u64,
    rc: i32,
    phases: BTreeMap<String, f64>,
    /// Wasmtime fuel consumed by `_start` when this run had fuel enabled.
    /// Instantiation fuel is subtracted so only compiler work is counted.
    fuel: Option<u64>,
}

/// One deterministic compiler-work sample.
#[derive(Serialize, Clone)]
struct FuelRun {
    run: usize,
    fuel: u64,
}

#[derive(Serialize)]
struct SourceInfo {
    path: String,
    bytes: u64,
    sha256: String,
}

#[derive(Serialize)]
struct CompileResult {
    program: String,
    crate_name: String,
    load: String,
    module_load_ms: f64,
    load_source: String,
    fallback_reason: Option<String>,
    rustc_sha256: String,
    source: SourceInfo,
    warmup: Option<CompileRun>,
    runs: Vec<CompileRun>,
    phase_runs: Vec<CompileRun>,
    incremental: Option<IncrementalResult>,
    /// A normal (non-incremental) rebuild after the incremental runs, so the
    /// `out` module used for execution and size is the product's normal build.
    final_build: Option<CompileRun>,
    /// Deterministic compiler work samples, when `--fuel` was requested.
    fuel_runs: Vec<FuelRun>,
    /// Median fuel across `fuel_runs`; exact because every sample must agree.
    compiler_fuel: Option<u64>,
    medians: BTreeMap<String, f64>,
    ok: bool,
}

#[derive(Serialize)]
struct IncrementalResult {
    directory: String,
    cold_ms: f64,
    cold_peak_memory_bytes: u64,
    first_edit_ms: f64,
    runs: Vec<CompileRun>,
    phase_runs: Vec<CompileRun>,
    medians: BTreeMap<String, f64>,
}

fn compile_medians(
    runs: &[CompileRun],
    incremental: Option<&IncrementalResult>,
) -> BTreeMap<String, f64> {
    let mut medians = BTreeMap::new();
    medians.insert(
        "instantiate_ms".to_owned(),
        median(&collect(runs, |run| run.instantiate_ms)),
    );
    medians.insert(
        "rustc_total_ms".to_owned(),
        median(&collect(runs, |run| run.rustc_total_ms)),
    );
    medians.insert(
        "link_ms".to_owned(),
        median(&collect(runs, |run| run.link_ms)),
    );
    medians.insert(
        "compile_ms".to_owned(),
        median(&collect(runs, |run| run.compile_ms)),
    );
    medians.insert(
        "peak_memory_bytes".to_owned(),
        median(&collect(runs, |run| as_f64(run.peak_memory_bytes))),
    );
    if let Some(value) = incremental {
        for (name, values) in [
            ("incremental_ms", collect(&value.runs, |run| run.compile_ms)),
            (
                "incremental_link_ms",
                collect(&value.runs, |run| run.link_ms),
            ),
            (
                "incremental_instantiate_ms",
                collect(&value.runs, |run| run.instantiate_ms),
            ),
            (
                "incremental_peak_memory_bytes",
                collect(&value.runs, |run| as_f64(run.peak_memory_bytes)),
            ),
        ] {
            medians.insert(name.to_owned(), median(&values));
        }
    }
    medians
}

fn run_phases(
    engine: &Engine,
    module: &Module,
    options: &CompileOptions,
    linker: &Path,
    count: usize,
) -> Result<Vec<CompileRun>> {
    let phase_options = CompileOptions {
        sysroot: options.sysroot.clone(),
        linker: options.linker.clone(),
        src: options.src.clone(),
        out: options.out.clone(),
        crate_name: options.crate_name.clone(),
        externs: options.externs.clone(),
        search_paths: options.search_paths.clone(),
        incremental: None,
        time_passes: true,
        extra_args: options.extra_args.clone(),
    };
    let mut runs = Vec::with_capacity(count);
    for index in 0..count {
        runs.push(compile_once(
            engine,
            module,
            &phase_options,
            linker,
            index,
            None,
            false,
        )?);
    }
    Ok(runs)
}

struct LoadedCompiler<'a> {
    engine: &'a Engine,
    module: &'a Module,
    load_source: String,
    fallback_reason: Option<String>,
    module_load_ms: f64,
}

fn compile_with(
    args: &CompileArgs,
    rustc: &Path,
    crate_source: &Path,
    out: &Path,
    work: &Path,
    loaded: &LoadedCompiler<'_>,
) -> Result<CompileResult> {
    let engine = loaded.engine;
    let module = loaded.module;
    let sysroot = absolute_arg(&args.sysroot)?;
    let linker = absolute_arg(&args.linker)?;
    let options = CompileOptions {
        sysroot,
        linker: linker.clone(),
        src: crate_source.to_path_buf(),
        out: out.to_path_buf(),
        crate_name: args.crate_name.clone(),
        externs: args.externs.clone(),
        search_paths: args.search_paths.clone(),
        incremental: None,
        time_passes: false,
        extra_args: args.rustc_args.clone(),
    };

    let warmup = compile_once(engine, module, &options, &linker, 0, None, false)?;
    let mut runs = Vec::with_capacity(args.runs);
    for index in 0..args.runs {
        runs.push(compile_once(
            engine, module, &options, &linker, index, None, false,
        )?);
    }
    let phase_runs = run_phases(engine, module, &options, &linker, args.phase_runs)?;
    let incremental = if args.incremental {
        let directory = args
            .incremental_dir
            .clone()
            .unwrap_or_else(|| work.join("incremental"));
        let directory = absolute_arg(&directory)?;
        clear_directory(&directory)?;
        std::fs::create_dir_all(&directory)?;
        Some(run_incremental(
            engine,
            module,
            &options,
            &linker,
            &directory,
            args.runs,
            args.phase_runs,
        )?)
    } else {
        None
    };

    let final_build = if incremental.is_some() {
        Some(compile_once(
            engine, module, &options, &linker, 0, None, false,
        )?)
    } else {
        None
    };
    let medians = compile_medians(&runs, incremental.as_ref());
    let source_bytes = std::fs::read(crate_source)?;
    let result = CompileResult {
        program: args.crate_name.clone(),
        crate_name: args.crate_name.clone(),
        load: args.load.clone(),
        module_load_ms: loaded.module_load_ms,
        load_source: loaded.load_source.clone(),
        fallback_reason: loaded.fallback_reason.clone(),
        rustc_sha256: sha256_file(rustc)?,
        source: SourceInfo {
            path: crate_source.display().to_string(),
            bytes: u64::try_from(source_bytes.len()).unwrap_or(u64::MAX),
            sha256: hex(&Sha256::digest(&source_bytes)),
        },
        warmup: Some(warmup),
        runs,
        phase_runs,
        incremental,
        final_build,
        fuel_runs: Vec::new(),
        compiler_fuel: None,
        medians,
        ok: true,
    };

    Ok(result)
}

fn compile_command(args: &CompileArgs) -> Result<()> {
    if args.runs == 0 {
        bail!("--runs must be at least 1");
    }
    if args.fuel && args.fuel_runs < 2 {
        bail!("--fuel needs --fuel-runs of at least 2 to compare two samples");
    }
    let rustc = absolute_arg(&args.rustc)?;
    let src = absolute_arg(&args.src)?;
    let out = absolute_arg(&args.out)?;
    let work = absolute_arg(&args.work)?;
    let precompiled = args.precompiled.as_deref().map(absolute_arg).transpose()?;
    let cache_dir = args.cache_dir.as_deref().map(absolute_arg).transpose()?;
    std::fs::create_dir_all(&work)?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let crate_source = work.join(format!("{}.rs", args.crate_name));
    prepare_source(&src, &args.prelude, &crate_source)?;

    let (engine, load_source, fallback_reason, module, module_load_ms) = load_compiler(
        &rustc,
        precompiled.as_deref(),
        cache_dir.as_deref(),
        &args.load,
    )?;

    let loaded = LoadedCompiler {
        engine: &engine,
        module: &module,
        load_source,
        fallback_reason,
        module_load_ms,
    };
    let outcome = compile_with(args, &rustc, &crate_source, &out, &work, &loaded);
    // A compiled rustc.wasm takes seconds to free; skip teardown even when the
    // compile failed so an error is reported promptly.
    std::mem::forget(module);
    std::mem::forget(engine);
    let mut result = outcome?;
    if args.fuel {
        let (fuel_runs, compiler_fuel) =
            measure_compiler_fuel(args, &rustc, &crate_source, &out, cache_dir.as_deref())?;
        result
            .medians
            .insert("compiler_fuel".to_owned(), as_f64(compiler_fuel));
        result.fuel_runs = fuel_runs;
        result.compiler_fuel = Some(compiler_fuel);
    }
    let text = serde_json::to_string_pretty(&result)?;
    if let Some(path) = &args.json {
        std::fs::write(path, &text)?;
    }
    println!("{text}");
    Ok(())
}

/// Compile the same program on a fuel-enabled engine and require every sample
/// to consume the same number of fuel units.
///
/// Fuel is charged per wasm operator from a fixed cost table, so it counts
/// compiler work rather than host time. The engine is separate from the timed
/// one because enabling fuel instruments the generated code; using it for the
/// timed samples would change what the chart's compile line measures.
fn measure_compiler_fuel(
    args: &CompileArgs,
    rustc: &Path,
    crate_source: &Path,
    out: &Path,
    cache_dir: Option<&Path>,
) -> Result<(Vec<FuelRun>, u64)> {
    let mut config = stage_c_host::engine_config(cache_dir)?;
    config.consume_fuel(true);
    let engine = Engine::new(&config).wt("creating the fuel engine")?;
    let loaded = load_rustc_module(&engine, rustc, None, false)
        .context("loading rustc.wasm on the fuel engine")?;
    // Keep the benchmark's own module from the timed builds: the fuel runs
    // must not replace the artifact that execution and size measure.
    let fuel_out = out.with_file_name(format!("{}.fuel.wasm", args.crate_name));
    let options = CompileOptions {
        sysroot: absolute_arg(&args.sysroot)?,
        linker: absolute_arg(&args.linker)?,
        src: crate_source.to_path_buf(),
        out: fuel_out,
        crate_name: args.crate_name.clone(),
        externs: args.externs.clone(),
        search_paths: args.search_paths.clone(),
        incremental: None,
        time_passes: false,
        extra_args: args.rustc_args.clone(),
    };
    let linker = options.linker.clone();
    let mut runs = Vec::with_capacity(args.fuel_runs);
    for index in 0..args.fuel_runs {
        // Start every sample from the same state: rustc does slightly
        // different work when its output path already exists.
        if options.out.exists() {
            std::fs::remove_file(&options.out)
                .with_context(|| format!("removing {}", options.out.display()))?;
        }
        let run = compile_once(
            &engine,
            &loaded.module,
            &options,
            &linker,
            index,
            Some(FUEL_LIMIT),
            true,
        )?;
        let fuel = run
            .fuel
            .context("a fuel-enabled compile did not report its fuel")?;
        runs.push(FuelRun { run: index, fuel });
    }
    let first = runs.first().context("no fuel runs were requested")?.fuel;
    if let Some(different) = runs.iter().find(|run| run.fuel != first) {
        bail!(
            "compiler fuel is not deterministic for {}: run 0 used {first}, \
             run {} used {}",
            options.src.display(),
            different.run,
            different.fuel
        );
    }
    Ok((runs, first))
}

fn load_compiler(
    rustc: &Path,
    precompiled: Option<&Path>,
    cache_dir: Option<&Path>,
    mode: &str,
) -> Result<(Engine, String, Option<String>, Module, f64)> {
    let engine = match mode {
        "cold" => stage_c_host::engine(None)?,
        "cached" => {
            let directory = cache_dir.ok_or_else(|| anyhow!("--load cached needs --cache-dir"))?;
            std::fs::create_dir_all(directory)?;
            stage_c_host::engine(Some(directory))?
        }
        "precompiled" => {
            if precompiled.is_none() {
                bail!("--load precompiled needs --precompiled");
            }
            stage_c_host::engine(cache_dir)?
        }
        other => bail!("unknown --load {other}; use cold, cached or precompiled"),
    };
    let artifact = if mode == "precompiled" {
        precompiled
    } else {
        None
    };
    let started = Instant::now();
    let loaded = load_rustc_module(&engine, rustc, artifact, false)?;
    let module_load_ms = elapsed_ms(started);
    Ok((
        engine,
        loaded.source.as_str().to_owned(),
        loaded.fallback_reason,
        loaded.module,
        module_load_ms,
    ))
}

fn prepare_source(source: &Path, prelude: &Path, destination: &Path) -> Result<()> {
    let text = std::fs::read_to_string(source)
        .with_context(|| format!("reading benchmark source {}", source.display()))?;
    // The prepared copy lives under bench/work/, so the relative include in
    // the program would resolve against the wrong directory. Point it at the
    // committed prelude explicitly.
    let prelude = std::path::absolute(prelude)
        .with_context(|| format!("making {} absolute", prelude.display()))?;
    let mut prepared = text.replace(
        "include!(\"../prelude.rs\");",
        &format!("include!(\"{}\");", prelude.display()),
    );
    if !prepared.contains("BENCH_EDIT_TWEAK") {
        prepared.push_str(
            "\n// Added by the benchmark host: a one-line edit whose two spellings\n\
             // are both zero, so compile-time edits never change the result.\n\
             const BENCH_EDIT_TWEAK: u64 = 0u64;\n\
             #[unsafe(no_mangle)]\n\
             pub extern \"C\" fn bench_edit_probe() -> u64 { BENCH_EDIT_TWEAK }\n",
        );
    }
    std::fs::write(destination, prepared)
        .with_context(|| format!("writing prepared source {}", destination.display()))?;
    Ok(())
}

fn toggle_edit(source: &Path) -> Result<()> {
    let text =
        std::fs::read_to_string(source).with_context(|| format!("reading {}", source.display()))?;
    // Both spellings are zero, so the edit changes the source (and forces an
    // incremental recompile) without changing the program's result.
    let (from, to) = if text.contains("const BENCH_EDIT_TWEAK: u64 = 0x0_u64;") {
        (
            "const BENCH_EDIT_TWEAK: u64 = 0x0_u64;",
            "const BENCH_EDIT_TWEAK: u64 = 0u64;",
        )
    } else if text.contains("const BENCH_EDIT_TWEAK: u64 = 0u64;") {
        (
            "const BENCH_EDIT_TWEAK: u64 = 0u64;",
            "const BENCH_EDIT_TWEAK: u64 = 0x0_u64;",
        )
    } else {
        bail!(
            "{} has no BENCH_EDIT_TWEAK constant to edit",
            source.display()
        );
    };
    std::fs::write(source, text.replacen(from, to, 1))
        .with_context(|| format!("editing {}", source.display()))?;
    Ok(())
}

fn run_incremental(
    engine: &Engine,
    module: &Module,
    options: &CompileOptions,
    linker: &Path,
    directory: &Path,
    runs: usize,
    phase_runs: usize,
) -> Result<IncrementalResult> {
    let mut incremental_options = CompileOptions {
        sysroot: options.sysroot.clone(),
        linker: options.linker.clone(),
        src: options.src.clone(),
        out: options.out.clone(),
        crate_name: options.crate_name.clone(),
        externs: options.externs.clone(),
        search_paths: options.search_paths.clone(),
        incremental: Some(directory.to_path_buf()),
        time_passes: false,
        extra_args: options.extra_args.clone(),
    };
    // The first compile populates the incremental directory; edits after it
    // reuse the cached work products.
    let cold = compile_once(engine, module, &incremental_options, linker, 0, None, false)?;
    toggle_edit(&incremental_options.src)?;
    let first_edit = compile_once(engine, module, &incremental_options, linker, 1, None, false)?;
    toggle_edit(&incremental_options.src)?;
    let mut measured = Vec::with_capacity(runs);
    for index in 0..runs {
        toggle_edit(&incremental_options.src)?;
        measured.push(compile_once(
            engine,
            module,
            &incremental_options,
            linker,
            index,
            None,
            false,
        )?);
    }
    incremental_options.time_passes = true;
    let mut phases = Vec::with_capacity(phase_runs);
    for index in 0..phase_runs {
        toggle_edit(&incremental_options.src)?;
        phases.push(compile_once(
            engine,
            module,
            &incremental_options,
            linker,
            index,
            None,
            false,
        )?);
    }
    let mut medians = BTreeMap::new();
    medians.insert(
        "incremental_ms".to_owned(),
        median(&collect(&measured, |run| run.compile_ms)),
    );
    medians.insert(
        "incremental_link_ms".to_owned(),
        median(&collect(&measured, |run| run.link_ms)),
    );
    Ok(IncrementalResult {
        directory: directory.display().to_string(),
        cold_ms: cold.compile_ms,
        cold_peak_memory_bytes: cold.peak_memory_bytes,
        first_edit_ms: first_edit.compile_ms,
        runs: measured,
        phase_runs: phases,
        medians,
    })
}

fn compile_once(
    engine: &Engine,
    module: &Module,
    options: &CompileOptions,
    linker: &Path,
    index: usize,
    fuel_limit: Option<u64>,
    deterministic_random: bool,
) -> Result<CompileRun> {
    let args = options.rustc_args();
    let started = Instant::now();
    let (mut store, instance, stderr) = prepare_instance(
        engine,
        module,
        &args,
        linker,
        fuel_limit,
        deterministic_random,
    )?;
    let instantiate_ms = elapsed_ms(started);
    // Read the counter after instantiation so data-segment setup is not part
    // of the compiler-work number.
    let fuel_before = match fuel_limit {
        Some(_) => Some(store.get_fuel().wt("reading fuel after instantiation")?),
        None => None,
    };
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .wt("rustc.wasm _start export")?;
    let started = Instant::now();
    let call = start.call(&mut store, ());
    let rustc_total_ms = elapsed_ms(started);
    let fuel = match fuel_before {
        Some(before) => {
            Some(before.saturating_sub(store.get_fuel().wt("reading fuel after _start")?))
        }
        None => None,
    };
    let spawns = store
        .data()
        .spawns
        .lock()
        .expect("spawn log lock")
        .iter()
        .map(|spawn| SpawnSummary {
            program: spawn.program.clone(),
            args: spawn.args,
            ms: spawn.ms,
            status: spawn.status,
        })
        .collect::<Vec<_>>();
    let link_ms = spawns.iter().map(|spawn| spawn.ms).sum::<f64>();
    let captured = String::from_utf8_lossy(&stderr.contents()).into_owned();
    let memory_pages = instance
        .get_memory(&mut store, "memory")
        .map_or(0, |memory| memory.size(&store));
    let peak_pages = memory_pages.max(store.data().limits.max_pages);
    let rc = match call {
        Ok(()) => 0,
        Err(error)
            if error
                .downcast_ref::<I32Exit>()
                .is_some_and(|exit| exit.0 == 0) =>
        {
            0
        }
        Err(error) => {
            let tail = captured
                .lines()
                .rev()
                .take(40)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            bail!(
                "rustc.wasm failed for {} (run {index}): {error}\n{tail}",
                options.src.display()
            );
        }
    };
    if spawns.iter().any(|spawn| spawn.status != 0) {
        bail!("linker failed for {}: {spawns:?}", options.src.display());
    }
    if !options.out.exists() {
        bail!(
            "rustc.wasm reported success but {} does not exist",
            options.out.display()
        );
    }
    let phases = parse_time_passes(&captured);
    std::mem::forget(store);
    Ok(CompileRun {
        run: index,
        instantiate_ms,
        rustc_total_ms,
        link_ms,
        compile_ms: (rustc_total_ms - link_ms).max(0.0),
        peak_memory_bytes: peak_pages * PAGE_SIZE,
        rc,
        phases,
        fuel,
    })
}

/// Parse rustc's `-Ztime-passes` stderr. Each line is
/// `time: <seconds>\t<name>`; the last block belongs to this compile because
/// every run gets a fresh WASI stderr pipe.
fn parse_time_passes(stderr: &str) -> BTreeMap<String, f64> {
    let mut phases = BTreeMap::new();
    for line in stderr.lines() {
        let Some(rest) = line.trim().strip_prefix("time:") else {
            continue;
        };
        let mut parts = rest.split_whitespace();
        let Some(seconds) = parts.next() else {
            continue;
        };
        let Some(name) = parts.next() else {
            continue;
        };
        if let Ok(seconds) = seconds.parse::<f64>() {
            phases.insert(name.to_owned(), seconds * 1e3);
        }
    }
    phases
}

fn collect(runs: &[CompileRun], field: impl Fn(&CompileRun) -> f64) -> Vec<f64> {
    runs.iter().map(field).collect()
}

fn clear_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if !path.is_dir() {
        bail!("{} exists but is not a directory", path.display());
    }
    if path.components().count() < 3 {
        bail!(
            "refusing to delete {}: give a dedicated incremental directory",
            path.display()
        );
    }
    std::fs::remove_dir_all(path)
        .with_context(|| format!("clearing incremental directory {}", path.display()))
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("opening {} for hashing", path.display()))?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).with_context(|| format!("hashing {}", path.display()))?;
    Ok(hex(&hasher.finalize()))
}

// ---------------------------------------------------------------------------
// run: execute a linked module through the difftest corpus ABI
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct ExecRun {
    seed: u64,
    run: usize,
    exec_ms: f64,
    instantiate_ms: f64,
    result: u64,
    out_len: u32,
    out_sha256: String,
}

#[derive(Serialize, Clone)]
struct SeedOutcome {
    result: u64,
    out_len: u32,
    out_sha256: String,
}

#[derive(Serialize)]
struct RunResult {
    wasm: String,
    wasm_bytes: u64,
    wasm_sha256: String,
    module_compile_ms: f64,
    seeds: Vec<u64>,
    runs_per_seed: usize,
    runs: Vec<ExecRun>,
    outcomes: BTreeMap<String, SeedOutcome>,
    medians: BTreeMap<String, f64>,
    ok: bool,
}

fn measure_seed(
    engine: &Engine,
    linker: &Linker<()>,
    module: &Module,
    wasm: &Path,
    seed: u64,
    runs: usize,
    outcomes: &mut BTreeMap<String, SeedOutcome>,
) -> Result<Vec<ExecRun>> {
    let mut measured = Vec::with_capacity(runs);
    let mut observed: Option<SeedOutcome> = None;
    for index in 0..runs {
        let started = Instant::now();
        let mut store = Store::new(engine, ());
        let instance = linker
            .instantiate(&mut store, module)
            .wt("instantiating the benchmark module")?;
        let instantiate_ms = elapsed_ms(started);
        let run = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .wt("benchmark module must export run(seed: u64) -> u64")?;
        let started = Instant::now();
        let result = run.call(&mut store, seed).wt("calling run(seed)")?;
        let exec_ms = elapsed_ms(started);
        let memory = instance
            .get_memory(&mut store, "memory")
            .context("benchmark module must export memory")?;
        let out_ptr = instance
            .get_typed_func::<(), u32>(&mut store, "out_ptr")
            .wt("benchmark module must export out_ptr()")?
            .call(&mut store, ())?;
        let out_len = instance
            .get_typed_func::<(), u32>(&mut store, "out_len")
            .wt("benchmark module must export out_len()")?
            .call(&mut store, ())?;
        let bytes = read_output(&memory, &store, out_ptr, out_len)?;
        let out_sha256 = hex(&Sha256::digest(&bytes));
        let outcome = SeedOutcome {
            result,
            out_len,
            out_sha256: out_sha256.clone(),
        };
        if let Some(previous) = &observed {
            if previous.result != outcome.result || previous.out_sha256 != outcome.out_sha256 {
                bail!(
                    "benchmark module {} is not deterministic for seed {seed}: \
                     {} then {}",
                    wasm.display(),
                    previous.result,
                    outcome.result
                );
            }
        } else {
            outcomes.insert(seed.to_string(), outcome.clone());
            observed = Some(outcome);
        }
        measured.push(ExecRun {
            seed,
            run: index,
            exec_ms,
            instantiate_ms,
            result,
            out_len,
            out_sha256,
        });
    }
    Ok(measured)
}

fn run_command(args: &RunArgs) -> Result<()> {
    if args.runs == 0 {
        bail!("--runs must be at least 1");
    }
    let wasm = absolute_arg(&args.wasm)?;
    let seeds = parse_seeds(&args.seeds)?;
    let engine = stage_c_host::engine(None)?;
    let started = Instant::now();
    let module = Module::from_file(&engine, &wasm).wt("compiling the benchmark module")?;
    let module_compile_ms = elapsed_ms(started);
    let mut linker: Linker<()> = Linker::new(&engine);
    add_nodecraft_imports(&mut linker)?;
    let mut runs = Vec::with_capacity(seeds.len() * args.runs);
    let mut outcomes = BTreeMap::new();
    for seed in &seeds {
        runs.extend(measure_seed(
            &engine,
            &linker,
            &module,
            &wasm,
            *seed,
            args.runs,
            &mut outcomes,
        )?);
    }
    let mut medians = BTreeMap::new();
    medians.insert(
        "exec_ms".to_owned(),
        median(&runs.iter().map(|run| run.exec_ms).collect::<Vec<_>>()),
    );
    medians.insert(
        "instantiate_ms".to_owned(),
        median(
            &runs
                .iter()
                .map(|run| run.instantiate_ms)
                .collect::<Vec<_>>(),
        ),
    );
    let bytes = std::fs::read(&wasm)?;
    let result = RunResult {
        wasm: wasm.display().to_string(),
        wasm_bytes: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
        wasm_sha256: hex(&Sha256::digest(&bytes)),
        module_compile_ms,
        seeds,
        runs_per_seed: args.runs,
        runs,
        outcomes,
        medians,
        ok: true,
    };
    let text = serde_json::to_string_pretty(&result)?;
    if let Some(path) = &args.json {
        std::fs::write(path, &text)?;
    }
    println!("{text}");
    Ok(())
}

fn parse_seeds(text: &str) -> Result<Vec<u64>> {
    let mut seeds = Vec::new();
    for part in text.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let seed = if let Some(hex) = part.strip_prefix("0x") {
            u64::from_str_radix(hex, 16)
        } else {
            part.parse::<u64>()
        }
        .with_context(|| format!("parsing seed {part:?}"))?;
        seeds.push(seed);
    }
    if seeds.is_empty() {
        bail!("--seeds is empty");
    }
    Ok(seeds)
}

fn add_nodecraft_imports(linker: &mut Linker<()>) -> Result<()> {
    linker
        .func_wrap("nodecraft", "host_bias", |value: f64| value * 0.5 + 1.0)
        .wt("defining nodecraft::host_bias")?;
    linker
        .func_wrap("nodecraft", "dot2", |x1: f64, y1: f64, x2: f64, y2: f64| {
            x1 * x2 + y1 * y2
        })
        .wt("defining nodecraft::dot2")?;
    linker
        .func_wrap("nodecraft", "length2", |x: f64, y: f64| {
            (x * x + y * y).sqrt()
        })
        .wt("defining nodecraft::length2")?;
    linker
        .func_wrap("nodecraft", "clamp01", |x: f64| x.clamp(0.0, 1.0))
        .wt("defining nodecraft::clamp01")?;
    linker
        .func_wrap("nodecraft", "scale", |x: f64, k: f64| x * k)
        .wt("defining nodecraft::scale")?;
    linker
        .func_wrap("nodecraft", "mix", |a: f64, b: f64, t: f64| a + (b - a) * t)
        .wt("defining nodecraft::mix")?;
    Ok(())
}

fn read_output<T>(memory: &Memory, store: &Store<T>, pointer: u32, len: u32) -> Result<Vec<u8>> {
    let start = usize::try_from(pointer).context("output pointer")?;
    let len = usize::try_from(len).context("output length")?;
    let end = start.checked_add(len).context("output range")?;
    let memory_len = memory.data_size(store);
    if end > memory_len {
        bail!("output range {start}..{end} is outside the {memory_len}-byte memory");
    }
    let mut bytes = vec![0u8; len];
    memory
        .read(store, start, &mut bytes)
        .context("reading the output buffer")?;
    Ok(bytes)
}

// ---------------------------------------------------------------------------
// sizes
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SizesResult {
    wasm: String,
    total_bytes: u64,
    code_bytes: u64,
    data_bytes: u64,
    data_segments: u64,
    functions: u32,
    imports: u64,
    exports: u64,
}

fn sizes_command(args: &SizesArgs) -> Result<()> {
    let wasm = absolute_arg(&args.wasm)?;
    let bytes = std::fs::read(&wasm)?;
    let mut code_bytes = 0u64;
    let mut data_bytes = 0u64;
    let mut data_segments = 0u64;
    let mut functions = 0u32;
    let mut imports = 0u64;
    let mut exports = 0u64;
    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        match payload.context("parsing wasm")? {
            wasmparser::Payload::CodeSectionStart { count, size, .. } => {
                functions = count;
                code_bytes = u64::from(size);
            }
            wasmparser::Payload::DataSection(reader) => {
                for segment in reader {
                    data_bytes += u64::try_from(segment?.data.len()).unwrap_or(0);
                    data_segments += 1;
                }
            }
            wasmparser::Payload::ImportSection(reader) => {
                imports = u64::from(reader.count());
            }
            wasmparser::Payload::ExportSection(reader) => {
                exports = u64::from(reader.count());
            }
            _ => {}
        }
    }
    let total_bytes = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    let result = SizesResult {
        wasm: wasm.display().to_string(),
        total_bytes,
        code_bytes,
        data_bytes,
        data_segments,
        functions,
        imports,
        exports,
    };
    let text = serde_json::to_string_pretty(&result)?;
    if let Some(path) = &args.json {
        std::fs::write(path, &text)?;
    }
    println!("{text}");
    Ok(())
}
