#!/usr/bin/env bash
# Edit -> result for plugin-sized sources through the resident supervisor
# (G2.3 option A, docs/notes/warm-session.md): the four bench/programs/medium
# programs (309-562 lines plus the shared prelude, all using alloc), each
# measured with one serving rustc guest and with one guest per job, in the
# order serve, job, job, serve so drift cannot favour either. Every edit is
# also compiled in a fresh guest and must be byte-identical (--verify-cold).
#
# Prints a table and one ::notice per source. Fails only when edit-loop's
# correctness checks fail; the timings are a measurement, not a gate.
#
# Expects the toolchain built by ci/build-and-test.sh (rustc.wasm, sysroot,
# Wild, the precompiled .cwasm and the cogwasm-supervisor release binary).
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TOOL="$REPO/toolchain"
OUT="$TOOL/work/supervisor/sources"
SUPERVISOR="$TOOL/target/cogwasm-host/release/cogwasm-supervisor"
EDITS=${EDIT_LOOP_SOURCES_EDITS:-5}
SOURCES=${EDIT_LOOP_SOURCES:-json scene expr collections}
rm -rf "$OUT"
mkdir -p "$OUT"

# bench/run.py's prepare_source, minus the harness: an absolute prelude
# include (edit-loop copies the source elsewhere) and the benchmark edit probe.
prepare() { # name
    python3 - "$REPO/bench/programs/medium/$1.rs" "$REPO/bench/programs/prelude.rs" "$OUT/$1.rs" <<'PY'
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
}

run_one() { # mode name slot
    local mode=$1 name=$2 slot=$3 serve=1
    [ "$mode" = job ] && serve=0
    COGWASM_SERVE=$serve "$SUPERVISOR" edit-loop \
        --backend resident \
        --host "$TOOL/target/cogwasm-host/release/cogwasm-host" \
        --rustc "$TOOL/artifacts/rustc.wasm" \
        --sysroot "$TOOL/sysroot" \
        --linker "$TOOL/target/wild/release/wild" \
        --precompiled "$TOOL/work/precompiled/rustc.cwasm" \
        --cache-dir "$TOOL/cache" \
        --work "$OUT/work" \
        --store "$OUT/store-$name-$slot-$mode" \
        --compiler-id "sources-$name-$slot-$mode" \
        --source "$OUT/$name.rs" \
        --edits "$EDITS" \
        --verify-cold \
        --json "$OUT/$name-$slot-$mode.json" >"$OUT/$name-$slot-$mode.log" 2>&1 || {
            cat "$OUT/$name-$slot-$mode.log" >&2
            exit 1
        }
}

for name in $SOURCES; do
    prepare "$name"
    slot=0
    for mode in serve job job serve; do
        slot=$((slot + 1))
        run_one "$mode" "$name" "$slot"
    done
done

python3 - "$OUT" <<'PY'
import glob, json, os, statistics, sys
out = sys.argv[1]
rows = {}
for path in sorted(glob.glob(os.path.join(out, "*.json"))):
    name, slot, mode = os.path.basename(path)[:-5].split("-", 2)
    r = json.load(open(path))
    rows.setdefault(name, []).append((int(slot), mode, r["median"], r["min"], r["max"]))
print(f"{'source':12} {'#':>2} {'mode':6} {'median':>8} {'min':>8} {'max':>8}")
for name, entries in rows.items():
    by_mode = {}
    for slot, mode, median, lo, hi in sorted(entries):
        print(f"{name:12} {slot:>2} {mode:6} {median:8.1f} {lo:8.1f} {hi:8.1f}")
        by_mode.setdefault(mode, []).append(median)
    serve = statistics.median(by_mode["serve"])
    job = statistics.median(by_mode["job"])
    print(f"::notice title=edit to result, {name}::serving {serve:.1f} ms, one guest per job {job:.1f} ms ({serve - job:+.1f} ms, {(serve - job) / job * 100:+.0f}%), byte-identical to cold compiles")
PY
