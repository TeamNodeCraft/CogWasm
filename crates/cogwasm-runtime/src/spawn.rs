//! Reporting shape for `playground.spawn` calls.

use serde::Serialize;

/// One `playground.spawn` invocation, as reported in host JSON output.
///
/// The record kept while the guest runs stays in each host: the product host
/// keeps the full argv, while the benchmark host keeps the program name and
/// argument count. Only this reported summary is shared.
#[derive(Serialize, Clone, Debug)]
pub struct SpawnSummary {
    /// Program name: the first argv element.
    pub program: String,
    /// Number of arguments after the program name.
    pub args: usize,
    /// Wall-clock time spent in the linker subprocess, in milliseconds.
    pub ms: f64,
    /// Exit status, or -1 when the linker could not be started.
    pub status: i32,
}
