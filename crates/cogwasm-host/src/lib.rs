//! Shared library for the `CogWasm` host tools.
//!
//! The `cogwasm-host` binary and the `cogwasm-precompile` binary both use
//! [`config`] for the Wasmtime engine and [`precompiled`] for loading and
//! validating a precompiled `rustc.wasm`. [`compile`] owns the rustc.wasm
//! compile path so the CLI and future resident hosts share one implementation.

pub mod compile;
pub mod config;
pub mod precompiled;
pub mod serve;

pub use compile::{
    compile, compile_args, rustc_args, CompileOutcome, CompileRequest, Compiler, Teardown,
};
pub use config::{
    engine, engine_config, engine_config_with, engine_with, Allocation, ConfigSummary,
};
pub use precompiled::{
    compatibility_hash_hex, load_rustc_module, metadata_sidecar, sha256_file, sha256_hex,
    wasmtime_version, ArtifactMetadata, FileInfo, LoadedModule, ModuleSource, TargetInfo,
};
pub use serve::{module_supports_serve, ServeSession};
