//! Environment variables shared by the hosts.

use std::ffi::OsString;

/// Read a `COGWASM_<name>` environment variable, falling back to the old
/// `STAGE_C_<name>` spelling for one release (see `agents/plans/next-steps.md`,
/// M5).
pub fn env_var_compat(name: &str) -> Option<OsString> {
    std::env::var_os(format!("COGWASM_{name}"))
        .or_else(|| std::env::var_os(format!("STAGE_C_{name}")))
}
