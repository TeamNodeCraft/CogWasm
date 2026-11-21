#!/usr/bin/env bash
# Cargo RUSTC wrapper: compile wasm32-unknown-unknown crates with the CogWasm
# rustc.wasm under Wasmtime, and everything else (build scripts, proc macros)
# with the native stage1 compiler from the same rustc source.
#
# Set COGWASM_SYSROOT to add a plugin sysroot to target-crate compiles. With
# -Zbuild-std, Cargo passes --sysroot itself and no extra sysroot is added.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
TOOL="$REPO/toolchain"
STAGE1=${COGWASM_STAGE1:-${STAGE_C_STAGE1:-$TOOL/rust/build/host/stage1/bin/rustc}}
HOST_BIN=${COGWASM_HOST:-${STAGE_C_HOST:-$TOOL/target/cogwasm-host/release/cogwasm-host}}
RUSTC_WASM=${COGWASM_RUSTC_WASM:-${STAGE_C_RUSTC_WASM:-$TOOL/artifacts/rustc.wasm}}
WILD=${COGWASM_WILD:-${STAGE_C_WILD:-$TOOL/target/wild/release/wild}}
CACHE=${COGWASM_CACHE:-${STAGE_C_CACHE:-$TOOL/cache}}
SYSROOT=${COGWASM_SYSROOT:-${STAGE_C_SYSROOT:-}}
REAL_SRC=${COGWASM_REAL_SRC:-${STAGE_C_REAL_SRC:-$TOOL/rust}}

# The stage1 sysroot contains rust-src as a symlink under the build host's
# triple. Do not hardcode aarch64-apple-darwin: CI is x86_64 Linux.
HOST_TRIPLE=${COGWASM_HOST_TRIPLE:-${STAGE_C_HOST_TRIPLE:-}}
if [ -z "$HOST_TRIPLE" ] && [ -x "$STAGE1" ]; then
    HOST_TRIPLE=$("$STAGE1" -vV | sed -n 's/^host: //p')
fi
STD_SRC=${COGWASM_STD_SRC:-${STAGE_C_STD_SRC:-}}
if [ -z "$STD_SRC" ] && [ -n "$HOST_TRIPLE" ]; then
    STD_SRC="$TOOL/rust/build/$HOST_TRIPLE/stage1/lib/rustlib/src/rust"
fi
[ -n "$STD_SRC" ] && [ -e "$STD_SRC" ] || STD_SRC=

wasm=0
has_sysroot=0
mapped=()
skip_value=0
for a in "$@"; do
    # The value after --crate-name is a name, not a path; a crate whose name
    # matches a directory (e.g. adler2 in a vendored workspace) must not be
    # rewritten into an absolute path.
    if [ "$skip_value" = 1 ]; then
        mapped+=("$a")
        skip_value=0
        continue
    fi
    case "$a" in
        --crate-name) mapped+=("$a"); skip_value=1; continue ;;
        wasm32-unknown-unknown|--target=wasm32-unknown-unknown) wasm=1 ;;
        --sysroot|--sysroot=*) has_sysroot=1 ;;
    esac
    # stage1's lib/rustlib/src/rust is a symlink; WASI path_open refuses to
    # follow it, so hand rustc.wasm the real checkout path.
    if [ -n "$STD_SRC" ] && [ -n "$REAL_SRC" ] && [[ "$a" == "$STD_SRC"* ]]; then
        a="$REAL_SRC${a#"$STD_SRC"}"
    fi
    # WASI has no cwd. Cargo passes crate sources as paths relative to the
    # package directory; make existing relative paths absolute so rustc.wasm
    # can open them through the host preopen.
    if [[ "$a" != -* && "$a" != /* && -e "$a" ]]; then
        a="$PWD/$a"
    fi
    mapped+=("$a")
done
set -- "${mapped[@]}"

if [ "$wasm" = 1 ]; then
    args=(raw --rustc "$RUSTC_WASM" --linker "$WILD")
    [ -n "$CACHE" ] && args+=(--cache-dir "$CACHE")
    args+=(--)
    if [ -n "$SYSROOT" ] && [ "$has_sysroot" = 0 ]; then
        args+=(--sysroot "$SYSROOT")
    fi
    args+=("$@")
    args+=(-Zcodegen-backend=cranelift "-Clinker=$WILD" -Clinker-flavor=wasm-ld)
    exec "$HOST_BIN" "${args[@]}"
else
    exec "$STAGE1" "$@"
fi
