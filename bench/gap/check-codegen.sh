#!/usr/bin/env bash
# Code-generation structure regression checks for the memory fixes.
#
# Rebuilds sha256 and sort with rustc.wasm, regenerates the hot-function WAT
# and asserts on instructions, not on source or symbol names: no software
# memory routine (`memcpy`, `memmove`, `memset` or `copy_forward`) may be
# called from either program's hot functions.
#
# Against the pre-fix compiler (92f976a) this fails: sha256::compress has a
# 256-byte `call $memcpy` for the K table and sort calls `$memmove`/`$memcpy`
# for its 8-byte swaps. See bench/gap/REPORT.md, ranked fix 1.
set -euo pipefail

GAP=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd "$GAP/../.." && pwd)
TOOLCHAIN=${BENCH_TOOLCHAIN:-"$ROOT/toolchain"}
# Reuse the Wasmtime units bench/run.sh already built in CI.
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$TOOLCHAIN/target/stage-c-host"}

"$GAP/run.sh" --no-mir sha256 sort >&2

fail() {
    printf 'codegen structure check failed: %s\n' "$*" >&2
    exit 1
}

for program in sha256 sort; do
    for wat in "$GAP"/results/hot/"$program"/clif-top*.wat; do
        [ -f "$wat" ] || fail "missing $wat"
        if grep -nE 'call \$(memcpy|memmove|memset|copy_forward)' "$wat" >&2; then
            fail "$wat still calls a software memory routine"
        fi
    done
done

printf 'codegen structure checks passed\n'
