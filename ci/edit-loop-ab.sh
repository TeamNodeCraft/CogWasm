#!/usr/bin/env bash
# G2.3 experiment A0, confirmation run (docs/notes/warm-session.md): compare
# Wasmtime's default and pooling instance allocators for the resident
# supervisor's edit -> result, on the CI runner, within one job.
#
# Two sources (the small edit-loop fixture and bench/programs/medium/json.rs,
# a plugin-sized program that also loads liballoc) times six interleaved
# configurations, so order and runner drift cannot favour one allocator:
#
#   pool1024, default, pool0, pool256, default, pool1024
#
# Prints a table and one ::notice per source. A measurement, not a gate: it
# fails only when edit-loop's correctness checks fail. CI ran it for the A0
# decision (run 44972190861); it is kept for manual reruns, not run by CI.
#
# Expects the toolchain built by ci/build-and-test.sh (rustc.wasm, sysroot,
# Wild, the precompiled .cwasm and the cogwasm-supervisor release binary).
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TOOL="$REPO/toolchain"
OUT="$TOOL/work/supervisor/ab"
SUPERVISOR="$TOOL/target/cogwasm-host/release/cogwasm-supervisor"
EDITS=${EDIT_LOOP_AB_EDITS:-10}
rm -rf "$OUT"
mkdir -p "$OUT"

# bench/run.py's prepare_source, minus the harness: absolute prelude include
# (edit-loop copies the source elsewhere) and the benchmark edit probe.
python3 - "$REPO/bench/programs/medium/json.rs" "$REPO/bench/programs/prelude.rs" "$OUT/json.rs" <<'PY'
import re, sys
src, prelude, dest = sys.argv[1:]
text = open(src).read()
text, n = re.subn(r'include!\("[^"]*prelude\.rs"\);', f'include!("{prelude}");', text)
if n != 1:
    sys.exit(f"{src}: expected one prelude include, found {n}")
if "BENCH_EDIT_TWEAK" not in text:
    text += (
        "\nconst BENCH_EDIT_TWEAK: u64 = 0u64;\n"
        "#[unsafe(no_mangle)]\n"
        "pub extern \"C\" fn bench_edit_probe() -> u64 { BENCH_EDIT_TWEAK }\n"
    )
open(dest, "w").write(text)
PY

run_one() { # label source-name source-path
    local label=$1 name=$2 source=$3 slot=$4
    local -a pooling=(env COGWASM_POOLING=0 -u COGWASM_POOLING_KEEP_RESIDENT_MB)
    case "$label" in
        pool*) pooling=(env COGWASM_POOLING=1 "COGWASM_POOLING_KEEP_RESIDENT_MB=${label#pool}") ;;
    esac
    "${pooling[@]}" "$SUPERVISOR" edit-loop \
        --backend resident \
        --host "$TOOL/target/cogwasm-host/release/cogwasm-host" \
        --rustc "$TOOL/artifacts/rustc.wasm" \
        --sysroot "$TOOL/sysroot" \
        --linker "$TOOL/target/wild/release/wild" \
        --precompiled "$TOOL/work/precompiled/rustc.cwasm" \
        --cache-dir "$TOOL/cache" \
        --work "$OUT/work" \
        --store "$OUT/store-$name-$slot-$label" \
        --compiler-id "ab-$name-$slot-$label" \
        --source "$source" \
        --edits "$EDITS" \
        --json "$OUT/$name-$slot-$label.json" >"$OUT/$name-$slot-$label.log" 2>&1 || {
            cat "$OUT/$name-$slot-$label.log" >&2
            exit 1
        }
}

for spec in "fixture:$REPO/crates/cogwasm-supervisor/tests/fixtures/edit_loop.rs" "json:$OUT/json.rs"; do
    name=${spec%%:*}
    source=${spec#*:}
    slot=0
    for label in pool1024 default pool0 pool256 default pool1024; do
        slot=$((slot + 1))
        run_one "$label" "$name" "$source" "$slot"
    done
done

python3 - "$OUT" <<'PY'
import glob, json, os, statistics, sys
out = sys.argv[1]
rows = {}
for path in sorted(glob.glob(os.path.join(out, "*.json"))):
    name, slot, label = os.path.basename(path)[:-5].split("-", 2)
    r = json.load(open(path))
    rows.setdefault(name, []).append((int(slot), label, r["median"], r["min"], r["max"]))
print(f"{'source':8} {'#':>2} {'allocator':9} {'median':>8} {'min':>8} {'max':>8}")
for name, entries in rows.items():
    by_label = {}
    for slot, label, median, lo, hi in sorted(entries):
        print(f"{name:8} {slot:>2} {label:9} {median:8.1f} {lo:8.1f} {hi:8.1f}")
        by_label.setdefault(label, []).append(median)
    summary = ", ".join(
        f"{label} {statistics.median(v):.1f}" for label, v in sorted(by_label.items())
    )
    default = statistics.median(by_label["default"])
    best_label, best = min(
        ((label, statistics.median(v)) for label, v in by_label.items()), key=lambda x: x[1]
    )
    change = (best - default) / default * 100
    print(f"::notice title=A0 allocator comparison ({name})::median ms by allocator: {summary}; best {best_label} {change:+.0f}% vs default")
PY
