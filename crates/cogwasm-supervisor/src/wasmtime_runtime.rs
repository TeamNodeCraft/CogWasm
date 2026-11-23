//! The real runtime (M3/M4): instantiate an artifact in Wasmtime, call its
//! entry point, and optionally carry state across a swap.
//!
//! The plugins target `wasm32-unknown-unknown` and export `run(u64) -> u64`.
//! A stateful plugin additionally exports:
//!
//! - `cogwasm_state_version() -> i32`
//! - `cogwasm_snapshot(ptr, cap) -> i32`: with `cap = 0` return the negative
//!   required length; otherwise write the state at `ptr` and return its length.
//! - `cogwasm_restore(ptr, len) -> i32`: zero on success.
//!
//! and must export `wasm_alloc(size, align) -> ptr` so the host can provide the
//! buffer. A swap is a new instance; the old one is dropped only after the new
//! one exists and has accepted the state.

use anyhow::{Result, anyhow, bail};
use std::sync::Arc;
use wasmtime::{Config, Engine, Instance as WasmInstance, Memory, Module, Store, TypedFunc};

use crate::incremental_cache::InMemoryCacheStore;
use crate::runtime::{Instance, Runtime};
use crate::store::{Artifact, ArtifactMeta};

type SnapshotFn = TypedFunc<(i32, i32), i32>;
type RestoreFn = TypedFunc<(i32, i32), i32>;
type AllocFn = TypedFunc<(i32, i32), i32>;
type VersionFn = TypedFunc<(), i32>;

/// A Wasmtime engine that instantiates plugin artifacts.
///
/// Compiled modules are cached in memory by artifact key. A swap can then be
/// an instantiation (microseconds) instead of a wasm-to-native compile
/// (hundreds of milliseconds for a plugin), which is the production path: the
/// host precompiles the artifact once, then swaps cheaply.
pub struct WasmtimeRuntime {
    engine: Engine,
    modules: std::sync::Mutex<std::collections::HashMap<String, Module>>,
}

/// Builds the engine that compiles swaps.
///
/// The configuration otherwise matches [`Engine::default`]: the same Cranelift
/// compiler and optimisation level. Two environment variables make the swap
/// optimisations switchable for A/B measurement; both are enabled unless they
/// are set to `0`:
///
/// - `COGWASM_SWAP_PARALLEL=0` compiles the module's functions on one thread
///   instead of using [`Config::parallel_compilation`].
/// - `COGWASM_SWAP_INCREMENTAL=0` disables the in-memory
///   [`InMemoryCacheStore`] behind
///   [`Config::enable_incremental_compilation`], so every function is compiled
///   again on every swap.
#[must_use]
pub fn build_engine() -> Engine {
    let mut config = Config::new();
    config.parallel_compilation(env_enabled("COGWASM_SWAP_PARALLEL"));
    if env_enabled("COGWASM_SWAP_INCREMENTAL") {
        config
            .enable_incremental_compilation(Arc::new(InMemoryCacheStore::default()))
            .expect("the supervisor's incremental cache configuration is valid");
    }
    Engine::new(&config).expect("the supervisor's Wasmtime engine configuration is valid")
}

/// Whether an opt-out environment variable is unset or anything but `0`.
fn env_enabled(name: &str) -> bool {
    std::env::var_os(name).is_none_or(|value| value != "0")
}

impl Default for WasmtimeRuntime {
    fn default() -> Self {
        Self {
            engine: build_engine(),
            modules: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl std::fmt::Debug for WasmtimeRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WasmtimeRuntime")
            .finish_non_exhaustive()
    }
}

impl WasmtimeRuntime {
    /// A runtime with the default engine configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The underlying engine, for hosts that want to share a compiled module.
    #[must_use]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Compile `artifact` to native code and keep it for later swaps.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact cannot be compiled.
    pub fn precompile(&self, artifact: &Artifact) -> Result<()> {
        let mut modules = self
            .modules
            .lock()
            .map_err(|_| anyhow!("the module cache lock is poisoned"))?;
        if modules.contains_key(&artifact.key) {
            return Ok(());
        }
        let module = Module::new(&self.engine, &artifact.bytes)
            .map_err(|error| anyhow!("precompiling artifact {}: {error}", artifact.key))?;
        modules.insert(artifact.key.clone(), module);
        Ok(())
    }

    fn module(&self, artifact: &Artifact) -> Result<Module> {
        let cached = self
            .modules
            .lock()
            .map_err(|_| anyhow!("the module cache lock is poisoned"))?
            .get(&artifact.key)
            .cloned();
        if let Some(module) = cached {
            return Ok(module);
        }
        let module = Module::new(&self.engine, &artifact.bytes)
            .map_err(|error| anyhow!("compiling artifact {}: {error}", artifact.key))?;
        let mut modules = self
            .modules
            .lock()
            .map_err(|_| anyhow!("the module cache lock is poisoned"))?;
        modules.insert(artifact.key.clone(), module.clone());
        Ok(module)
    }
}

impl Runtime for WasmtimeRuntime {
    fn instantiate(&self, artifact: &Artifact, generation: u64) -> Result<Box<dyn Instance>> {
        let module = self.module(artifact)?;
        let mut store = Store::new(&self.engine, ());
        let instance = WasmInstance::new(&mut store, &module, &[]).map_err(|error| {
            anyhow!("instantiating the artifact (does it import anything?): {error}")
        })?;
        let run = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .map_err(|error| anyhow!("looking up the `run` export: {error}"))?;
        let memory = instance.get_memory(&mut store, "memory");
        let state_version = instance
            .get_typed_func::<(), i32>(&mut store, "cogwasm_state_version")
            .ok();
        let snapshot = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "cogwasm_snapshot")
            .ok();
        let restore = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "cogwasm_restore")
            .ok();
        let alloc = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "wasm_alloc")
            .ok();
        Ok(Box::new(WasmtimeInstance {
            store,
            run,
            generation,
            memory,
            state_version,
            snapshot,
            restore,
            alloc,
        }))
    }
}

struct WasmtimeInstance {
    store: Store<()>,
    run: TypedFunc<u64, u64>,
    generation: u64,
    memory: Option<Memory>,
    state_version: Option<VersionFn>,
    snapshot: Option<SnapshotFn>,
    restore: Option<RestoreFn>,
    alloc: Option<AllocFn>,
}

impl Instance for WasmtimeInstance {
    fn generation(&self) -> u64 {
        self.generation
    }

    fn call(&mut self, input: u64) -> Result<u64> {
        self.run
            .call(&mut self.store, input)
            .map_err(|error| anyhow!("calling the `run` export: {error}"))
    }

    fn state_version(&mut self) -> Result<Option<u32>> {
        let Some(version) = self.state_version.as_ref() else {
            return Ok(None);
        };
        let raw = version
            .call(&mut self.store, ())
            .map_err(|error| anyhow!("calling `cogwasm_state_version`: {error}"))?;
        Ok(Some(u32::try_from(raw).map_err(|_| {
            anyhow!("cogwasm_state_version returned a negative value ({raw})")
        })?))
    }

    fn snapshot(&mut self) -> Result<Option<Vec<u8>>> {
        let (Some(snapshot), Some(alloc)) = (self.snapshot.as_ref(), self.alloc.as_ref()) else {
            return Ok(None);
        };
        let memory = self
            .memory
            .ok_or_else(|| anyhow!("a stateful plugin must export `memory`"))?;
        let needed = snapshot
            .call(&mut self.store, (0, 0))
            .map_err(|error| anyhow!("asking for the state size: {error}"))?;
        if needed == 0 {
            return Ok(Some(Vec::new()));
        }
        let required = needed
            .checked_neg()
            .and_then(|value| u32::try_from(value).ok())
            .ok_or_else(|| anyhow!("cogwasm_snapshot reported an invalid size ({needed})"))?;
        let length = i32::try_from(required)
            .map_err(|_| anyhow!("state is too large for the wasm32 ABI ({required} bytes)"))?;
        let pointer = alloc
            .call(&mut self.store, (length, 8))
            .map_err(|error| anyhow!("allocating the state buffer: {error}"))?;
        let written = snapshot
            .call(&mut self.store, (pointer, length))
            .map_err(|error| anyhow!("writing the state: {error}"))?;
        if written != length {
            bail!("cogwasm_snapshot wrote {written} of {length} bytes");
        }
        let mut bytes = vec![0u8; required as usize];
        memory
            .read(&self.store, pointer_offset(pointer)?, &mut bytes)
            .map_err(|error| anyhow!("reading the state: {error}"))?;
        Ok(Some(bytes))
    }

    fn restore(&mut self, state: &[u8]) -> Result<()> {
        let (Some(restore), Some(alloc)) = (self.restore.as_ref(), self.alloc.as_ref()) else {
            bail!("the new plugin does not export the state contract");
        };
        let memory = self
            .memory
            .ok_or_else(|| anyhow!("a stateful plugin must export `memory`"))?;
        let length = i32::try_from(state.len()).map_err(|_| {
            anyhow!(
                "state is too large for the wasm32 ABI ({} bytes)",
                state.len()
            )
        })?;
        let pointer = alloc
            .call(&mut self.store, (length, 8))
            .map_err(|error| anyhow!("allocating the restore buffer: {error}"))?;
        memory
            .write(&mut self.store, pointer_offset(pointer)?, state)
            .map_err(|error| anyhow!("writing the state: {error}"))?;
        let status = restore
            .call(&mut self.store, (pointer, length))
            .map_err(|error| anyhow!("calling `cogwasm_restore`: {error}"))?;
        if status != 0 {
            bail!("cogwasm_restore rejected the state (status {status})");
        }
        Ok(())
    }
}

fn pointer_offset(pointer: i32) -> Result<usize> {
    usize::try_from(pointer)
        .map_err(|_| anyhow!("the plugin returned a negative pointer ({pointer})"))
}

/// An artifact whose bytes are WAT, for tests that need an instance without a
/// toolchain.
///
/// The key is derived from the bytes, so two different modules never collide
/// in the runtime's module cache.
#[must_use]
pub fn wat_artifact(wat: &str) -> Artifact {
    use sha2::{Digest, Sha256};
    let bytes = wat.as_bytes().to_vec();
    let digest = Sha256::digest(&bytes);
    let key = format!(
        "wat-{}",
        digest[..6].iter().fold(String::new(), |mut out, byte| {
            use std::fmt::Write as _;
            write!(out, "{byte:02x}").expect("writing to a String cannot fail");
            out
        })
    );
    let length = u64::try_from(bytes.len()).expect("artifact length fits in u64");
    Artifact {
        key: key.clone(),
        meta: ArtifactMeta {
            schema: crate::job::SCHEMA,
            key,
            options: crate::job::CompileOptions::default(),
            kind: crate::job::CrateKind::Cdylib,
            crate_name: "wat-test".to_owned(),
            compiler: "wat".to_owned(),
            source_sha256: String::new(),
            source_path: String::new(),
            bytes: length,
            compile_ms: None,
            link_ms: None,
        },
        bytes,
    }
}
