#!/usr/bin/env bash
# One entry point for the pure-Rust cranelift-wasm CI (and for local runs on
# macOS/Linux):
#
#   1. fetch the pinned toolchain inputs (rustup, rust + prior-art patches,
#      clif-to-wasm, Wild, wasi-sdk)
#   2. build the CogWasm rustc.wasm (wasm32-wasip1, cg_clif only, single-threaded)
#   3. build the Wasmtime host from compiler/upstreams.toml (the sysroot
#      builder executes rustc.wasm)
#   4. build core/alloc/compiler_builtins with rustc.wasm itself (strict)
#   5. run plugin (c) end to end and assert eval(1,2,3,4) == 2.25
#   6. build plugins (a)-(f) and assert A.2 (e)/(f) correctness
#   7. incremental compilation inside the resident rustc.wasm, and the
#      precompiled .cwasm load + mismatch/truncation fallback
#   8. differential test LLVM rustc vs the Cranelift->wasm pipeline
#   9. audit rustc.wasm for LLVM imports/symbols
#
# Logs are teed by the workflow through ci/build-with-diagnostics.sh; the
# individual build scripts also keep their own logs under docs/results/.
#
#   --reuse-compiler   skip steps 1, 2 and 4 and use the compiler that is
#                      already under toolchain/. CI sets this only when its
#                      cache key - the patch series plus the scripts that pin
#                      the rust commit, the nightly and wasi-sdk - proves the
#                      restored compiler was built from the current inputs.
#                      Steps 3 and 5-9 still run, so a reuse run tests the
#                      restored compiler exactly like a freshly built one.
set -euo pipefail
CI_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO=$(cd "$CI_DIR/.." && pwd)
UPSTREAM="$REPO/compiler/scripts/upstream.py"
WASMTIME=$(python3 "$UPSTREAM" get wasmtime version)
TOOL="$REPO/toolchain"
RUSTC_WASM="$TOOL/artifacts/rustc.wasm"
SYSROOT="$TOOL/sysroot"
HOST_BIN="$TOOL/target/cogwasm-host/release/cogwasm-host"
STAGE1="$TOOL/rust/build/host/stage1/bin/rustc"
WILD="$TOOL/target/wild/release/wild"
START=$(date +%s)
CURRENT_STEP=setup

REUSE_COMPILER=${CI_REUSE_COMPILER:-false}
while [ $# -gt 0 ]; do
    case "$1" in
        --reuse-compiler) REUSE_COMPILER=true ;;
        -h|--help)
            printf 'usage: %s [--reuse-compiler]\n' "$(basename "$0")" >&2
            exit 0
            ;;
        *)
            echo "unknown argument: $1 (try --help)" >&2
            exit 2
            ;;
    esac
    shift
done

step() {
    CURRENT_STEP=$1
    printf '\n== [%s] %s\n' "$(date -u +%H:%M:%S)" "$1" >&2
}
elapsed() { echo $(( $(date +%s) - START )); }
free_gb() {
    df -Pk "$1" | awk 'NR==2 {printf "%d", $4/1024/1024}'
}
sha256_file() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}
on_err() {
    local status=$?
    echo "::error title=cranelift-wasm CI failed::step '${CURRENT_STEP}' exited ${status} after $(elapsed)s; the full log is published to the ci-logs release" >&2
    exit "$status"
}
trap on_err ERR

JOBS=${BUILD_JOBS:-$(
    if command -v nproc >/dev/null 2>&1; then nproc; else sysctl -n hw.ncpu; fi
)}
export BUILD_JOBS="$JOBS"
export CARGO_BUILD_JOBS="$JOBS"
export CARGO_TERM_COLOR=always

if [ "$(free_gb "$REPO")" -lt 15 ]; then
    echo "::error title=not enough disk::only $(free_gb "$REPO") GiB free under $REPO; stopping before the build" >&2
    exit 1
fi
echo "budget: $(free_gb "$REPO") GiB free, $JOBS build jobs, $(uname -s) $(uname -m)"

# Cheap manifest sanity check before any checkout/build: the patch lists in
# compiler/upstreams.toml must match compiler/patches/, the root Cargo.toml
# Wasmtime pins must match the manifest, and every differing PR bundle must
# have a DIVERGES.md. A failure here means the toolchain setup would apply a
# different series than the manifest.
step "upstream manifest"
python3 "$REPO/compiler/scripts/check-upstreams.py"

if [ "$REUSE_COMPILER" = true ]; then
    step "reuse the cached compiler build"
    # Fail loudly instead of rebuilding: a partial restore means the cache key
    # and the checkout disagree, which is a bug worth seeing, not papering over
    # with a 30-minute rebuild that hides it.
    for cached in "$RUSTC_WASM" "$SYSROOT" "$STAGE1"; do
        [ -e "$cached" ] || {
            echo "::error title=compiler cache incomplete::asked to reuse the compiler build but $cached is missing" >&2
            exit 1
        }
    done
    # The Wild binary is cached with the compiler because setup-toolchain.sh
    # builds it from the same pinned sources. If only it is missing, the cheap
    # fix is a setup run rather than failing a run that can still be correct.
    # The isolated rustup/cargo are cached separately and saved only when a run
    # succeeds, so a compiler cache hit can come with no toolchain (the compiler
    # cache is saved right after the build; a later step can still fail). Later
    # steps call $TOOL/cargo/bin/cargo directly, so set up in that case too.
    if [ ! -x "$WILD" ] || [ ! -x "$TOOL/cargo/bin/cargo" ]; then
        step "Wild or the isolated toolchain missing from the caches; running setup"
        "$CI_DIR/setup-toolchain.sh"
    fi
    mkdir -p "$TOOL/cache"
    echo "::notice title=cranelift-wasm build reused::rustc.wasm, pure sysroot, stage1 and Wild restored from a cache keyed on the patch series; steps 3 and 5-9 still run" >&2
else
    step "setup pinned toolchain and sources"
    "$CI_DIR/setup-toolchain.sh"

    step "build the CogWasm rustc.wasm (wasm32-wasip1, cranelift-only)"
    "$REPO/compiler/scripts/build-rustc-wasm.sh"
fi

step "build Wasmtime $WASMTIME host"
"$REPO/compiler/scripts/build-host.sh"

if [ "$REUSE_COMPILER" != true ]; then
    step "build pure sysroot with rustc.wasm (strict)"
    "$REPO/compiler/scripts/build-pure-sysroot.sh"
fi

step "plugin (c): compile, link, eval"
"$REPO/compiler/run.sh" check

step "build plugins (a), (b), (c), (e), (f) with rustc.wasm"
"$REPO/compiler/run.sh" plugins

step "A.2 (e)/(f) correctness assertions"
"$REPO/compiler/run.sh" a2check

# Incremental compilation must work when the host keeps rustc.wasm resident,
# and a precompiled artifact must load or fall back safely. Both are opt-in
# host paths, so CI exercises them explicitly.
step "incremental compilation in resident rustc.wasm"
"$REPO/compiler/run.sh" incremental
"$REPO/difftest/incremental-check.sh"

step "precompiled rustc.wasm (.cwasm) load and fallback"
"$REPO/compiler/run.sh" precompiled-check

# The supervisor's resident backend keeps one rustc.wasm loaded across jobs
# (G2.1). Its test needs the real toolchain, so quality.yml always skips it;
# run it here, after precompiled-check has left a .cwasm to load, and make a
# missing toolchain a failure rather than a skip.
step "resident compiler backend (supervisor)"
(
    export RUSTUP_HOME="$TOOL/rustup" CARGO_HOME="$TOOL/cargo"
    export CARGO_TARGET_DIR="$TOOL/target/cogwasm-host"
    # Same stable toolchain build-host.sh uses, so the host crates are reused.
    export RUSTUP_TOOLCHAIN="$(python3 "$UPSTREAM" get rust-stable version)"
    COGWASM_REQUIRE_TOOLCHAIN=1 "$TOOL/cargo/bin/cargo" test --release \
        --manifest-path "$REPO/crates/cogwasm-supervisor/Cargo.toml" \
        --test resident -- --nocapture
)

# Measure edit -> result through the resident supervisor (G2.1 step 4). The
# fixture is copied into the work dir, so the committed source is never edited;
# the copy path is unique per run, so every edit is a real cache miss. This is
# a measurement, not a gate: it fails only on a correctness failure.
step "edit → result through the resident supervisor"
(
    export RUSTUP_HOME="$TOOL/rustup" CARGO_HOME="$TOOL/cargo"
    export CARGO_TARGET_DIR="$TOOL/target/cogwasm-host"
    export RUSTUP_TOOLCHAIN="$(python3 "$UPSTREAM" get rust-stable version)"
    COGWASM_REQUIRE_TOOLCHAIN=1 "$TOOL/cargo/bin/cargo" run --release \
        --manifest-path "$REPO/crates/cogwasm-supervisor/Cargo.toml" \
        --bin cogwasm-supervisor -- edit-loop \
        --backend resident \
        --host "$HOST_BIN" \
        --rustc "$RUSTC_WASM" \
        --sysroot "$SYSROOT" \
        --linker "$WILD" \
        --precompiled "$TOOL/work/precompiled/rustc.cwasm" \
        --cache-dir "$TOOL/cache" \
        --work "$TOOL/work/supervisor" \
        --store "$TOOL/work/supervisor/store" \
        --compiler-id "resident-ci" \
        --source "$REPO/crates/cogwasm-supervisor/tests/fixtures/edit_loop.rs" \
        --edits 10 \
        --verify-cold \
        --json "$TOOL/work/supervisor/edit-loop.json"
)
python3 -c 'import json,sys; r=json.load(open(sys.argv[1])); load=r["module_load_ms"]; print("::notice title=edit to result::median %.1f ms (target %.0f ms: %s), module load %s ms" % (r["median"], r["target_ms"], "MET" if r["met"] else "MISSED", "unknown" if load is None else "%.1f" % load))' "$TOOL/work/supervisor/edit-loop.json"


# Plugin-sized sources (the four bench/programs/medium programs) through the
# same resident supervisor: serving vs one guest per job, every edit checked
# byte-identical to a cold compile. Larger programs exercise more of the
# compiler's per-thread state than the fixture above.
step "edit -> result, plugin-sized sources (serving vs per job)"
"$CI_DIR/edit-loop-sources.sh"

# Differential test: the same 48 no_std+alloc corpus programs plus 24 seeded
# random programs are compiled by LLVM rustc and by rustc.wasm+cg_clif, run in
# Wasmtime, and compared bit-for-bit (u64 return + output buffer). A trap in
# both is a match only for corpus programs listed in corpus/expect_trap.txt.
# This runs unconditionally and fails the job on any mismatch; full per-program
# details go to stdout (and to the published log).
step "differential test: LLVM rustc vs Cranelift->wasm"
"$REPO/difftest/run.sh" --seeds 24

step "no-LLVM audit"
mkdir -p "$TOOL/work/compiler"
"$HOST_BIN" inspect --wasm "$RUSTC_WASM" > "$TOOL/work/compiler/rustc-inspect.json"
python3 "$CI_DIR/audit-llvm.py" "$TOOL/work/compiler/rustc-inspect.json" "$RUSTC_WASM"

step "sizes and checksums"
RUSTC_BYTES=$(wc -c < "$RUSTC_WASM" | tr -d ' ')
RUSTC_SHA=$(sha256_file "$RUSTC_WASM")
SYS_BYTES=$(du -sk "$SYSROOT" | cut -f1)
PLUGIN_BYTES=$(wc -c < "$TOOL/work/compiler/c_plugin.wasm" | tr -d ' ')
echo "rustc.wasm: ${RUSTC_BYTES} bytes, sha256 ${RUSTC_SHA}"
echo "sysroot-pure: ${SYS_BYTES} KiB"
echo "plugin (c): ${PLUGIN_BYTES} bytes"
echo "toolchain dir: $(du -sh "$TOOL" | cut -f1); free disk: $(free_gb "$REPO") GiB"

trap - ERR
TOTAL=$(elapsed)
echo "::notice title=cranelift-wasm CI passed::rustc.wasm ${RUSTC_BYTES} B sha256 ${RUSTC_SHA:0:16}...; plugin (c) eval(1,2,3,4)=2.25; A.2 (e)/(f) checks passed; no LLVM inside rustc.wasm; ${TOTAL}s total; $(free_gb "$REPO") GiB free"
