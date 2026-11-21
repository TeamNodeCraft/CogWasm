#!/usr/bin/env bash
# Build the stage-C Wasmtime 50.0.2 host (plain WASI p1, no wasi-threads).
# The sysroot builder also needs it: rustc.wasm is executed through this host
# while Cargo compiles core/alloc/compiler_builtins.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
TOOL="$REPO/toolchain"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
# The host is a normal native binary; stable 1.99.1 builds Wasmtime 50.0.2.
export RUSTUP_TOOLCHAIN="${STAGE_C_HOST_TOOLCHAIN:-1.99.1}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$TOOL/target/stage-c-host}"

cargo build --release --manifest-path "$REPO/crates/cogwasm-host/Cargo.toml"
ls -lh "$CARGO_TARGET_DIR/release/stage-c-host"
