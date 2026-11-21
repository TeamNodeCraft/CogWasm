//! The swap seam.
//!
//! WebAssembly cannot replace a function in a live instance, so a "swap" is a
//! new instance that becomes current at a quiescent point. [`Swapper`] owns the
//! current instance and a generation counter; the new instance is created
//! before the old one is dropped, so a failed instantiation leaves the running
//! one untouched.
//!
//! The reference implementation is single-threaded, which models quiescence by
//! construction. A real runtime must additionally wait for in-flight calls on
//! host threads before it installs the new instance.

use std::sync::Arc;

use anyhow::{Context, Result, anyhow};

use crate::runtime::{Instance, Runtime};
use crate::store::Artifact;

/// The current instance and the generation it belongs to.
pub struct Swapper {
    runtime: Arc<dyn Runtime>,
    current: Option<Box<dyn Instance>>,
    generation: u64,
}

impl Swapper {
    /// A swapper with no instance installed.
    pub fn new(runtime: Arc<dyn Runtime>) -> Self {
        Self {
            runtime,
            current: None,
            generation: 0,
        }
    }

    /// Instantiate `artifact` and make it current, returning its generation.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact cannot be instantiated or the
    /// generation counter overflows. On error the previous instance stays
    /// current.
    pub fn install(&mut self, artifact: &Artifact) -> Result<u64> {
        let generation = self
            .generation
            .checked_add(1)
            .context("generation counter overflowed")?;
        let next = self
            .runtime
            .instantiate(artifact, generation)
            .with_context(|| format!("instantiating artifact {}", artifact.key))?;
        self.current = Some(next);
        self.generation = generation;
        Ok(generation)
    }

    /// Call the current instance.
    ///
    /// # Errors
    ///
    /// Returns an error when no instance is installed or the call traps.
    pub fn call(&mut self, input: u64) -> Result<u64> {
        let instance = self
            .current
            .as_mut()
            .ok_or_else(|| anyhow!("no instance is installed"))?;
        instance.call(input)
    }

    /// The generation of the current instance, or zero before the first
    /// install.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Whether an instance is installed.
    #[must_use]
    pub fn is_installed(&self) -> bool {
        self.current.is_some()
    }
}
