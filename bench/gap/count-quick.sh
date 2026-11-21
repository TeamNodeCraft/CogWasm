#!/usr/bin/env bash
# Add exact executed-instruction counts to one quick benchmark JSON.
#
#   bench/gap/count-quick.sh bench/results/quick-<run>.json
#
# It reuses the modules the quick suite already built under bench/work/wasm/
# and the compiler-work samples run.py wrote with --fuel. It builds gap-tool
# once (with the shared Cargo target directory) and never rebuilds rustc.wasm.
#
# Environment, matching bench/run.sh:
#   BENCH_TOOLCHAIN      toolchain that produced the modules
#   BENCH_CARGO_TARGET_DIR  optional Cargo target directory for gap-tool
set -euo pipefail

GAP=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
BENCH=$(cd "$GAP/.." && pwd)
ROOT=$(cd "$BENCH/.." && pwd)
TOOLCHAIN=${BENCH_TOOLCHAIN:-$ROOT/toolchain}
CARGO_HOME=${CARGO_HOME:-$TOOLCHAIN/cargo}
RUSTUP_HOME=${RUSTUP_HOME:-$TOOLCHAIN/rustup}
export CARGO_HOME RUSTUP_HOME
export PATH="$CARGO_HOME/bin:$PATH"
RUSTUP_TOOLCHAIN=${BENCH_HOST_TOOLCHAIN:-1.99.1}
export RUSTUP_TOOLCHAIN
export CARGO_TARGET_DIR=${BENCH_CARGO_TARGET_DIR:-$TOOLCHAIN/target/bench-host}

json=${1:?usage: bench/gap/count-quick.sh <quick-result.json>}
[ -f "$json" ] || { printf 'error: %s does not exist\n' "$json" >&2; exit 1; }
[ -d "$BENCH/work/wasm/clif" ] || {
    printf 'error: %s has no quick modules; run bench/run.sh --quick first\n' \
        "$BENCH/work/wasm" >&2
    exit 1
}

printf 'building gap-tool\n' >&2
cargo build --release --locked --manifest-path "$GAP/Cargo.toml" >&2
TOOL="$CARGO_TARGET_DIR/release/gap-tool"
[ -x "$TOOL" ] || { printf 'error: %s was not built\n' "$TOOL" >&2; exit 1; }

python3 "$GAP/count.py" \
    --quick "$json" \
    --modules "$BENCH/work/wasm" \
    --tool "$TOOL" \
    --work "$GAP/work/quick" \
    --verify-runs 2
