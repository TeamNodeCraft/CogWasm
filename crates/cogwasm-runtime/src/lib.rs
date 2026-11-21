//! Shared host-side runtime helpers for the `CogWasm` Wasmtime hosts.
//!
//! The stage-C product host and the benchmark host deliberately keep separate
//! engine setup, WASI store construction, spawn recording and startup
//! reporting: those have drifted in ways that are observable (stderr capture,
//! deterministic randomness, resource limits, log prefixes and error text).
//! This crate holds only the items that are identical in both binaries, so
//! moving one here cannot change either host's behaviour.

mod error;
mod spawn;

pub use error::Wt;
pub use spawn::SpawnSummary;
