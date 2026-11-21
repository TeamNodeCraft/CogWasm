//! One Wasmtime [`Engine`] configuration for every stage-C tool.
//!
//! The host binary and the `stage-c-precompile` binary must agree exactly:
//! a `.cwasm` may only be deserialized by an engine whose configuration is
//! compatible with the one that produced it. Keeping the configuration in one
//! place is what makes that true.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use wasmtime::{Cache, CacheConfig, Config, Engine};

/// Guest stack limit for the rustc.wasm module.
pub const MAX_WASM_STACK: usize = 8 << 20;
/// Host async stack size (unused by the synchronous p1 host, kept identical to
/// the stage-C baseline configuration).
pub const ASYNC_STACK_SIZE: usize = 16 << 20;

/// The stage-C engine configuration, before an optional module cache.
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
    /// The configuration shipped by the stage-C host.
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

/// The shipped configuration, with an optional module cache directory.
pub fn engine_config(cache_dir: Option<&Path>) -> Result<Config> {
    engine_config_with(&ConfigSummary::shipped(), cache_dir)
}

/// Create the stage-C engine.
pub fn engine(cache_dir: Option<&Path>) -> Result<Engine> {
    let config = engine_config(cache_dir)?;
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
