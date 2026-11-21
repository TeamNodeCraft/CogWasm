//! Stage C host: run rustc.wasm (wasm32-wasip1, cg_clif only) under the latest
//! stable Wasmtime with plain WASI preview 1, service its `playground.spawn`
//! linker import with native Wild, then load and evaluate the linked plugin.
//!
//! rustc is patched to run the compiler on the calling thread on wasm, so the
//! module needs no `wasi:thread-spawn` import and no wasi-threads host.
//!
//! Subcommands:
//!   compile      run rustc.wasm once on a source file, writing a .wasm
//!   eval         load a plugin .wasm, call eval(1,2,3,4), print the result
//!   check        compile + eval and assert the expected result
//!   bench        compile + eval N times, plus cold/cached module-load timings
//!   incremental  cold compile, then N one-line edits with `-C incremental`
//!   inspect      validate a wasm and scan its name/data sections for LLVM symbols

use anyhow::{anyhow, bail, Context, Result};
use cogwasm_runtime::{SpawnSummary, Wt};
use serde::Serialize;
use stage_c_host::precompiled::load_rustc_module;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wasmtime::{Caller, Engine, Extern, Linker, Memory, Module, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::{FsPerms, I32Exit, WasiCtxBuilder};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Opts {
    rustc: PathBuf,
    sysroot: PathBuf,
    src: PathBuf,
    out: PathBuf,
    linker: PathBuf,
    work: PathBuf,
    cache_dir: Option<PathBuf>,
    runs: usize,
    json: Option<PathBuf>,
    expect: f64,
    /// Precompiled Wasmtime artifact for `--rustc`, if supplied.
    precompiled: Option<PathBuf>,
    /// Check the full SHA-256 of the .wasm and .cwasm before deserializing.
    verify_precompiled: bool,
    incremental: Option<PathBuf>,
    fresh_incremental: bool,
    kind: EvalKind,
    crate_name: String,
    /// `-Ccodegen-units=N`, when set. Incremental builds default to 256 CGUs,
    /// which trades output size and inlining for reuse granularity.
    codegen_units: Option<u32>,
    /// `--extern NAME=PATH` arguments for direct (non-cargo) compiles, for
    /// example `libm=/path/liblibm-*.rlib` for the A.2 maths node.
    externs: Vec<String>,
    raw: Vec<String>,
}

/// Which program ABI the linked module is checked with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EvalKind {
    /// Stage-A plugin ABI: `eval(ptr, len, out)` with the `nodecraft` host
    /// import; the default `check` result is 2.25.
    Plugin,
    /// A.2 maths node: `eval`/`eval_stats` over 100k f64 points, checked
    /// against a host-side reference like the A.2 harness does.
    Math,
}

impl EvalKind {
    fn name(self) -> &'static str {
        match self {
            EvalKind::Plugin => "c-plugin",
            EvalKind::Math => "e-math",
        }
    }
}

fn parse_kind(value: &str) -> Result<EvalKind> {
    match value {
        "c-plugin" | "plugin" => Ok(EvalKind::Plugin),
        "e-math" | "math" => Ok(EvalKind::Math),
        other => bail!("--kind must be `c-plugin` or `e-math`, got `{other}`"),
    }
}

/// Infer the crate name from `<crate>/src/lib.rs`, falling back to the historical
/// `c_plugin` name for other layouts (the difftest corpus uses `raw` instead).
fn infer_crate_name(src: &Path) -> String {
    src.parent()
        .and_then(Path::parent)
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("c_plugin")
        .to_owned()
}

/// Infer the ABI from the source path, so `check` works for both the plugin and
/// the A.2 maths node without requiring an extra flag; `--kind` overrides it.
fn infer_kind(src: &Path) -> EvalKind {
    if src.to_string_lossy().contains("e_math") {
        EvalKind::Math
    } else {
        EvalKind::Plugin
    }
}

fn usage() -> &'static str {
    "usage: stage-c-host <compile|eval|check|bench|incremental|inspect> [options]\n\
     \n\
     compile/eval/check/bench/incremental options:\n\
       --rustc PATH       rustc.wasm\n\
       --sysroot DIR      wasm32-unknown-unknown sysroot (pure-Rust core/alloc)\n\
       --src PATH         plugin source (copied into --work before building)\n\
       --out PATH         linked output wasm\n\
       --linker PATH      native Wild binary\n\
       --work DIR         scratch directory (preopened to the guest as /)\n\
       --cache-dir DIR    enable Wasmtime's compiled-module cache\n\
       --precompiled PATH use this precompiled rustc.wasm (.cwasm) when compatible\n\
       --verify-precompiled  also check the full SHA-256 digests before loading\n\
       --runs N           number of measured runs (default 5)\n\
       --json PATH        write machine-readable results\n\
       --expect F         expected eval result (default 2.25)\n\
       --kind KIND        c-plugin or e-math (default: inferred from --src)\n\
       --incremental DIR  pass -Cincremental=DIR (absolute; off by default)\n\
       --fresh-incremental  delete DIR before compiling (needs --incremental)\n\
       --codegen-units N  pass -Ccodegen-units=N (cargo/rustc default otherwise)\n\
       --extern NAME=PATH pass --extern to rustc (repeatable; e.g. libm=...)\n\
     inspect options: --wasm PATH"
}

fn parse_args() -> Result<(String, Opts, Option<PathBuf>)> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "check".into());
    let mut o = Opts {
        rustc: PathBuf::new(),
        sysroot: PathBuf::new(),
        src: PathBuf::from("programs/c_plugin/src/lib.rs"),
        out: PathBuf::from("c_plugin.wasm"),
        linker: PathBuf::new(),
        work: PathBuf::from("work"),
        cache_dir: None,
        runs: 5,
        json: None,
        expect: 2.25,
        precompiled: None,
        verify_precompiled: false,
        incremental: None,
        fresh_incremental: false,
        kind: EvalKind::Plugin,
        crate_name: String::new(),
        codegen_units: None,
        externs: Vec::new(),
        raw: Vec::new(),
    };
    let mut kind = None;
    let mut inspect = None;
    let mut raw_mode = false;
    while let Some(a) = args.next() {
        if raw_mode {
            o.raw.push(a);
            continue;
        }
        if a == "--" {
            raw_mode = true;
            continue;
        }
        let mut val = |name: &str| -> Result<String> {
            args.next().ok_or_else(|| anyhow!("{name} needs a value"))
        };
        match a.as_str() {
            "--rustc" => o.rustc = val("--rustc")?.into(),
            "--sysroot" => o.sysroot = val("--sysroot")?.into(),
            "--src" => o.src = val("--src")?.into(),
            "--out" => o.out = val("--out")?.into(),
            "--linker" => o.linker = val("--linker")?.into(),
            "--work" => o.work = val("--work")?.into(),
            "--cache-dir" => o.cache_dir = Some(val("--cache-dir")?.into()),
            "--precompiled" => o.precompiled = Some(val("--precompiled")?.into()),
            "--verify-precompiled" => o.verify_precompiled = true,
            "--runs" => o.runs = val("--runs")?.parse().context("--runs")?,
            "--json" => o.json = Some(val("--json")?.into()),
            "--expect" => o.expect = val("--expect")?.parse().context("--expect")?,
            "--kind" => kind = Some(parse_kind(&val("--kind")?)?),
            "--incremental" => o.incremental = Some(val("--incremental")?.into()),
            "--fresh-incremental" => o.fresh_incremental = true,
            "--codegen-units" => {
                o.codegen_units = Some(val("--codegen-units")?.parse().context("--codegen-units")?)
            }
            "--extern" => o.externs.push(val("--extern")?),
            "--wasm" => inspect = Some(PathBuf::from(val("--wasm")?)),
            "-h" | "--help" => bail!("{}", usage()),
            other => bail!("unknown option {other}\n{}", usage()),
        }
    }
    o.kind = kind.unwrap_or_else(|| infer_kind(&o.src));
    o.crate_name = infer_crate_name(&o.src);
    if let Some(dir) = &o.incremental {
        if !dir.is_absolute() {
            bail!(
                "--incremental must be an absolute path: rustc.wasm sees the host root as guest `/`, \
                 so a relative path would resolve to the guest's current directory, not this one"
            );
        }
    }
    if o.fresh_incremental {
        let dir = o
            .incremental
            .as_ref()
            .ok_or_else(|| anyhow!("--fresh-incremental needs --incremental DIR"))?;
        clear_incremental_dir(dir)?;
    }
    Ok((cmd, o, inspect))
}

/// Delete an incremental directory for `--fresh-incremental`. This is never
/// done implicitly: persistent incremental state is only removed when the
/// caller asks for it, and only for a dedicated directory.
fn clear_incremental_dir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    if !dir.is_dir() {
        bail!(
            "--incremental {} exists but is not a directory",
            dir.display()
        );
    }
    // Refuse the root and shallow system directories. An incremental directory
    // is always a dedicated subdirectory such as /tmp/inc/inc.
    if dir.components().count() < 3 {
        bail!(
            "refusing to delete --incremental {}: give a dedicated directory such as /tmp/inc/inc",
            dir.display()
        );
    }
    if std::env::var_os("HOME").is_some_and(|home| Path::new(&home) == dir) {
        bail!("refusing to delete the home directory as an incremental directory");
    }
    std::fs::remove_dir_all(dir)
        .with_context(|| format!("removing incremental directory {}", dir.display()))
}

// ---------------------------------------------------------------------------
// Engine / store
// ---------------------------------------------------------------------------

/// The one engine configuration shared with `stage-c-precompile`; a `.cwasm`
/// is only valid for an engine built from exactly this configuration.
fn engine(cache_dir: Option<&Path>) -> Result<Engine> {
    stage_c_host::config::engine(cache_dir)
}

/// The pieces needed to build a fresh WASI context for an instance.
#[derive(Clone)]
struct WasiSpec {
    args: Vec<String>,
    env: Vec<(String, String)>,
    preopens: Vec<(PathBuf, String)>,
}

impl WasiSpec {
    fn build(&self) -> Result<WasiP1Ctx> {
        let mut b = WasiCtxBuilder::new();
        b.inherit_stdout().inherit_stderr().args(&self.args);
        for (k, v) in &self.env {
            b.env(k, v);
        }
        for (host, guest) in &self.preopens {
            b.preopened_dir(host, guest, FsPerms::ReadWrite)
                .wt("preopening guest directory")?;
        }
        Ok(b.build_p1())
    }
}

struct SpawnRecord {
    argv: Vec<String>,
    ms: f64,
    status: i32,
}

/// Per-store host state. The spawn log is shared so the top level can report
/// all link invocations.
struct HostState {
    wasi: Option<WasiP1Ctx>,
    spawns: Arc<Mutex<Vec<SpawnRecord>>>,
}

fn new_store(engine: &Engine, args: Vec<String>) -> Result<Store<HostState>> {
    let spec = Arc::new(WasiSpec {
        args,
        env: {
            let mut env = vec![
                ("RUST_MIN_STACK".into(), "16777216".into()),
                ("RUST_BACKTRACE".into(), "1".into()),
            ];
            // cg_clif's strict mode is read inside the guest.
            if let Ok(v) = std::env::var("CG_CLIF_WASM_STRICT") {
                env.push(("CG_CLIF_WASM_STRICT".into(), v));
            }
            env
        },
        // rustc sees the host filesystem at the same absolute paths, so the
        // native Wild process can consume the linker arguments unchanged.
        preopens: vec![(PathBuf::from("/"), "/".into())],
    });
    let wasi = spec.build()?;
    Ok(Store::new(
        engine,
        HostState {
            wasi: Some(wasi),
            spawns: Arc::new(Mutex::new(Vec::new())),
        },
    ))
}

#[derive(Clone)]
struct GuestMem(Memory);

impl GuestMem {
    fn read(&self, caller: &Caller<'_, HostState>, off: usize, buf: &mut [u8]) -> Result<()> {
        self.0
            .read(caller, off, buf)
            .with_context(|| format!("reading guest memory at {off:#x}"))
    }
}

fn guest_memory(caller: &mut Caller<'_, HostState>) -> Result<GuestMem> {
    match caller.get_export("memory") {
        Some(Extern::Memory(m)) => Ok(GuestMem(m)),
        Some(_) => bail!("rustc.wasm export `memory` is not a memory"),
        None => bail!("rustc.wasm does not export memory"),
    }
}

fn read_cstr(caller: &Caller<'_, HostState>, mem: &GuestMem, ptr: u32) -> Result<String> {
    let mut out = Vec::new();
    let mut off = ptr as usize;
    for _ in 0..(1 << 20) {
        let mut byte = [0u8; 1];
        mem.read(caller, off, &mut byte)
            .with_context(|| format!("reading guest string at {ptr:#x}"))?;
        if byte[0] == 0 {
            break;
        }
        out.push(byte[0]);
        off += 1;
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

fn add_playground_spawn(linker: &mut Linker<HostState>, o: &Opts) -> Result<()> {
    let linker_path = o.linker.clone();
    linker
        .func_wrap(
            "playground",
            "spawn",
            move |mut caller: Caller<'_, HostState>, argv: u32, argc: u32| -> i32 {
                match spawn_impl(&mut caller, argv, argc, &linker_path) {
                    Ok(code) => code,
                    Err(e) => {
                        eprintln!("[stage-c host] spawn failed: {e:#}");
                        -1
                    }
                }
            },
        )
        .wt("defining playground::spawn")?;
    Ok(())
}

fn spawn_impl(
    caller: &mut Caller<'_, HostState>,
    argv: u32,
    argc: u32,
    linker: &Path,
) -> Result<i32> {
    let mem = guest_memory(caller)?;
    let mut raw = vec![0u8; (argc as usize) * 4];
    mem.read(caller, argv as usize, &mut raw)
        .context("reading argv pointer array")?;
    let mut args = Vec::with_capacity(argc as usize);
    for chunk in raw.chunks_exact(4) {
        let p = u32::from_le_bytes(chunk.try_into().unwrap());
        args.push(read_cstr(caller, &mem, p)?);
    }
    if std::env::var_os("STAGE_C_SPAWN_DEBUG").is_some() {
        eprintln!("[stage-c host] spawn argv: {args:?}");
    }
    // The rustc patch puts the linker program name first. Run native Wild on
    // the remaining arguments, inheriting stdio and the filesystem view.
    let t = Instant::now();
    let status = Command::new(linker)
        .args(args.iter().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let code = match status {
        Ok(s) if s.success() => 0,
        Ok(s) => s.code().unwrap_or(-1),
        Err(e) => {
            eprintln!("[stage-c host] cannot run {}: {e}", linker.display());
            -1
        }
    };
    eprintln!(
        "[stage-c host] spawn: {} ({} args) -> {code} in {ms:.1} ms",
        args.first().map(String::as_str).unwrap_or("<empty>"),
        args.len().saturating_sub(1)
    );
    caller.data().spawns.lock().unwrap().push(SpawnRecord {
        argv: args,
        ms,
        status: code,
    });
    Ok(code)
}

/// Create the full linker (WASI p1 and playground.spawn) and the store for one
/// rustc.wasm instance.
fn prepare_instance(
    engine: &Engine,
    module: &Module,
    o: &Opts,
    args: Vec<String>,
) -> Result<(Store<HostState>, wasmtime::Instance)> {
    let mut linker: Linker<HostState> = Linker::new(engine);
    p1::add_to_linker_sync(&mut linker, |s: &mut HostState| {
        s.wasi.as_mut().expect("WASI context present")
    })
    .wt("adding WASI p1 imports")?;
    add_playground_spawn(&mut linker, o)?;

    let mut store = new_store(engine, args)?;
    let instance = linker
        .instantiate(&mut store, module)
        .wt("instantiating rustc.wasm")?;
    Ok((store, instance))
}

// ---------------------------------------------------------------------------
// rustc.wasm
// ---------------------------------------------------------------------------

fn rustc_args(o: &Opts) -> Vec<String> {
    let mut args = vec![
        "rustc".into(),
        o.src.display().to_string(),
        "--crate-name".into(),
        o.crate_name.clone(),
        "--crate-type=cdylib".into(),
        "--edition=2021".into(),
        "--target=wasm32-unknown-unknown".into(),
        "--sysroot".into(),
        o.sysroot.display().to_string(),
        "-Copt-level=2".into(),
        "-Cpanic=abort".into(),
        "-Zcodegen-backend=cranelift".into(),
        "-Clinker=wild".into(),
        "-Clinker-flavor=wasm-ld".into(),
        "-o".into(),
        o.out.display().to_string(),
    ];
    if let Some(dir) = &o.incremental {
        args.push(format!("-Cincremental={}", dir.display()));
    }
    if let Some(units) = o.codegen_units {
        args.push(format!("-Ccodegen-units={units}"));
    }
    for external in &o.externs {
        args.push("--extern".into());
        args.push(external.clone());
    }
    args
}

#[derive(Serialize, Clone)]
struct RunResult {
    module_load_ms: f64,
    instantiate_ms: f64,
    rustc_total_ms: f64,
    link_ms: f64,
    compile_ms: f64,
    plugin_load_ms: f64,
    eval_ms: f64,
    end_to_end_ms: f64,
    rc: i32,
    result: f64,
    spawns: Vec<SpawnSummary>,
}

fn run_rustc_with(
    o: &Opts,
    engine: &Engine,
    module: &Module,
    measure_plugin: bool,
) -> Result<RunResult> {
    let args = rustc_args(o);
    let t = Instant::now();
    let (mut store, instance) = prepare_instance(engine, module, o, args)?;
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
        .spawns
        .lock()
        .unwrap()
        .iter()
        .map(|s| SpawnSummary {
            program: s.argv.first().cloned().unwrap_or_default(),
            args: s.argv.len().saturating_sub(1),
            ms: s.ms,
            status: s.status,
        })
        .collect::<Vec<_>>();
    let link_ms: f64 = spawns.iter().map(|s| s.ms).sum();
    if spawns.iter().any(|s| s.status != 0) {
        bail!("linker failed: {:?}", spawns);
    }

    let (plugin_load_ms, eval_ms, rc, result) = if measure_plugin {
        eval_output(o)?
    } else {
        (0.0, 0.0, 0, f64::NAN)
    };

    // A 137 MiB compiled module + 1 GiB shared memory can take seconds to
    // tear down. A hot-reload host would keep one engine/module alive, so the
    // measured edit-to-result path deliberately excludes teardown.
    std::mem::forget(store);

    Ok(RunResult {
        module_load_ms: 0.0,
        instantiate_ms,
        rustc_total_ms,
        link_ms,
        compile_ms: (rustc_total_ms - link_ms).max(0.0),
        plugin_load_ms,
        eval_ms,
        end_to_end_ms: instantiate_ms + rustc_total_ms + plugin_load_ms + eval_ms,
        rc,
        result,
        spawns,
    })
}

fn run_rustc(o: &Opts, expect: f64, measure_plugin: bool) -> Result<(RunResult, f64)> {
    let engine = engine(o.cache_dir.as_deref())?;
    let t = Instant::now();
    let loaded = load_rustc_module(
        &engine,
        &o.rustc,
        o.precompiled.as_deref(),
        o.verify_precompiled,
    )?;
    let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
    if let Some(reason) = &loaded.fallback_reason {
        eprintln!("[stage-c host] precompiled artifact rejected: {reason}");
    }
    eprintln!(
        "[stage-c host] rustc.wasm loaded from {} in {module_load_ms:.1} ms",
        loaded.source.as_str()
    );
    let mut r = run_rustc_with(o, &engine, &loaded.module, measure_plugin)?;
    r.module_load_ms = module_load_ms;
    let _ = expect;
    std::mem::forget(loaded.module);
    std::mem::forget(engine);
    Ok((r, module_load_ms))
}

// ---------------------------------------------------------------------------
// plugin evaluation
// ---------------------------------------------------------------------------

fn eval_plugin(o: &Opts, expect: f64) -> Result<(f64, f64, i32, f64)> {
    let engine = engine(None)?;
    let t = Instant::now();
    let module = Module::from_file(&engine, &o.out).wt("loading plugin")?;
    let load_ms = t.elapsed().as_secs_f64() * 1e3;

    let mut linker = Linker::new(&engine);
    linker
        .func_wrap("nodecraft", "host_bias", |x: f64| -> f64 { x * 0.5 + 1.0 })
        .wt("defining nodecraft::host_bias")?;
    let mut store = Store::new(&engine, ());
    let instance = linker
        .instantiate(&mut store, &module)
        .wt("instantiating plugin")?;
    let memory = instance
        .get_memory(&mut store, "memory")
        .context("plugin memory export")?;
    let alloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
        .wt("plugin wasm_alloc")?;
    let eval = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval")
        .wt("plugin eval")?;
    let params = [1.0f64, 2.0, 3.0, 4.0];
    let in_ptr = alloc.call(&mut store, ((params.len() * 8) as i32, 8))?;
    let out_ptr = alloc.call(&mut store, (8, 8))?;
    let mut bytes = Vec::with_capacity(params.len() * 8);
    for p in params {
        bytes.extend_from_slice(&p.to_le_bytes());
    }
    memory.write(&mut store, in_ptr as usize, &bytes)?;
    let t = Instant::now();
    let rc = eval.call(&mut store, (in_ptr, params.len() as i32, out_ptr))?;
    let eval_ms = t.elapsed().as_secs_f64() * 1e3;
    let mut out = [0u8; 8];
    memory.read(&store, out_ptr as usize, &mut out)?;
    let result = f64::from_le_bytes(out);
    if rc != 0 {
        bail!("plugin eval returned {rc}");
    }
    if (result - expect).abs() > 1e-9 {
        bail!("plugin eval returned {result}, expected {expect}");
    }
    Ok((load_ms, eval_ms, rc, result))
}

/// Run the check that matches the linked crate's ABI. `eval_plugin` validates
/// the expected 2.25; `eval_math` repeats the A.2 harness's reference check.
fn eval_output(o: &Opts) -> Result<(f64, f64, i32, f64)> {
    match o.kind {
        EvalKind::Plugin => eval_plugin(o, o.expect),
        EvalKind::Math => eval_math(o),
    }
}

fn math_input(n: usize, i: usize) -> f64 {
    (i as f64 - n as f64 / 2.0) * 0.001
}

fn math_transform(x: f64) -> f64 {
    x.sin() * 0.5 + (x * 0.25).cos() + x.abs().sqrt()
}

fn math_close(a: f64, b: f64) -> bool {
    if a == b {
        return true;
    }
    // Same tolerance as the A.2 harness: libm in the guest and the host's
    // libm can differ by a few ULP.
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)
}

/// Compile-and-run check for the A.2 maths node `(e)`. It uses the same input
/// sequence, transform and `eval_stats` call as `a2-harness check`, so an
/// incremental build is only accepted when it returns the reference mean, min
/// and max.
fn eval_math(o: &Opts) -> Result<(f64, f64, i32, f64)> {
    eval_math_expect(o, 0.0)
}

/// `mean_tweak` shifts only the expected mean. The benchmark uses it for the
/// semantics-changing edit, which moves the mean by `TWEAK` and leaves min and
/// max unchanged.
fn eval_math_expect(o: &Opts, mean_tweak: f64) -> Result<(f64, f64, i32, f64)> {
    let engine = engine(None)?;
    let t = Instant::now();
    let module = Module::from_file(&engine, &o.out).wt("loading e_math")?;
    let load_ms = t.elapsed().as_secs_f64() * 1e3;

    let mut store = Store::new(&engine, ());
    let instance = Linker::new(&engine)
        .instantiate(&mut store, &module)
        .wt("instantiating e_math")?;
    let memory = instance
        .get_memory(&mut store, "memory")
        .context("e_math memory export")?;
    let alloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
        .wt("e_math wasm_alloc")?;
    let eval_stats = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval_stats")
        .wt("e_math eval_stats")?;

    const N: usize = 100_000;
    let in_ptr = alloc.call(&mut store, ((N * 8) as i32, 8))?;
    let out_ptr = alloc.call(&mut store, (24, 8))?;
    if in_ptr == 0 || out_ptr == 0 {
        bail!("e_math wasm_alloc returned null");
    }

    let mut input = Vec::with_capacity(N * 8);
    let mut ref_sum = 0.0f64;
    let mut ref_min = f64::INFINITY;
    let mut ref_max = f64::NEG_INFINITY;
    for i in 0..N {
        let x = math_input(N, i);
        let y = math_transform(x);
        ref_sum += y;
        ref_min = ref_min.min(y);
        ref_max = ref_max.max(y);
        input.extend_from_slice(&x.to_le_bytes());
    }
    // The semantics-changing edit adds a constant to the mean, not min/max.
    let ref_mean = ref_sum / N as f64 + mean_tweak;
    memory.write(&mut store, in_ptr as usize, &input)?;

    let t = Instant::now();
    let rc = eval_stats.call(&mut store, (in_ptr, N as i32, out_ptr))?;
    let eval_ms = t.elapsed().as_secs_f64() * 1e3;
    if rc != 0 {
        bail!("e_math eval_stats returned {rc}");
    }
    let mut stats = [0u8; 24];
    memory.read(&store, out_ptr as usize, &mut stats)?;
    let mean = f64::from_le_bytes(stats[0..8].try_into().unwrap());
    let min = f64::from_le_bytes(stats[8..16].try_into().unwrap());
    let max = f64::from_le_bytes(stats[16..24].try_into().unwrap());
    if !math_close(mean, ref_mean) {
        bail!("e_math mean {mean:.17e} does not match reference {ref_mean:.17e}");
    }
    if !math_close(min, ref_min) {
        bail!("e_math min {min:.17e} does not match reference {ref_min:.17e}");
    }
    if !math_close(max, ref_max) {
        bail!("e_math max {max:.17e} does not match reference {ref_max:.17e}");
    }
    Ok((load_ms, eval_ms, rc, mean))
}

// ---------------------------------------------------------------------------
// inspect / LLVM scan
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct Inspect {
    bytes: usize,
    code_bytes: u64,
    functions: u64,
    valid: bool,
    imports: Vec<(String, String)>,
    exports: Vec<String>,
    llvm_raw_occurrences: usize,
    llvm_function_names: Vec<String>,
    llvm_imports: Vec<String>,
    llvm_strings_in_data: usize,
}

fn inspect_wasm(path: &Path) -> Result<Inspect> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(&bytes)
        .context("wasm validation")?;
    let mut imports = Vec::new();
    let mut exports = Vec::new();
    let mut code_bytes = 0u64;
    let mut functions = 0u64;
    let mut llvm_function_names = Vec::new();
    let mut llvm_imports = Vec::new();
    let mut llvm_strings_in_data = 0usize;
    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        match payload? {
            wasmparser::Payload::ImportSection(s) => {
                for group in s {
                    for item in group? {
                        let (_off, imp) = item?;
                        let module = imp.module.to_string();
                        let name = imp.name.to_string();
                        if module.to_ascii_lowercase().contains("llvm")
                            || name.to_ascii_lowercase().contains("llvm")
                        {
                            llvm_imports.push(format!("{module}::{name}"));
                        }
                        imports.push((module, name));
                    }
                }
            }
            wasmparser::Payload::ExportSection(s) => {
                for e in s {
                    exports.push(e?.name.to_string());
                }
            }
            wasmparser::Payload::CodeSectionEntry(body) => {
                code_bytes += body.range().end - body.range().start;
                functions += 1;
            }
            wasmparser::Payload::CustomSection(c) => {
                if c.name() == "name" {
                    let mut r = wasmparser::NameSectionReader::new(wasmparser::BinaryReader::new(
                        c.data(),
                        c.data_offset(),
                    ));
                    while let Some(Ok(entry)) = r.next() {
                        if let wasmparser::Name::Function(n) = entry {
                            for ass in n {
                                let Ok(ass) = ass else { continue };
                                let name = ass.name.to_string();
                                if name.to_ascii_lowercase().contains("llvm") {
                                    llvm_function_names.push(name);
                                }
                            }
                        }
                    }
                }
            }
            wasmparser::Payload::DataSection(s) => {
                for d in s {
                    let d = d?;
                    llvm_strings_in_data += count_llvm(d.data);
                }
            }
            _ => {}
        }
    }
    Ok(Inspect {
        bytes: bytes.len(),
        code_bytes,
        functions,
        valid: true,
        imports,
        exports,
        llvm_raw_occurrences: count_llvm(&bytes),
        llvm_function_names,
        llvm_imports,
        llvm_strings_in_data,
    })
}

fn count_llvm(bytes: &[u8]) -> usize {
    bytes
        .windows(4)
        .filter(|w| w.eq_ignore_ascii_case(b"llvm"))
        .count()
}

// ---------------------------------------------------------------------------
// bench
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct Bench {
    rustc_bytes: usize,
    plugin_bytes: usize,
    rustc_sha256: String,
    module_load_cold_no_cache_ms: Vec<f64>,
    module_load_cached_ms: Vec<f64>,
    instantiate_ms: Vec<f64>,
    runs: Vec<RunResult>,
    medians: BTreeMap<String, f64>,
    result: f64,
    expect: f64,
    ok: bool,
}

fn sha256_hex(bytes: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = bytes.to_vec();
    let bit_len = (bytes.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

fn median(v: &[f64]) -> f64 {
    let mut x = v.to_vec();
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if x.is_empty() {
        f64::NAN
    } else if x.len() % 2 == 0 {
        (x[x.len() / 2 - 1] + x[x.len() / 2]) / 2.0
    } else {
        x[x.len() / 2]
    }
}

fn startup_once(o: &Opts, cached: bool) -> Result<()> {
    let e = engine(if cached { o.cache_dir.as_deref() } else { None })?;
    let t = Instant::now();
    let loaded = load_rustc_module(&e, &o.rustc, o.precompiled.as_deref(), o.verify_precompiled)?;
    let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
    let bytes = std::fs::metadata(&o.rustc)
        .with_context(|| format!("reading size of {}", o.rustc.display()))?
        .len();
    let t = Instant::now();
    let (store, _instance) = prepare_instance(&e, &loaded.module, o, rustc_args(o))?;
    let instantiate_ms = t.elapsed().as_secs_f64() * 1e3;
    std::mem::forget(store);
    println!(
        "{}",
        serde_json::json!({
            "mode": if cached { "cached" } else { "cold" },
            "module_load_ms": module_load_ms,
            "instantiate_ms": instantiate_ms,
            "bytes": bytes,
            "source": loaded.source.as_str(),
            "fallback_reason": loaded.fallback_reason,
        })
    );
    std::process::exit(0);
}

fn self_args(o: &Opts, cmd: &str, cached: bool) -> Vec<String> {
    let mut v = vec![
        cmd.into(),
        "--rustc".into(),
        o.rustc.display().to_string(),
        "--sysroot".into(),
        o.sysroot.display().to_string(),
        "--src".into(),
        o.src.display().to_string(),
        "--linker".into(),
        o.linker.display().to_string(),
        "--work".into(),
        o.work.display().to_string(),
        "--out".into(),
        o.out.display().to_string(),
    ];
    if cached {
        v.push("--cache-dir".into());
        v.push(o.cache_dir.as_ref().unwrap().display().to_string());
    }
    if let Some(precompiled) = &o.precompiled {
        v.push("--precompiled".into());
        v.push(precompiled.display().to_string());
    }
    if o.verify_precompiled {
        v.push("--verify-precompiled".into());
    }
    v
}

fn subprocess_startup(o: &Opts, cmd: &str, cached: bool) -> Result<(f64, f64)> {
    let exe = std::env::current_exe()?;
    let out = Command::new(exe)
        .args(self_args(o, cmd, cached))
        .output()
        .with_context(|| format!("running {cmd} subprocess"))?;
    if !out.status.success() {
        bail!(
            "{cmd} failed: {}
{}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text
        .lines()
        .rev()
        .find(|l| l.trim_start().starts_with('{'))
        .ok_or_else(|| anyhow!("{cmd}: no JSON in output: {text}"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;
    Ok((
        v["module_load_ms"].as_f64().unwrap_or(f64::NAN),
        v["instantiate_ms"].as_f64().unwrap_or(f64::NAN),
    ))
}

fn bench(o: &Opts) -> Result<Bench> {
    std::fs::create_dir_all(&o.work)?;
    let rustc_bytes = std::fs::read(&o.rustc)?;
    let sha = sha256_hex(&rustc_bytes);

    // Cold module compilation and cached loads are measured in child
    // processes: a 137 MiB compiled module takes seconds to free, and keeping
    // several alive at once thrashes memory. Each child exits without running
    // destructors.
    let mut cold = Vec::new();
    let mut inst = Vec::new();
    for _ in 0..o.runs {
        let (load, i) = subprocess_startup(o, "cold-once", false)?;
        cold.push(load);
        inst.push(i);
    }

    let mut cached = Vec::new();
    if let Some(dir) = &o.cache_dir {
        std::fs::create_dir_all(dir)?;
        // Prime the cache once; this is the only cold compile on this path.
        let _ = subprocess_startup(o, "cached-once", true)?;
        for _ in 0..o.runs {
            let (load, _) = subprocess_startup(o, "cached-once", true)?;
            cached.push(load);
        }
    }

    // Warm edit-to-result: keep one engine and module resident (as a hot-reload
    // host does) and instantiate rustc.wasm fresh for each one-line edit.
    let warm_engine = engine(o.cache_dir.as_deref())?;
    let t = Instant::now();
    let warm_loaded = load_rustc_module(
        &warm_engine,
        &o.rustc,
        o.precompiled.as_deref(),
        o.verify_precompiled,
    )?;
    let warm_module = warm_loaded.module;
    let warm_module_load_ms = t.elapsed().as_secs_f64() * 1e3;
    if warm_module_load_ms > 5000.0 {
        eprintln!(
            "[stage-c host] warning: rustc.wasm module load was {warm_module_load_ms:.0} ms; pass --cache-dir for warm runs"
        );
    }

    let mut runs = Vec::new();
    for i in 0..o.runs {
        tweak_source(&o.src, o.kind, i % 2 == 0)?;
        let r = run_rustc_with(o, &warm_engine, &warm_module, true)?;
        println!(
            "run {i}: instantiate {:.1} ms, rustc {:.1} ms (compile {:.1}, link {:.1}), plugin load {:.1} ms, eval {:.1} ms, edit-to-result {:.1} ms, result {}",
            r.instantiate_ms, r.rustc_total_ms, r.compile_ms, r.link_ms,
            r.plugin_load_ms, r.eval_ms, r.end_to_end_ms, r.result
        );
        runs.push(r);
    }

    let mut med = BTreeMap::new();
    med.insert("module_load_cold_no_cache_ms".into(), median(&cold));
    med.insert("module_load_cached_ms".into(), median(&cached));
    med.insert("warm_module_load_ms".into(), warm_module_load_ms);
    med.insert("instantiate_ms".into(), median(&inst));
    med.insert(
        "instantiate_warm_ms".into(),
        median(&runs.iter().map(|r| r.instantiate_ms).collect::<Vec<_>>()),
    );
    for (name, f) in [
        (
            "rustc_total_ms",
            (|r: &RunResult| r.rustc_total_ms) as fn(&RunResult) -> f64,
        ),
        ("compile_ms", |r: &RunResult| r.compile_ms),
        ("link_ms", |r: &RunResult| r.link_ms),
        ("plugin_load_ms", |r: &RunResult| r.plugin_load_ms),
        ("eval_ms", |r: &RunResult| r.eval_ms),
        ("edit_to_result_ms", |r: &RunResult| r.end_to_end_ms),
    ] {
        med.insert(name.into(), median(&runs.iter().map(f).collect::<Vec<_>>()));
    }
    let result = runs.last().map(|r| r.result).unwrap_or(f64::NAN);
    // `eval_math` has already compared against the A.2 reference; for the
    // plugin, `eval_plugin` enforces `--expect` and this repeats the check for
    // the JSON summary.
    let ok = match o.kind {
        EvalKind::Plugin => (result - o.expect).abs() < 1e-9,
        EvalKind::Math => !runs.is_empty(),
    };
    let out = Bench {
        rustc_bytes: rustc_bytes.len(),
        plugin_bytes: std::fs::metadata(&o.out)
            .map(|m| m.len() as usize)
            .unwrap_or(0),
        rustc_sha256: sha,
        module_load_cold_no_cache_ms: cold,
        module_load_cached_ms: cached,
        instantiate_ms: inst,
        runs,
        medians: med,
        result,
        expect: o.expect,
        ok,
    };
    std::mem::forget(warm_module);
    std::mem::forget(warm_engine);
    Ok(out)
}

// ---------------------------------------------------------------------------
// incremental
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct Incremental {
    kind: String,
    crate_name: String,
    incremental_dir: String,
    cold: RunResult,
    runs: Vec<RunResult>,
    first_incremental_compile_ms: f64,
    first_incremental_edit_to_result_ms: f64,
    cold_wasm_sha256: String,
    incremental_wasm_sha256: Vec<String>,
    wasm_identical: bool,
    results_identical: bool,
    /// The extra edit that does change the result was compiled with the same
    /// incremental directory and returned the expected new result.
    semantics_change_verified: bool,
    changed_edit_wasm_differs: bool,
    changed_edit_result: f64,
    changed_edit_expected: f64,
    incremental_dir_files: usize,
    incremental_dir_bytes: u64,
    medians: BTreeMap<String, f64>,
    ok: bool,
}

/// Recursively count regular files and their bytes under `path`.
fn directory_stats(path: &Path) -> Result<(usize, u64)> {
    if !path.exists() {
        return Ok((0, 0));
    }
    let mut files = 0usize;
    let mut bytes = 0u64;
    let mut pending = vec![path.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(&dir)
            .with_context(|| format!("reading incremental directory {}", dir.display()))?
        {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                files += 1;
                bytes += entry.metadata()?.len();
            }
        }
    }
    Ok((files, bytes))
}

fn incremental(o: &mut Opts) -> Result<Incremental> {
    let dir = o
        .incremental
        .clone()
        .ok_or_else(|| anyhow!("incremental needs --incremental DIR"))?;
    prepare_source(o)?;

    // The first tweak inserts the toggled constant; later tweaks are one-line
    // edits. Do it before the cold compile so both builds see the same source
    // shape.
    tweak_source(&o.src, o.kind, true)?;

    // One engine and one compiled rustc.wasm module stay resident, as in a
    // hot-reload host. Every measured compile still gets a fresh instance.
    let engine = engine(o.cache_dir.as_deref())?;
    let module = load_rustc_module(
        &engine,
        &o.rustc,
        o.precompiled.as_deref(),
        o.verify_precompiled,
    )?
    .module;

    // Cold compile: same host path, but no incremental directory at all.
    let cold_opts = Opts {
        incremental: None,
        ..o.clone()
    };
    let cold = run_rustc_with(&cold_opts, &engine, &module, true)?;
    let cold_sha = sha256_hex(&std::fs::read(&o.out)?);
    println!(
        "cold: instantiate {:.1} ms, rustc {:.1} ms (compile {:.1}, link {:.1}), eval {:.1} ms, edit-to-result {:.1} ms, result {}",
        cold.instantiate_ms,
        cold.rustc_total_ms,
        cold.compile_ms,
        cold.link_ms,
        cold.eval_ms,
        cold.end_to_end_ms,
        cold.result
    );

    let mut runs = Vec::with_capacity(o.runs);
    let mut shas = Vec::with_capacity(o.runs);
    for i in 0..o.runs {
        let before = std::fs::read_to_string(&o.src)
            .with_context(|| format!("reading source {}", o.src.display()))?;
        tweak_source(&o.src, o.kind, i % 2 == 1)?;
        let after = std::fs::read_to_string(&o.src)
            .with_context(|| format!("reading source {}", o.src.display()))?;
        if before == after {
            bail!(
                "the one-line edit did not change {}; refusing to report a no-op as an edit",
                o.src.display()
            );
        }

        let r = run_rustc_with(o, &engine, &module, true)?;
        shas.push(sha256_hex(&std::fs::read(&o.out)?));
        println!(
            "incremental run {i}: instantiate {:.1} ms, rustc {:.1} ms (compile {:.1}, link {:.1}), eval {:.1} ms, edit-to-result {:.1} ms, result {}",
            r.instantiate_ms,
            r.rustc_total_ms,
            r.compile_ms,
            r.link_ms,
            r.eval_ms,
            r.end_to_end_ms,
            r.result
        );
        runs.push(r);
    }

    // The measured edits preserve the result on purpose, so a cache bug that
    // ignored a source change would still pass every check above. Prove that a
    // semantics-changing edit is not served from the stale cache: set TWEAK to
    // 1.0 and require the new result (measured compile time is not reported).
    set_tweak(&o.src, 1.0)?;
    let changed_compile = run_rustc_with(o, &engine, &module, false)?;
    let changed_sha = sha256_hex(&std::fs::read(&o.out)?);
    let (changed_expected, changed_result) = match o.kind {
        EvalKind::Plugin => {
            // `eval` divides the sum by its four parameters, so TWEAK = 1.0
            // moves the mean by 1/4.
            let expected = cold.result + 1.0 / 4.0;
            let (_, _, _, result) = eval_plugin(o, expected)?;
            (expected, result)
        }
        EvalKind::Math => {
            let expected = cold.result + 1.0;
            let (_, _, _, result) = eval_math_expect(o, 1.0)?;
            (expected, result)
        }
    };
    let changed_edit_wasm_differs = changed_sha != cold_sha;
    println!(
        "semantics-changing edit: compile {:.1} ms, result {} (expected {}), wasm differs from cold: {}",
        changed_compile.compile_ms,
        changed_result,
        changed_expected,
        changed_edit_wasm_differs
    );

    let mut med = BTreeMap::new();
    med.insert("cold_compile_ms".into(), cold.compile_ms);
    med.insert("cold_link_ms".into(), cold.link_ms);
    med.insert("cold_edit_to_result_ms".into(), cold.end_to_end_ms);
    if let Some(first) = runs.first() {
        med.insert("first_incremental_compile_ms".into(), first.compile_ms);
        med.insert(
            "first_incremental_edit_to_result_ms".into(),
            first.end_to_end_ms,
        );
    }
    for (name, f) in [
        (
            "incremental_compile_ms",
            (|r: &RunResult| r.compile_ms) as fn(&RunResult) -> f64,
        ),
        ("incremental_link_ms", |r: &RunResult| r.link_ms),
        ("incremental_edit_to_result_ms", |r: &RunResult| {
            r.end_to_end_ms
        }),
    ] {
        med.insert(name.into(), median(&runs.iter().map(f).collect::<Vec<_>>()));
    }
    // The first incremental run after a cold build has no cache to reuse, so
    // report the steady state (the remaining edits) separately as well.
    if runs.len() > 1 {
        let steady = &runs[1..];
        med.insert(
            "steady_incremental_compile_ms".into(),
            median(&steady.iter().map(|r| r.compile_ms).collect::<Vec<_>>()),
        );
        med.insert(
            "steady_incremental_edit_to_result_ms".into(),
            median(&steady.iter().map(|r| r.end_to_end_ms).collect::<Vec<_>>()),
        );
    }

    let wasm_identical = shas.iter().all(|sha| *sha == cold_sha);
    let results_identical = runs
        .iter()
        .all(|r| r.result.to_bits() == cold.result.to_bits());
    let (files, bytes) = directory_stats(&dir)?;
    let first_incremental_compile_ms = runs.first().map(|r| r.compile_ms).unwrap_or(f64::NAN);
    let first_incremental_edit_to_result_ms =
        runs.first().map(|r| r.end_to_end_ms).unwrap_or(f64::NAN);
    let out = Incremental {
        kind: o.kind.name().to_owned(),
        crate_name: o.crate_name.clone(),
        incremental_dir: dir.display().to_string(),
        cold: cold.clone(),
        runs,
        first_incremental_compile_ms,
        first_incremental_edit_to_result_ms,
        cold_wasm_sha256: cold_sha,
        incremental_wasm_sha256: shas,
        wasm_identical,
        results_identical,
        semantics_change_verified: true,
        changed_edit_wasm_differs,
        changed_edit_result: changed_result,
        changed_edit_expected: changed_expected,
        incremental_dir_files: files,
        incremental_dir_bytes: bytes,
        medians: med,
        ok: results_identical,
    };
    std::mem::forget(module);
    std::mem::forget(engine);
    Ok(out)
}

/// Set the `TWEAK` constant inserted by `tweak_source` to an explicit value.
fn set_tweak(src: &Path, value: f64) -> Result<()> {
    let s = std::fs::read_to_string(src)
        .with_context(|| format!("reading source {}", src.display()))?;
    let mut found = false;
    let patched = s
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("const TWEAK: f64 = ") {
                found = true;
                format!("const TWEAK: f64 = {value:?};")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !found {
        bail!(
            "no TWEAK constant in {}; tweak_source must run first",
            src.display()
        );
    }
    std::fs::write(src, patched).with_context(|| format!("writing {}", src.display()))?;
    Ok(())
}

/// Toggle a constant in the source so each measured run is a real one-line
/// edit whose result stays the same: adding `+ 0.0` / `+ -0.0` is exact for
/// every finite value in these programs.
fn tweak_source(src: &Path, kind: EvalKind, positive: bool) -> Result<()> {
    let s = std::fs::read_to_string(src)
        .with_context(|| format!("reading source {}", src.display()))?;
    let mut patched = if s.contains("const TWEAK: f64 = ") {
        s.lines()
            .map(|l| {
                if l.trim_start().starts_with("const TWEAK: f64 = ") {
                    "const TWEAK: f64 = 0.0;"
                } else {
                    l
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        match kind {
            EvalKind::Plugin => s.replace(
                "let sum: f64 = adjusted.iter().sum();",
                "const TWEAK: f64 = 0.0;\n    let sum: f64 = adjusted.iter().sum::<f64>() + TWEAK;",
            ),
            EvalKind::Math => s
                .replace(
                    "/// The transform under test.",
                    "const TWEAK: f64 = 0.0;\n\n/// The transform under test.",
                )
                .replace("        sum += y;", "        sum += y + TWEAK;"),
        }
    };
    if !positive {
        patched = patched.replace("const TWEAK: f64 = 0.0;", "const TWEAK: f64 = -0.0;");
    }
    if !patched.contains("const TWEAK: f64 = ") || !patched.contains("+ TWEAK;") {
        bail!(
            "cannot find the edit site in {} ({}); update tweak_source",
            src.display(),
            kind.name()
        );
    }
    std::fs::write(src, patched).with_context(|| format!("writing {}", src.display()))?;
    Ok(())
}

/// Keep a mutable copy of the plugin source in the work directory so a bench
/// run's one-line edits never touch the committed Stage A source.
fn prepare_source(o: &mut Opts) -> Result<()> {
    std::fs::create_dir_all(&o.work)?;
    let name = o.src.file_name().unwrap_or_default();
    let dst = o.work.join(name);
    std::fs::copy(&o.src, &dst)
        .with_context(|| format!("copying {} to {}", o.src.display(), dst.display()))?;
    o.src = dst;
    Ok(())
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let (cmd, mut o, inspect_path) = parse_args()?;
    match cmd.as_str() {
        "compile" => {
            prepare_source(&mut o)?;
            let (r, _) = run_rustc(&o, o.expect, false)?;
            println!(
                "compiled in {:.1} ms (compile {:.1}, link {:.1}), module load {:.1} ms -> {}",
                r.rustc_total_ms,
                r.compile_ms,
                r.link_ms,
                r.module_load_ms,
                o.out.display()
            );
        }
        "eval" => {
            let (_, _, rc, result) = eval_output(&o)?;
            println!("eval rc={rc} result={result}");
        }
        "check" => {
            prepare_source(&mut o)?;
            let (r, _) = run_rustc(&o, o.expect, true)?;
            match o.kind {
                EvalKind::Plugin => println!(
                    "PASS eval(1,2,3,4) = {} (expected {}); rustc {:.1} ms, link {:.1} ms, e2e {:.1} ms",
                    r.result, o.expect, r.rustc_total_ms, r.link_ms, r.end_to_end_ms
                ),
                EvalKind::Math => println!(
                    "PASS e_math eval mean = {:.17e} (A.2 reference check); rustc {:.1} ms, link {:.1} ms, e2e {:.1} ms",
                    r.result, r.rustc_total_ms, r.link_ms, r.end_to_end_ms
                ),
            }
        }
        "bench" => {
            prepare_source(&mut o)?;
            let b = bench(&o)?;
            let text = serde_json::to_string_pretty(&b)?;
            if let Some(p) = &o.json {
                std::fs::write(p, &text)?;
            }
            println!("{text}");
        }
        "incremental" => {
            let result = incremental(&mut o)?;
            let text = serde_json::to_string_pretty(&result)?;
            if let Some(p) = &o.json {
                std::fs::write(p, &text)?;
            }
            println!("{text}");
            if !result.wasm_identical {
                eprintln!(
                    "[stage-c host] note: incremental wasm bytes differ from the cold build; \
                     acceptance is by identical eval results"
                );
            }
            if !result.ok {
                bail!(
                    "incremental correctness check failed for {}: results_identical={}, \
                     wasm_identical={}",
                    result.crate_name,
                    result.results_identical,
                    result.wasm_identical
                );
            }
        }
        "raw" => {
            let engine = engine(o.cache_dir.as_deref())?;
            let module = load_rustc_module(
                &engine,
                &o.rustc,
                o.precompiled.as_deref(),
                o.verify_precompiled,
            )?
            .module;
            let mut args = vec!["rustc".to_string()];
            if let Some(dir) = &o.incremental {
                args.push(format!("-Cincremental={}", dir.display()));
            }
            args.extend(o.raw.iter().cloned());
            let (mut store, instance) = prepare_instance(&engine, &module, &o, args)?;
            let start = instance
                .get_typed_func::<(), ()>(&mut store, "_start")
                .wt("rustc.wasm _start")?;
            let t = Instant::now();
            let call = start.call(&mut store, ());
            let rustc_total_ms = t.elapsed().as_secs_f64() * 1e3;
            let link_ms: f64 = store
                .data()
                .spawns
                .lock()
                .unwrap()
                .iter()
                .map(|s| s.ms)
                .sum();
            if std::env::var_os("STAGE_C_TIME").is_some() {
                eprintln!(
                    "TIMING rustc_total_ms={rustc_total_ms:.3} link_ms={link_ms:.3} compile_ms={:.3}",
                    (rustc_total_ms - link_ms).max(0.0)
                );
            }
            let code = match call {
                Ok(()) => 0,
                Err(e) => match e.downcast_ref::<I32Exit>() {
                    Some(exit) => exit.0,
                    None => {
                        eprintln!("rustc.wasm trapped: {e}");
                        1
                    }
                },
            };
            std::process::exit(code);
        }
        "cold-once" => startup_once(&o, false)?,
        "cached-once" => startup_once(&o, true)?,
        "inspect" => {
            let p = inspect_path
                .or_else(|| Some(o.out.clone()))
                .ok_or_else(|| anyhow!("inspect needs --wasm PATH"))?;
            let i = inspect_wasm(&p)?;
            println!("{}", serde_json::to_string_pretty(&i)?);
        }
        other => bail!("unknown command {other}\n{}", usage()),
    }
    // Skip Wasmtime's multi-second teardown of a 137 MiB compiled module and
    // the cache worker; the cache entry is written before this point. A
    // long-lived hot-reload host keeps the engine alive instead.
    std::process::exit(0);
}
