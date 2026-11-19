#!/usr/bin/env bash
# CogWasm compile-time + execution benchmark suite.
#
#   bench/run.sh --quick                  # CI-sized subset (default 3 runs)
#   bench/run.sh --full                   # all programs (default 5 runs)
#   bench/run.sh --full --runs 7 --out bench/results/local.json
#   bench/run.sh --programs a_add,expr,sha256 --runs 5
#   bench/run.sh --no-incremental        # for a compiler without the incremental patches
#   bench/run.sh --cold-runs 1            # fewer 38 s cold module compiles
#   bench/run.sh --fuel                   # also count compiler work with Wasmtime fuel
#
# Toolchain overrides (used by the baseline run in bench/README.md):
#   BENCH_TOOLCHAIN      experiments/cranelift-wasm/toolchain to measure
#   BENCH_LLVM_RUSTC     LLVM rustc used for the reference builds
#   BENCH_HOST_TRIPLE    host triple for the isolated stage1 compiler
#   BENCH_NIGHTLY        pinned nightly (default nightly-2026-10-19)
#   BENCH_HOST_TOOLCHAIN stable toolchain for the host binary (default 1.99.1)
#   BENCH_REBUILD_DEPS   rebuild the vendored libm/miniz_oxide rlibs
set -euo pipefail

BENCH=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd "$BENCH/.." && pwd)
TOOLCHAIN=${BENCH_TOOLCHAIN:-$ROOT/experiments/cranelift-wasm/toolchain}
NIGHTLY=${BENCH_NIGHTLY:-nightly-2026-10-19}
HOST_TOOLCHAIN=${BENCH_HOST_TOOLCHAIN:-1.99.1}
MODE=full
PROGRAMS=
RUNS=
COLD_RUNS=
OUT=
PREPARE_PRECOMPILED=1
NO_INCREMENTAL=0
FUEL=0

usage() {
    sed -n '2,18p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [ $# -gt 0 ]; do
    case "$1" in
        --quick) MODE=quick ;;
        --full) MODE=full ;;
        --programs) PROGRAMS=${2:?--programs needs a comma-separated list}; shift ;;
        --programs=*) PROGRAMS=${1#--programs=} ;;
        --runs) RUNS=${2:?--runs needs a value}; shift ;;
        --runs=*) RUNS=${1#--runs=} ;;
        --cold-runs) COLD_RUNS=${2:?--cold-runs needs a value}; shift ;;
        --cold-runs=*) COLD_RUNS=${1#--cold-runs=} ;;
        --out) OUT=${2:?--out needs a path}; shift ;;
        --out=*) OUT=${1#--out=} ;;
        --no-precompiled) PREPARE_PRECOMPILED=0 ;;
        --no-incremental) NO_INCREMENTAL=1 ;;
        --fuel) FUEL=1 ;;
        -h|--help) usage; exit 0 ;;
        *) printf 'unknown argument: %s (try --help)\n' "$1" >&2; exit 2 ;;
    esac
    shift
done

if [ -z "$RUNS" ]; then
    if [ "$MODE" = quick ]; then RUNS=3; else RUNS=5; fi
fi
case "$RUNS" in ''|*[!0-9]*) printf '%s\n' "--runs must be a positive integer" >&2; exit 2 ;; esac
[ "$RUNS" -ge 1 ] || { printf '%s\n' "--runs must be at least 1" >&2; exit 2; }

free_gb() { df -Pk "$1" | awk 'NR==2 {printf "%d", $4/1024/1024}'; }
need() { [ -e "$1" ] || { printf 'error: missing %s\n' "$1" >&2; exit 1; }; }

if [ ! -d "$TOOLCHAIN" ]; then
    printf 'error: toolchain directory %s does not exist\n' "$TOOLCHAIN" >&2
    printf 'build it first: experiments/cranelift-wasm/ci/build-and-test.sh\n' >&2
    exit 1
fi

RUSTC_WASM="$TOOLCHAIN/stage-c/artifacts/rustc.wasm"
SYSROOT="$TOOLCHAIN/stage-c/sysroot-pure"
WILD="$TOOLCHAIN/target/wild/release/wild"
export RUSTUP_HOME=${RUSTUP_HOME:-$TOOLCHAIN/rustup}
export CARGO_HOME=${CARGO_HOME:-$TOOLCHAIN/cargo}
need "$RUSTC_WASM"
need "$SYSROOT"
need "$WILD"
need "$CARGO_HOME/bin/cargo"

if [ "$(free_gb "$ROOT")" -lt 15 ]; then
    printf 'error: only %s GiB free under %s; stopping under 15 GiB\n' \
        "$(free_gb "$ROOT")" "$ROOT" >&2
    exit 1
fi

# LLVM reference compiler: pick the pinned nightly for this host without
# hardcoding a triple. An explicit BENCH_LLVM_RUSTC wins.
LLVM_RUSTC=${BENCH_LLVM_RUSTC:-}
if [ -z "$LLVM_RUSTC" ]; then
    shopt -s nullglob
    for candidate in "$RUSTUP_HOME"/toolchains/"$NIGHTLY"-*/bin/rustc; do
        LLVM_RUSTC=$candidate
        break
    done
    shopt -u nullglob
fi
need "${LLVM_RUSTC:?no $NIGHTLY rustc found; set BENCH_LLVM_RUSTC}"

HOST_TRIPLE=${BENCH_HOST_TRIPLE:-}
if [ -z "$HOST_TRIPLE" ]; then
    STAGE1="$TOOLCHAIN/stage-b/rust/build/host/stage1/bin/rustc"
    if [ -x "$STAGE1" ]; then
        HOST_TRIPLE=$("$STAGE1" -vV | sed -n 's/^host: //p')
    else
        # Wasmtime's target triple is the only thing needed by the host; the
        # pinned nightly directory name carries the host triple.
        HOST_TRIPLE=$(basename "$(dirname "$(dirname "$LLVM_RUSTC")")")
        HOST_TRIPLE=${HOST_TRIPLE#"$NIGHTLY"-}
    fi
fi
export BENCH_HOST_TRIPLE="$HOST_TRIPLE"

# rust-objcopy lives in the nightly's rustlib host bin; libm's build script
# needs it when the vendored dependencies are compiled with rustc.wasm.
shopt -s nullglob
OBJCOPY_DIRS=("$RUSTUP_HOME"/toolchains/"$NIGHTLY"-*/lib/rustlib/*/bin)
shopt -u nullglob
if [ "${#OBJCOPY_DIRS[@]}" -gt 0 ]; then
    PATH="${OBJCOPY_DIRS[0]}:$CARGO_HOME/bin:$PATH"
else
    PATH="$CARGO_HOME/bin:$PATH"
fi
export PATH

# Build the benchmark host into the same target directory as the stage-C host
# so CI reuses the compiled Wasmtime units instead of building them again.
unset RUSTC
HOST_TARGET="$TOOLCHAIN/target/stage-c-host"
HOST_BIN="$HOST_TARGET/release/bench-host"
BUILD_TOOLCHAIN=$HOST_TOOLCHAIN
# `[ -d "$path"-* ]` tests a literal glob, not the matches, so the fallback was
# never taken once the prefix had no directory. Collect the matches instead.
shopt -s nullglob
HOST_TOOLCHAIN_DIRS=("$RUSTUP_HOME"/toolchains/"$BUILD_TOOLCHAIN"-*)
shopt -u nullglob
if [ "${#HOST_TOOLCHAIN_DIRS[@]}" -eq 0 ]; then
    BUILD_TOOLCHAIN=$NIGHTLY
fi
printf 'building bench-host (%s)\n' "$BUILD_TOOLCHAIN" >&2
CARGO_TARGET_DIR="$HOST_TARGET" RUSTUP_TOOLCHAIN="$BUILD_TOOLCHAIN" \
    "$CARGO_HOME/bin/cargo" build --release \
    --manifest-path "$BENCH/host/Cargo.toml" >&2
need "$HOST_BIN"

WORK="$BENCH/work"
mkdir -p "$WORK"
PRECOMPILED=
STARTUP_PRECOMPILED=
if [ "$PREPARE_PRECOMPILED" = 1 ]; then
    PRECOMPILE="$HOST_TARGET/release/stage-c-precompile"
    if [ -x "$PRECOMPILE" ]; then
        if [ ! -f "$WORK/rustc.cwasm" ] || [ "$RUSTC_WASM" -nt "$WORK/rustc.cwasm" ]; then
            printf 'precompiling rustc.wasm for the precompiled-load metric\n' >&2
            "$PRECOMPILE" precompile --wasm "$RUSTC_WASM" \
                --out "$WORK/rustc.cwasm" --metadata "$WORK/rustc.cwasm.json" >&2
        fi
        if [ "$MODE" = full ]; then
            PRECOMPILED="$WORK/rustc.cwasm"
        else
            # Quick runs keep the program-compile path on the cached load (so
            # old quick points stay comparable) but still measure the
            # precompiled compiler load for the trend chart.
            STARTUP_PRECOMPILED="$WORK/rustc.cwasm"
        fi
    else
        printf 'note: stage-c-precompile is not built; measuring cached loads only\n' >&2
    fi
fi

ARGS=(
    --mode "$MODE"
    --runs "$RUNS"
    --llvm-runs "$([ "$MODE" = quick ] && echo 2 || echo "$RUNS")"
    --phase-runs 1
    --cold-runs "${COLD_RUNS:-$([ "$MODE" = quick ] && echo 1 || echo "$RUNS")}"
    --host "$HOST_BIN"
    --toolchain "$TOOLCHAIN"
    --llvm-rustc "$LLVM_RUSTC"
)
if [ -n "$PROGRAMS" ]; then
    ARGS+=(--programs "$PROGRAMS")
fi
if [ -n "$OUT" ]; then
    ARGS+=(--out "$OUT")
fi
if [ -n "$PRECOMPILED" ]; then
    ARGS+=(--precompiled "$PRECOMPILED")
fi
if [ -n "$STARTUP_PRECOMPILED" ]; then
    ARGS+=(--startup-precompiled "$STARTUP_PRECOMPILED")
fi
if [ "$NO_INCREMENTAL" = 1 ]; then
    ARGS+=(--no-incremental)
fi
if [ "$FUEL" = 1 ]; then
    ARGS+=(--fuel)
fi

# A baseline run points BENCH_TOOLCHAIN at a detached worktree; record that
# checkout's commit, not the main checkout's.
COMMIT_ROOT=$(git -C "$TOOLCHAIN" rev-parse --show-toplevel 2>/dev/null || printf '%s' "$ROOT")
export BENCH_COMMIT_ROOT="$COMMIT_ROOT"
export BENCH_TOOLCHAIN="$TOOLCHAIN"
export BENCH_NIGHTLY="$NIGHTLY"
export BENCH_HOST_TOOLCHAIN="$BUILD_TOOLCHAIN"
printf 'bench: mode=%s runs=%s toolchain=%s\n' "$MODE" "$RUNS" "$TOOLCHAIN" >&2
exec python3 "$BENCH/run.py" "${ARGS[@]}"
