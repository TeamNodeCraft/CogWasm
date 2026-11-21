//! Run a stage-C plugin with the growable allocator and check both the result
//! and that `memory.grow` was actually used:
//!
//!     c_plugin_harness path/to/c_plugin.wasm [1 2 3 4]
//!
//! The plugin starts with only its static data and shadow stack; the first
//! `wasm_alloc` must grow linear memory. `eval` then allocates a `Vec` in those
//! new pages and must return the mean of `host_bias(x) = x / 2 + 1`.

use anyhow::{anyhow, bail, Context, Result};
use wasmtime::{Engine, Linker, Module, Store};

trait Wt<T> {
    fn wt(self, what: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, what: &'static str) -> Result<T> {
        self.map_err(|e| anyhow!("{what}: {e}"))
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .context("usage: c_plugin_harness <c_plugin.wasm> [f64...]")?;
    let params: Vec<f64> = args
        .map(|s| {
            s.parse::<f64>()
                .map_err(|e| anyhow!("bad parameter {s}: {e}"))
        })
        .collect::<Result<_>>()?;
    let params = if params.is_empty() {
        vec![1.0, 2.0, 3.0, 4.0]
    } else {
        params
    };

    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).wt("load wasm")?;
    let mut linker = Linker::new(&engine);
    linker
        .func_wrap("nodecraft", "host_bias", |x: f64| -> f64 { x * 0.5 + 1.0 })
        .wt("define host_bias")?;
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module).wt("instantiate")?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .context("memory export")?;
    let alloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
        .wt("get wasm_alloc")?;
    let eval = instance
        .get_typed_func::<(i32, i32, i32), i32>(&mut store, "eval")
        .wt("get eval")?;

    let before = memory.size(&store);
    println!("initial memory: {before} pages");

    let n = params.len() as i32;
    let in_ptr = alloc.call(&mut store, (n * 8, 8)).wt("allocate input")?;
    let out_ptr = alloc.call(&mut store, (8, 8)).wt("allocate output")?;
    let aligned_ptr = alloc.call(&mut store, (64, 64)).wt("allocate aligned")?;
    if in_ptr as usize % 8 != 0 || out_ptr as usize % 8 != 0 || aligned_ptr as usize % 64 != 0 {
        bail!("allocator alignment: in={in_ptr:#x} out={out_ptr:#x} aligned={aligned_ptr:#x}");
    }
    println!("alignment: in={in_ptr:#x} out={out_ptr:#x} aligned64={aligned_ptr:#x}");
    let after_alloc = memory.size(&store);
    println!("after wasm_alloc: {after_alloc} pages");
    if after_alloc == before {
        bail!("the growable allocator did not grow memory");
    }

    let mut bytes = Vec::with_capacity(params.len() * 8);
    for p in &params {
        bytes.extend_from_slice(&p.to_le_bytes());
    }
    memory
        .write(&mut store, in_ptr as usize, &bytes)
        .context("write parameters")?;

    let rc = eval.call(&mut store, (in_ptr, n, out_ptr)).wt("eval")?;
    let mut out = [0u8; 8];
    memory
        .read(&store, out_ptr as usize, &mut out)
        .context("read result")?;
    let result = f64::from_le_bytes(out);
    let after_eval = memory.size(&store);
    println!("after eval: {after_eval} pages");
    println!("eval rc={rc} result={result}");
    if rc != 0 {
        bail!("eval returned {rc}");
    }
    if result != 2.25 {
        bail!("eval returned {result}, expected 2.25");
    }

    // A second call reuses the bump heap; the plugin must still be correct.
    let rc = eval
        .call(&mut store, (in_ptr, n, out_ptr))
        .wt("eval again")?;
    memory
        .read(&store, out_ptr as usize, &mut out)
        .context("read second result")?;
    let result = f64::from_le_bytes(out);
    if rc != 0 || result != 2.25 {
        bail!("second eval: rc={rc} result={result}");
    }
    println!(
        "second eval: rc={rc} result={result}, memory {} pages",
        memory.size(&store)
    );
    println!(
        "PASS c_plugin: eval(1,2,3,4) = 2.25, memory {before} -> {after_eval} -> {} pages",
        memory.size(&store)
    );
    Ok(())
}
