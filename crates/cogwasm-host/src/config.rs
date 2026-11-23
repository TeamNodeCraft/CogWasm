//! One Wasmtime [`Engine`] configuration for every `CogWasm` host tool.
//!
//! The host binary and the `cogwasm-precompile` binary must agree exactly:
//! a `.cwasm` may only be deserialized by an engine whose configuration is
//! compatible with the one that produced it. Keeping the configuration in one
//! place is what makes that true.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use wasmtime::{
    Cache, CacheConfig, Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig,
};

/// Guest stack limit for the rustc.wasm module.
pub const MAX_WASM_STACK: usize = 8 << 20;
/// Host async stack size (unused by the synchronous p1 host, kept identical to
/// the historical baseline configuration).
pub const ASYNC_STACK_SIZE: usize = 16 << 20;

/// The `CogWasm` engine configuration, before an optional module cache.
///
/// This is recorded in `.cwasm` metadata so a mismatch can be reported with a
/// readable reason instead of only an opaque compatibility hash.
#[expect(
    clippy::struct_excessive_bools,
    reason = "mirrors the Wasmtime Config flags recorded in the sidecar"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigSummary {
    /// `Config::wasm_threads`.
    pub wasm_threads: bool,
    /// `Config::wasm_bulk_memory`.
    pub wasm_bulk_memory: bool,
    /// `Config::wasm_multi_memory`.
    pub wasm_multi_memory: bool,
    /// `Config::wasm_simd`.
    pub wasm_simd: bool,
    /// `Config::wasm_relaxed_simd` (Wasmtime enables it by default).
    pub wasm_relaxed_simd: bool,
    /// `Config::max_wasm_stack`.
    pub max_wasm_stack: usize,
    /// `Config::async_stack_size`.
    pub async_stack_size: usize,
}

impl ConfigSummary {
    /// The configuration shipped by the cogwasm-host.
    pub const fn shipped() -> Self {
        Self {
            wasm_threads: false,
            wasm_bulk_memory: true,
            wasm_multi_memory: true,
            wasm_simd: true,
            wasm_relaxed_simd: true,
            max_wasm_stack: MAX_WASM_STACK,
            async_stack_size: ASYNC_STACK_SIZE,
        }
    }
}

/// Build the Wasmtime configuration from a summary plus an optional module
/// cache directory.
pub fn engine_config_with(summary: &ConfigSummary, cache_dir: Option<&Path>) -> Result<Config> {
    let mut config = Config::new();
    config
        .wasm_threads(summary.wasm_threads)
        .wasm_bulk_memory(summary.wasm_bulk_memory)
        .wasm_multi_memory(summary.wasm_multi_memory)
        .wasm_simd(summary.wasm_simd)
        // Wasmtime enables relaxed SIMD by default; setting it explicitly keeps
        // the engine and the recorded summary in lockstep (same behaviour as
        // the old main.rs configuration).
        .wasm_relaxed_simd(summary.wasm_relaxed_simd)
        .max_wasm_stack(summary.max_wasm_stack)
        .async_stack_size(summary.async_stack_size);
    if let Some(dir) = cache_dir {
        let mut cache_config = CacheConfig::new();
        // The 137 MiB rustc.wasm is much faster to mmap/deserialize than to
        // decompress on every warm start; the cache is local and disposable.
        cache_config
            .with_directory(dir)
            .with_baseline_compression_level(0)
            .with_optimized_compression_level(0);
        let cache = Cache::new(cache_config)
            .map_err(|error| anyhow!("creating Wasmtime module cache: {error}"))?;
        config.cache(Some(cache));
    } else {
        config.cache(None);
    }
    Ok(config)
}

/// How the engine allocates guest instances.
///
/// A runtime setting: it does not change the `.cwasm` compatibility hash (the
/// existing precompiled artifact loads under both, measured 2026-11-22).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allocation {
    /// Wasmtime's default: fresh linear memory mapped for every instance. The
    /// one-shot CLI and `cogwasm-precompile` use this.
    OnDemand,
    /// Wasmtime's pooling allocator: instances reuse a linear-memory slot, and
    /// the first `keep_resident_mib` MiB of it stay resident and are zeroed in
    /// place instead of being unmapped and faulted in again.
    Pooling {
        /// MiB of each memory slot kept resident between instances.
        keep_resident_mib: usize,
    },
}

impl Allocation {
    /// The resident backend's default, from experiment A0
    /// (`docs/notes/warm-session.md`): on the Linux CI runner keeping 256 MiB
    /// resident saved about 31-36 ms per job (fixture 125 -> 93 ms, json
    /// 539 -> 503 ms), as much as 1 GiB; pooling with nothing resident saved
    /// nothing. macOS shows no difference either way.
    pub const RESIDENT_DEFAULT: Self = Self::Pooling {
        keep_resident_mib: 256,
    };

    /// `default`, overridden by the environment for experiments:
    /// `COGWASM_POOLING=0` selects [`Allocation::OnDemand`],
    /// `COGWASM_POOLING=1` selects pooling, and
    /// `COGWASM_POOLING_KEEP_RESIDENT_MB` sets the resident size (pooling only;
    /// 256 when unset).
    #[must_use]
    pub fn from_env_or(default: Self) -> Self {
        let pooling = match std::env::var("COGWASM_POOLING").ok().as_deref() {
            Some("0") => return Self::OnDemand,
            Some(_) => true,
            None => matches!(default, Self::Pooling { .. }),
        };
        if !pooling {
            return Self::OnDemand;
        }
        let default_mib = match default {
            Self::Pooling { keep_resident_mib } => keep_resident_mib,
            Self::OnDemand => 256,
        };
        let keep_resident_mib = std::env::var("COGWASM_POOLING_KEEP_RESIDENT_MB")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default_mib);
        Self::Pooling { keep_resident_mib }
    }

    fn apply(self, config: &mut Config) {
        let Self::Pooling { keep_resident_mib } = self else {
            return;
        };
        let mut pooling = PoolingAllocationConfig::default();
        pooling
            // A resident backend runs one job at a time; a few slots leave
            // room for a supervisor tier running beside it.
            .total_core_instances(4)
            .total_memories(4)
            .total_tables(4)
            .total_stacks(4)
            // rustc.wasm's linear memory may grow to the full 32-bit range.
            .max_memory_size(4 << 30)
            // rustc.wasm's indirect-call table and instance state are far
            // larger than the pooling defaults, which target small modules.
            .table_elements(1 << 20)
            .max_core_instance_size(64 << 20)
            .linear_memory_keep_resident(keep_resident_mib << 20);
        config.allocation_strategy(InstanceAllocationStrategy::Pooling(pooling));
    }
}

/// The shipped configuration, with an optional module cache directory.
pub fn engine_config(cache_dir: Option<&Path>) -> Result<Config> {
    engine_config_with(&ConfigSummary::shipped(), cache_dir)
}

/// Create the `CogWasm` engine with the default (on-demand) allocator.
pub fn engine(cache_dir: Option<&Path>) -> Result<Engine> {
    engine_with(cache_dir, Allocation::OnDemand)
}

/// Create the `CogWasm` engine with an explicit instance allocation strategy.
pub fn engine_with(cache_dir: Option<&Path>, allocation: Allocation) -> Result<Engine> {
    let mut config = engine_config(cache_dir)?;
    allocation.apply(&mut config);
    Engine::new(&config).map_err(|error| anyhow!("creating Wasmtime engine: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precompiled::compatibility_hash_hex;

    #[test]
    fn shipped_config_matches_the_historical_host_config() {
        let mut historical = Config::new();
        historical
            .wasm_threads(false)
            .wasm_bulk_memory(true)
            .wasm_multi_memory(true)
            .wasm_simd(true)
            .max_wasm_stack(8 << 20)
            .async_stack_size(16 << 20);
        let historical = Engine::new(&historical).expect("creating the historical engine");
        let shipped = engine(None).expect("creating the shipped engine");
        assert_eq!(
            compatibility_hash_hex(&historical),
            compatibility_hash_hex(&shipped),
            "the moved configuration must be byte-identical to main.rs's old one"
        );
    }
}
