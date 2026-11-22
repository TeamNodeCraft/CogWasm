//! Shared host-side runtime helpers for the `CogWasm` Wasmtime hosts.
//!
//! The product host and the benchmark host keep separate CLIs and startup
//! reporting, but the pieces that must not drift live here: guest-memory
//! reads, the timing median, store/instance construction, and
//! `playground.spawn`. Per-host policy (stderr capture, randomness, fuel,
//! limits, what a spawn records) is an explicit option or recorder, so the
//! differences stay visible instead of becoming two copies.

mod env;
mod error;
mod mem;
mod serve;
mod spawn;
mod stats;
mod store;

pub use env::env_var_compat;
pub use error::Wt;
pub use mem::{GuestMem, MAX_CSTR_BYTES, guest_memory};
pub use serve::{
    NextRequest, ServeEvent, ServeFinish, ServeHooks, add_serve_imports, decode_request,
    encode_request,
};
pub use spawn::{
    ArgvRecord, ArgvRecorder, SpawnRecorder, SpawnSummary, SummaryRecorder, add_playground_spawn,
};
pub use stats::median;
pub use store::{
    HostState, Limits, PAGE_SIZE, PreparedInstance, STDERR_CAPACITY, StoreOptions, prepare_instance,
};
