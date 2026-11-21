#!/usr/bin/env bash
# Build the CogWasm Wasmtime 50.0.2 host (plain WASI p1, no wasi-threads).
# The sysroot builder also needs it: rustc.wasm is executed through this host
# while Cargo compiles core/alloc/compiler_builtins.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
UPSTREAM="$REPO/compiler/scripts/upstream.py"
TOOL="$REPO/toolchain"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
# The host is a normal native binary; the stable toolchain from the manifest
# builds the Wasmtime host.
STABLE=$(python3 "$UPSTREAM" get rust-stable version)
export RUSTUP_TOOLCHAIN="${COGWASM_HOST_TOOLCHAIN:-${STAGE_C_HOST_TOOLCHAIN:-$STABLE}}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$TOOL/target/cogwasm-host}"

cargo build --release --manifest-path "$REPO/crates/cogwasm-host/Cargo.toml"
ls -lh "$CARGO_TARGET_DIR/release/cogwasm-host"
