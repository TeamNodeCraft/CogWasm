#!/usr/bin/env bash
# Incremental-compilation regression check for the CogWasm compiler.
#
# Compile one corpus program with rustc.wasm three ways:
#   1. cold, no -C incremental
#   2. with a persistent incremental directory (first, cache-creating run)
#   3. with the same incremental directory (cache-reusing run)
# then run all three modules in Wasmtime through difftest-host and compare
# `run(seed)` and the output buffer bit-for-bit. The hard requirement is
# semantic identity: incremental reuse must not change what the program does.
#
# Cold and incremental output are NOT byte-identical, and the script reports
# the size difference instead of failing on it: an incremental build uses a
# different CGU/symbol split than the default non-incremental cg_clif build, so
# the incremental module keeps more duplicate generic instantiations and a
# bigger name section. That is a real output-size cost; only a semantic
# mismatch is a failure.
#
# The real programs are checked by `compiler/run.sh incremental`
# (plugin (c) eval == 2.25 and the A.2 e_math reference check, with a
# semantics-changing edit proving the cache cannot serve stale code).
#
#   difftest/incremental-check.sh
#
# Tool paths can be overridden with the same DIFFTEST_* variables as run.sh.
set -euo pipefail

DIFFTEST=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO=$(cd "$DIFFTEST/.." && pwd)
UPSTREAM="$REPO/compiler/scripts/upstream.py"
pin() { python3 "$UPSTREAM" get "$1" "$2"; }
TOOL="$REPO/toolchain"
SRC="$DIFFTEST/corpus/smoke.rs"
WORK=${DIFFTEST_WORK:-$TOOL/difftest-work}/incremental-check
INC="$WORK/inc"

RUSTC_WASM=${DIFFTEST_RUSTC_WASM:-$TOOL/artifacts/rustc.wasm}
SYSROOT=${DIFFTEST_SYSROOT:-$TOOL/sysroot}
COMPILER_HOST=${DIFFTEST_COMPILER_HOST:-${DIFFTEST_STAGE_C_HOST:-$TOOL/target/cogwasm-host/release/cogwasm-host}}
WILD=${DIFFTEST_WILD:-$TOOL/target/wild/release/wild}
CACHE_DIR=${DIFFTEST_CACHE_DIR:-$TOOL/cache}
TARGET_DIR=${DIFFTEST_TARGET_DIR:-$TOOL/target/cogwasm-host}
RUNNER=${DIFFTEST_HOST_RUNNER:-$TARGET_DIR/release/difftest-host}

need() { [ -e "$1" ] || { printf 'error: missing %s (set the matching DIFFTEST_* override, or build it first)\n' "$1" >&2; exit 1; }; }
need "$SRC"
need "$RUSTC_WASM"
need "$SYSROOT"
need "$COMPILER_HOST"
need "$WILD"

# Refuse obvious unsafe roots before removing the per-run work directory.
case "$WORK" in ''|/) printf '%s\n' "error: refusing WORK=$WORK" >&2; exit 1 ;; esac
case "$WORK" in "$TOOL"|"$REPO"|"$HOME"|"$HOME/") printf 'error: refusing WORK=%s\n' "$WORK" >&2; exit 1 ;; esac
mkdir -p "$WORK"
WORK=$(cd "$WORK" && pwd)
rm -rf "${WORK:?}/inc" "${WORK:?}/cold" "${WORK:?}/incremental-1" "${WORK:?}/incremental-2"
mkdir -p "$WORK/cold" "$WORK/incremental-1" "$WORK/incremental-2"

# difftest-host compares the modules and reports value/output mismatches.
if [ ! -x "$RUNNER" ]; then
    export RUSTUP_HOME=${RUSTUP_HOME:-$TOOL/rustup}
    export CARGO_HOME=${CARGO_HOME:-$TOOL/cargo}
    CARGO=${DIFFTEST_CARGO:-$CARGO_HOME/bin/cargo}
    HOST_TOOLCHAIN=${DIFFTEST_HOST_TOOLCHAIN:-$(pin rust-stable version)}
    CARGO_TARGET_DIR="$TARGET_DIR" RUSTUP_TOOLCHAIN="$HOST_TOOLCHAIN" \
        "$CARGO" build --release --manifest-path "$REPO/crates/difftest-host/Cargo.toml" --bin difftest-host
fi
need "$RUNNER"

compile() {
    local out=$1
    shift
    "$COMPILER_HOST" raw \
        --rustc "$RUSTC_WASM" --sysroot "$SYSROOT" --linker "$WILD" \
        --cache-dir "$CACHE_DIR" "$@" -- \
        --target wasm32-unknown-unknown --sysroot "$SYSROOT" \
        --edition=2021 --crate-type cdylib -C panic=abort -C opt-level=2 \
        -Zcodegen-backend=cranelift "-Clinker=$WILD" -Clinker-flavor=wasm-ld \
        "$SRC" -o "$out"
}

echo "== incremental-check: cold compile"
compile "$WORK/cold/smoke.wasm"

echo "== incremental-check: first run with a persistent incremental directory"
compile "$WORK/incremental-1/smoke.wasm" --incremental "$INC"

echo "== incremental-check: second run reusing the incremental directory"
compile "$WORK/incremental-2/smoke.wasm" --incremental "$INC"

printf 'smoke\thandwritten\t0\n' > "$WORK/manifest.tsv"
for run in incremental-1 incremental-2; do
    echo "== incremental-check: compare cold vs $run in Wasmtime"
    "$RUNNER" compare \
        --manifest "$WORK/manifest.tsv" \
        --llvm "$WORK/cold" --clif "$WORK/$run" \
        --run-seeds "0,1,42,0x9e3779b97f4a7c15"
done

# Output-size report. `inspect` parses the wasm sections; use it for the code
# size and function count as well as the total file size.
inspect() {
    "$COMPILER_HOST" inspect --wasm "$1" | python3 -c '
import json, sys
d = json.load(sys.stdin)
print("{} bytes ({} code, {} functions)".format(d["bytes"], d["code_bytes"], d["functions"]))
'
}
cold_desc=$(inspect "$WORK/cold/smoke.wasm")
inc_desc=$(inspect "$WORK/incremental-1/smoke.wasm")
cold_bytes=$("$COMPILER_HOST" inspect --wasm "$WORK/cold/smoke.wasm" | python3 -c 'import json,sys; print(json.load(sys.stdin)["bytes"])')
inc_bytes=$("$COMPILER_HOST" inspect --wasm "$WORK/incremental-1/smoke.wasm" | python3 -c 'import json,sys; print(json.load(sys.stdin)["bytes"])')
delta_pct=$(awk -v a="$cold_bytes" -v b="$inc_bytes" 'BEGIN { printf "%.1f", (b-a)*100/a }')
printf 'cold:                %s\n' "$cold_desc"
printf 'incremental (first): %s\n' "$inc_desc"
printf 'note: incremental output is %s bytes (+%s%%); the extra size is CGU\n' "$inc_bytes" "$delta_pct"
printf '      partitioning and duplicate generic instantiations, not a semantic change.\n'
printf 'PASS incremental-check: cold and incremental smoke.wasm have identical results\n'
