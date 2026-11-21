//! The runtime seam.
//!
//! A swap needs a runtime that can instantiate an artifact as a fresh instance
//! and call it. The real Wasmtime runtime lands in M3; [`FakeRuntime`] records
//! which generation receives each call so the swap ordering can be tested
//! without an engine.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result, anyhow, bail};

use crate::store::Artifact;

/// One running instance.
///
/// A plugin may implement the optional state contract:
/// `cogwasm_state_version() -> i32`, `cogwasm_snapshot(ptr, cap) -> i32` and
/// `cogwasm_restore(ptr, len) -> i32`. The defaults below make every instance
/// stateless, which is the M1-M3 behaviour.
pub trait Instance: Send {
    /// The generation this instance was created for.
    fn generation(&self) -> u64;

    /// Call the instance's entry point.
    ///
    /// # Errors
    ///
    /// Returns an error when the instance traps.
    fn call(&mut self, input: u64) -> Result<u64>;

    /// The plugin's state ABI version, or `None` when it has no state contract.
    ///
    /// # Errors
    ///
    /// Returns an error when the version export traps.
    fn state_version(&mut self) -> Result<Option<u32>> {
        Ok(None)
    }

    /// Snapshot the plugin's state, or `None` when it has no state contract.
    ///
    /// # Errors
    ///
    /// Returns an error when the snapshot protocol is violated.
    fn snapshot(&mut self) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    /// Restore a snapshot produced by [`Instance::snapshot`].
    ///
    /// # Errors
    ///
    /// Returns an error when the instance cannot restore state or the plugin
    /// rejects it.
    fn restore(&mut self, _state: &[u8]) -> Result<()> {
        bail!("this instance cannot restore state")
    }
}

/// Instantiates artifacts.
pub trait Runtime: Send + Sync {
    /// Instantiate `artifact` as `generation`.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact cannot be instantiated.
    fn instantiate(&self, artifact: &Artifact, generation: u64) -> Result<Box<dyn Instance>>;
}

/// A runtime that records calls per generation.
#[derive(Debug, Default)]
pub struct FakeRuntime {
    calls: Arc<Mutex<HashMap<u64, u64>>>,
}

impl FakeRuntime {
    /// A fresh fake runtime with no instances.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Calls recorded for `generation`.
    ///
    /// # Errors
    ///
    /// Returns an error only if the call log is poisoned.
    pub fn calls(&self, generation: u64) -> Result<u64> {
        let calls = self
            .calls
            .lock()
            .map_err(|_| anyhow!("the call log lock is poisoned"))?;
        Ok(calls.get(&generation).copied().unwrap_or(0))
    }
}

impl Runtime for FakeRuntime {
    fn instantiate(&self, artifact: &Artifact, generation: u64) -> Result<Box<dyn Instance>> {
        let recorded = u64::try_from(artifact.bytes.len()).context("artifact length")?;
        Ok(Box::new(FakeInstance {
            generation,
            seed: generation ^ recorded,
            calls: Arc::clone(&self.calls),
        }))
    }
}

struct FakeInstance {
    generation: u64,
    seed: u64,
    calls: Arc<Mutex<HashMap<u64, u64>>>,
}

impl Instance for FakeInstance {
    fn generation(&self) -> u64 {
        self.generation
    }

    fn call(&mut self, input: u64) -> Result<u64> {
        let mut calls = self
            .calls
            .lock()
            .map_err(|_| anyhow!("the call log lock is poisoned"))?;
        let entry = calls.entry(self.generation).or_insert(0);
        *entry += 1;
        Ok(self.seed ^ input)
    }
}
