//! Stage B host: run rustc.wasm (wasm32-wasip1-threads, cg_clif only) under
//! Wasmtime, service its `playground.spawn` linker import with native Wild,
//! then load and evaluate the linked plugin.
//!
//! Wasmtime 46.0.3 is pinned because it is the last release with
//! `wasmtime-wasi-threads`; rustc always runs its compilation on a spawned
//! thread, so the `wasi:thread-spawn` import is required. Wasmtime 47+ removed
//! that support.
//!
//! Subcommands:
//!   compile  run rustc.wasm once on a source file, writing a .wasm
//!   eval     load a plugin .wasm, call eval(1,2,3,4), print the result
//!   check    compile + eval and assert 2.25
//!   bench    compile + eval N times, plus cold/cached module-load timings
//!   inspect  validate a wasm and scan its name/data sections for LLVM symbols

use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wasmtime::{
    Cache, CacheConfig, Caller, Config, Engine, Extern, Linker, Memory, Module, SharedMemory, Store,
};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::{DirPerms, FilePerms, I32Exit, WasiCtxBuilder};
use wasmtime_wasi_threads::WasiThreadsCtx;

/// Convert wasmtime errors (which do not implement anyhow's `Context` in 46)
/// into anyhow errors with a message.
trait Wt<T> {
    fn wt(self, ctx: &'static str) -> Result<T>;
}
impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, ctx: &'static str) -> Result<T> {
        self.map_err(|e| anyhow!("{ctx}: {e}"))
    }
}

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
    raw: Vec<String>,
}

fn usage() -> &'static str {
    "usage: stage-b-host <compile|eval|check|bench|inspect> [options]\n\
     \n\
     compile/eval/check/bench options:\n\
       --rustc PATH       rustc.wasm\n\
       --sysroot DIR      wasm32-unknown-unknown sysroot (pure-Rust core/alloc)\n\
       --src PATH         plugin source (copied into --work before building)\n\
       --out PATH         linked output wasm\n\
       --linker PATH      native Wild binary\n\
       --work DIR         scratch directory (preopened to the guest as /)\n\
       --cache-dir DIR    enable Wasmtime's compiled-module cache\n\
       --runs N           number of measured runs (default 5)\n\
       --json PATH        write machine-readable results\n\
       --expect F         expected eval result (default 2.25)\n\
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
        raw: Vec::new(),
    };
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
            "--runs" => o.runs = val("--runs")?.parse().context("--runs")?,
            "--json" => o.json = Some(val("--json")?.into()),
            "--expect" => o.expect = val("--expect")?.parse().context("--expect")?,
            "--wasm" => inspect = Some(PathBuf::from(val("--wasm")?)),
            "-h" | "--help" => bail!("{}", usage()),
            other => bail!("unknown option {other}\n{}", usage()),
        }
    }
    Ok((cmd, o, inspect))
}

// ---------------------------------------------------------------------------
// Engine / store
// ---------------------------------------------------------------------------

fn engine_config(cache_dir: Option<&Path>) -> Result<Config> {
    let mut cfg = Config::new();
    cfg.wasm_threads(true)
        .wasm_bulk_memory(true)
        .wasm_multi_memory(true)
        .wasm_simd(true)
        .shared_memory(true)
        .max_wasm_stack(8 << 20)
        .async_stack_size(16 << 20);
    if let Some(dir) = cache_dir {
        let mut cc = CacheConfig::new();
        // The 137 MiB rustc.wasm is much faster to mmap/deserialize than to
        // decompress on every warm start; the cache is local and disposable.
        cc.with_directory(dir)
            .with_baseline_compression_level(0)
            .with_optimized_compression_level(0);
        let cache = Cache::new(cc).wt("creating Wasmtime module cache")?;
        cfg.cache(Some(cache));
    } else {
        cfg.cache(None);
    }
    Ok(cfg)
}

fn engine(cache_dir: Option<&Path>) -> Result<Engine> {
    Engine::new(&engine_config(cache_dir)?).wt("creating Wasmtime engine")
}

/// The pieces needed to build a fresh WASI context for each wasi-thread.
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
            b.preopened_dir(host, guest, DirPerms::all(), FilePerms::all())
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

/// Per-store host state. `Clone` is required by `wasmtime-wasi-threads`: each
/// thread gets its own store and, here, its own WASI fd table over the same
/// host filesystem. The spawn log is shared so the parent can report all link
/// invocations.
struct HostState {
    wasi: Option<WasiP1Ctx>,
    spec: Arc<WasiSpec>,
    linker: PathBuf,
    spawns: Arc<Mutex<Vec<SpawnRecord>>>,
    wasi_threads: Option<Arc<WasiThreadsCtx<HostState>>>,
}

impl Clone for HostState {
    fn clone(&self) -> Self {
        // `WasiP1Ctx` is not cloneable. A fresh context with the same
        // arguments, environment and preopens is enough for rustc's compiler
        // thread: it opens files by path, and the host filesystem is shared.
        let wasi = self
            .spec
            .build()
            .expect("rebuilding WASI context for thread");
        HostState {
            wasi: Some(wasi),
            spec: Arc::clone(&self.spec),
            linker: self.linker.clone(),
            spawns: Arc::clone(&self.spawns),
            wasi_threads: self.wasi_threads.clone(),
        }
    }
}

fn new_store(engine: &Engine, o: &Opts, args: Vec<String>) -> Result<Store<HostState>> {
    let spec = Arc::new(WasiSpec {
        args,
        env: vec![
            ("RUST_MIN_STACK".into(), "16777216".into()),
            ("RUST_BACKTRACE".into(), "1".into()),
        ],
        // rustc sees the host filesystem at the same absolute paths, so the
        // native Wild process can consume the linker arguments unchanged.
        preopens: vec![(PathBuf::from("/"), "/".into())],
    });
    let wasi = spec.build()?;
    Ok(Store::new(
        engine,
        HostState {
            wasi: Some(wasi),
            spec,
            linker: o.linker.clone(),
            spawns: Arc::new(Mutex::new(Vec::new())),
            wasi_threads: None,
        },
    ))
}

/// rustc.wasm's memory is a *shared* memory (wasm32-wasip1-threads), so
/// Wasmtime exposes it as `Extern::SharedMemory`, not `Extern::Memory`.
#[derive(Clone)]
enum GuestMem {
    Normal(Memory),
    Shared(SharedMemory),
}

impl GuestMem {
    fn read(&self, caller: &Caller<'_, HostState>, off: usize, buf: &mut [u8]) -> Result<()> {
        match self {
            GuestMem::Normal(m) => m
                .read(caller, off, buf)
                .with_context(|| format!("reading guest memory at {off:#x}"))?,
            GuestMem::Shared(m) => {
                let data = m.data();
                let end = off
                    .checked_add(buf.len())
                    .ok_or_else(|| anyhow!("guest memory offset overflow"))?;
                if end > data.len() {
                    bail!(
                        "guest memory out of bounds: {off:#x}..{end:#x} > {:#x}",
                        data.len()
                    );
                }
                for (i, b) in buf.iter_mut().enumerate() {
                    // The guest is stopped in this host call; the argv strings
                    // are on this thread's stack and are not concurrently
                    // mutated. `UnsafeCell` is how Wasmtime models shared
                    // memory, and this is the read side of the boundary.
                    *b = unsafe { *data[off + i].get() };
                }
            }
        }
        Ok(())
    }
}

fn guest_memory(caller: &mut Caller<'_, HostState>) -> Result<GuestMem> {
    match caller.get_export("memory") {
        Some(Extern::Memory(m)) => Ok(GuestMem::Normal(m)),
        Some(Extern::SharedMemory(m)) => Ok(GuestMem::Shared(m)),
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
                        eprintln!("[stage-b host] spawn failed: {e:#}");
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
            eprintln!("[stage-b host] cannot run {}: {e}", linker.display());
            -1
        }
    };
    eprintln!(
        "[stage-b host] spawn: {} ({} args) -> {code} in {ms:.1} ms",
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

/// Create the full linker (WASI p1, playground.spawn, wasi:thread-spawn and
/// the shared memory import) and the store for one rustc.wasm instance.
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

    let mut store = new_store(engine, o, args)?;
    wasmtime_wasi_threads::add_to_linker(&mut linker, &store, module, |h| {
        h.wasi_threads
            .as_ref()
            .expect("wasi-threads context installed before instantiation")
    })
    .wt("adding wasi:thread-spawn")?;
    let ctx = WasiThreadsCtx::new(module.clone(), Arc::new(linker.clone()), false)
        .wt("creating wasi-threads context")?;
    store.data_mut().wasi_threads = Some(Arc::new(ctx));

    let instance = linker
        .instantiate(&mut store, module)
        .wt("instantiating rustc.wasm")?;
    Ok((store, instance))
}

// ---------------------------------------------------------------------------
// rustc.wasm
// ---------------------------------------------------------------------------

fn rustc_args(o: &Opts) -> Vec<String> {
    vec![
        "rustc".into(),
        o.src.display().to_string(),
        "--crate-name".into(),
        "c_plugin".into(),
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
    ]
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

#[derive(Serialize, Clone, Debug)]
struct SpawnSummary {
    program: String,
    args: usize,
    ms: f64,
    status: i32,
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
        eval_plugin(o, o.expect)?
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
    let bytes =
        std::fs::read(&o.rustc).with_context(|| format!("reading {}", o.rustc.display()))?;
    let engine = engine(o.cache_dir.as_deref())?;
    let t = Instant::now();
    let module = Module::new(&engine, &bytes).wt("compiling rustc.wasm")?;
    let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
    let mut r = run_rustc_with(o, &engine, &module, measure_plugin)?;
    r.module_load_ms = module_load_ms;
    let _ = expect;
    std::mem::forget(module);
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

// ---------------------------------------------------------------------------
// inspect / LLVM scan
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct Inspect {
    bytes: usize,
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
    let bytes =
        std::fs::read(&o.rustc).with_context(|| format!("reading {}", o.rustc.display()))?;
    let e = engine(if cached { o.cache_dir.as_deref() } else { None })?;
    let t = Instant::now();
    let m = Module::new(&e, &bytes).wt("startup Module::new")?;
    let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let (store, _instance) = prepare_instance(&e, &m, o, rustc_args(o))?;
    let instantiate_ms = t.elapsed().as_secs_f64() * 1e3;
    std::mem::forget(store);
    println!(
        "{}",
        serde_json::json!({
            "mode": if cached { "cached" } else { "cold" },
            "module_load_ms": module_load_ms,
            "instantiate_ms": instantiate_ms,
            "bytes": bytes.len(),
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
    let warm_module = Module::new(&warm_engine, &rustc_bytes).wt("loading rustc.wasm")?;
    let warm_module_load_ms = t.elapsed().as_secs_f64() * 1e3;
    if warm_module_load_ms > 5000.0 {
        eprintln!(
            "[stage-b host] warning: rustc.wasm module load was {warm_module_load_ms:.0} ms; pass --cache-dir for warm runs"
        );
    }

    let mut runs = Vec::new();
    for i in 0..o.runs {
        tweak_source(&o.src, i % 2 == 0)?;
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
    let ok = (result - o.expect).abs() < 1e-9;
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

/// Toggle a constant in the plugin source so each measured run is a real
/// one-line edit, while the result stays 2.25.
fn tweak_source(src: &Path, positive: bool) -> Result<()> {
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
        s.replace(
            "let sum: f64 = adjusted.iter().sum();",
            "const TWEAK: f64 = 0.0;\n    let sum: f64 = adjusted.iter().sum::<f64>() + TWEAK;",
        )
    };
    if !positive {
        patched = patched.replace("const TWEAK: f64 = 0.0;", "const TWEAK: f64 = -0.0;");
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
            let (_, _, rc, result) = eval_plugin(&o, o.expect)?;
            println!("eval rc={rc} result={result}");
        }
        "check" => {
            prepare_source(&mut o)?;
            let (r, _) = run_rustc(&o, o.expect, true)?;
            println!(
                "PASS eval(1,2,3,4) = {} (expected {}); rustc {:.1} ms, link {:.1} ms, e2e {:.1} ms",
                r.result, o.expect, r.rustc_total_ms, r.link_ms, r.end_to_end_ms
            );
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
        "raw" => {
            let bytes = std::fs::read(&o.rustc)
                .with_context(|| format!("reading {}", o.rustc.display()))?;
            let engine = engine(o.cache_dir.as_deref())?;
            let module = Module::new(&engine, &bytes).wt("compiling rustc.wasm")?;
            let mut args = vec!["rustc".to_string()];
            args.extend(o.raw.iter().cloned());
            let (mut store, instance) = prepare_instance(&engine, &module, &o, args)?;
            let start = instance
                .get_typed_func::<(), ()>(&mut store, "_start")
                .wt("rustc.wasm _start")?;
            let call = start.call(&mut store, ());
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
