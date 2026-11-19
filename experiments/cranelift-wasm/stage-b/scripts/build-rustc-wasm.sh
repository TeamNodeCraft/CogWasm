#!/usr/bin/env bash
# Build rustc.wasm for wasm32-wasip1-threads with cg_clif + clif-to-wasm as the
# only codegen backend, using the prior-art patch series from
# ryancbuckley/rust-playground-inbrowser.
#
# The native stage1 compiler is still built with LLVM (stage0's LLVM backend,
# plus a CI LLVM download for the native compiler). That is the one build-time
# LLVM use; rustc.wasm itself contains only cg_clif. See ../README.md.
#
# Everything downloaded/built lives under toolchain/ (gitignored).
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)   # experiments/cranelift-wasm
TOOL="$ROOT/toolchain"
WORK="$TOOL/stage-b"
SRC="$WORK/rust"
PLAYGROUND="$TOOL/playground"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
export RUSTUP_TOOLCHAIN=nightly-2026-10-19

RUST_COMMIT=b6eeb93c992027a03e1fd120b6652546396b142c
PLAYGROUND_REV=f5298630dda0f2dc2b44f6ae1b54add7a77d8316

# wasi-sdk publishes one archive per host. The spike was developed on Apple
# silicon; CI is x86_64 Linux, so pick the archive instead of hardcoding macOS.
case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)          WASI_SDK=wasi-sdk-34.0-arm64-macos ;;
    Darwin-x86_64)         WASI_SDK=wasi-sdk-34.0-x86_64-macos ;;
    Linux-x86_64|Linux-amd64) WASI_SDK=wasi-sdk-34.0-x86_64-linux ;;
    Linux-aarch64|Linux-arm64) WASI_SDK=wasi-sdk-34.0-arm64-linux ;;
    *)
        echo "unsupported host for wasi-sdk: $(uname -s) $(uname -m)" >&2
        exit 1
        ;;
esac
WASI_SDK_URL="https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-34/$WASI_SDK.tar.gz"

log() { printf '\n== %s\n' "$*" >&2; }
jobs() {
    if [ -n "${BUILD_JOBS:-}" ]; then
        printf '%s' "$BUILD_JOBS"
    elif command -v nproc >/dev/null 2>&1; then
        nproc
    else
        sysctl -n hw.ncpu
    fi
}
JOBS=$(jobs)

mkdir -p "$WORK"

# The 23-patch series lives in the prior-art checkout; fetch it if Stage A's
# toolchain/playground is not present.
if [ ! -d "$PLAYGROUND/.git" ]; then
    log "fetching prior art @ $PLAYGROUND_REV"
    git init -q "$PLAYGROUND"
    git -C "$PLAYGROUND" remote add origin https://github.com/ryancbuckley/rust-playground-inbrowser
    git -C "$PLAYGROUND" fetch -q --depth 1 origin "$PLAYGROUND_REV"
    git -C "$PLAYGROUND" checkout -q -b pinned FETCH_HEAD
fi
got=$(git -C "$PLAYGROUND" rev-parse HEAD)
[ "$got" = "$PLAYGROUND_REV" ] || {
    echo "prior art is at $got, expected $PLAYGROUND_REV" >&2; exit 1; }

if [ ! -d "$SRC/.git" ]; then
    log "fetching rust-lang/rust @ $RUST_COMMIT"
    git init -q "$SRC"
    git -C "$SRC" remote add origin https://github.com/rust-lang/rust
    git -C "$SRC" fetch -q --depth 1 origin "$RUST_COMMIT"
    git -C "$SRC" checkout -q -b wasm-playground FETCH_HEAD
fi
git -C "$SRC" cat-file -e "$RUST_COMMIT^{commit}" 2>/dev/null || {
    echo "rust checkout does not contain $RUST_COMMIT" >&2; exit 1; }

# Apply the patch series once. `git am` leaves the last patch as HEAD, so look
# for one of the patch subjects instead of comparing a count. Later stages add
# commits on top, so search the history rather than only HEAD. Use --grep with
# a command substitution, not `git log | grep -q`: under `pipefail`, grep
# exiting early can make git log fail with SIGPIPE and re-apply the series.
if [ -z "$(git -C "$SRC" log --format=%s -n 60 --grep='^cg_clif: update to clif-to-wasm 0\.3\.0$')" ]; then
    log "applying the prior-art rust patch series (23 patches)"
    git -C "$SRC" config user.name playground
    git -C "$SRC" config user.email playground@localhost
    git -C "$SRC" am -q "$PLAYGROUND"/patches/rust/*.patch
fi

if [ ! -d "$WORK/$WASI_SDK" ]; then
    log "downloading $WASI_SDK"
    curl -fsSL "$WASI_SDK_URL" | tar xz -C "$WORK"
fi
ln -sfn "$WASI_SDK" "$WORK/wasi-sdk"
export WASI_SDK_PATH="$WORK/wasi-sdk"

# The patch series hardcodes a path on the author's machine for wasi-sdk.
python3 - "$SRC/bootstrap.toml" "$WASI_SDK_PATH" <<'PY'
import re, sys
p, sdk = sys.argv[1], sys.argv[2]
s = open(p).read()
# Replace the directory prefix up to /wasi-sdk-* with ours, keeping the
# /share/wasi-sysroot or /bin/... suffix.
s = re.sub(r'/[^"\n]*/wasi-sdk-[^/"]*', sdk, s)
open(p, 'w').write(s)
PY
grep -q "$WASI_SDK_PATH/share/wasi-sysroot" "$SRC/bootstrap.toml" || {
    echo "failed to rewrite bootstrap.toml wasi-sdk paths" >&2; exit 1; }

# setup-toolchain.sh uses this to fetch and patch the pinned sources without
# paying for the stage-B rustc.wasm build; stage C then builds wasm32-wasip1.
if [ "${SETUP_ONLY:-0}" = 1 ]; then
    log "setup only: rust source, prior-art patches and wasi-sdk are ready"
    exit 0
fi

host=$(rustc +"$RUSTUP_TOOLCHAIN" -vV | sed -n 's/^host: //p')
# Keep paths out of the shipped artifacts. The two x.py runs must agree, or the
# second builds a second stage1 compiler.
flags='["-Ztranslate-remapped-path-to-local-path=no"]'
NO_LOCAL_PATHS=(--set rust.remap-debuginfo=true
    --set "rust.rustflags=$flags"
    --set "target.$host.rustflags=$flags"
    --set target.wasm32-wasip1-threads.rustflags="$flags"
    --set target.wasm32-wasip1.rustflags="$flags")

log "rustc.wasm build (host=$host; stage0 LLVM compiles rustc; cg_clif for the wasm target)"
cd "$SRC"
# --ci=false: on GitHub Actions bootstrap treats HEAD^1 as an upstream commit
# and download-ci-llvm 404s on our `git am` commits. See the stage-C build
# script for the same explanation.
./x.py install --ci=false "${NO_LOCAL_PATHS[@]}" --set build.jobs="$JOBS"

test -f "$SRC/build/wasm32-wasip1-threads/stage2/bin/rustc.wasm"
log "building the stage1 library for the host + wasm32-wasip1 (wasm proc macros)"
./x.py build library --ci=false --stage 1 --host "$host" \
    --target "$host,wasm32-wasip1,wasm32-wasip1-threads" "${NO_LOCAL_PATHS[@]}" \
    --set build.jobs="$JOBS"

log "artifacts"
ls -lh "$SRC/build/wasm32-wasip1-threads/stage2/bin/rustc.wasm"
