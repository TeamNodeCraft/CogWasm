#!/usr/bin/env bash
# Set up the CogWasm rustc source tree: fetch/verify the pinned rust-lang/rust
# checkout, apply the prior-art rust-to-wasm patch series once, fetch/select
# wasi-sdk, point bootstrap.toml at it, and stop.
#
# This is the source setup the CogWasm compiler needs. It is idempotent and never builds
# rustc.wasm; compiler/scripts/build-rustc-wasm.sh does that afterwards. The
# native stage1 compiler is still built with LLVM (stage0's LLVM backend, plus
# a CI LLVM download for the native compiler). That is the one build-time LLVM
# use; rustc.wasm itself contains only cg_clif. See docs/compiler.md.
#
# Everything downloaded/built lives under toolchain/ (gitignored).
# The layout is role-based: rust source at toolchain/rust, wasi-sdk at
# toolchain/wasi-sdk.
set -euo pipefail

REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
UPSTREAM="$REPO/compiler/scripts/upstream.py"
TOOL="$REPO/toolchain"
WORK="$TOOL"
SRC="$WORK/rust"
PLAYGROUND="$TOOL/playground"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
RUSTUP_TOOLCHAIN=$(python3 "$UPSTREAM" get rust-nightly version)
export RUSTUP_TOOLCHAIN

RUST_COMMIT=$(python3 "$UPSTREAM" get rust rev)
RUST_URL=$(python3 "$UPSTREAM" get rust repo)
PLAYGROUND_REV=$(python3 "$UPSTREAM" get playground rev)
PLAYGROUND_URL=$(python3 "$UPSTREAM" get playground repo)
PLAYGROUND_PATCH_DIR=$(python3 "$UPSTREAM" get playground patch_dir)
PLAYGROUND_PATCHES=$(python3 "$UPSTREAM" get playground patch_series)
WASI_SDK_VERSION=$(python3 "$UPSTREAM" get wasi-sdk version)
WASI_SDK_BASE_URL=$(python3 "$UPSTREAM" get wasi-sdk url)

# wasi-sdk publishes one archive per host. The spike was developed on Apple
# silicon; CI is x86_64 Linux, so pick the archive instead of hardcoding macOS.
case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)          WASI_SDK="wasi-sdk-$WASI_SDK_VERSION-arm64-macos" ;;
    Darwin-x86_64)         WASI_SDK="wasi-sdk-$WASI_SDK_VERSION-x86_64-macos" ;;
    Linux-x86_64|Linux-amd64) WASI_SDK="wasi-sdk-$WASI_SDK_VERSION-x86_64-linux" ;;
    Linux-aarch64|Linux-arm64) WASI_SDK="wasi-sdk-$WASI_SDK_VERSION-arm64-linux" ;;
    *)
        echo "unsupported host for wasi-sdk: $(uname -s) $(uname -m)" >&2
        exit 1
        ;;
esac
WASI_SDK_URL="$WASI_SDK_BASE_URL/$WASI_SDK.tar.gz"

log() { printf '\n== %s\n' "$*" >&2; }

mkdir -p "$WORK"

# The prior-art patch series lives in that checkout; fetch it if the
# toolchain's playground checkout is not present.
if [ ! -d "$PLAYGROUND/.git" ]; then
    log "fetching prior art @ $PLAYGROUND_REV"
    git init -q "$PLAYGROUND"
    git -C "$PLAYGROUND" remote add origin "$PLAYGROUND_URL"
    git -C "$PLAYGROUND" fetch -q --depth 1 origin "$PLAYGROUND_REV"
    git -C "$PLAYGROUND" checkout -q -b pinned FETCH_HEAD
fi
got=$(git -C "$PLAYGROUND" rev-parse HEAD)
[ "$got" = "$PLAYGROUND_REV" ] || {
    echo "prior art is at $got, expected $PLAYGROUND_REV" >&2; exit 1; }

if [ ! -d "$SRC/.git" ]; then
    log "fetching rust-lang/rust @ $RUST_COMMIT"
    git init -q "$SRC"
    git -C "$SRC" remote add origin "$RUST_URL"
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
    log "applying the prior-art rust patch series ($PLAYGROUND_PATCHES patches)"
    git -C "$SRC" config user.name playground
    git -C "$SRC" config user.email playground@localhost
    git -C "$SRC" am -q "$PLAYGROUND/$PLAYGROUND_PATCH_DIR"/*.patch
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
# /share/wasi-sysroot or /bin/... suffix. After the first run the path ends in
# our /wasi-sdk symlink (no version suffix); match that too, so a moved
# toolchain directory is rewritten instead of failing the check below.
s = re.sub(r'/[^"\n]*/wasi-sdk(?:-[^/"]*)?(?=/)', sdk, s)
open(p, 'w').write(s)
PY
grep -q "$WASI_SDK_PATH/share/wasi-sysroot" "$SRC/bootstrap.toml" || {
    echo "failed to rewrite bootstrap.toml wasi-sdk paths" >&2; exit 1; }

log "source setup: rust source, prior-art patches and wasi-sdk are ready"
