#!/usr/bin/env bash
# Differential test: compile every corpus program with LLVM rustc and with the
# stage-C Cranelift->wasm pipeline, then run both modules in Wasmtime and
# compare run(seed) and the output buffer bit-for-bit.
#
# One entry point for local use and CI:
#
#   difftest/run.sh                     # 48 handwritten + 24 generated programs
#   difftest/run.sh --seeds 64          # more generated programs
#   difftest/run.sh --quick             # 8 generated programs, 2 run seeds
#
# Tool paths can be overridden with environment variables:
#   DIFFTEST_LLVM_RUSTC, DIFFTEST_RUSTC_WASM, DIFFTEST_SYSROOT,
#   DIFFTEST_STAGE_C_HOST, DIFFTEST_WILD, DIFFTEST_CACHE_DIR,
#   DIFFTEST_TARGET_DIR, DIFFTEST_WORK, DIFFTEST_RESULTS,
#   DIFFTEST_CARGO, DIFFTEST_HOST_TOOLCHAIN, DIFFTEST_GEN_START
#
# Full output is tee'd to difftest/results/run.log (gitignored) and printed to
# stdout; the runner additionally writes difftest/results/details.log. Exits
# nonzero on any mismatch, compile failure or missing artifact.
#
# Works on macOS arm64 and Linux x86_64: the LLVM rustc is discovered from the
# isolated rustup directory (toolchain directory names carry the host triple),
# never hardcoded to aarch64-apple-darwin.
set -euo pipefail

DIFFTEST=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO=$(cd "$DIFFTEST/.." && pwd)
TOOL="$REPO/toolchain"
CORPUS="$DIFFTEST/corpus"
RESULTS=${DIFFTEST_RESULTS:-$DIFFTEST/results}
WORK=${DIFFTEST_WORK:-$TOOL/difftest-work}
TARGET_DIR=${DIFFTEST_TARGET_DIR:-$TOOL/target/stage-c-host}

mkdir -p "$RESULTS"
LOG="$RESULTS/run.log"
: > "$LOG"

say() { printf '%s\n' "$*"; printf '%s\n' "$*" >> "$LOG"; }
step() { say ""; say "== [$(date -u +%H:%M:%S)] $1"; }
elapsed() { echo $(( $(date +%s) - START )); }
free_gb() { df -Pk "$1" | awk 'NR==2 {printf "%d", $4/1024/1024}'; }

on_err() {
    local rc=$?
    local msg="!! difftest failed (exit $rc) near line $1; see $LOG"
    printf '%s\n' "$msg" >&2
    printf '%s\n' "$msg" >> "$LOG"
    exit "$rc"
}
trap 'on_err $LINENO' ERR

SEEDS=24
RUN_SEEDS="0,1,42,0x9e3779b97f4a7c15"
while [ $# -gt 0 ]; do
    case "$1" in
        --seeds) SEEDS=${2:?--seeds needs a value}; shift 2 ;;
        --seeds=*) SEEDS=${1#--seeds=}; shift ;;
        --quick) SEEDS=8; RUN_SEEDS="0,1"; shift ;;
        -h|--help)
            sed -n '2,29p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
            exit 0 ;;
        *) printf 'unknown argument: %s (try --help)\n' "$1" >&2; exit 2 ;;
    esac
done
case "$SEEDS" in ''|*[!0-9]*) printf '%s\n' "--seeds must be a positive integer" >&2; exit 2 ;; esac
[ "$SEEDS" -ge 8 ] || printf 'warning: --seeds %s is below the 8 the README documents\n' "$SEEDS" >&2

START=$(date +%s)
say "difftest: $(uname -s) $(uname -m), $(free_gb "$REPO") GiB free, seeds=$SEEDS, run_seeds=$RUN_SEEDS"

# ---------------------------------------------------------------------------
# Tool discovery / overrides
# ---------------------------------------------------------------------------
RUSTC_WASM=${DIFFTEST_RUSTC_WASM:-$TOOL/stage-c/artifacts/rustc.wasm}
SYSROOT=${DIFFTEST_SYSROOT:-$TOOL/stage-c/sysroot-pure}
STAGE_C_HOST=${DIFFTEST_STAGE_C_HOST:-$TOOL/target/stage-c-host/release/stage-c-host}
WILD=${DIFFTEST_WILD:-$TOOL/target/wild/release/wild}
CACHE_DIR=${DIFFTEST_CACHE_DIR:-$TOOL/stage-c/cache}
GEN_START=${DIFFTEST_GEN_START:-1}

# Isolated rustup/cargo for the LLVM side. The stage-C scripts use the same
# environment; keep it overridable for a caller that already exported it.
export RUSTUP_HOME=${RUSTUP_HOME:-$TOOL/rustup}
export CARGO_HOME=${CARGO_HOME:-$TOOL/cargo}
export RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN:-nightly-2026-10-19}
if [ -x "$CARGO_HOME/bin/cargo" ]; then
    PATH="$CARGO_HOME/bin:$PATH"
fi
export PATH
if [ -n "${DIFFTEST_CARGO:-}" ]; then
    CARGO=$DIFFTEST_CARGO
elif [ -x "$CARGO_HOME/bin/cargo" ]; then
    CARGO=$CARGO_HOME/bin/cargo
elif command -v cargo >/dev/null 2>&1; then
    CARGO=$(command -v cargo)
else
    printf '%s\n' "error: no cargo found; set DIFFTEST_CARGO" >&2
    exit 1
fi

LLVM_RUSTC=${DIFFTEST_LLVM_RUSTC:-}
if [ -z "$LLVM_RUSTC" ]; then
    # Toolchain directories are named nightly-<date>-<host-triple>; pick the
    # pinned nightly for whatever host we are on. No triple is hardcoded.
    shopt -s nullglob
    for d in "$RUSTUP_HOME"/toolchains/nightly-*-*; do
        if [ -x "$d/bin/rustc" ]; then
            LLVM_RUSTC="$d/bin/rustc"
            break
        fi
    done
    shopt -u nullglob
fi
[ -n "$LLVM_RUSTC" ] || { printf '%s\n' "error: no LLVM rustc found under $RUSTUP_HOME/toolchains; set DIFFTEST_LLVM_RUSTC" >&2; exit 1; }

need() { [ -e "$1" ] || { printf 'error: missing %s (set the matching DIFFTEST_* override, or run compiler/scripts/build-*.sh)\n' "$1" >&2; exit 1; }; }
need "$LLVM_RUSTC"
need "$RUSTC_WASM"
need "$SYSROOT"
need "$STAGE_C_HOST"
need "$WILD"

# Absolute paths: rustc.wasm runs under WASI with `/` preopened, so relative
# source/output paths would be interpreted differently. Refuse obvious unsafe
# roots before deleting the per-run subdirectories.
case "$WORK" in ''|/) printf '%s\n' "error: refusing DIFFTEST_WORK=$WORK" >&2; exit 1 ;; esac
case "$WORK" in "$TOOL"|"$REPO"|"$HOME"|"$HOME/") printf 'error: refusing DIFFTEST_WORK=%s\n' "$WORK" >&2; exit 1 ;; esac
mkdir -p "$WORK"
WORK=$(cd "$WORK" && pwd)
if [ "$(free_gb "$WORK")" -lt 15 ]; then
    printf 'error: only %s GiB free under %s; stopping before compiling (need 15 GiB)\n' "$(free_gb "$WORK")" "$WORK" >&2
    exit 1
fi
rm -rf "$WORK/llvm" "$WORK/clif" "$WORK/generated" "$WORK/logs"
mkdir -p "$WORK/llvm" "$WORK/clif" "$WORK/generated" "$WORK/logs"
MANIFEST="$WORK/manifest.tsv"
: > "$MANIFEST"

# ---------------------------------------------------------------------------
# Build the runner + generator. Reusing the stage-c-host target directory means
# the Wasmtime dependency is already compiled in CI.
# ---------------------------------------------------------------------------
step "build difftest runner and generator"
# The stage-C host is built with stable 1.99.1; use the same toolchain so the
# release Wasmtime dependency units in the shared target dir are reused in CI.
HOST_TOOLCHAIN=${DIFFTEST_HOST_TOOLCHAIN:-1.99.1}
shopt -s nullglob
host_tc_dirs=("$RUSTUP_HOME"/toolchains/"$HOST_TOOLCHAIN"-*)
shopt -u nullglob
if [ "${#host_tc_dirs[@]}" -eq 0 ]; then
    echo "note: $HOST_TOOLCHAIN not installed; building the runner with $RUSTUP_TOOLCHAIN" >&2
    HOST_TOOLCHAIN=$RUSTUP_TOOLCHAIN
fi
CARGO_TARGET_DIR="$TARGET_DIR" RUSTUP_TOOLCHAIN="$HOST_TOOLCHAIN" "$CARGO" build --release \
    --manifest-path "$REPO/crates/difftest-host/Cargo.toml" --bins 2>&1 | tee -a "$LOG"
RUNNER="$TARGET_DIR/release/difftest-host"
GENERATOR="$TARGET_DIR/release/difftest-gen"
need "$RUNNER"
need "$GENERATOR"

# ---------------------------------------------------------------------------
# Compile helpers
# ---------------------------------------------------------------------------
compile_llvm() {
    local name=$1 src=$2 out=$3 log="$WORK/logs/$1.llvm.log"
    if ! "$LLVM_RUSTC" --edition=2021 --target wasm32-unknown-unknown \
        --crate-type cdylib -C panic=abort -C opt-level=2 \
        "$src" -o "$out" >"$log" 2>&1; then
        printf 'error: LLVM compile failed for %s\n' "$name" >&2
        cat "$log" >&2
        { printf 'error: LLVM compile failed for %s\n' "$name"; cat "$log"; } >> "$LOG"
        return 1
    fi
}

compile_clif() {
    local name=$1 src=$2 out=$3 log="$WORK/logs/$1.clif.log"
    # Goal 1 step 5b experiment: opt the clif compile into the loop-field
    # hoist. The LLVM reference compiler has no such pass.
    local extra=()
    if [ "${DIFFTEST_MIR_HOIST:-0}" = "1" ]; then
        extra=(-Zmir-hoist-loop-field-reads)
    fi
    # Goal 1 step 5b experiment: opt the clif compile into the loop-aware MIR
    # inliner. The LLVM reference compiler has no such pass.
    if [ "${DIFFTEST_INLINE_WASM:-0}" = "1" ]; then
        extra+=(-Zmir-inline-wasm)
    fi
    if ! "$STAGE_C_HOST" raw \
        --rustc "$RUSTC_WASM" --sysroot "$SYSROOT" --linker "$WILD" \
        --cache-dir "$CACHE_DIR" -- \
        --target wasm32-unknown-unknown --sysroot "$SYSROOT" \
        --edition=2021 --crate-type cdylib -C panic=abort -C opt-level=2 \
        ${extra[@]+"${extra[@]}"} \
        -Zcodegen-backend=cranelift "-Clinker=$WILD" -Clinker-flavor=wasm-ld \
        "$src" -o "$out" >"$log" 2>&1; then
        printf 'error: Cranelift compile failed for %s\n' "$name" >&2
        cat "$log" >&2
        { printf 'error: Cranelift compile failed for %s\n' "$name"; cat "$log"; } >> "$LOG"
        return 1
    fi
}

# ---------------------------------------------------------------------------
# Handwritten corpus
# ---------------------------------------------------------------------------
TRAPS="$CORPUS/expect_trap.txt"
HAND=()
for f in "$CORPUS"/*.rs; do
    n=$(basename "$f" .rs)
    [ "$n" = prelude ] || HAND+=("$n")
done
[ "${#HAND[@]}" -ge 30 ] || { printf 'error: only %s handwritten programs; expected 30-60\n' "${#HAND[@]}" >&2; exit 1; }

step "compile ${#HAND[@]} handwritten corpus programs with both backends"
T0=$(date +%s)
i=0
for n in "${HAND[@]}"; do
    i=$((i + 1))
    say "$(printf '[difftest] compile %2d/%d %s' "$i" "${#HAND[@]}" "$n")"
    compile_llvm "$n" "$CORPUS/$n.rs" "$WORK/llvm/$n.wasm"
    compile_clif "$n" "$CORPUS/$n.rs" "$WORK/clif/$n.wasm"
    trap=0
    if [ -f "$TRAPS" ] && grep -qx "$n" "$TRAPS"; then trap=1; fi
    printf '%s\thandwritten\t%s\n' "$n" "$trap" >> "$MANIFEST"
done
say "compiled ${#HAND[@]} handwritten programs in $(( $(date +%s) - T0 ))s"

# ---------------------------------------------------------------------------
# Generated corpus
# ---------------------------------------------------------------------------
step "generate and compile $SEEDS deterministic random programs"
"$GENERATOR" \
    --out "$WORK/generated" --seeds "$SEEDS" --start "$GEN_START" \
    --prelude "$CORPUS/prelude.rs" 2>&1 | tee -a "$LOG"
T0=$(date +%s)
g=0
for f in "$WORK"/generated/gen_*.rs; do
    n=$(basename "$f" .rs)
    g=$((g + 1))
    say "$(printf '[difftest] compile %2d/%d %s (generated)' "$g" "$SEEDS" "$n")"
    compile_llvm "$n" "$f" "$WORK/llvm/$n.wasm"
    compile_clif "$n" "$f" "$WORK/clif/$n.wasm"
done
# The generator's manifest already has name<TAB>generated<TAB>0<TAB>checksum.
grep -v '^#' "$WORK/generated/manifest.tsv" >> "$MANIFEST"
say "compiled $SEEDS generated programs in $(( $(date +%s) - T0 ))s"

# ---------------------------------------------------------------------------
# Run both backends and compare
# ---------------------------------------------------------------------------
step "run and compare in Wasmtime"
set +e
"$RUNNER" compare \
    --manifest "$MANIFEST" \
    --llvm "$WORK/llvm" \
    --clif "$WORK/clif" \
    --run-seeds "$RUN_SEEDS" \
    --log "$RESULTS/details.log" 2>&1 | tee -a "$LOG"
RC=${PIPESTATUS[0]}
set -e
[ "$RC" -eq 0 ] || { say "RESULT: FAIL (runner exit $RC; details in $RESULTS/details.log)"; exit "$RC"; }

TOTAL=$(elapsed)
say ""
say "PASS: ${#HAND[@]} handwritten + $SEEDS generated programs, seeds [$RUN_SEEDS], ${TOTAL}s total"
say "logs: $RESULTS/run.log and $RESULTS/details.log"
