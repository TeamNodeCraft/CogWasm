#!/usr/bin/env bash
# Stage A spike driver. All downloads are from official project repos / static.rust-lang.org.
# Everything except committed sources lives in toolchain/ (gitignored).
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
export RUSTUP_HOME="$ROOT/toolchain/rustup"
export CARGO_HOME="$ROOT/toolchain/cargo"
export PATH="$CARGO_HOME/bin:$PATH"
NIGHTLY=nightly-2026-10-19
WILD_TOOLCHAIN=1.99.1
CG_CLIF_REV=4f78fd3e5f2ce8a688865b4a6197e6d33211c09c
CLIF_TO_WASM_REV=0e4e0458f5a190a7fcdbb2298d9ad780678c44bc
WILD_REV=fd79578abb968275788428a6c639d967d974128f
case "$(uname -s)" in
    Darwin) LIBEXT=dylib ;;
    *) LIBEXT=so ;;
esac
DYLIB="$ROOT/toolchain/target/cg_clif/release/librustc_codegen_cranelift.$LIBEXT"
export CARGO_TARGET_DIR="$ROOT/toolchain/target/programs"
export RUSTUP_TOOLCHAIN="$NIGHTLY"

log() { printf '\n== %s\n' "$*" >&2; }

ensure_rustup() {
    if [ ! -x "$CARGO_HOME/bin/rustup" ]; then
        case "$(uname -s)-$(uname -m)" in
            Darwin-arm64)   RUSTUP_HOST=aarch64-apple-darwin ;;
            Darwin-x86_64)  RUSTUP_HOST=x86_64-apple-darwin ;;
            Linux-x86_64)   RUSTUP_HOST=x86_64-unknown-linux-gnu ;;
            Linux-aarch64)  RUSTUP_HOST=aarch64-unknown-linux-gnu ;;
            *) echo "unsupported host: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
        esac
        log "installing isolated rustup for $RUSTUP_HOST"
        mkdir -p "$ROOT/toolchain"
        curl -sSfL -o "$ROOT/toolchain/rustup-init" \
            "https://static.rust-lang.org/rustup/dist/$RUSTUP_HOST/rustup-init"
        chmod +x "$ROOT/toolchain/rustup-init"
        "$ROOT/toolchain/rustup-init" -y --no-modify-path --profile minimal --default-toolchain none
    fi
    rustup toolchain install "$NIGHTLY" --profile minimal \
        --component rust-src,rustc-dev,llvm-tools,rustfmt \
        --target wasm32-unknown-unknown,wasm32-wasip1
    rustup toolchain install "$WILD_TOOLCHAIN" --profile minimal --target wasm32-wasip1
}

checkout() { # dir url rev
    local dir=$1 url=$2 rev=$3
    if [ ! -d "$ROOT/toolchain/$dir/.git" ]; then
        log "fetching $dir @ $rev"
        git init -q "$ROOT/toolchain/$dir"
        git -C "$ROOT/toolchain/$dir" remote add origin "$url"
        git -C "$ROOT/toolchain/$dir" fetch -q --depth 1 origin "$rev"
        git -C "$ROOT/toolchain/$dir" checkout -q -b pinned FETCH_HEAD
    fi
    local got
    got=$(git -C "$ROOT/toolchain/$dir" rev-parse HEAD)
    [ "$got" = "$rev" ] || { echo "$dir is at $got, expected $rev" >&2; exit 1; }
}

setup() {
    ensure_rustup
    checkout cg_clif https://github.com/rust-lang/rustc_codegen_cranelift "$CG_CLIF_REV"
    checkout clif-to-wasm https://github.com/ryancbuckley/clif-to-wasm "$CLIF_TO_WASM_REV"
    checkout wild https://github.com/wild-linker/wild "$WILD_REV"
    if ! git -C "$ROOT/toolchain/cg_clif" apply --reverse --check "$ROOT/patches/0001-cg_clif-wasm-backend.patch" 2>/dev/null; then
        log "applying wasm backend patch to cg_clif"
        git -C "$ROOT/toolchain/cg_clif" apply "$ROOT/patches/0001-cg_clif-wasm-backend.patch"
    fi
}

tools() {
    setup
    log "building cg_clif"
    ( cd "$ROOT/toolchain/cg_clif" && RUSTUP_TOOLCHAIN="$NIGHTLY" \
        CARGO_TARGET_DIR="$ROOT/toolchain/target/cg_clif" cargo build --release )
    log "building wild"
    ( cd "$ROOT/toolchain/wild" && RUSTUP_TOOLCHAIN="$WILD_TOOLCHAIN" \
        CARGO_TARGET_DIR="$ROOT/toolchain/target/wild" cargo build --release -p wild-linker )
    log "building Wasmtime host"
    ( cd "$ROOT/programs/host" && RUSTUP_TOOLCHAIN="$WILD_TOOLCHAIN" \
        CARGO_TARGET_DIR="$ROOT/toolchain/target/host" cargo build --release )
}

build_program() { # program, extra cargo args..., using env RUSTFLAGS
    local prog=$1
    ( cd "$ROOT/programs/$prog" && cargo build --release --target wasm32-unknown-unknown "${@:2}" )
}

programs() {
    tools
    log "cg_clif + pure-Rust core/alloc (rust-lld)"
    for prog in a_add b_alloc c_plugin; do
        ( cd "$ROOT/programs/$prog" && \
          RUSTFLAGS="-Zcodegen-backend=$DYLIB -Cpanic=abort" \
          cargo build --release --target wasm32-unknown-unknown \
              -Zbuild-std=core,alloc,compiler_builtins )
    done
    log "strict check (no unsupported functions may become traps)"
    for prog in a_add b_alloc c_plugin; do
        ( cd "$ROOT/programs/$prog" && \
          CG_CLIF_WASM_STRICT=1 RUSTFLAGS="-Zcodegen-backend=$DYLIB -Cpanic=abort" \
          CARGO_TARGET_DIR="$ROOT/toolchain/target/programs-strict" \
          cargo build --release --target wasm32-unknown-unknown \
              -Zbuild-std=core,alloc,compiler_builtins )
    done
    log "LLVM + prebuilt std (rust-lld), for comparison"
    for prog in a_add b_alloc c_plugin; do
        ( cd "$ROOT/programs/$prog" && \
          CARGO_TARGET_DIR="$ROOT/toolchain/target/programs-llvm" \
          cargo build --release --target wasm32-unknown-unknown )
    done
    log "std hello on wasm32-wasip1 (LLVM, prebuilt std)"
    ( cd "$ROOT/programs/d_hello_std" && \
      CARGO_TARGET_DIR="$ROOT/toolchain/target/programs-llvm" \
      cargo build --release --target wasm32-wasip1 )
    log "smoke tests"
    H="$ROOT/toolchain/target/host/release/host"
    "$H" add "$ROOT/toolchain/target/programs/wasm32-unknown-unknown/release/a_add.wasm" 20 22
    "$H" bench "$ROOT/toolchain/target/programs/wasm32-unknown-unknown/release/b_alloc.wasm" 100000
    "$H" plugin "$ROOT/toolchain/target/programs/wasm32-unknown-unknown/release/c_plugin.wasm" 1 2 3 4
    "$H" hello "$ROOT/toolchain/target/programs-llvm/wasm32-wasip1/release/d_hello_std.wasm"
}

measure() {
    tools
    "$ROOT/scripts/measure.py"
}

case "${1:-all}" in
    setup) setup ;;
    tools) tools ;;
    programs) programs ;;
    measure) measure ;;
    all) programs; measure ;;
    *) echo "usage: $0 [setup|tools|programs|measure|all]" >&2; exit 1 ;;
esac
