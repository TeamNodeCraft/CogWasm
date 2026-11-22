#!/usr/bin/env bash
# G2.3 experiments E1, E2 and E4 on this machine (docs/notes/warm-session.md).
#
#   E4  rustc's per-pass times (-Ztime-passes, JSON) for an empty no_std
#       cdylib and the edit-loop fixture, median of five, so the CI runner's
#       breakdown can be compared with the macOS one in the note.
#   E1  the metadata-read probe (what rustc's wasm32 Mmap::map does) for
#       libcore + compiler_builtins rmeta, three fresh instances.
#   E2  the cost of restoring a snapshot of a compile's guest memory: a host
#       memmove of the measured peak (~143 MiB) into an already-resident
#       buffer, and into a freshly allocated one.
#
# A measurement, not a gate. Prints tables and ::notice lines. CI ran it for
# runs 44975284082 and 44978461782; kept for manual reruns, not run by CI.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TOOL="$REPO/toolchain"
OUT="$TOOL/work/profile"
HOST="$TOOL/target/cogwasm-host/release/cogwasm-host"
WILD="$TOOL/target/wild/release/wild"
SYSROOT="$TOOL/sysroot"
CWASM="$TOOL/work/precompiled/rustc.cwasm"
RUNS=${PROFILE_RUNS:-5}
rm -rf "$OUT"
mkdir -p "$OUT"

printf '#![no_std]\n#[panic_handler]\nfn p(_: &core::panic::PanicInfo) -> ! { loop {} }\n' >"$OUT/empty.rs"
cp "$REPO/crates/cogwasm-supervisor/tests/fixtures/edit_loop.rs" "$OUT/fixture.rs"

compile() { # name -> time-passes JSON on stderr
    local name=$1
    "$HOST" raw --rustc "$TOOL/artifacts/rustc.wasm" --precompiled "$CWASM" \
        --sysroot "$SYSROOT" --linker "$WILD" -- \
        --target wasm32-unknown-unknown --sysroot "$SYSROOT" --edition 2021 \
        --crate-name "$name" --crate-type cdylib -C panic=abort \
        -Zcodegen-backend=cranelift "-Clinker=$WILD" -Clinker-flavor=wasm-ld \
        -Copt-level=2 -Ztime-passes -Ztime-passes-format=json \
        "$OUT/$name.rs" -o "$OUT/$name.wasm"
}

echo "== E4: -Ztime-passes, median of $RUNS ($(uname -s) $(uname -m))"
for i in $(seq 1 "$RUNS"); do
    compile empty >"$OUT/empty-$i.out" 2>&1
    compile fixture >"$OUT/fixture-$i.out" 2>&1
done
python3 - "$OUT" "$RUNS" <<'PY'
import glob, json, os, statistics, sys
out, runs = sys.argv[1], int(sys.argv[2])
def load(prefix):
    result = []
    for path in sorted(glob.glob(os.path.join(out, f"{prefix}-*.out"))):
        passes = {}
        for line in open(path):
            if line.startswith("time: {"):
                entry = json.loads(line[6:])
                passes[entry["pass"]] = passes.get(entry["pass"], 0) + entry["time"] * 1e3
        result.append(passes)
    return result
empty, fixture = load("empty"), load("fixture")
names = list(dict.fromkeys([k for r in empty + fixture for k in r]))
med = lambda rs, k: statistics.median([r.get(k, 0.0) for r in rs])
print(f"{'pass':40}{'empty':>9}{'fixture':>9}")
for name in names:
    e, f = med(empty, name), med(fixture, name)
    if max(e, f) >= 0.3:
        print(f"{name:40}{e:9.2f}{f:9.2f}")
keys = ["macro_expand_crate", "resolve_crate", "link", "total"]
summary = ", ".join(f"{k} {med(empty, k):.1f}/{med(fixture, k):.1f}" for k in keys)
print(f"::notice title=E4 rustc passes (empty/fixture ms)::{summary}")
PY

echo "== E1: metadata read in the guest"
RUSTUP_HOME="$TOOL/rustup" CARGO_HOME="$TOOL/cargo" \
    RUSTUP_TOOLCHAIN="$(python3 "$REPO/compiler/scripts/upstream.py" get rust-stable version)" \
    "$TOOL/cargo/bin/rustc" --edition 2021 -O --target wasm32-wasip1 \
    "$REPO/docs/notes/warm-session-e1-probe.rs" -o "$OUT/probe.wasm"
LIB="$SYSROOT/lib/rustlib/wasm32-unknown-unknown/lib"
CORE=$(ls "$LIB"/libcore-*.rmeta)
CB=$(ls "$LIB"/libcompiler_builtins-*.rmeta)
for i in 1 2 3; do
    "$HOST" raw --rustc "$OUT/probe.wasm" --sysroot "$SYSROOT" --linker "$WILD" -- 2 "$CORE" "$CB" 2>&1 |
        grep -E "^round [0-9]+ total" | sed "s/^/instance $i: /"
done | tee "$OUT/e1.txt"
python3 - "$OUT/e1.txt" <<'PY'
import re, statistics, sys
first, again = [], []
for line in open(sys.argv[1]):
    m = re.search(r"round (\d+) total ([0-9.]+) ms", line)
    if m:
        (first if m.group(1) == "0" else again).append(float(m.group(2)))
print(f"::notice title=E1 rmeta read in the guest::fresh instance {statistics.median(first):.1f} ms, memory already grown {statistics.median(again):.1f} ms (core + compiler_builtins)")
PY

echo "== E2: restoring ~143 MiB of guest memory (host memmove)"
python3 - <<'PY'
import statistics, time
size = 149_422_080  # median peak guest memory of a small compile (bench history)
src = bytearray(b"\x5a") * size
dst = bytearray(size)
dst[:] = src  # touch every page of the destination once
resident, fresh = [], []
for _ in range(7):
    t = time.perf_counter(); dst[:] = src; resident.append((time.perf_counter() - t) * 1e3)
    t = time.perf_counter(); copy = bytes(src); fresh.append((time.perf_counter() - t) * 1e3); del copy
r, f = statistics.median(resident), statistics.median(fresh)
print(f"resident destination: {r:.1f} ms; fresh allocation: {f:.1f} ms (median of 7, {size/2**20:.1f} MiB)")
print(f"::notice title=E2 snapshot restore cost::{size/2**20:.0f} MiB memmove: {r:.1f} ms into resident memory, {f:.1f} ms into fresh memory")
PY
