#!/usr/bin/env bash
# Code-generation structure regression checks for the two memory fixes.
#
# Rebuilds sha256 and sort with rustc.wasm, regenerates the hot-function WAT
# and asserts on instructions, not on source or symbol names:
#   * no `call $memcpy`/`$memmove`/`$memset`/`copy_forward` in either hot set;
#   * sha256 has no 256-byte K-table copy left in `compress`;
#   * sort's swaps are i64 loads/stores, not calls.
#
# Against the pre-fix compiler (92f976a) this fails: sha256::compress has a
# 256-byte `call $memcpy` and sort calls `$memmove`/`$memcpy` for its 8-byte
# swaps. See bench/gap/REPORT.md, ranked fix 1.
set -euo pipefail

GAP=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd "$GAP/../.." && pwd)
TOOLCHAIN=${BENCH_TOOLCHAIN:-"$ROOT/experiments/cranelift-wasm/toolchain"}
# Reuse the Wasmtime units bench/run.sh already built in CI.
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$TOOLCHAIN/target/stage-c-host"}

"$GAP/run.sh" --no-mir sha256 sort >&2

fail() {
    printf 'codegen structure check failed: %s\n' "$*" >&2
    exit 1
}

hot_files() {
    printf '%s\n' "$GAP"/results/hot/"$1"/clif-top*.wat
}

for program in sha256 sort; do
    for wat in $(hot_files "$program"); do
        [ -f "$wat" ] || fail "missing $wat"
        if grep -Eq 'call \$(memcpy|memmove|memset|copy_forward)' "$wat"; then
            fail "$wat still calls a software memory routine"
        fi
    done
done

# F1a: sha256's compress indexes the constant K allocation directly. The old
# shape materialised it in the frame and copied all 256 bytes with `$memcpy`.
if grep -q 'i32.const 256' "$GAP/results/hot/sha256/clif-top1.wat"; then
    # The loop bound also contains 256; only fail when it feeds a call.
    if grep -A2 'i32.const 256' "$GAP/results/hot/sha256/clif-top1.wat" | grep -q 'call'; then
        fail "sha256 still copies the 256-byte K table"
    fi
fi

# F1b: sort's 8-byte swaps must be widened loads/stores. A regression to a
# call is caught above; this catches a regression to byte-at-a-time copies.
for wat in $(hot_files sort); do
    if grep -q 'i64.load\|i64.store' "$wat"; then
        found_load_store=1
    fi
done
[ "${found_load_store:-0}" = 1 ] || fail 'sort has no widened i64 load/store for its swaps'

printf 'codegen structure checks passed\n'
