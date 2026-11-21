//! Shared library for the stage-C tools.
//!
//! The `stage-c-host` binary and the `stage-c-precompile` binary both use
//! [`config`] for the Wasmtime engine and [`precompiled`] for loading and
//! validating a precompiled `rustc.wasm`.

pub mod config;
pub mod precompiled;

pub use config::{engine, engine_config, engine_config_with, ConfigSummary};
pub use precompiled::{
    compatibility_hash_hex, load_rustc_module, metadata_sidecar, sha256_file, sha256_hex,
    wasmtime_version, ArtifactMetadata, FileInfo, LoadedModule, ModuleSource, TargetInfo,
};
