#!/usr/bin/env bash
# CogWasm compiler driver: rustc.wasm on wasm32-wasip1 (no wasi-threads), pure
# rustc.wasm-built plugin sysroot, growable plugin heap, latest Wasmtime.
#
#   ./run.sh build      build the Wasmtime 50 host (crates/cogwasm-host)
#   ./run.sh check      compile + link + eval plugin (c) against the pure sysroot
#   ./run.sh plugins    build (a),(b),(c),(e),(f) with rustc.wasm (strict)
#   ./run.sh a2check    A.2 (e)/(f) correctness assertions (no timing checks)
#   ./run.sh bench      stage-B comparison table (history) on latest Wasmtime
#   ./run.sh a2         A.2 runtime/load measurements for (e),(f)
#   ./run.sh incremental  cold vs -C incremental, one-line edits, (c) and (e)
#   ./run.sh precompiled-check  .cwasm load + mismatch/truncation fallback
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
UPSTREAM="$REPO/compiler/scripts/upstream.py"
TOOL="$REPO/toolchain"
RUSTC_WASM="$TOOL/artifacts/rustc.wasm"
PURE="$TOOL/sysroot"
HOST="$TOOL/target/cogwasm-host/release/cogwasm-host"
WILD="$TOOL/target/wild/release/wild"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
NIGHTLY=$(python3 "$UPSTREAM" get rust-nightly version)
export RUSTUP_TOOLCHAIN="$NIGHTLY"
STAGE1="${COGWASM_STAGE1:-${STAGE_C_STAGE1:-$TOOL/rust/build/host/stage1/bin/rustc}}"
if [ -x "$STAGE1" ]; then
    HOST_TRIPLE=$("$STAGE1" -vV | sed -n 's/^host: //p')
else
    HOST_TRIPLE=
fi
# rust-objcopy for libm's build script (and compiler_builtins in the sysroot).
if [ -n "$HOST_TRIPLE" ]; then
    export PATH="$RUSTUP_HOME/toolchains/$NIGHTLY-$HOST_TRIPLE/lib/rustlib/$HOST_TRIPLE/bin:$CARGO_HOME/bin:$PATH"
else
    export PATH="$CARGO_HOME/bin:$PATH"
fi
export COGWASM_HOST_TRIPLE="$HOST_TRIPLE"
export RUSTC="$REPO/compiler/scripts/rustc-wasm-wrapper.sh"
export COGWASM_SYSROOT="$PURE"
export COGWASM_RUSTC_WASM="$RUSTC_WASM"
export COGWASM_HOST="$HOST"
export COGWASM_WILD="$WILD"
export CG_CLIF_WASM_STRICT=1
export CARGO_TARGET_DIR="$TOOL/target/pure-wasm"

need() { [ -e "$1" ] || { echo "missing $1" >&2; exit 1; }; }

case "${1:-check}" in
    build)
        "$REPO/compiler/scripts/build-host.sh"
        ;;
    plugins)
        need "$RUSTC_WASM"; need "$PURE"
        for p in a_add b_alloc c_plugin e_math f_geometry; do
            echo "== $p (rustc.wasm, strict)"
            ( cd "$REPO/guests/$p" && RUSTFLAGS="-Cpanic=abort" \
                cargo build --release --target wasm32-unknown-unknown )
        done
        ;;
    check)
        need "$RUSTC_WASM"; need "$PURE"; need "$HOST"
        mkdir -p "$TOOL/work/compiler"
        "$HOST" check --rustc "$RUSTC_WASM" --sysroot "$PURE" \
            --src "$REPO/guests/c_plugin/src/lib.rs" --linker "$WILD" \
            --work "$TOOL/work/compiler" --out "$TOOL/work/compiler/c_plugin.wasm" \
            --cache-dir "$TOOL/cache"
        ;;
    a2check)
        need "$PURE"
        H="$TOOL/target/a2/harness/release/a2-harness"
        ( cd "$REPO/guests/a2_harness" && \
            CARGO_TARGET_DIR="$TOOL/target/a2/harness" cargo build --release )
        P="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release"
        need "$P/e_math.wasm"; need "$P/f_geometry.wasm"
        "$H" check "$P/e_math.wasm" "$P/f_geometry.wasm"
        ;;
    bench)
        need "$RUSTC_WASM"; need "$PURE"; need "$HOST"
        mkdir -p "$TOOL/work/bench" "$REPO/docs/results"
        "$HOST" bench --rustc "$RUSTC_WASM" --sysroot "$PURE" \
            --src "$REPO/guests/c_plugin/src/lib.rs" --linker "$WILD" \
            --work "$TOOL/work/bench" --out "$TOOL/work/bench/c_plugin.wasm" \
            --cache-dir "$TOOL/cache" --runs 5 \
            --json "$REPO/docs/results/stage-c.json"
        ;;
    incremental)
        need "$RUSTC_WASM"; need "$PURE"; need "$HOST"; need "$WILD"
        mkdir -p "$TOOL/work/incremental" "$REPO/docs/results"
        # e_math uses the `libm` crate. A direct rustc.wasm invocation (which is
        # what measures one edit in the resident host) needs the rlib that
        # cargo builds, so build the program once through the usual wrapper
        # first, then pass that exact rlib to the host.
        echo "== building e_math and its libm rlib with rustc.wasm"
        # Touch the source so cargo prints the e_math rustc invocation even when
        # the previous build is up to date: the verbose log is where the exact
        # `--extern libm=...` rlib that cargo selected can be read without
        # guessing between stale build directories.
        touch "$REPO/guests/e_math/src/lib.rs"
        BUILD_LOG="$TOOL/work/incremental/e-build.log"
        mkdir -p "$TOOL/work/incremental"
        ( cd "$REPO/guests/e_math" && RUSTFLAGS="-Cpanic=abort" \
            cargo build --release --target wasm32-unknown-unknown -v ) 2>&1 | tee "$BUILD_LOG"
        # Cargo passes both the rlib and its rmeta: the rlib contains the object
        # files but only a metadata stub, and the rmeta carries the full
        # metadata. rustc needs both (passing only the rlib fails with "only
        # metadata stub found").
        LIBM_RLIB=$(sed -n 's/.*--extern libm=\([^ ]*\.rlib\).*/\1/p' "$BUILD_LOG" | tail -n 1)
        LIBM_RMETA=$(sed -n 's/.*--extern libm=\([^ ]*\.rmeta\).*/\1/p' "$BUILD_LOG" | tail -n 1)
        [ -n "$LIBM_RLIB" ] && [ -n "$LIBM_RMETA" ] || {
            echo "could not read the libm rlib/rmeta paths from $BUILD_LOG" >&2; exit 1; }
        # rustc.wasm sees guest `/` as the host root, so the extern paths have to
        # be absolute; cargo sometimes prints them relative to the package.
        for libm_file in "$LIBM_RLIB" "$LIBM_RMETA"; do
            case "$libm_file" in
                /*) ;;
                *) echo "libm path is not absolute: $libm_file" >&2; exit 1 ;;
            esac
            [ -f "$libm_file" ] || { echo "libm artifact does not exist: $libm_file" >&2; exit 1; }
        done

        # Each program starts from a fresh incremental directory (explicitly
        # requested with --fresh-incremental) and uses the same resident host
        # path as bench. The host exits nonzero unless the incremental result
        # matches the cold build: plugin (c) eval == 2.25, e_math mean/min/max
        # against the same reference check as the A.2 harness.
        run_incremental() {
            local spec=$1 tag=$2
            shift 2
            echo "== incremental $spec"
            "$HOST" incremental --rustc "$RUSTC_WASM" --sysroot "$PURE" \
                --src "$REPO/guests/$spec/src/lib.rs" --linker "$WILD" \
                --work "$TOOL/work/incremental/$tag" \
                --out "$TOOL/work/incremental/$tag/$spec.wasm" \
                --cache-dir "$TOOL/cache" \
                --incremental "$TOOL/work/incremental/$tag/inc" \
                --fresh-incremental --runs "${INCREMENTAL_RUNS:-5}" "$@" \
                --json "$REPO/docs/results/incremental-$tag.json"
        }
        run_incremental c_plugin c
        run_incremental e_math e \
            --extern "libm=$LIBM_RLIB" --extern "libm=$LIBM_RMETA"
        ;;
    precompiled-check)
        # Build a .cwasm for this host, load it, then prove that a mismatched
        # sidecar and a truncated artifact both fall back to the .wasm. This is
        # the regression check for the precompiled path (Track A).
        need "$RUSTC_WASM"; need "$PURE"; need "$HOST"; need "$WILD"
        PRE="$TOOL/work/precompiled"
        PC="$TOOL/target/cogwasm-host/release/cogwasm-precompile"
        need "$PC"
        mkdir -p "$PRE"
        "$PC" precompile --wasm "$RUSTC_WASM" \
            --out "$PRE/rustc.cwasm" --metadata "$PRE/rustc.cwasm.json"
        common=(--rustc "$RUSTC_WASM" --sysroot "$PURE" \
            --src "$REPO/guests/c_plugin/src/lib.rs" --linker "$WILD" \
            --work "$PRE" --out "$PRE/c_plugin.wasm" \
            --cache-dir "$TOOL/cache")
        echo "== precompiled .cwasm load"
        "$HOST" check "${common[@]}" --precompiled "$PRE/rustc.cwasm"
        echo "== mismatched sidecar falls back"
        cp "$PRE/rustc.cwasm" "$PRE/mismatch.cwasm"
        python3 - "$PRE/rustc.cwasm.json" "$PRE/mismatch.cwasm.json" <<'PYEOF'
import json
import sys
metadata = json.load(open(sys.argv[1], encoding="utf-8"))
metadata["compatibility_hash"] = "00" * 32
json.dump(metadata, open(sys.argv[2], "w", encoding="utf-8"), indent=2)
PYEOF
        "$HOST" check "${common[@]}" --precompiled "$PRE/mismatch.cwasm" \
            2>&1 | tee "$PRE/mismatch.log"
        grep -q "falling back to compiling" "$PRE/mismatch.log"
        echo "== truncated .cwasm falls back"
        head -c 4096 "$PRE/rustc.cwasm" > "$PRE/truncated.cwasm"
        cp "$PRE/rustc.cwasm.json" "$PRE/truncated.cwasm.json"
        "$HOST" check "${common[@]}" --precompiled "$PRE/truncated.cwasm" \
            2>&1 | tee "$PRE/truncated.log"
        grep -q "falling back to compiling" "$PRE/truncated.log"
        echo "PASS precompiled-check: .cwasm loads; mismatched and truncated artifacts fall back"
        ;;
    a2)
        need "$PURE"
        H="$TOOL/target/a2/harness/release/a2-harness"
        ( cd "$REPO/guests/a2_harness" && \
            CARGO_TARGET_DIR="$TOOL/target/a2/harness" cargo build --release )
        P="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release"
        for p in e_math f_geometry; do
            "$H" inspect "$P/$p.wasm"
            "$H" load "$P/$p.wasm"
        done
        "$H" bench_math "$P/e_math.wasm" 100000 5
        "$H" bench_geom "$P/f_geometry.wasm" 40000 5
        ;;
    *)
        echo "usage: $0 [build|plugins|check|a2check|bench|a2|incremental|precompiled-check]" >&2
        exit 1
        ;;
esac
