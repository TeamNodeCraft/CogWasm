//! Convert Wasmtime's error type into an `anyhow` error with context.

use anyhow::{Result, anyhow};

/// Convert a `wasmtime::Result` (its own error type) into `anyhow::Result`
/// with a readable context.
pub trait Wt<T> {
    /// Map a Wasmtime error, prefixing `context`.
    fn wt(self, context: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, context: &'static str) -> Result<T> {
        self.map_err(|error| anyhow!("{context}: {error}"))
    }
}
