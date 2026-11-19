use anyhow::{anyhow, bail, Context, Result};
use std::time::Instant;
use wasmtime::{Engine, Linker, Module, Store};

fn wt(e: wasmtime::Error) -> anyhow::Error {
    anyhow!("{e}")
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let cmd = args
        .next()
        .ok_or_else(|| anyhow!("usage: host <inspect|add|bench|plugin|hello> <wasm> [args...]"))?;
    match cmd.as_str() {
        "inspect" => inspect(args.next().context("wasm path")?),
        "add" => add(
            args.next().context("wasm path")?,
            args.next().unwrap_or("2".into()).parse()?,
            args.next().unwrap_or("3".into()).parse()?,
        ),
        "bench" => bench(
            args.next().context("wasm path")?,
            args.next().unwrap_or("1000000".into()).parse()?,
        ),
        "plugin" => plugin(
            args.next().context("wasm path")?,
            args.collect::<Vec<_>>()
                .iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
        ),
        "hello" => hello(args.next().context("wasm path")?),
        _ => bail!("unknown command {cmd}"),
    }
}

fn inspect(path: String) -> Result<()> {
    let bytes = std::fs::read(&path)?;
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(&bytes)
        .context("wasm validation")?;
    println!("valid: {} bytes", bytes.len());
    let engine = Engine::default();
    let module = Module::new(&engine, &bytes).map_err(wt)?;
    for imp in module.imports() {
        println!("import {}::{} {:?}", imp.module(), imp.name(), imp.ty());
    }
    for exp in module.exports() {
        println!("export {} {:?}", exp.name(), exp.ty());
    }
    Ok(())
}

fn add(path: String, a: i32, b: i32) -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).map_err(wt)?;
    let mut store = Store::new(&engine, ());
    let instance = Linker::new(&engine)
        .instantiate(&mut store, &module)
        .map_err(wt)?;
    let f = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")
        .map_err(wt)?;
    let t = Instant::now();
    let r = f.call(&mut store, (a, b)).map_err(wt)?;
    println!("add({a},{b}) = {r} ({} us)", t.elapsed().as_micros());
    Ok(())
}

fn bench(path: String, n: u32) -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).map_err(wt)?;
    let mut store = Store::new(&engine, ());
    let instance = Linker::new(&engine)
        .instantiate(&mut store, &module)
        .map_err(wt)?;
    let f = instance
        .get_typed_func::<u32, f64>(&mut store, "bench")
        .map_err(wt)?;
    // warm-up
    let warm = f.call(&mut store, 1000).map_err(wt)?;
    let t = Instant::now();
    let r = f.call(&mut store, n).map_err(wt)?;
    let elapsed = t.elapsed();
    println!(
        "bench({n}) = {warm} / {r} in {:?} ({:.2} ns/iter)",
        elapsed,
        elapsed.as_nanos() as f64 / n as f64
    );
    Ok(())
}

fn plugin(path: String, params: Vec<f64>) -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).map_err(wt)?;
    let mut linker = Linker::new(&engine);
    linker
        .func_wrap("nodecraft", "host_bias", |x: f64| -> f64 { x * 0.5 + 1.0 })
        .map_err(wt)?;
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module).map_err(wt)?;
    let memory = instance
        .get_memory(&mut store, "memory")
        .context("memory export")?;
    let alloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
        .map_err(wt)?;
    let eval = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval")
        .map_err(wt)?;
    let n = params.len() as i32;
    let in_ptr = alloc.call(&mut store, ((n * 8) as i32, 8)).map_err(wt)?;
    let out_ptr = alloc.call(&mut store, (8, 8)).map_err(wt)?;
    let mut bytes = Vec::with_capacity(params.len() * 8);
    for p in &params {
        bytes.extend_from_slice(&p.to_le_bytes());
    }
    memory.write(&mut store, in_ptr as usize, &bytes)?;
    let t = Instant::now();
    let rc = eval.call(&mut store, (in_ptr, n, out_ptr)).map_err(wt)?;
    let elapsed = t.elapsed();
    let mut out = [0u8; 8];
    memory.read(&store, out_ptr as usize, &mut out)?;
    println!(
        "eval rc={rc} result={} in {:?}",
        f64::from_le_bytes(out),
        elapsed
    );
    Ok(())
}

fn hello(path: String) -> Result<()> {
    use wasmtime_wasi::p1::{add_to_linker_sync, WasiP1Ctx};
    use wasmtime_wasi::WasiCtxBuilder;
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).map_err(wt)?;
    let mut linker = Linker::new(&engine);
    add_to_linker_sync(&mut linker, |ctx: &mut WasiP1Ctx| ctx).map_err(wt)?;
    let wasi = WasiCtxBuilder::new().inherit_stdio().build_p1();
    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module).map_err(wt)?;
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .map_err(wt)?;
    start.call(&mut store, ()).map_err(wt)?;
    Ok(())
}
