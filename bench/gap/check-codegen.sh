#!/usr/bin/env bash
# Code-generation structure regression check for constant aggregate locals.
#
# Rebuilds sha256 with rustc.wasm, regenerates the hot-function WAT and
# asserts on instructions, not on source or symbol names: no software memory
# routine may be called, and the 256-byte K-table copy in `compress` is gone.
#
# Against the pre-fix compiler (92f976a) this fails: sha256::compress copies
# the whole K table with a 256-byte `call $memcpy`. See bench/gap/REPORT.md,
# ranked fix 1.
set -euo pipefail

GAP=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd "$GAP/../.." && pwd)
TOOLCHAIN=${BENCH_TOOLCHAIN:-"$ROOT/experiments/cranelift-wasm/toolchain"}
# Reuse the Wasmtime units bench/run.sh already built in CI.
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$TOOLCHAIN/target/stage-c-host"}

"$GAP/run.sh" --no-mir sha256 >&2

fail() {
    printf 'codegen structure check failed: %s\n' "$*" >&2
    exit 1
}

for wat in "$GAP"/results/hot/sha256/clif-top*.wat; do
    [ -f "$wat" ] || fail "missing $wat"
    if grep -Eq 'call \$(memcpy|memmove|memset|copy_forward)' "$wat"; then
        fail "$wat still calls a software memory routine"
    fi
done

# The old shape materialised K in the frame and copied all 256 bytes with
# `$memcpy`. If a 256 constant now feeds a call, the copy is back.
if grep -A2 'i32.const 256' "$GAP"/results/hot/sha256/clif-top*.wat | grep -q 'call'; then
    fail "sha256 still copies the 256-byte K table"
fi

printf 'codegen structure checks passed\n'
