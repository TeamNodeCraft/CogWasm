#!/usr/bin/env bash
# Gap attribution against LLVM -O2.
#
#   bench/gap/run.sh                  # all programs, including the MIR experiment
#   bench/gap/run.sh sha256 sort      # a subset
#   bench/gap/run.sh --no-mir         # counts only, no MIR-optimisation experiment
#   bench/gap/run.sh --skip-build     # reuse bench/gap/work/modules
#
# The script builds the modules through bench/run.py's compile paths, rewrites
# each function so every basic block bumps an exported i64 counter, runs the
# same seeds as the suite, and writes counts and analysis JSON under
# bench/gap/results/. It never changes the wasm under test.
set -euo pipefail

GAP=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
BENCH=$(cd "$GAP/.." && pwd)
ROOT=$(cd "$BENCH/.." && pwd)
UPSTREAM="$ROOT/compiler/scripts/upstream.py"
pin() { python3 "$UPSTREAM" get "$1" "$2"; }
TOOLCHAIN=${BENCH_TOOLCHAIN:-$ROOT/toolchain}
CARGO_HOME=${CARGO_HOME:-$TOOLCHAIN/cargo}
RUSTUP_HOME=${RUSTUP_HOME:-$TOOLCHAIN/rustup}
export CARGO_HOME RUSTUP_HOME
export PATH="$CARGO_HOME/bin:$PATH"
export RUSTUP_TOOLCHAIN=${BENCH_HOST_TOOLCHAIN:-$(pin rust-stable version)}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$TOOLCHAIN/target/bench-host}
export BENCH_NIGHTLY=${BENCH_NIGHTLY:-$(pin rust-nightly version)}
export BENCH_HOST_TOOLCHAIN=${BENCH_HOST_TOOLCHAIN:-$(pin rust-stable version)}
# build.py and mir.py detect BENCH_HOST_TRIPLE from the toolchain when it is
# not set; do not hard-code one host here.

MIR=1
SKIP_BUILD=0
PROGRAMS=()
for argument in "$@"; do
    case "$argument" in
        --mir) MIR=1 ;;
        --no-mir) MIR=0 ;;
        --skip-build) SKIP_BUILD=1 ;;
        -h|--help) sed -n '2,14p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) printf 'unknown argument: %s\n' "$argument" >&2; exit 2 ;;
        *) PROGRAMS+=("$argument") ;;
    esac
done

free_gb() { df -Pk "$1" | awk 'NR==2 {printf "%d", $4/1024/1024}'; }
if [ "$(free_gb "$ROOT")" -lt 15 ]; then
    printf 'error: only %s GiB free; stopping under 15 GiB\n' "$(free_gb "$ROOT")" >&2
    exit 1
fi

printf 'building gap-tool\n' >&2
cargo build --release --locked --manifest-path "$GAP/Cargo.toml" >&2
TOOL="$CARGO_TARGET_DIR/release/gap-tool"
export GAP_TOOL="$TOOL"

if [ "$SKIP_BUILD" = 0 ]; then
    if [ "${#PROGRAMS[@]}" -eq 0 ]; then
        python3 "$GAP/build.py" >&2
    else
        python3 "$GAP/build.py" "${PROGRAMS[@]}" >&2
    fi
fi

SEEDS=$(python3 -c 'import json,sys; print(",".join(str(s) for s in json.load(open(sys.argv[1]))["seeds"]))' "$BENCH/programs.json")
mkdir -p "$GAP/results/counts"
if [ "${#PROGRAMS[@]}" -eq 0 ]; then
    while IFS= read -r name; do
        PROGRAMS+=("$name")
    done < <(python3 -c 'import json,sys; print("\n".join(p["name"] for p in json.load(open(sys.argv[1]))["programs"]))' "$BENCH/programs.json")
fi
for name in "${PROGRAMS[@]}"; do
    for build in clif llvm_o2; do
        wasm="$GAP/work/modules/$build/$name.wasm"
        [ -e "$wasm" ] || { printf 'error: missing %s\n' "$wasm" >&2; exit 1; }
        "$TOOL" instrument --wasm "$wasm" \
            --out "$GAP/work/instrumented/$name-$build.wasm" \
            --meta "$GAP/results/counts/$name-$build.meta.json" >&2
        "$TOOL" measure --wasm "$GAP/work/instrumented/$name-$build.wasm" \
            --meta "$GAP/results/counts/$name-$build.meta.json" \
            --seeds "$SEEDS" \
            --fuel-wasm "$wasm" \
            --out "$GAP/results/counts/$name-$build.counts.json" >&2
    done
    printf 'measured %s\n' "$name" >&2
done

python3 "$GAP/analyze.py" --counts "$GAP/results/counts" \
    --modules "$GAP/work/modules" "${PROGRAMS[@]}" >&2

if [ "$MIR" = 1 ]; then
    python3 "$GAP/mir.py" "${PROGRAMS[@]}" >&2
fi
printf 'wrote %s\n' "$GAP/results/analysis.json" >&2
