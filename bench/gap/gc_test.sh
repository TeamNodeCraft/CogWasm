#!/usr/bin/env bash
# Does Wild actually remove unused sections for our wasm output?
#
# Compile the same source three ways through the product pipeline: the default
# (Wild's wasm target enables --gc-sections by default), with --gc-sections
# explicit, and with --no-gc-sections. The size difference is the evidence.
set -euo pipefail
GAP=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
BENCH=$(cd "$GAP/.." && pwd)
ROOT=$(cd "$BENCH/.." && pwd)
TOOLCHAIN=${BENCH_TOOLCHAIN:-$ROOT/experiments/cranelift-wasm/toolchain}
HOST=$TOOLCHAIN/target/stage-c-host/release/bench-host
OUT=$GAP/results/gc.json
mkdir -p "$GAP/work/gc" "$GAP/results"
printf '{"programs": {' > "$OUT"
first=1
for name in a_add sha256; do
    [ "$first" = 1 ] || printf ',' >> "$OUT"
    first=0
    printf '"%s": {' "$name" >> "$OUT"
    inner=1
    for variant in default gc no-gc; do
        args=()
        case "$variant" in
            gc) args+=(--rustc-arg=-Clink-arg=--gc-sections) ;;
            no-gc) args+=(--rustc-arg=-Clink-arg=--no-gc-sections) ;;
        esac
        out=$GAP/work/gc/$name-$variant.wasm
        "$HOST" compile --rustc "$TOOLCHAIN/stage-c/artifacts/rustc.wasm" \
            --sysroot "$TOOLCHAIN/stage-c/sysroot-pure" \
            --linker "$TOOLCHAIN/target/wild/release/wild" \
            --src "$BENCH/work/sources/$name.rs" \
            --prelude "$BENCH/programs/prelude.rs" \
            --out "$out" --work "$GAP/work/gc/$name-$variant" \
            --crate-name "$name" --runs 1 --phase-runs 0 --load cached \
            --cache-dir "$TOOLCHAIN/stage-c/cache" ${args[@]+"${args[@]}"} \
            --json "$GAP/work/gc/$name-$variant.json" >/dev/null
        [ "$inner" = 1 ] || printf ',' >> "$OUT"
        inner=0
        printf '"%s": %s' "$variant" "$(stat -f%z "$out" 2>/dev/null || stat -c%s "$out")" >> "$OUT"
    done
    printf '}' >> "$OUT"
done
printf '}}\n' >> "$OUT"
cat "$OUT"
