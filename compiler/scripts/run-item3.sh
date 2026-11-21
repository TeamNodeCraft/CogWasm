#!/usr/bin/env bash
# CogWasm compiler, item 3: wasm memory.grow / memory.size support.
#
#   ./run-item3.sh build     rebuild the native cg_clif dylib with the item-3 patch
#   ./run-item3.sh test      run clif-to-wasm's tests (including tests/memory.rs)
#   ./run-item3.sh harness   build the two native Wasmtime harnesses
#   ./run-item3.sh programs  build mem_grow and c_plugin with rust-lld and Wild
#   ./run-item3.sh run       run both programs from both linkers
#   ./run-item3.sh all       build + test + harness + programs + run
#
# Everything is built in toolchain/target (gitignored). The only source
# changes are the patches under compiler/patches; the toolchain sources are left
# with the patches applied.
set -euo pipefail

REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
TOOL="$REPO/toolchain"
NIGHTLY=nightly-2026-10-19
HOST_TOOLCHAIN=1.99.1

export RUSTUP_HOME="$TOOL/rustup"
export CARGO_HOME="$TOOL/cargo"
export PATH="$CARGO_HOME/bin:$PATH"

DYLIB="$TOOL/target/cg_clif/release/librustc_codegen_cranelift.dylib"
WILD="$TOOL/target/wild/release/wild"
HOST="$TOOL/target/host/release/host"
HARNESS="$TOOL/target/harness/release"
RUST_LLD="$TOOL/rustup/toolchains/$NIGHTLY-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/rust-lld"

lld_target="$TOOL/target/lld-plugins"
wild_target="$TOOL/target/wild-plugins"

log() { printf '\n== %s\n' "$*" >&2; }

build_dylib() {
    log "cg_clif (patched) -> $DYLIB"
    ( cd "$TOOL/cg_clif" && RUSTUP_TOOLCHAIN="$NIGHTLY" \
        CARGO_TARGET_DIR="$TOOL/target/cg_clif" cargo build --release )
}

test_clif() {
    log "clif-to-wasm tests"
    ( cd "$TOOL/clif-to-wasm" && RUSTUP_TOOLCHAIN="$NIGHTLY" \
        CARGO_TARGET_DIR="$TOOL/target/clif-to-wasm" RUST_LLD="$RUST_LLD" \
        cargo test --features module --test memory --test clif --test object )
}

build_harnesses() {
    for prog in mem_grow_harness c_plugin_harness; do
        log "harness $prog"
        ( cd "$REPO/guests/$prog" && RUSTUP_TOOLCHAIN="$HOST_TOOLCHAIN" \
            CARGO_TARGET_DIR="$TOOL/target/harness" cargo build --release )
    done
}

build_program() { # program, target dir, linker ("lld" or "wild")
    local prog=$1 target=$2 linker=$3
    local flags="-Zcodegen-backend=$DYLIB -Cpanic=abort"
    if [ "$linker" = wild ]; then
        flags="$flags -Clinker=$WILD -Clinker-flavor=wasm-ld"
    fi
    log "$prog with $linker"
    ( cd "$REPO/guests/$prog" && CG_CLIF_WASM_STRICT=1 RUSTFLAGS="$flags" \
        RUSTUP_TOOLCHAIN="$NIGHTLY" CARGO_TARGET_DIR="$target" \
        cargo build --release --target wasm32-unknown-unknown \
            -Zbuild-std=core,alloc,compiler_builtins )
}

build_programs() {
    for linker in lld wild; do
        if [ "$linker" = wild ]; then target="$wild_target"; else target="$lld_target"; fi
        build_program mem_grow "$target" "$linker"
        build_program c_plugin "$target" "$linker"
    done
}

run_all() {
    for linker in lld wild; do
        if [ "$linker" = wild ]; then target="$wild_target"; else target="$lld_target"; fi
        local mem="$target/wasm32-unknown-unknown/release/mem_grow.wasm"
        local plugin="$target/wasm32-unknown-unknown/release/c_plugin.wasm"
        log "mem_grow ($linker, $(wc -c < "$mem" | tr -d ' ') bytes)"
        "$HARNESS/mem_grow_harness" "$mem"
        log "c_plugin ($linker, $(wc -c < "$plugin" | tr -d ' ') bytes)"
        "$HARNESS/c_plugin_harness" "$plugin" 1 2 3 4
        "$HOST" plugin "$plugin" 1 2 3 4
        "$HOST" inspect "$plugin" | sed -n '1p'
    done
}

case "${1:-all}" in
    build) build_dylib ;;
    test) test_clif ;;
    harness) build_harnesses ;;
    programs) build_programs ;;
    run) run_all ;;
    all) build_dylib; test_clif; build_harnesses; build_programs; run_all ;;
    *) echo "usage: $0 [build|test|harness|programs|run|all]" >&2; exit 1 ;;
esac
