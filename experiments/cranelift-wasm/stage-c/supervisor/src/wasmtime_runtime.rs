//! The real runtime (M3): instantiate an artifact in Wasmtime and call its
//! entry point.
//!
//! The plugins target `wasm32-unknown-unknown` and export `run(u64) -> u64`
//! with no imports (the host-call plugins are out of scope for the reference
//! runtime until the host-import contract lands). A swap is a new instance;
//! the old one is dropped only after the new one exists.

use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Result, anyhow};
use wasmtime::{Engine, Instance as WasmInstance, Module, Store, TypedFunc};

use crate::runtime::{Instance, Runtime};
use crate::store::{Artifact, ArtifactMeta};

/// A Wasmtime engine that instantiates plugin artifacts.
///
/// Compiled modules are cached in memory by artifact key. A swap can then be
/// an instantiation (microseconds) instead of a wasm-to-native compile
/// (hundreds of milliseconds for a plugin), which is the production path: the
/// host precompiles the artifact once, then swaps cheaply.
pub struct WasmtimeRuntime {
    engine: Engine,
    modules: Mutex<HashMap<String, Module>>,
}

impl Default for WasmtimeRuntime {
    fn default() -> Self {
        Self {
            engine: Engine::default(),
            modules: Mutex::new(HashMap::new()),
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
}

impl Runtime for WasmtimeRuntime {
    fn instantiate(&self, artifact: &Artifact, generation: u64) -> Result<Box<dyn Instance>> {
        let cached = self
            .modules
            .lock()
            .map_err(|_| anyhow!("the module cache lock is poisoned"))?
            .get(&artifact.key)
            .cloned();
        let module = if let Some(module) = cached {
            module
        } else {
            let module = Module::new(&self.engine, &artifact.bytes)
                .map_err(|error| anyhow!("compiling artifact {}: {error}", artifact.key))?;
            let mut modules = self
                .modules
                .lock()
                .map_err(|_| anyhow!("the module cache lock is poisoned"))?;
            modules.insert(artifact.key.clone(), module.clone());
            module
        };
        let mut store = Store::new(&self.engine, ());
        let instance = WasmInstance::new(&mut store, &module, &[]).map_err(|error| {
            anyhow!("instantiating the artifact (does it import anything?): {error}")
        })?;
        let run = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .map_err(|error| anyhow!("looking up the `run` export: {error}"))?;
        Ok(Box::new(WasmtimeInstance {
            store,
            run,
            generation,
        }))
    }
}

struct WasmtimeInstance {
    store: Store<()>,
    run: TypedFunc<u64, u64>,
    generation: u64,
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
}

/// An artifact whose bytes are WAT, for tests that need an instance without a
/// toolchain.
#[must_use]
pub fn wat_artifact(wat: &str) -> Artifact {
    let bytes = wat.as_bytes().to_vec();
    let length = u64::try_from(bytes.len()).expect("artifact length fits in u64");
    Artifact {
        key: "wat-test".to_owned(),
        tier: crate::job::Tier::Fast,
        meta: ArtifactMeta {
            schema: crate::job::SCHEMA,
            key: "wat-test".to_owned(),
            tier: crate::job::Tier::Fast,
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
