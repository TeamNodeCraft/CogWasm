//! Build-speed measurements: cold and resident warm compiles, the edit loop,
//! rustc's `-Ztime-passes` breakdown and deterministic compiler fuel.

use crate::exec::BenchRuntime;
use crate::programs::{DependencySet, Program, externs_for, search_paths};
use crate::stats::Metric;
use crate::toolchain::Toolchain;
use crate::util;
use anyhow::{Context, Result, anyhow, bail};
use cogwasm_host::{Compiler, ServeSession};
use cogwasm_runtime::{Limits, StoreOptions, SummaryRecorder, prepare_instance};
use cogwasm_supervisor::{
    Artifact, ArtifactMeta, CompileOptions, CrateKind, Swapper, apply_edit, probe_value,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use wasmtime::{Engine, Module};
use wasmtime_wasi::I32Exit;

/// Map a Wasmtime error into this tool's error type.
trait Wt<T> {
    fn wt(self, context: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, context: &'static str) -> Result<T> {
        self.map_err(|error| anyhow!("{context}: {error}"))
    }
}

/// Fuel cap per compiler sample. It must be large enough not to trap; fuel is
/// read back as a counter, so the exact cap does not affect the measurement.
pub const FUEL_LIMIT: u64 = 1_000_000_000_000_000;
/// The benchmark sources' edit probe declaration.
const PROBE_DECL: &str = "const BENCH_EDIT_TWEAK: u64 =";

/// One compiler configuration under test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub flags: Vec<String>,
    /// Whether the configuration only takes effect in incremental mode.
    pub incremental: bool,
}

impl Config {
    /// Parse a named configuration.
    ///
    /// # Errors
    ///
    /// Returns an error naming the accepted configurations.
    pub fn parse(name: &str) -> Result<Self> {
        match name {
            "default" => Ok(Self {
                name: name.to_owned(),
                flags: Vec::new(),
                incremental: false,
            }),
            "inline" => Ok(Self {
                name: name.to_owned(),
                flags: vec!["-Zmir-inline-wasm".to_owned()],
                incremental: false,
            }),
            "capture" => Ok(Self {
                name: name.to_owned(),
                flags: vec![
                    "-Zmir-inline-wasm".to_owned(),
                    "-Zmir-capture-hoist".to_owned(),
                ],
                incremental: false,
            }),
            "cgu-per-fn" => Ok(Self {
                name: name.to_owned(),
                flags: vec!["-Zcogwasm-cgu-per-fn".to_owned()],
                incremental: true,
            }),
            other => bail!("unknown config {other:?} (default, inline, capture or cgu-per-fn)"),
        }
    }
}

/// One edit-loop sample with the build/link/swap split.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditRow {
    pub edit: usize,
    pub probe: u64,
    pub build_ms: f64,
    pub link_ms: f64,
    pub build_no_link_ms: f64,
    pub swap_ms: f64,
    pub call_ms: f64,
    pub edit_to_result_ms: f64,
}

/// The edit-to-result summary, including the p50/p90 the V2 plan asks for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditSummary {
    pub p50_ms: f64,
    pub p90_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub build_ms: Metric,
    pub link_ms: Metric,
    pub build_no_link_ms: Metric,
    pub swap_ms: Metric,
    pub call_ms: Metric,
    pub edits: Vec<EditRow>,
    pub byte_identical_to_cold: bool,
}

/// The rustc phase buckets the report shows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseBreakdown {
    pub front_end: Metric,
    pub expansion: Metric,
    pub type_borrow_check: Metric,
    pub monomorphisation: Metric,
    pub codegen: Metric,
    pub incremental: Metric,
    pub link: Metric,
    pub total: Metric,
    pub raw: BTreeMap<String, f64>,
}

/// A deterministic compiler-work sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuelMeasurement {
    pub fuel: Metric,
    pub samples: Vec<u64>,
}

/// Everything measured for one program x config build.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildMeasurement {
    pub cold_compile_ms: Metric,
    pub cold_wall_ms: Metric,
    pub warm_compile_ms: Metric,
    pub warm_wall_ms: Metric,
    pub peak_guest_memory_bytes: Metric,
    pub host_rss_peak_bytes: Option<Metric>,
    pub edit: EditSummary,
    pub phases: PhaseBreakdown,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<FuelMeasurement>,
    pub incremental_dir_bytes: u64,
    pub incremental_files: u64,
    pub intermediate_object_files: u64,
    pub intermediate_objects_bytes: u64,
}

/// The build measurement plus the output module execution and size consume.
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub measurement: BuildMeasurement,
    pub wasm: PathBuf,
}

/// Inputs for one build measurement.
pub struct BuildRequest<'a> {
    pub toolchain: &'a Toolchain,
    pub compiler: &'a Compiler,
    pub serve: &'a mut ServeCompiler,
    pub fuel: Option<&'a FuelCompiler>,
    pub program: &'a Program,
    pub config: &'a Config,
    pub dependencies: &'a DependencySet,
    pub work: PathBuf,
    pub runs: usize,
    pub seeds: &'a [u64],
}

/// Build the rustc argv for one compile.
#[expect(
    clippy::too_many_arguments,
    reason = "the argv is the explicit contract with rustc; a struct would only rename the fields"
)]
pub fn rustc_args(
    toolchain: &Toolchain,
    source: &Path,
    out: &Path,
    crate_name: &str,
    externs: &[String],
    search: &[String],
    incremental: Option<&Path>,
    time_passes: bool,
    extra_flags: &[String],
) -> Vec<String> {
    let mut args = vec![
        "rustc".to_owned(),
        source.display().to_string(),
        "--crate-name".to_owned(),
        crate_name.to_owned(),
        "--crate-type=cdylib".to_owned(),
        "--edition=2021".to_owned(),
        "--target=wasm32-unknown-unknown".to_owned(),
        "--sysroot".to_owned(),
        toolchain.sysroot.display().to_string(),
        "-Copt-level=2".to_owned(),
        "-Cpanic=abort".to_owned(),
        "-Cstrip=debuginfo".to_owned(),
        "-Zcodegen-backend=cranelift".to_owned(),
        "-Clinker=wild".to_owned(),
        "-Clinker-flavor=wasm-ld".to_owned(),
        "-o".to_owned(),
        out.display().to_string(),
    ];
    if let Some(directory) = incremental {
        args.push(format!("-Cincremental={}", directory.display()));
    }
    for external in externs {
        args.push("--extern".to_owned());
        args.push(external.clone());
    }
    for directory in search {
        args.push(format!("-Ldependency={directory}"));
    }
    if time_passes {
        args.push("-Ztime-passes".to_owned());
    }
    args.extend(extra_flags.iter().cloned());
    args
}

struct FreshOptions {
    capture_stderr: bool,
    limits: bool,
    fuel: Option<u64>,
    deterministic: bool,
}

struct FreshResult {
    instantiate_ms: f64,
    rustc_total_ms: f64,
    compile_ms: f64,
    peak_bytes: u64,
    stderr: String,
    fuel_used: Option<u64>,
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "FreshOptions is a small settings struct passed to exactly one function"
)]
fn run_fresh(
    engine: &Engine,
    module: &Module,
    args: &[String],
    linker: &Path,
    options: FreshOptions,
) -> Result<FreshResult> {
    let started = Instant::now();
    let prepared = prepare_instance(
        engine,
        module,
        args,
        linker,
        "[cogwasm-bench]",
        StoreOptions {
            capture_stderr: options.capture_stderr,
            deterministic_random: options.deterministic.then_some(0),
            fuel: options.fuel,
            limits: options.limits.then(Limits::default),
            deterministic_clock: options.deterministic,
            ..StoreOptions::default()
        },
        SummaryRecorder::default(),
    )
    .context("instantiating rustc.wasm")?;
    let mut store = prepared.store;
    let instance = prepared.instance;
    let stderr = prepared.stderr;
    let instantiate_ms = started.elapsed().as_secs_f64() * 1e3;
    let fuel_before = options
        .fuel
        .map(|_| store.get_fuel().wt("reading fuel after instantiation"))
        .transpose()?;
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .wt("rustc.wasm has no `_start` export")?;
    let started = Instant::now();
    let call = start.call(&mut store, ());
    let rustc_total_ms = started.elapsed().as_secs_f64() * 1e3;
    let fuel_used = fuel_before
        .map(|before| {
            store
                .get_fuel()
                .map(|after| before.saturating_sub(after))
                .wt("reading fuel after `_start`")
        })
        .transpose()?;
    let spawns = store.data().recorder().summaries();
    let link_ms: f64 = spawns.iter().map(|spawn| spawn.ms).sum();
    let peak_pages = instance
        .get_memory(&mut store, "memory")
        .map_or(0, |memory| memory.size(&store))
        .max(store.data().limits.max_pages);
    let stderr = stderr.map(|pipe| String::from_utf8_lossy(&pipe.contents()).into_owned());
    match call {
        Ok(()) => {}
        Err(error)
            if error
                .downcast_ref::<I32Exit>()
                .is_some_and(|exit| exit.0 == 0) => {}
        Err(error) => {
            let tail = stderr
                .as_deref()
                .unwrap_or_default()
                .lines()
                .rev()
                .take(30)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            bail!("rustc.wasm failed: {error}\n{tail}");
        }
    }
    if spawns.iter().any(|spawn| spawn.status != 0) {
        bail!("linker failed: {spawns:?}");
    }
    Ok(FreshResult {
        instantiate_ms,
        rustc_total_ms,
        compile_ms: (rustc_total_ms - link_ms).max(0.0),
        peak_bytes: peak_pages.saturating_mul(cogwasm_runtime::PAGE_SIZE),
        stderr: stderr.unwrap_or_default(),
        fuel_used,
    })
}

/// A resident serving guest, recreated if it dies.
pub struct ServeCompiler {
    engine: Engine,
    module: Module,
    linker: PathBuf,
    preopens: Vec<PathBuf>,
    session: Option<ServeSession>,
    pub start_ms: f64,
    pub restarts: usize,
}

impl ServeCompiler {
    /// Start the first serving guest.
    ///
    /// # Errors
    ///
    /// Returns an error when the guest cannot start.
    pub fn new(
        engine: &Engine,
        module: &Module,
        linker: &Path,
        preopens: Vec<PathBuf>,
    ) -> Result<Self> {
        let started = Instant::now();
        let session = ServeSession::new(engine, module, linker, &preopens)
            .context("starting the rustc.wasm serve session")?;
        Ok(Self {
            engine: engine.clone(),
            module: module.clone(),
            linker: linker.to_path_buf(),
            preopens,
            session: Some(session),
            start_ms: started.elapsed().as_secs_f64() * 1e3,
            restarts: 0,
        })
    }

    /// Compile one request on the long-lived guest.
    ///
    /// # Errors
    ///
    /// Returns an error when the guest rejects the compile; a dead session is
    /// replaced before the next request.
    pub fn compile(&mut self, args: &[String]) -> Result<cogwasm_host::CompileOutcome> {
        if self.session.as_ref().is_none_or(ServeSession::is_dead) {
            self.session = Some(
                ServeSession::new(&self.engine, &self.module, &self.linker, &self.preopens)
                    .context("restarting the rustc.wasm serve session")?,
            );
            self.restarts += 1;
        }
        let session = self
            .session
            .as_ref()
            .context("the serve session is not running")?;
        let result = session.compile_args(args);
        if result.is_err() && session.is_dead() {
            self.session = None;
        }
        result
    }
}

/// A fuel-enabled engine with rustc.wasm loaded, shared by every fuel sample.
pub struct FuelCompiler {
    engine: Engine,
    module: Module,
}

impl FuelCompiler {
    /// Create the fuel engine and load rustc.wasm. The first call is slow
    /// because fuel instrumentation changes the compiled code.
    ///
    /// # Errors
    ///
    /// Returns an error when the engine cannot be created or rustc.wasm cannot
    /// be loaded.
    pub fn load(toolchain: &Toolchain) -> Result<Self> {
        let mut config = cogwasm_host::engine_config(Some(&toolchain.cache_dir))?;
        config.consume_fuel(true);
        let engine = Engine::new(&config).wt("creating the fuel engine")?;
        let loaded = cogwasm_host::load_rustc_module(&engine, &toolchain.rustc_wasm, None, false)
            .context("loading rustc.wasm on the fuel engine")?;
        Ok(Self {
            engine,
            module: loaded.module,
        })
    }
}

/// Measure one program x config.
///
/// # Errors
///
/// Returns an error when any compile fails, an edit is not byte-identical to a
/// cold compile, or fuel is not deterministic.
#[expect(
    clippy::cast_precision_loss,
    reason = "guest memory and host RSS are far below 2^53 bytes"
)]
pub fn measure_build(request: &mut BuildRequest<'_>) -> Result<BuildResult> {
    let config = request.config.clone();
    let name = request.program.def.name.clone();
    let dir = request.work.join("build").join(&name).join(&config.name);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let externs = externs_for(&request.program.def, request.dependencies, "clif")?;
    let search = search_paths(&request.dependencies.clif);
    let source = request.program.source.clone();
    let output_dir = request.work.join("output").join(&name).join(&config.name);
    std::fs::create_dir_all(&output_dir)?;

    let cold_incremental = config.incremental.then(|| dir.join("cold-incremental"));
    if let Some(path) = &cold_incremental {
        clear_directory(path)?;
    }
    let cold_out = dir.join("cold.wasm");
    remove_file_if_exists(&cold_out)?;
    let cold_args = rustc_args(
        request.toolchain,
        &source,
        &cold_out,
        &name,
        &externs,
        &search,
        cold_incremental.as_deref(),
        false,
        &config.flags,
    );
    let cold = run_fresh(
        request.compiler.engine(),
        request.compiler.module(),
        &cold_args,
        &request.toolchain.wild,
        FreshOptions {
            capture_stderr: false,
            limits: true,
            fuel: None,
            deterministic: false,
        },
    )?;
    if !cold_out.is_file() {
        bail!(
            "cold compile of {name} reported success but wrote no {}",
            cold_out.display()
        );
    }

    let (warm_compile, warm_wall, warm_out) = measure_warm(request, &dir, &externs, &search)?;

    let (edit, edit_wasm) = measure_edit_loop(request, &dir, &externs, &search)?;
    let incremental_dir = dir.join("incremental");
    let (incremental_files, incremental_dir_bytes) = count_files(&incremental_dir)?;
    let (intermediate_object_files, intermediate_objects_bytes) =
        util::files_with_extensions(&incremental_dir, &["o", "obj", "rlib", "rmeta"])?;

    let (phases, peak_bytes) = measure_phases(request, &dir, &externs, &search)?;
    let fuel = match request.fuel {
        Some(fuel) => Some(measure_fuel(
            fuel,
            request.toolchain,
            &source,
            &name,
            &externs,
            &search,
            &config,
            &dir,
        )?),
        None => None,
    };
    let final_wasm = if config.incremental {
        edit_wasm
    } else {
        warm_out
    };
    let final_path = output_dir.join("clif.wasm");
    std::fs::copy(&final_wasm, &final_path).with_context(|| {
        format!(
            "copying {} to {}",
            final_wasm.display(),
            final_path.display()
        )
    })?;

    let measurement = BuildMeasurement {
        cold_compile_ms: Metric::one(cold.compile_ms, false)?,
        cold_wall_ms: Metric::one(cold.instantiate_ms + cold.rustc_total_ms, false)?,
        warm_compile_ms: warm_compile,
        warm_wall_ms: warm_wall,
        peak_guest_memory_bytes: Metric::one(peak_bytes.max(cold.peak_bytes) as f64, false)?,
        host_rss_peak_bytes: util::peak_rss_bytes()
            .map(|bytes| Metric::one(bytes as f64, false))
            .transpose()?,
        edit,
        phases,
        fuel,
        incremental_dir_bytes,
        incremental_files,
        intermediate_object_files,
        intermediate_objects_bytes,
    };
    Ok(BuildResult {
        measurement,
        wasm: final_path,
    })
}

fn measure_warm(
    request: &mut BuildRequest<'_>,
    dir: &Path,
    externs: &[String],
    search: &[String],
) -> Result<(Metric, Metric, PathBuf)> {
    let name = &request.program.def.name;
    let config = request.config;
    let warm_incremental = config.incremental.then(|| dir.join("warm-incremental"));
    if let Some(path) = &warm_incremental {
        clear_directory(path)?;
        let seed_out = dir.join("warm-seed.wasm");
        remove_file_if_exists(&seed_out)?;
        let args = rustc_args(
            request.toolchain,
            &request.program.source,
            &seed_out,
            name,
            externs,
            search,
            Some(path),
            false,
            &config.flags,
        );
        run_fresh(
            request.compiler.engine(),
            request.compiler.module(),
            &args,
            &request.toolchain.wild,
            FreshOptions {
                capture_stderr: false,
                limits: false,
                fuel: None,
                deterministic: false,
            },
        )?;
    }
    let warmup = dir.join("warmup.wasm");
    remove_file_if_exists(&warmup)?;
    let warmup_args = rustc_args(
        request.toolchain,
        &request.program.source,
        &warmup,
        name,
        externs,
        search,
        warm_incremental.as_deref(),
        false,
        &config.flags,
    );
    request.serve.compile(&warmup_args)?;
    let mut compile_values = Vec::with_capacity(request.runs);
    let mut wall_values = Vec::with_capacity(request.runs);
    let mut final_out = warmup;
    for index in 0..request.runs {
        let out = dir.join(format!("warm-{index}.wasm"));
        remove_file_if_exists(&out)?;
        let args = rustc_args(
            request.toolchain,
            &request.program.source,
            &out,
            name,
            externs,
            search,
            warm_incremental.as_deref(),
            false,
            &config.flags,
        );
        let outcome = request
            .serve
            .compile(&args)
            .with_context(|| format!("warm compile {index} of {name}"))?;
        compile_values.push(outcome.compile_ms);
        wall_values.push(outcome.rustc_total_ms);
        final_out = out;
    }
    Ok((
        Metric::new(&compile_values, false)?,
        Metric::new(&wall_values, false)?,
        final_out,
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "the edit loop is one sequence: cold build, edits, verification, summary"
)]
fn measure_edit_loop(
    request: &mut BuildRequest<'_>,
    dir: &Path,
    externs: &[String],
    search: &[String],
) -> Result<(EditSummary, PathBuf)> {
    let name = request.program.def.name.clone();
    let config = request.config;
    let incremental = dir.join("incremental");
    clear_directory(&incremental)?;
    let cold_incremental = dir.join("incremental-cold");
    clear_directory(&cold_incremental)?;
    let copy_dir = dir.join("source");
    std::fs::create_dir_all(&copy_dir)?;
    let copy = copy_dir.join(format!("{name}.rs"));
    std::fs::copy(&request.program.source, &copy)?;
    let original = std::fs::read_to_string(&copy)?;
    if !original.contains(PROBE_DECL) {
        bail!(
            "{} has no {PROBE_DECL} benchmark edit probe",
            copy.display()
        );
    }
    let initial_probe = probe_value(&original).context("the edit probe is malformed")?;
    let cold_out = dir.join("edit-cold.wasm");
    remove_file_if_exists(&cold_out)?;
    let cold_args = rustc_args(
        request.toolchain,
        &copy,
        &cold_out,
        &name,
        externs,
        search,
        Some(&incremental),
        false,
        &config.flags,
    );
    request
        .serve
        .compile(&cold_args)
        .with_context(|| format!("the edit loop's cold build of {name}"))?;
    let runtime = Arc::new(BenchRuntime::for_swaps()?);
    let mut swapper = Swapper::new(runtime);
    let cold_artifact = artifact_from_file(&cold_out, &name)?;
    swapper.install(&cold_artifact)?;
    let seed = request.seeds.first().copied().unwrap_or(0);
    let baseline = swapper.call(seed)?;
    let mut expected = BTreeMap::new();
    expected.insert(initial_probe, baseline);
    let mut text = original;
    let mut rows = Vec::with_capacity(request.runs);
    let mut totals = Vec::with_capacity(request.runs);
    let mut final_wasm = cold_out;
    for offset in 0..request.runs {
        let index = offset + 1;
        let probe = u64::from(offset % 2 != 0);
        let next = apply_edit(&text, index, probe)?;
        std::fs::write(&copy, &next)
            .with_context(|| format!("writing {} for edit {index}", copy.display()))?;
        text = next;
        let out_dir = dir.join(format!("edit-{index}"));
        std::fs::create_dir_all(&out_dir)?;
        let out = out_dir.join("artifact.wasm");
        remove_file_if_exists(&out)?;
        let args = rustc_args(
            request.toolchain,
            &copy,
            &out,
            &name,
            externs,
            search,
            Some(&incremental),
            false,
            &config.flags,
        );
        let started = Instant::now();
        let outcome = request
            .serve
            .compile(&args)
            .with_context(|| format!("edit {index} of {name}"))?;
        let build_ms = started.elapsed().as_secs_f64() * 1e3;
        let bytes = std::fs::read(&out).with_context(|| format!("reading {}", out.display()))?;
        verify_cold(
            request,
            &copy,
            &name,
            externs,
            search,
            config,
            &cold_incremental,
            &bytes,
            index,
        )?;
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "link time is a non-negative wall-clock duration in whole milliseconds"
        )]
        let link_rounded = outcome.link_ms.round() as u64;
        let artifact = artifact_from_bytes(bytes, &name, Some(link_rounded))?;
        let started = Instant::now();
        swapper.install(&artifact)?;
        let swap_ms = started.elapsed().as_secs_f64() * 1e3;
        let started = Instant::now();
        let result = swapper.call(seed)?;
        let call_ms = started.elapsed().as_secs_f64() * 1e3;
        if let Some(want) = expected.get(&probe) {
            if *want != result {
                bail!(
                    "edit {index}: run({seed}) = {result} with probe {probe}u64, expected {want}"
                );
            }
        } else {
            expected.insert(probe, result);
        }
        let link_ms = outcome.link_ms;
        let edit_to_result_ms = build_ms + swap_ms + call_ms;
        totals.push(edit_to_result_ms);
        rows.push(EditRow {
            edit: index,
            probe,
            build_ms,
            link_ms,
            build_no_link_ms: (build_ms - link_ms).max(0.0),
            swap_ms,
            call_ms,
            edit_to_result_ms,
        });
        final_wasm = out;
    }
    let p50 = Metric::percentile(&totals, 0.5)?;
    let p90 = Metric::percentile(&totals, 0.9)?;
    let min = totals.iter().copied().fold(f64::INFINITY, f64::min);
    let max = totals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Ok((
        EditSummary {
            p50_ms: p50,
            p90_ms: p90,
            min_ms: min,
            max_ms: max,
            build_ms: Metric::new(
                &rows.iter().map(|row| row.build_ms).collect::<Vec<_>>(),
                false,
            )?,
            link_ms: Metric::new(
                &rows.iter().map(|row| row.link_ms).collect::<Vec<_>>(),
                false,
            )?,
            build_no_link_ms: Metric::new(
                &rows
                    .iter()
                    .map(|row| row.build_no_link_ms)
                    .collect::<Vec<_>>(),
                false,
            )?,
            swap_ms: Metric::new(
                &rows.iter().map(|row| row.swap_ms).collect::<Vec<_>>(),
                false,
            )?,
            call_ms: Metric::new(
                &rows.iter().map(|row| row.call_ms).collect::<Vec<_>>(),
                false,
            )?,
            edits: rows,
            byte_identical_to_cold: true,
        },
        final_wasm,
    ))
}

#[expect(
    clippy::too_many_arguments,
    reason = "the verification needs the source, config and work paths that produced the warm bytes"
)]
fn verify_cold(
    request: &BuildRequest<'_>,
    source: &Path,
    name: &str,
    externs: &[String],
    search: &[String],
    config: &Config,
    cold_incremental: &Path,
    warm_bytes: &[u8],
    index: usize,
) -> Result<()> {
    clear_directory(cold_incremental)?;
    let out_dir = cold_incremental.with_file_name(format!("cold-edit-{index}"));
    std::fs::create_dir_all(&out_dir)?;
    let out = out_dir.join("artifact.wasm");
    remove_file_if_exists(&out)?;
    let args = rustc_args(
        request.toolchain,
        source,
        &out,
        name,
        externs,
        search,
        Some(cold_incremental),
        false,
        &config.flags,
    );
    run_fresh(
        request.compiler.engine(),
        request.compiler.module(),
        &args,
        &request.toolchain.wild,
        FreshOptions {
            capture_stderr: false,
            limits: false,
            fuel: None,
            deterministic: false,
        },
    )
    .with_context(|| format!("compiling edit {index} of {name} in a fresh guest"))?;
    let cold = std::fs::read(&out).with_context(|| format!("reading {}", out.display()))?;
    if cold != warm_bytes {
        bail!(
            "edit {index} of {name}: warm artifact is {} bytes but a cold compile is {} bytes; \
             the serving guest did not produce a byte-identical wasm",
            warm_bytes.len(),
            cold.len()
        );
    }
    Ok(())
}

fn measure_phases(
    request: &BuildRequest<'_>,
    dir: &Path,
    externs: &[String],
    search: &[String],
) -> Result<(PhaseBreakdown, u64)> {
    let name = &request.program.def.name;
    let config = request.config;
    // The cold phase run has every top-level timer (parse, expansion, type
    // check, codegen, link); an incremental run often reports only the queries
    // that actually re-executed, so it is used only for the incremental
    // save/load bucket.
    let cold_out = dir.join("phase-cold.wasm");
    remove_file_if_exists(&cold_out)?;
    let cold_args = rustc_args(
        request.toolchain,
        &request.program.source,
        &cold_out,
        name,
        externs,
        search,
        None,
        true,
        &config.flags,
    );
    let cold = run_fresh(
        request.compiler.engine(),
        request.compiler.module(),
        &cold_args,
        &request.toolchain.wild,
        FreshOptions {
            capture_stderr: true,
            limits: true,
            fuel: None,
            deterministic: false,
        },
    )
    .with_context(|| format!("cold phase run for {name}"))?;
    let mut phases = phase_breakdown(&parse_time_passes(&cold.stderr))?;

    let incremental = dir.join("incremental");
    let copy = dir.join("source").join(format!("{name}.rs"));
    let text = std::fs::read_to_string(&copy)?;
    let next = apply_edit(&text, request.runs + 1, 0)?;
    std::fs::write(&copy, next)?;
    let out = dir.join("phase-incremental.wasm");
    remove_file_if_exists(&out)?;
    let args = rustc_args(
        request.toolchain,
        &copy,
        &out,
        name,
        externs,
        search,
        Some(&incremental),
        true,
        &config.flags,
    );
    let fresh = run_fresh(
        request.compiler.engine(),
        request.compiler.module(),
        &args,
        &request.toolchain.wild,
        FreshOptions {
            capture_stderr: true,
            limits: true,
            fuel: None,
            deterministic: false,
        },
    )
    .with_context(|| format!("incremental phase run for {name}"))?;
    let incremental_phases = phase_breakdown(&parse_time_passes(&fresh.stderr))?;
    phases.incremental = incremental_phases.incremental;
    Ok((phases, fresh.peak_bytes.max(cold.peak_bytes)))
}
#[expect(
    clippy::too_many_arguments,
    clippy::cast_precision_loss,
    reason = "a fuel sample needs the toolchain, source, config and work paths; fuel counts are far below 2^53"
)]
fn measure_fuel(
    fuel: &FuelCompiler,
    toolchain: &Toolchain,
    source: &Path,
    name: &str,
    externs: &[String],
    search: &[String],
    config: &Config,
    dir: &Path,
) -> Result<FuelMeasurement> {
    let out = dir.join("fuel.wasm");
    let fuel_incremental = config.incremental.then(|| dir.join("fuel-incremental"));
    let mut samples = Vec::with_capacity(2);
    for index in 0..2 {
        if let Some(path) = &fuel_incremental {
            clear_directory(path)?;
        }
        remove_file_if_exists(&out)?;
        let args = rustc_args(
            toolchain,
            source,
            &out,
            name,
            externs,
            search,
            fuel_incremental.as_deref(),
            false,
            &config.flags,
        );
        let fresh = run_fresh(
            &fuel.engine,
            &fuel.module,
            &args,
            &toolchain.wild,
            FreshOptions {
                capture_stderr: true,
                limits: true,
                fuel: Some(FUEL_LIMIT),
                deterministic: true,
            },
        )
        .with_context(|| format!("fuel sample {index} for {name}"))?;
        let fuel_used = fresh
            .fuel_used
            .context("a fuel-enabled compile reported no fuel")?;
        samples.push(fuel_used);
    }
    if samples[0] != samples[1] {
        bail!(
            "compiler fuel is not deterministic for {name}: {} then {}",
            samples[0],
            samples[1]
        );
    }
    Ok(FuelMeasurement {
        fuel: Metric::new(
            &samples
                .iter()
                .map(|value| *value as f64)
                .collect::<Vec<_>>(),
            true,
        )?,
        samples,
    })
}

/// Parse rustc's `-Ztime-passes` stderr into phase milliseconds.
///
/// The last line wins for a repeated phase, matching the benchmark host.
#[must_use]
pub fn parse_time_passes(stderr: &str) -> BTreeMap<String, f64> {
    let mut phases = BTreeMap::new();
    for line in stderr.lines() {
        let Some(rest) = line.trim().strip_prefix("time:") else {
            continue;
        };
        let rest = rest.trim_start();
        let mut parts = rest.splitn(2, char::is_whitespace);
        let Some(seconds) = parts.next() else {
            continue;
        };
        let Some(name) = parts.next() else {
            continue;
        };
        if let Ok(seconds) = seconds.parse::<f64>() {
            phases.insert(name.trim().to_owned(), seconds * 1e3);
        }
    }
    phases
}

/// Bucket the raw phase map into the categories the report shows.
///
/// # Errors
///
/// Returns an error when a bucket cannot be represented as a metric.
pub fn phase_breakdown(raw: &BTreeMap<String, f64>) -> Result<PhaseBreakdown> {
    // `-Ztime-passes` prints nested timers (for example `link`, `link_crate`,
    // `link_binary` and `run_linker`), so summing every matching key would
    // count the same work several times. Use the outermost timer for each
    // family and let the front-end bucket absorb everything else.
    let has_macro_expand = raw
        .keys()
        .any(|name| name.to_ascii_lowercase().contains("macro_expand"));
    let has_link = raw.keys().any(|name| name.eq_ignore_ascii_case("link"));
    let mut expansion = 0.0;
    let mut type_borrow = 0.0;
    let mut mono = 0.0;
    let mut codegen = 0.0;
    let mut incremental = 0.0;
    let mut link = 0.0;
    for (name, value) in raw {
        let lower = name.to_ascii_lowercase();
        if lower == "total" {
            continue;
        }
        if lower == "expand_crate" && has_macro_expand {
            continue;
        }
        if lower.contains("link") && lower != "link" && has_link {
            continue;
        }
        if lower.contains("incr") || lower.contains("incremental") || lower.contains("save") {
            incremental += value;
        } else if lower == "link" || lower.contains("link") {
            link += value;
        } else if lower.contains("codegen") || lower.contains("llvm") || lower.contains("write") {
            codegen += value;
        } else if lower.contains("monomorph")
            || lower.contains("partition")
            || lower.contains("mono_item")
        {
            mono += value;
        } else if lower.contains("type_check")
            || lower.contains("typeck")
            || lower.contains("borrow")
            || lower.contains("region")
        {
            type_borrow += value;
        } else if lower.contains("expand") {
            expansion += value;
        }
    }
    let total = raw.get("total").copied().unwrap_or_else(|| {
        raw.iter()
            .filter(|(name, _)| name.as_str() != "total")
            .map(|(_, value)| *value)
            .sum()
    });
    let front_end =
        (total - expansion - type_borrow - mono - codegen - incremental - link).max(0.0);
    Ok(PhaseBreakdown {
        front_end: Metric::one(front_end, false)?,
        expansion: Metric::one(expansion, false)?,
        type_borrow_check: Metric::one(type_borrow, false)?,
        monomorphisation: Metric::one(mono, false)?,
        codegen: Metric::one(codegen, false)?,
        incremental: Metric::one(incremental, false)?,
        link: Metric::one(link, false)?,
        total: Metric::one(total, false)?,
        raw: raw.clone(),
    })
}
fn artifact_from_file(path: &Path, name: &str) -> Result<Artifact> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    artifact_from_bytes(bytes, name, None)
}

fn artifact_from_bytes(bytes: Vec<u8>, name: &str, link_ms: Option<u64>) -> Result<Artifact> {
    let key = util::sha256_hex(&bytes);
    let length = u64::try_from(bytes.len()).context("artifact length")?;
    Ok(Artifact {
        key: key.clone(),
        meta: ArtifactMeta {
            schema: cogwasm_supervisor::SCHEMA,
            key,
            options: CompileOptions::default(),
            kind: CrateKind::Cdylib,
            crate_name: name.to_owned(),
            compiler: "cogwasm-bench".to_owned(),
            source_sha256: String::new(),
            source_path: String::new(),
            bytes: length,
            compile_ms: None,
            link_ms,
        },
        bytes,
    })
}

fn remove_file_if_exists(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("removing {}", path.display())),
    }
}

/// Remove a benchmark-owned directory, refusing obviously dangerous paths.
pub(crate) fn clear_directory(path: &Path) -> Result<()> {
    if !path.is_absolute() || path.components().count() < 4 {
        bail!(
            "refusing to clear {}: expected an absolute benchmark work directory",
            path.display()
        );
    }
    match std::fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("clearing {}", path.display())),
    }
}

fn count_files(root: &Path) -> Result<(u64, u64)> {
    if !root.exists() {
        return Ok((0, 0));
    }
    let mut count = 0u64;
    let mut total = 0u64;
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .with_context(|| format!("reading {}", directory.display()))?
        {
            let entry =
                entry.with_context(|| format!("reading an entry of {}", directory.display()))?;
            let file_type = entry
                .file_type()
                .with_context(|| format!("reading the type of {}", entry.path().display()))?;
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                count += 1;
                total = total.saturating_add(
                    entry
                        .metadata()
                        .with_context(|| format!("reading the size of {}", entry.path().display()))?
                        .len(),
                );
            }
        }
    }
    Ok((count, total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_time_passes_tab_and_space_lines() {
        let stderr = "random text\n\
                      time: 0.125\tparse_crate\n\
                      time: 0.250 expand_crate\n\
                      time: 0.500\ttotal\n";
        let phases = parse_time_passes(stderr);
        assert_eq!(phases.get("parse_crate"), Some(&125.0));
        assert_eq!(phases.get("expand_crate"), Some(&250.0));
        assert_eq!(phases.get("total"), Some(&500.0));
        assert_eq!(phases.len(), 3);
    }

    #[test]
    fn phase_buckets_do_not_double_count_and_leave_a_front_end_residual() {
        let raw = BTreeMap::from([
            ("parse_crate".to_owned(), 10.0),
            ("expand_crate".to_owned(), 20.0),
            ("type_check_crate".to_owned(), 30.0),
            ("borrow_check".to_owned(), 5.0),
            ("monomorphize".to_owned(), 15.0),
            ("partitioning".to_owned(), 5.0),
            ("codegen_crate".to_owned(), 40.0),
            ("incr_comp_persist".to_owned(), 7.0),
            ("link".to_owned(), 3.0),
            ("total".to_owned(), 200.0),
        ]);
        let phases = phase_breakdown(&raw).expect("breakdown");
        assert_eq!(phases.expansion.median, 20.0);
        assert_eq!(phases.type_borrow_check.median, 35.0);
        assert_eq!(phases.monomorphisation.median, 20.0);
        assert_eq!(phases.codegen.median, 40.0);
        assert_eq!(phases.incremental.median, 7.0);
        assert_eq!(phases.link.median, 3.0);
        assert_eq!(phases.front_end.median, 75.0);
        assert_eq!(phases.total.median, 200.0);
    }

    #[test]
    fn config_names_and_flags_are_the_documented_ones() {
        assert_eq!(
            Config::parse("default").expect("default").flags,
            Vec::<String>::new()
        );
        assert_eq!(
            Config::parse("capture").expect("capture").flags,
            vec!["-Zmir-inline-wasm", "-Zmir-capture-hoist"]
        );
        assert!(Config::parse("cgu-per-fn").expect("cgu").incremental);
        assert!(Config::parse("fast").is_err());
    }

    #[test]
    fn edit_probe_value_is_read_from_the_prepared_source() {
        let text = "fn main() {}\nconst BENCH_EDIT_TWEAK: u64 = 0u64;\n";
        assert_eq!(probe_value(text), Some(0));
    }
}
