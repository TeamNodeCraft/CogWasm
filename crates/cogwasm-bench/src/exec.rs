//! Execute compiled benchmark modules: timing, result equality and exact
//! executed-instruction counts.

use crate::idle::IdleMonitor;
use crate::instrument::{COUNTER_EXPORT, instrument};
use crate::stats::{Metric, MetricClass};
use anyhow::{Context, Result, anyhow, bail};
use cogwasm_supervisor::{Artifact, Instance, Runtime};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::sync::Mutex;
use std::time::Instant;
use wasmtime::{Engine, Linker, Memory, Module, Store, TypedFunc, Val};

/// Map a Wasmtime error into this tool's error type.
trait Wt<T> {
    fn wt(self, context: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, context: &'static str) -> Result<T> {
        self.map_err(|error| anyhow!("{context}: {error}"))
    }
}

/// One seed's deterministic result and output-buffer digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub result: u64,
    pub out_len: u32,
    pub out_sha256: String,
}

/// Execution timing plus the outcomes used for the correctness gate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecMeasurement {
    pub exec_ms: Metric,
    pub instantiate_ms: Metric,
    pub outcomes: BTreeMap<String, Outcome>,
}

/// Exact executed wasm instructions over the suite's seeds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstructionMeasurement {
    pub instructions: Metric,
    pub blocks: u64,
    pub static_instructions: u64,
    pub per_seed: BTreeMap<String, u64>,
}

/// A Wasmtime runtime for benchmark modules, including the `nodecraft` host
/// imports the suite uses.
pub struct BenchRuntime {
    engine: Engine,
    linker: Linker<()>,
    modules: Mutex<HashMap<String, Module>>,
}

impl BenchRuntime {
    /// Build the runtime and its linker.
    ///
    /// # Errors
    ///
    /// Returns an error when the engine or the host imports cannot be built.
    pub fn new(_label: &str) -> Result<Self> {
        let engine = cogwasm_host::engine(None).context("creating the benchmark engine")?;
        Self::with_engine(engine)
    }

    /// A runtime on the supervisor's swap engine (parallel compilation and the
    /// in-memory incremental cache), so edit-loop swaps cost what they cost in
    /// `cogwasm-supervisor edit-loop`.
    ///
    /// # Errors
    ///
    /// Returns an error when the host imports cannot be added.
    pub fn for_swaps() -> Result<Self> {
        Self::with_engine(cogwasm_supervisor::wasmtime_runtime::build_engine())
    }

    fn with_engine(engine: Engine) -> Result<Self> {
        let mut linker = Linker::new(&engine);
        add_nodecraft_imports(&mut linker)?;
        Ok(Self {
            engine,
            linker,
            modules: Mutex::new(HashMap::new()),
        })
    }

    fn module(&self, artifact: &Artifact) -> Result<Module> {
        if let Some(module) = self
            .modules
            .lock()
            .map_err(|_| anyhow!("the module cache lock is poisoned"))?
            .get(&artifact.key)
            .cloned()
        {
            return Ok(module);
        }
        let module = Module::new(&self.engine, &artifact.bytes).wt("compiling an artifact")?;
        self.modules
            .lock()
            .map_err(|_| anyhow!("the module cache lock is poisoned"))?
            .insert(artifact.key.clone(), module.clone());
        Ok(module)
    }

    /// Compile (and cache) the module for a swapped artifact, without
    /// instantiating it. The edit loop calls this so the Wasmtime compile's
    /// instructions can be counted separately; the timed swap still includes
    /// the same compile work.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact is not a valid module.
    pub fn compile_swapped(&self, artifact: &Artifact) -> Result<()> {
        self.module(artifact).map(|_| ())
    }

    /// Execute a module over every seed for `runs` samples per seed.
    ///
    /// # Errors
    ///
    /// Returns an error when the module does not export the benchmark ABI, a
    /// run traps, a seed is non-deterministic within the module, or the output
    /// buffer is out of bounds.
    pub fn execute(
        &self,
        wasm: &[u8],
        seeds: &[u64],
        runs: usize,
        idle: &IdleMonitor,
        timed: bool,
    ) -> Result<ExecMeasurement> {
        let window = timed.then(|| idle.window("exec"));
        let module = Module::new(&self.engine, wasm).wt("compiling the benchmark module")?;
        let mut exec_values = Vec::with_capacity(seeds.len() * runs);
        let mut instantiate_values = Vec::with_capacity(seeds.len() * runs);
        let mut outcomes = BTreeMap::new();
        for seed in seeds {
            let mut observed: Option<Outcome> = None;
            for index in 0..runs {
                let started = Instant::now();
                let mut store = Store::new(&self.engine, ());
                let instance = self
                    .linker
                    .instantiate(&mut store, &module)
                    .wt("instantiating the benchmark module")?;
                let instantiate_ms = started.elapsed().as_secs_f64() * 1e3;
                let run = instance
                    .get_typed_func::<u64, u64>(&mut store, "run")
                    .wt("benchmark module must export run(seed: u64) -> u64")?;
                let started = Instant::now();
                let result = run.call(&mut store, *seed).wt("calling run(seed)")?;
                let exec_ms = started.elapsed().as_secs_f64() * 1e3;
                let outcome = read_outcome(&instance, &mut store, result)?;
                match &observed {
                    Some(previous) if previous != &outcome => bail!(
                        "benchmark module is not deterministic for seed {seed} \
                         (run {index}): {previous:?} then {outcome:?}"
                    ),
                    Some(_) => {}
                    None => {
                        outcomes.insert(seed.to_string(), outcome.clone());
                        observed = Some(outcome);
                    }
                }
                exec_values.push(exec_ms);
                instantiate_values.push(instantiate_ms);
            }
        }
        let sample = window.map(|window| idle.finish(window));
        let qualified = sample.as_ref().is_some_and(|sample| sample.qualified);
        Ok(ExecMeasurement {
            exec_ms: Metric::new(&exec_values, MetricClass::Wall)?.with_idle_qualified(qualified),
            instantiate_ms: Metric::new(&instantiate_values, MetricClass::Wall)?
                .with_idle_qualified(qualified),
            outcomes,
        })
    }

    /// Instrument `wasm` and count executed instructions over every seed.
    ///
    /// Each seed is executed twice; a disagreement is a hard error, so the
    /// counter is reported as deterministic only after it reproduced.
    ///
    /// # Errors
    ///
    /// Returns an error when instrumentation, instantiation or execution fails,
    /// when the instrumented module disagrees with `expected`, or when two
    /// runs count different instructions.
    pub fn measure_instructions(
        &self,
        wasm: &[u8],
        seeds: &[u64],
        expected: &BTreeMap<String, Outcome>,
    ) -> Result<InstructionMeasurement> {
        let instrumented = instrument(wasm)?;
        let module = Module::new(&self.engine, &instrumented.bytes)
            .wt("compiling the instrumented module")?;
        let mut per_seed = BTreeMap::new();
        let mut total = 0u64;
        for seed in seeds {
            let mut counts = Vec::with_capacity(2);
            for run in 0..2 {
                let count = execute_instrumented(
                    &self.engine,
                    &self.linker,
                    &module,
                    *seed,
                    expected.get(&seed.to_string()),
                )
                .with_context(|| format!("instrumented run {run} for seed {seed}"))?;
                counts.push(count);
            }
            if counts[0] != counts[1] {
                bail!(
                    "executed instructions for seed {seed} are not deterministic: {} then {}",
                    counts[0],
                    counts[1]
                );
            }
            total = total.saturating_add(counts[0]);
            per_seed.insert(seed.to_string(), counts[0]);
        }
        Ok(InstructionMeasurement {
            instructions: Metric::one(u64_as_f64(total), MetricClass::Exact)?,
            blocks: instrumented.blocks,
            static_instructions: instrumented.static_instructions,
            per_seed,
        })
    }
}

impl Runtime for BenchRuntime {
    fn instantiate(&self, artifact: &Artifact, generation: u64) -> Result<Box<dyn Instance>> {
        let module = self.module(artifact)?;
        let mut store = Store::new(&self.engine, ());
        let instance = self
            .linker
            .instantiate(&mut store, &module)
            .wt("instantiating an artifact")?;
        let run = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .wt("an artifact must export run(u64) -> u64")?;
        Ok(Box::new(BenchInstance {
            store,
            run,
            generation,
        }))
    }
}

struct BenchInstance {
    store: Store<()>,
    run: TypedFunc<u64, u64>,
    generation: u64,
}

impl Instance for BenchInstance {
    fn generation(&self) -> u64 {
        self.generation
    }

    fn call(&mut self, input: u64) -> Result<u64> {
        self.run
            .call(&mut self.store, input)
            .map_err(|error| anyhow!("calling run({input}): {error}"))
    }
}

fn execute_instrumented(
    engine: &Engine,
    linker: &Linker<()>,
    module: &Module,
    seed: u64,
    expected: Option<&Outcome>,
) -> Result<u64> {
    let mut store = Store::new(engine, ());
    let instance = linker
        .instantiate(&mut store, module)
        .wt("instantiating the instrumented module")?;
    let run = instance
        .get_typed_func::<u64, u64>(&mut store, "run")
        .wt("instrumented module must export run(u64) -> u64")?;
    let result = run
        .call(&mut store, seed)
        .wt("calling run(seed) on the instrumented module")?;
    let counter = instance
        .get_global(&mut store, COUNTER_EXPORT)
        .context("instrumented module is missing its instruction counter")?
        .get(&mut store);
    let count = match counter {
        Val::I64(value) => u64::try_from(value).context("a negative instruction count")?,
        other => bail!("the instruction counter is {other:?}, expected i64"),
    };
    let outcome = read_outcome(&instance, &mut store, result)?;
    if let Some(expected) = expected
        && expected != &outcome
    {
        bail!(
            "instrumented module disagrees with the uninstrumented module: \
             {outcome:?} vs {expected:?}"
        );
    }
    Ok(count)
}

fn read_outcome(
    instance: &wasmtime::Instance,
    store: &mut Store<()>,
    result: u64,
) -> Result<Outcome> {
    let memory = instance
        .get_memory(&mut *store, "memory")
        .context("benchmark module must export memory")?;
    let out_ptr = instance
        .get_typed_func::<(), u32>(&mut *store, "out_ptr")
        .wt("benchmark module must export out_ptr()")?
        .call(&mut *store, ())
        .wt("calling out_ptr()")?;
    let out_len = instance
        .get_typed_func::<(), u32>(&mut *store, "out_len")
        .wt("benchmark module must export out_len()")?
        .call(&mut *store, ())
        .wt("calling out_len()")?;
    let bytes = read_output(&memory, store, out_ptr, out_len)?;
    Ok(Outcome {
        result,
        out_len,
        out_sha256: hex(&Sha256::digest(&bytes)),
    })
}

fn read_output(memory: &Memory, store: &Store<()>, pointer: u32, len: u32) -> Result<Vec<u8>> {
    let start = usize::try_from(pointer).context("output pointer")?;
    let len = usize::try_from(len).context("output length")?;
    let end = start.checked_add(len).context("output range overflow")?;
    let memory_len = memory.data_size(store);
    if end > memory_len {
        bail!("output range {start}..{end} is outside the {memory_len}-byte memory");
    }
    let mut bytes = vec![0u8; len];
    memory
        .read(store, start, &mut bytes)
        .map_err(|error| anyhow!("reading the output buffer: {error}"))?;
    Ok(bytes)
}

/// Define the suite's `nodecraft` imports, matching `bench/host`.
///
/// # Errors
///
/// Returns an error when Wasmtime rejects an import definition.
pub fn add_nodecraft_imports(linker: &mut Linker<()>) -> Result<()> {
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

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
    }
    out
}

#[expect(
    clippy::cast_precision_loss,
    reason = "instruction counts on the suite are far below 2^53"
)]
fn u64_as_f64(value: u64) -> f64 {
    value as f64
}
