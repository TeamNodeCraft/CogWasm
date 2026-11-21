#!/usr/bin/env bash
# Build core + alloc + compiler_builtins for wasm32-unknown-unknown with
# the CogWasm rustc.wasm itself (cg_clif), so the rlib metadata matches the
# compiler exactly. No native rustc compiles any shipped plugin crate.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
TOOL="$REPO/toolchain"
SYS="$TOOL/sysroot"
TARGET="$TOOL/target/sysroot"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export RUSTUP_TOOLCHAIN=nightly-2026-10-19
export RUSTC="$REPO/compiler/scripts/rustc-wasm-wrapper.sh"
export RUSTC_BOOTSTRAP=1
# Turn clif-to-wasm's "replace an unsupported function with a trap" into an
# error: the shipped sysroot must not contain trap stubs.
export CG_CLIF_WASM_STRICT="${CG_CLIF_WASM_STRICT:-1}"

# compiler_builtins' build script invokes rust-objcopy from the rustup
# llvm-tools component. The toolchain directory is host-specific.
STAGE1="${COGWASM_STAGE1:-${STAGE_C_STAGE1:-$TOOL/rust/build/host/stage1/bin/rustc}}"
[ -x "$STAGE1" ] || { echo "missing native stage1 rustc: $STAGE1" >&2; exit 1; }
HOST=$("$STAGE1" -vV | sed -n 's/^host: //p')
export COGWASM_HOST_TRIPLE="$HOST"
export PATH="$RUSTUP_HOME/toolchains/nightly-2026-10-19-$HOST/lib/rustlib/$HOST/bin:$CARGO_HOME/bin:$PATH"

unset COGWASM_SYSROOT STAGE_C_SYSROOT || true
rm -rf "$SYS"
mkdir -p "$SYS/lib/rustlib/wasm32-unknown-unknown/lib"
cd "$REPO/guests/c_plugin"
RUSTFLAGS="-Cpanic=abort" CARGO_TARGET_DIR="$TARGET" \
    cargo build --release --target wasm32-unknown-unknown \
        -Zbuild-std=core,alloc,compiler_builtins
for crate in core alloc compiler_builtins; do
    found=0
    for f in "$TARGET/wasm32-unknown-unknown/release/build/$crate"/*/out/lib"$crate"-*.rlib \
             "$TARGET/wasm32-unknown-unknown/release/build/$crate"/*/out/lib"$crate"-*.rmeta; do
        [ -f "$f" ] || continue
        cp "$f" "$SYS/lib/rustlib/wasm32-unknown-unknown/lib/"
        found=1
    done
    [ "$found" = 1 ] || { echo "no $crate rlib under $TARGET" >&2; exit 1; }
done
echo "pure rustc.wasm-built sysroot at $SYS"
ls -lh "$SYS/lib/rustlib/wasm32-unknown-unknown/lib"
