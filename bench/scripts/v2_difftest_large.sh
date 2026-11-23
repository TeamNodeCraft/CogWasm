#!/usr/bin/env bash
# Difftest-style check for the V2 phase-0 large program: compile the same
# source with LLVM rustc and with the CogWasm pipeline, run both modules in
# Wasmtime, and require identical run(seed) results and output buffers.
#
#   bench/scripts/v2_difftest_large.sh [large.rs]
#
# The default source is the generated program under toolchain/work/v2 (see
# bench/scripts/gen_large.py). Bulk output stays under toolchain/work/v2.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
TOOL="$REPO/toolchain"
SRC=${1:-$TOOL/work/v2/large/large.rs}
OUT="$TOOL/work/v2/large/difftest"
SEEDS="0,1,42,18446744073709551615"
[ -f "$SRC" ] || { echo "missing $SRC; run bench/scripts/gen_large.py first" >&2; exit 1; }
rm -rf "$OUT"
mkdir -p "$OUT/work"

shopt -s nullglob
LLVM_RUSTC=("$TOOL"/rustup/toolchains/nightly-*/bin/rustc)
shopt -u nullglob
[ "${#LLVM_RUSTC[@]}" -gt 0 ] || { echo "no pinned nightly rustc under $TOOL/rustup" >&2; exit 1; }

echo "compiling with LLVM..." >&2
"${LLVM_RUSTC[0]}" --edition=2021 --target wasm32-unknown-unknown \
    --crate-type cdylib -C panic=abort -C opt-level=2 -C strip=debuginfo \
    "$SRC" -o "$OUT/llvm.wasm"

echo "compiling with the CogWasm pipeline..." >&2
"$TOOL/target/cogwasm-host/release/cogwasm-host" raw \
    --rustc "$TOOL/artifacts/rustc.wasm" \
    --sysroot "$TOOL/sysroot" \
    --linker "$TOOL/target/wild/release/wild" \
    --cache-dir "$TOOL/cache" \
    --work "$OUT/work" -- \
    --target wasm32-unknown-unknown --sysroot "$TOOL/sysroot" \
    --edition=2021 --crate-type cdylib -C panic=abort -C opt-level=2 \
    -Zcodegen-backend=cranelift "-Clinker=$TOOL/target/wild/release/wild" \
    -Clinker-flavor=wasm-ld "$SRC" -o "$OUT/clif.wasm"

for backend in llvm clif; do
    "$TOOL/target/cogwasm-host/release/bench-host" run \
        --wasm "$OUT/$backend.wasm" --runs 1 --seeds "$SEEDS" \
        --json "$OUT/$backend-run.json" >/dev/null
done

python3 - "$OUT" <<'PY'
import json, sys
from pathlib import Path
out = Path(sys.argv[1])
left = json.load(open(out / "llvm-run.json"))
right = json.load(open(out / "clif-run.json"))
if left["outcomes"] != right["outcomes"]:
    print("MISMATCH", file=sys.stderr)
    for seed in sorted(set(left["outcomes"]) | set(right["outcomes"])):
        if left["outcomes"].get(seed) != right["outcomes"].get(seed):
            print(seed, left["outcomes"].get(seed), right["outcomes"].get(seed), file=sys.stderr)
    raise SystemExit(1)
print(
    f"PASS: {left['wasm_bytes']} B LLVM, {right['wasm_bytes']} B CogWasm; "
    f"{len(left['outcomes'])} seeds identical"
)
PY
