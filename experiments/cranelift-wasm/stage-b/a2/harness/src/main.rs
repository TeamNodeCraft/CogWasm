//! A.2 measurement harness: validation, section sizes, Wasmtime load/instantiate,
//! maths-node runtime and host-import-call cost.
//!
//! Output is line-oriented so the Python runner can parse it without serde.
//! Commands:
//!   inspect <wasm>
//!   load <wasm>
//!   check <e_math.wasm> <f_geometry.wasm>   correctness assertions (no timings)
//!   bench_math <wasm> <n_points> <reps>
//!   bench_geom <wasm> <n_iters> <reps>

use anyhow::{bail, Context, Result};
use std::time::Instant;
use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

/// wasmtime's `Error` deliberately does not implement `std::error::Error`
/// (see wasmtime-internal-core), so adapt it to anyhow explicitly.
trait Wt<T> {
    fn wt(self) -> Result<T>;
    fn wtc(self, ctx: &'static str) -> Result<T>;
}

impl<T> Wt<T> for std::result::Result<T, wasmtime::Error> {
    fn wt(self) -> Result<T> {
        self.map_err(|e| anyhow::anyhow!("{e}"))
    }
    fn wtc(self, ctx: &'static str) -> Result<T> {
        self.map_err(|e| anyhow::anyhow!("{ctx}: {e}"))
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let cmd = args
        .next()
        .context("usage: a2-harness <inspect|load|bench_math|bench_geom> ...")?;
    match cmd.as_str() {
        "inspect" => inspect(&args.next().context("wasm path")?),
        "load" => load(&args.next().context("wasm path")?),
        "check" => {
            let math = args.next().context("e_math wasm path")?;
            let geom = args.next().context("f_geometry wasm path")?;
            check_math(&math)?;
            check_geom(&geom)
        }
        "bench_math" => {
            let path = args.next().context("wasm path")?;
            let n: usize = args.next().unwrap_or_else(|| "100000".into()).parse()?;
            let reps: usize = args.next().unwrap_or_else(|| "5".into()).parse()?;
            bench_math(&path, n, reps)
        }
        "bench_geom" => {
            let path = args.next().context("wasm path")?;
            let n: u32 = args.next().unwrap_or_else(|| "40000".into()).parse()?;
            let reps: usize = args.next().unwrap_or_else(|| "5".into()).parse()?;
            bench_geom(&path, n, reps)
        }
        _ => bail!("unknown command {cmd}"),
    }
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn inspect(path: &str) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path}"))?;
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(&bytes)
        .context("wasm validation")?;

    let mut code_bytes = 0u64;
    let mut data_bytes = 0u64;
    let mut data_segments = 0u64;
    let mut functions = 0u64;
    let mut imports: Vec<String> = Vec::new();
    let mut exports: Vec<String> = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(&bytes) {
        match payload? {
            wasmparser::Payload::CodeSectionEntry(body) => {
                code_bytes += body.range().end - body.range().start;
                functions += 1;
            }
            wasmparser::Payload::DataSection(reader) => {
                for data in reader {
                    data_bytes += data?.data.len() as u64;
                    data_segments += 1;
                }
            }
            wasmparser::Payload::ImportSection(reader) => {
                for imp in reader.into_imports() {
                    let imp = imp?;
                    imports.push(format!("{}::{}", imp.module, imp.name));
                }
            }
            wasmparser::Payload::ExportSection(reader) => {
                for exp in reader {
                    exports.push(exp?.name.to_string());
                }
            }
            _ => {}
        }
    }
    let imports_json = imports
        .iter()
        .map(|s| format!("\"{}\"", esc(s)))
        .collect::<Vec<_>>()
        .join(",");
    let exports_json = exports
        .iter()
        .map(|s| format!("\"{}\"", esc(s)))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "INSPECT {{\"valid\":true,\"size\":{},\"code_bytes\":{},\"data_bytes\":{},\"data_segments\":{},\"functions\":{},\"imports\":[{}],\"exports\":[{}]}}",
        bytes.len(), code_bytes, data_bytes, data_segments, functions, imports_json, exports_json
    );
    Ok(())
}

fn link_geometry(linker: &mut Linker<()>) -> Result<()> {
    // The five imports used by f_geometry::run. Every definition is here; a
    // missing import would make instantiation fail loudly.
    linker
        .func_wrap(
            "nodecraft",
            "dot2",
            |x1: f64, y1: f64, x2: f64, y2: f64| -> f64 { x1 * x2 + y1 * y2 },
        )
        .wt()?;
    linker
        .func_wrap("nodecraft", "length2", |x: f64, y: f64| -> f64 {
            (x * x + y * y).sqrt()
        })
        .wt()?;
    linker
        .func_wrap("nodecraft", "clamp01", |x: f64| -> f64 {
            if x < 0.0 {
                0.0
            } else if x > 1.0 {
                1.0
            } else {
                x
            }
        })
        .wt()?;
    linker
        .func_wrap("nodecraft", "scale", |x: f64, k: f64| -> f64 { x * k })
        .wt()?;
    linker
        .func_wrap("nodecraft", "mix", |a: f64, b: f64, t: f64| -> f64 {
            a + (b - a) * t
        })
        .wt()?;
    Ok(())
}

fn instantiate<'a>(engine: &Engine, module: &Module, store: &'a mut Store<()>) -> Result<Instance> {
    let mut linker = Linker::new(engine);
    link_geometry(&mut linker)?;
    linker.instantiate(store, module).wtc("instantiate")
}

fn load(path: &str) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path}"))?;

    // Fresh Engine for every compile+instantiate: no engine-level reuse at all.
    for i in 0..5 {
        let engine = Engine::default();
        let t = Instant::now();
        let module = Module::new(&engine, &bytes).wtc("Module::new (fresh engine)")?;
        let module_ms = t.elapsed().as_secs_f64() * 1000.0;
        let mut store = Store::new(&engine, ());
        let t = Instant::now();
        let _instance = instantiate(&engine, &module, &mut store)?;
        let inst_ms = t.elapsed().as_secs_f64() * 1000.0;
        println!("LOAD fresh {i} {module_ms:.6} {inst_ms:.6}");
    }

    // One reused Engine, new Module each iteration (Wasmtime's common path).
    let engine = Engine::default();
    for i in 0..5 {
        let t = Instant::now();
        let module = Module::new(&engine, &bytes).wtc("Module::new (reused engine)")?;
        let module_ms = t.elapsed().as_secs_f64() * 1000.0;
        let mut store = Store::new(&engine, ());
        let t = Instant::now();
        let _instance = instantiate(&engine, &module, &mut store)?;
        let inst_ms = t.elapsed().as_secs_f64() * 1000.0;
        println!("LOAD reused {i} {module_ms:.6} {inst_ms:.6}");
    }
    Ok(())
}

fn math_input(n: usize, i: usize) -> f64 {
    (i as f64 - n as f64 / 2.0) * 0.001
}

fn math_transform(x: f64) -> f64 {
    x.sin() * 0.5 + (x * 0.25).cos() + x.abs().sqrt()
}

fn close(a: f64, b: f64, tol: f64) -> bool {
    if a == b {
        return true;
    }
    (a - b).abs() <= tol * a.abs().max(b.abs()).max(1.0)
}

fn assert_close(what: &str, got: f64, want: f64, tol: f64) -> Result<()> {
    if !close(got, want, tol) {
        bail!("{what}: got {got:.17e}, want {want:.17e} (tolerance {tol:e})");
    }
    Ok(())
}

/// Compile+run the maths node and compare mean/min/max with a host-side
/// reference. This is a correctness check, not a timing measurement.
fn check_math(path: &str) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path}"))?;
    let engine = Engine::default();
    let module = Module::new(&engine, &bytes).wtc("Module::new")?;
    let mut store = Store::new(&engine, ());
    let instance = instantiate(&engine, &module, &mut store)?;

    let memory: Memory = instance
        .get_memory(&mut store, "memory")
        .context("memory export")?;
    let alloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
        .wtc("wasm_alloc export")?;
    let eval = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval")
        .wtc("eval export")?;
    let eval_stats = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval_stats")
        .wtc("eval_stats export")?;

    let n = 100_000usize;
    let in_bytes = (n * 8) as i32;
    let in_ptr = alloc.call(&mut store, (in_bytes, 8)).wtc("alloc input")?;
    let out_ptr = alloc.call(&mut store, (24, 8)).wtc("alloc stats")?;
    if in_ptr == 0 || out_ptr == 0 {
        bail!("wasm_alloc returned null");
    }

    let mut input = Vec::with_capacity(n * 8);
    let mut ref_sum = 0.0f64;
    let mut ref_min = f64::INFINITY;
    let mut ref_max = f64::NEG_INFINITY;
    for i in 0..n {
        let x = math_input(n, i);
        let y = math_transform(x);
        ref_sum += y;
        ref_min = ref_min.min(y);
        ref_max = ref_max.max(y);
        input.extend_from_slice(&x.to_le_bytes());
    }
    let ref_mean = ref_sum / n as f64;
    memory.write(&mut store, in_ptr as usize, &input)?;

    let rc = eval_stats
        .call(&mut store, (in_ptr, n as i32, out_ptr))
        .wtc("eval_stats")?;
    if rc != 0 {
        bail!("e_math eval_stats returned {rc}");
    }
    let mut stats = [0u8; 24];
    memory.read(&store, out_ptr as usize, &mut stats)?;
    let mean = f64::from_le_bytes(stats[0..8].try_into().unwrap());
    let min = f64::from_le_bytes(stats[8..16].try_into().unwrap());
    let max = f64::from_le_bytes(stats[16..24].try_into().unwrap());
    // libm inside the guest and the host's libm can differ by a few ULP.
    assert_close("e_math mean", mean, ref_mean, 1e-9)?;
    assert_close("e_math min", min, ref_min, 1e-9)?;
    assert_close("e_math max", max, ref_max, 1e-9)?;

    let rc = eval
        .call(&mut store, (in_ptr, n as i32, out_ptr))
        .wtc("eval")?;
    if rc != 0 {
        bail!("e_math eval returned {rc}");
    }
    let mut out = [0u8; 8];
    memory.read(&store, out_ptr as usize, &mut out)?;
    let eval_mean = f64::from_le_bytes(out);
    assert_close("e_math eval mean", eval_mean, ref_mean, 1e-9)?;

    println!(
        "MATH_CHECK PASS n={n} mean={mean:.17e} min={min:.17e} max={max:.17e} ref_mean={ref_mean:.17e}"
    );
    Ok(())
}

/// Run the geometry node and compare its result with the host-side reference
/// (every operation in the guest is one of the five imported host calls).
fn check_geom(path: &str) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path}"))?;
    let engine = Engine::default();
    let module = Module::new(&engine, &bytes).wtc("Module::new")?;
    let mut store = Store::new(&engine, ());
    let instance = instantiate(&engine, &module, &mut store)?;

    let run: TypedFunc<u32, f64> = instance
        .get_typed_func::<u32, f64>(&mut store, "run")
        .wtc("run export")?;
    let cpi: TypedFunc<(), i32> = instance
        .get_typed_func::<(), i32>(&mut store, "calls_per_iter")
        .wtc("calls_per_iter export")?;
    let calls_per_iter = cpi.call(&mut store, ()).wt()?;
    if calls_per_iter != 5 {
        bail!("f_geometry reports {calls_per_iter} calls/iter, expected 5");
    }

    let n = 40_000u32;
    let got = run.call(&mut store, n).wtc("run")?;
    let want = geom_ref(n);
    assert_close("f_geometry run", got, want, 1e-12)?;
    println!(
        "GEOM_CHECK PASS n={n} value={got:.17e} ref={want:.17e} calls={}",
        5u64 * n as u64
    );
    Ok(())
}

fn bench_math(path: &str, n: usize, reps: usize) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path}"))?;
    let engine = Engine::default();
    let module = Module::new(&engine, &bytes).wtc("Module::new")?;
    let mut store = Store::new(&engine, ());
    let instance = instantiate(&engine, &module, &mut store)?;

    let memory: Memory = instance
        .get_memory(&mut store, "memory")
        .context("memory export")?;
    let alloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
        .wtc("wasm_alloc export")?;
    let eval = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval")
        .wtc("eval export")?;
    let eval_stats = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval_stats")
        .ok();

    let in_bytes = n.checked_mul(8).context("n*8 overflow")? as i32;
    let in_ptr = alloc.call(&mut store, (in_bytes, 8)).wtc("alloc input")?;
    let out_ptr = alloc.call(&mut store, (8, 8)).wtc("alloc out")?;
    if in_ptr == 0 || out_ptr == 0 {
        bail!("wasm_alloc returned null (input {in_ptr}, out {out_ptr})");
    }

    let mut input = Vec::with_capacity(n * 8);
    let mut ref_sum = 0.0f64;
    let mut ref_min = f64::INFINITY;
    let mut ref_max = f64::NEG_INFINITY;
    for i in 0..n {
        let x = math_input(n, i);
        let y = math_transform(x);
        ref_sum += y;
        ref_min = ref_min.min(y);
        ref_max = ref_max.max(y);
        input.extend_from_slice(&x.to_le_bytes());
    }
    memory.write(&mut store, in_ptr as usize, &input)?;
    let ref_mean = ref_sum / n as f64;

    // Warm-up call, then read the result once.
    let rc = eval
        .call(&mut store, (in_ptr, n as i32, out_ptr))
        .wtc("eval warmup")?;
    let mut out = [0u8; 8];
    memory.read(&store, out_ptr as usize, &mut out)?;
    println!(
        "MATH_META n={n} calls=1 rc={rc} ref_mean={ref_mean:.17e} ref_min={ref_min:.17e} ref_max={ref_max:.17e}"
    );

    for r in 0..reps {
        let t = Instant::now();
        let rc = eval
            .call(&mut store, (in_ptr, n as i32, out_ptr))
            .wtc("eval")?;
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        memory.read(&store, out_ptr as usize, &mut out)?;
        let val = f64::from_le_bytes(out);
        let ns_per_point = ms * 1.0e6 / n as f64;
        println!("MATH_RUN {r} {ms:.6} {rc} {val:.17e} {ns_per_point:.6}");
    }

    if let Some(f) = eval_stats {
        let stats_ptr = alloc.call(&mut store, (24, 8)).wtc("alloc stats")?;
        if stats_ptr != 0 {
            let rc = f
                .call(&mut store, (in_ptr, n as i32, stats_ptr))
                .wtc("eval_stats")?;
            let mut stats = [0u8; 24];
            memory.read(&store, stats_ptr as usize, &mut stats)?;
            let mean = f64::from_le_bytes(stats[0..8].try_into().unwrap());
            let min = f64::from_le_bytes(stats[8..16].try_into().unwrap());
            let max = f64::from_le_bytes(stats[16..24].try_into().unwrap());
            println!("MATH_STATS rc={rc} mean={mean:.17e} min={min:.17e} max={max:.17e}");
        }
    }
    Ok(())
}

fn geom_ref(n: u32) -> f64 {
    const PTS: [f64; 16] = [
        0.0, 1.0, -1.0, 0.5, -0.5, 2.0, -2.0, 0.25, 0.75, -0.75, 1.5, -1.5, 0.125, -0.125, 3.0,
        -3.0,
    ];
    let clamp01 = |x: f64| {
        if x < 0.0 {
            0.0
        } else if x > 1.0 {
            1.0
        } else {
            x
        }
    };
    let mut acc = 0.0f64;
    for i in 0..n {
        let j = (i as usize) & 7;
        let x = PTS[j];
        let y = PTS[j + 8];
        let t = (i as f64) * 1.0e-6;
        let a = x * 0.5 + y * 0.25; // dot2(x, y, 0.5, 0.25)
        let b = ((a + t) * (a + t) + x * x).sqrt(); // length2(a+t, x)
        let c = clamp01(b * 0.001); // clamp01
        let d = c * 2.0; // scale(c, 2.0)
        let e = d + (y - d) * 0.5; // mix(d, y, 0.5)
        acc += e;
    }
    acc
}

fn bench_geom(path: &str, n: u32, reps: usize) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("read {path}"))?;
    let engine = Engine::default();
    let module = Module::new(&engine, &bytes).wtc("Module::new")?;
    let mut store = Store::new(&engine, ());
    let instance = instantiate(&engine, &module, &mut store)?;

    let run: TypedFunc<u32, f64> = instance
        .get_typed_func::<u32, f64>(&mut store, "run")
        .wtc("run export")?;
    let cpi: TypedFunc<(), i32> = instance
        .get_typed_func::<(), i32>(&mut store, "calls_per_iter")
        .wtc("calls_per_iter export")?;
    let calls_per_iter = cpi.call(&mut store, ()).wt()? as u64;
    if calls_per_iter != 5 {
        bail!("f_geometry reports {calls_per_iter} calls/iter, expected 5");
    }
    let calls = calls_per_iter * n as u64;
    let ref_val = geom_ref(n);

    // Warm-up call; not timed.
    let warm = run.call(&mut store, 1000).wtc("run warmup")?;
    println!("GEOM_META n={n} calls_per_iter={calls_per_iter} calls={calls} warm={warm:.17e} ref={ref_val:.17e}");

    for r in 0..reps {
        let t = Instant::now();
        let val = run.call(&mut store, n).wtc("run")?;
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        let ns_per_call = ms * 1.0e6 / calls as f64;
        println!("GEOM_RUN {r} {ms:.6} {val:.17e} {ns_per_call:.6}");
    }
    Ok(())
}
