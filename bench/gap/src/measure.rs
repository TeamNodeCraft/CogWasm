//! Execute an instrumented module and read its per-block counters.

use crate::instrument::Meta;
use crate::{option, required_option, write_output};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::path::Path;
use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, Val};

#[derive(Serialize)]
struct SeedResult {
    seed: u64,
    result: u64,
    out_len: u32,
    out_hash: u64,
    blocks: Vec<u64>,
}

#[derive(Serialize)]
struct FuelResult {
    seed: u64,
    result: u64,
    fuel: u64,
    empty_fuel: u64,
}

#[derive(Serialize)]
struct MeasureResult {
    wasm: String,
    meta: String,
    block_count: usize,
    seeds: Vec<SeedResult>,
    totals: Vec<u64>,
    total_instructions_basis: u64,
    fuel: Vec<FuelResult>,
}

trait Wt<T> {
    fn wt(self, context: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, context: &'static str) -> Result<T> {
        self.map_err(|error| anyhow::anyhow!("{context}: {error}"))
    }
}

pub fn run(rest: &[String]) -> Result<()> {
    let wasm = required_option(rest, "--wasm")?;
    let meta_path = required_option(rest, "--meta")?;
    let seeds = parse_seeds(&required_option(rest, "--seeds")?)?;
    let fuel_wasm = option(rest, "--fuel-wasm")?;
    let out = option(rest, "--out")?;
    let meta: Meta = serde_json::from_str(
        &std::fs::read_to_string(&meta_path)
            .with_context(|| format!("reading meta {meta_path}"))?,
    )
    .with_context(|| format!("parsing meta {meta_path}"))?;
    let block_count = meta.blocks.len();
    let bytes = std::fs::read(&wasm).with_context(|| format!("reading {wasm}"))?;
    let engine = Engine::default();
    let module = Module::new(&engine, &bytes).wt("compiling the instrumented module")?;
    let mut linker: Linker<()> = Linker::new(&engine);
    add_nodecraft_imports(&mut linker)?;

    let mut results = Vec::new();
    let mut totals = vec![0u64; block_count];
    for seed in seeds {
        let mut store = Store::new(&engine, ());
        let instance = linker
            .instantiate(&mut store, &module)
            .wt("instantiating the instrumented module")?;
        let result = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .wt("instrumented module must export run(u64) -> u64")?
            .call(&mut store, seed)
            .wt("calling run(seed)")?;
        // Read the counters before calling out_ptr/out_len: those helpers are
        // instrumented too, and counting them would inflate the benchmark.
        let blocks = read_counters(&instance, &mut store, block_count, &mut totals)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .context("instrumented module must export memory")?;
        let out_ptr = instance
            .get_typed_func::<(), u32>(&mut store, "out_ptr")
            .wt("instrumented module must export out_ptr()")?
            .call(&mut store, ())?;
        let out_len = instance
            .get_typed_func::<(), u32>(&mut store, "out_len")
            .wt("instrumented module must export out_len()")?
            .call(&mut store, ())?;
        let out_hash = hash_output(&memory, &store, out_ptr, out_len)?;
        results.push(SeedResult {
            seed,
            result,
            out_len,
            out_hash,
            blocks,
        });
    }

    let mut fuel = Vec::new();
    if let Some(fuel_wasm) = fuel_wasm {
        let bytes = std::fs::read(&fuel_wasm).with_context(|| format!("reading {fuel_wasm}"))?;
        let mut config = wasmtime::Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        let module = Module::new(&engine, &bytes).wt("compiling the fuel module")?;
        let mut linker: Linker<()> = Linker::new(&engine);
        add_nodecraft_imports(&mut linker)?;
        for seed in parse_seeds(&required_option(rest, "--seeds")?)? {
            let mut store = Store::new(&engine, ());
            store.set_fuel(1_000_000_000_000_000).wt("setting fuel")?;
            let instance = linker.instantiate(&mut store, &module)?;
            // Fuel left after instantiation, before `run`: subtracting keeps
            // data-segment setup out of the cross-check.
            let after_instantiation = store.get_fuel().unwrap_or(0);
            let result = instance
                .get_typed_func::<u64, u64>(&mut store, "run")?
                .call(&mut store, seed)?;
            let remaining = store.get_fuel().unwrap_or(0);
            fuel.push(FuelResult {
                seed,
                result,
                fuel: after_instantiation.saturating_sub(remaining),
                empty_fuel: 0,
            });
        }
    }

    let report = MeasureResult {
        wasm: wasm.clone(),
        meta: meta_path.clone(),
        block_count,
        seeds: results,
        totals,
        total_instructions_basis: meta
            .blocks
            .iter()
            .map(|block| u64::from(block.instructions))
            .sum(),
        fuel,
    };
    let text = serde_json::to_string_pretty(&report)?;
    if let Some(path) = out {
        write_output(&path, text.as_bytes())?;
    } else {
        println!("{text}");
    }
    Ok(())
}

fn read_counters(
    instance: &Instance,
    store: &mut Store<()>,
    block_count: usize,
    totals: &mut [u64],
) -> Result<Vec<u64>> {
    let mut blocks = Vec::with_capacity(block_count);
    for (id, total) in totals.iter_mut().enumerate().take(block_count) {
        let name = format!("__gap_b{id}");
        let global = instance
            .get_global(&mut *store, &name)
            .with_context(|| format!("missing {name}"))?;
        let value = match global.get(&mut *store) {
            Val::I64(value) => u64::try_from(value).unwrap_or(0),
            other => bail!("{name} is {other:?}, expected i64"),
        };
        *total = total.saturating_add(value);
        blocks.push(value);
    }
    Ok(blocks)
}

fn parse_seeds(text: &str) -> Result<Vec<u64>> {
    let mut seeds = Vec::new();
    for item in text.split(',') {
        seeds.push(
            item.trim()
                .parse::<u64>()
                .with_context(|| format!("seed {item:?}"))?,
        );
    }
    if seeds.is_empty() {
        bail!("no seeds");
    }
    Ok(seeds)
}

fn hash_output<T>(memory: &Memory, store: &Store<T>, pointer: u32, len: u32) -> Result<u64> {
    let start = usize::try_from(pointer).context("output pointer")?;
    let len = usize::try_from(len).context("output length")?;
    let mut bytes = vec![0u8; len];
    memory
        .read(store, start, &mut bytes)
        .context("reading output bytes")?;
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    Ok(hash)
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

#[allow(dead_code)]
fn _unused(_: &Path, _: &Instance) {}
