#!/usr/bin/env bash
# Fetch the pinned cranelift-wasm toolchain inputs on macOS or Linux.
#
# Everything lives under toolchain/ (gitignored). This script is idempotent and
# keeps the network downloads out of build-and-test.sh; the workflow caches
# toolchain/rustup, toolchain/cargo and the pinned checkouts between runs.
#
# It also applies the prior-art rust patch series (through
# compiler/scripts/setup-sources.sh) and downloads wasi-sdk, but does NOT build
# rustc.wasm: the CogWasm compiler builds wasm32-wasip1 directly from this tree.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
UPSTREAM="$REPO/compiler/scripts/upstream.py"
TOOL="$REPO/toolchain"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"

CLIF_REV=$(python3 "$UPSTREAM" get clif-to-wasm rev)
CLIF_URL=$(python3 "$UPSTREAM" get clif-to-wasm repo)
WILD_REV=$(python3 "$UPSTREAM" get wild rev)
WILD_URL=$(python3 "$UPSTREAM" get wild repo)
NIGHTLY=$(python3 "$UPSTREAM" get rust-nightly version)
STABLE=$(python3 "$UPSTREAM" get rust-stable version)
RUSTUP_DIST=$(python3 "$UPSTREAM" get rust-nightly rustup_dist)

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
export BUILD_JOBS="$JOBS"

mkdir -p "$TOOL"
df -h "$TOOL" || true

# rustup-init and wasi-sdk are published per host triple. CI is x86_64 Linux;
# the spike's local host is Apple silicon.
case "$(uname -s)" in
    Darwin) os=apple-darwin ;;
    Linux)  os=unknown-linux-gnu ;;
    *) echo "unsupported OS: $(uname -s)" >&2; exit 1 ;;
esac
case "$(uname -m)" in
    arm64|aarch64) arch=aarch64 ;;
    x86_64|amd64)  arch=x86_64 ;;
    *) echo "unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac
RUSTUP_HOST="$arch-$os"

log "installing isolated rustup ($RUSTUP_HOST)"
if [ ! -x "$CARGO_HOME/bin/rustup" ]; then
    if [ ! -x "$TOOL/rustup-init" ]; then
        curl --retry 3 --retry-delay 5 -fL \
            "$RUSTUP_DIST/$RUSTUP_HOST/rustup-init" \
            -o "$TOOL/rustup-init"
        chmod +x "$TOOL/rustup-init"
    fi
    "$TOOL/rustup-init" -y --no-modify-path --profile minimal --default-toolchain none
fi

log "installing $NIGHTLY with rustc-dev, llvm-tools and rust-src"
rustup toolchain install "$NIGHTLY" --profile minimal \
    --component rustc-dev --component llvm-tools --component rust-src \
    --target wasm32-unknown-unknown --target wasm32-wasip1
rustup component add rustc-dev llvm-tools rust-src --toolchain "$NIGHTLY"
rustup target add wasm32-unknown-unknown wasm32-wasip1 --toolchain "$NIGHTLY"

log "installing $STABLE (used to build Wild)"
rustup toolchain install "$STABLE" --profile minimal \
    --component clippy --target wasm32-wasip1
rustup component add clippy --toolchain "$STABLE"
rustup target add wasm32-wasip1 --toolchain "$STABLE"

# Fetch a pinned checkout without relying on a branch name. Reusing an
# actions/cache-restored checkout is fine as long as the revision matches.
checkout() {
    local dir=$1 url=$2 rev=$3
    if [ ! -d "$dir/.git" ]; then
        rm -rf "$dir"
        git init -q "$dir"
        git -C "$dir" remote add origin "$url"
    fi
    if ! git -C "$dir" cat-file -e "$rev^{commit}" 2>/dev/null; then
        log "fetching $url @ $rev"
        git -C "$dir" fetch -q --depth 1 origin "$rev"
    fi
    local got
    got=$(git -C "$dir" rev-parse HEAD 2>/dev/null || true)
    if [ "$got" != "$rev" ]; then
        git -C "$dir" checkout -q -B pinned "$rev"
    fi
    got=$(git -C "$dir" rev-parse HEAD)
    [ "$got" = "$rev" ] || {
        echo "$dir is at $got, expected $rev" >&2; exit 1; }
}

log "fetching pinned clif-to-wasm and Wild"
checkout "$TOOL/clif-to-wasm" "$CLIF_URL" "$CLIF_REV"
checkout "$TOOL/wild" "$WILD_URL" "$WILD_REV"

# Wild patches. The reverse check makes reruns idempotent whether the patch
# is applied as a commit or left uncommitted. If a patch is newly applied the
# cached binary predates it, so rebuild even when the executable exists.
WILD_PATCHES=()
while IFS= read -r p; do
    [ -n "$p" ] && WILD_PATCHES+=("$p")
done < <(python3 "$UPSTREAM" patches wild)
[ "${#WILD_PATCHES[@]}" -gt 0 ] || {
    echo "no Wild patches listed for upstream 'wild' in compiler/upstreams.toml" >&2; exit 1; }
wild_patch_applied=false
for p in "${WILD_PATCHES[@]}"; do
    if git -C "$TOOL/wild" apply --reverse --check "$p" 2>/dev/null; then
        echo "$(basename "$p") already applied"
    else
        echo "applying $(basename "$p")"
        git -C "$TOOL/wild" apply "$p"
        wild_patch_applied=true
    fi
    git -C "$TOOL/wild" apply --reverse --check "$p" || {
        echo "$(basename "$p") is not present after apply" >&2; exit 1; }
done

log "building native Wild ($JOBS jobs)"
if [ "$wild_patch_applied" = true ] || [ ! -x "$TOOL/target/wild/release/wild" ]; then
    ( cd "$TOOL/wild" && RUSTUP_TOOLCHAIN="$STABLE" \
        CARGO_TARGET_DIR="$TOOL/target/wild" \
        cargo build --release -p wild-linker )
fi
"$TOOL/target/wild/release/wild" --version >/dev/null

# Reuse the compiler source setup (rust checkout + 23-patch series + wasi-sdk +
# bootstrap.toml rewrite); it does not build rustc.wasm.
log "preparing rust source, prior-art patches and wasi-sdk"
"$REPO/compiler/scripts/setup-sources.sh"

log "toolchain inputs ready"
du -sh "$TOOL/rustup" "$TOOL/cargo" "$TOOL/clif-to-wasm" "$TOOL/wild" 2>/dev/null || true
df -h "$TOOL" || true
