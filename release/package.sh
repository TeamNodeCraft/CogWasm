#!/usr/bin/env bash
# Build the reference host and Wild for this machine, precompile rustc.wasm
# with that exact engine configuration, and assemble one host's release
# bundle: rustc-<tag>.wasm, rustc-<tag>-<host>.cwasm (+ .json), the pure
# sysroot tarball, Wild, the host binary, README, THIRD_PARTY_NOTICES and
# SHA256SUMS.
#
# Used by .github/workflows/release.yml and by the local dry run in
# docs/notes/release.md. Everything is written under --out; existing files
# with the same names are replaced.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TOOL="$REPO/toolchain"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
# Keep the host binaries in the same target directory build-host.sh uses, even
# when the caller overrides it.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$TOOL/target/stage-c-host}"

TAG=
HOST_TRIPLE=
RUSTC_WASM="$TOOL/stage-c/artifacts/rustc.wasm"
SYSROOT="$TOOL/stage-c/sysroot-pure"
WILD="$TOOL/target/wild/release/wild"
HOST_BIN="$CARGO_TARGET_DIR/release/stage-c-host"
PRECOMPILE="$CARGO_TARGET_DIR/release/stage-c-precompile"
REUSE_PRECOMPILED=
NOTICES=
OUT="$REPO/release/out"
SKIP_BUILD=0

usage() {
    cat >&2 <<'EOF'
usage: release/package.sh --tag TAG [options]

  --tag TAG            release tag used in file names (required)
  --host-triple TRIPLE Rust host triple (default: rustc -vV host)
  --rustc-wasm PATH    rustc.wasm (default: toolchain/stage-c/artifacts/rustc.wasm)
  --sysroot DIR        pure sysroot (default: toolchain/stage-c/sysroot-pure)
  --wild PATH          Wild binary (default: toolchain/target/wild/release/wild)
  --reuse-precompiled PATH  copy this .cwasm (and PATH.json) instead of
                       running precompile; it must have been built from --rustc-wasm
  --notices PATH       pre-generated THIRD_PARTY_NOTICES (default: generate one)
  --out DIR            bundle directory (default: release/out)
  --skip-build         do not build the host or Wild (artifacts must exist)
EOF
    exit 2
}

while [ $# -gt 0 ]; do
    case "$1" in
        --tag) TAG=$2; shift 2 ;;
        --host-triple) HOST_TRIPLE=$2; shift 2 ;;
        --rustc-wasm) RUSTC_WASM=$2; shift 2 ;;
        --sysroot) SYSROOT=$2; shift 2 ;;
        --wild) WILD=$2; shift 2 ;;
        --reuse-precompiled) REUSE_PRECOMPILED=$2; shift 2 ;;
        --notices) NOTICES=$2; shift 2 ;;
        --out) OUT=$2; shift 2 ;;
        --skip-build) SKIP_BUILD=1; shift ;;
        -h|--help) usage ;;
        *) echo "unknown option $1" >&2; usage ;;
    esac
done

[ -n "$TAG" ] || { echo "--tag is required" >&2; usage; }
case "$TAG" in
    *[!A-Za-z0-9._-]*) echo "tag '$TAG' contains characters that are not safe in file names" >&2; exit 2 ;;
esac
need() { [ -e "$1" ] || { echo "missing $1" >&2; exit 1; }; }

if [ -z "$HOST_TRIPLE" ]; then
    need "$PRECOMPILE"
    HOST_TRIPLE=$(RUSTUP_TOOLCHAIN=1.99.1 rustc -vV | sed -n 's/^host: //p')
fi
[ -n "$HOST_TRIPLE" ] || { echo "could not determine the host triple" >&2; exit 1; }

if [ "$SKIP_BUILD" = 0 ]; then
    echo "== building the reference host"
    "$REPO/compiler/scripts/build-host.sh"
    echo "== building Wild for $HOST_TRIPLE"
    ( cd "$TOOL/wild" && RUSTUP_TOOLCHAIN=1.99.1 \
        CARGO_TARGET_DIR="$TOOL/target/wild" \
        cargo build --release -p wild-linker )
fi

need "$RUSTC_WASM"; need "$SYSROOT"; need "$WILD"; need "$PRECOMPILE"
mkdir -p "$OUT"
OUT=$(cd "$OUT" && pwd)
# Generated bundle files from any previous tag. Only these known patterns are
# removed; nothing else in the output directory is touched. nullglob keeps a
# missing pattern from expanding to a literal name.
shopt -s nullglob
rm -f "$OUT"/rustc-*.wasm \
      "$OUT"/rustc-*.cwasm \
      "$OUT"/rustc-*.cwasm.json \
      "$OUT"/sysroot-pure-*.tar.gz \
      "$OUT"/wild-* \
      "$OUT"/stage-c-host-* \
      "$OUT"/README-*.md \
      "$OUT"/THIRD_PARTY_NOTICES \
      "$OUT"/SHA256SUMS \
      "$OUT"/SHA256SUMS-* \
      "$OUT"/release-metadata-*.json

cp "$RUSTC_WASM" "$OUT/rustc-$TAG.wasm"
if [ -n "$REUSE_PRECOMPILED" ]; then
    need "$REUSE_PRECOMPILED"
    need "$REUSE_PRECOMPILED.json"
    echo "== reusing $REUSE_PRECOMPILED for $HOST_TRIPLE"
    cp "$REUSE_PRECOMPILED" "$OUT/rustc-$TAG-$HOST_TRIPLE.cwasm"
    cp "$REUSE_PRECOMPILED.json" "$OUT/rustc-$TAG-$HOST_TRIPLE.cwasm.json"
else
    echo "== precompiling rustc.wasm for $HOST_TRIPLE"
    "$PRECOMPILE" precompile \
        --wasm "$OUT/rustc-$TAG.wasm" \
        --out "$OUT/rustc-$TAG-$HOST_TRIPLE.cwasm" \
        --metadata "$OUT/rustc-$TAG-$HOST_TRIPLE.cwasm.json"
fi

echo "== packaging sysroot, linker, host, README and notices"
tar -czf "$OUT/sysroot-pure-$TAG.tar.gz" -C "$(dirname "$SYSROOT")" "$(basename "$SYSROOT")"
cp "$WILD" "$OUT/wild-$TAG-$HOST_TRIPLE"
cp "$HOST_BIN" "$OUT/stage-c-host-$TAG-$HOST_TRIPLE"
cp "$REPO/release/README.md" "$OUT/README-$TAG.md"
if [ -n "$NOTICES" ]; then
    need "$NOTICES"
    cp "$NOTICES" "$OUT/THIRD_PARTY_NOTICES"
else
    python3 "$REPO/release/gen-notices.py" --out "$OUT/THIRD_PARTY_NOTICES"
fi

sha256_file() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}
echo "== writing SHA256SUMS"
(
    cd "$OUT"
    : > SHA256SUMS
    while IFS= read -r file; do
        [ "$file" = SHA256SUMS ] && continue
        printf '%s  %s\n' "$(sha256_file "$file")" "$file" >> SHA256SUMS
    done < <(LC_ALL=C ls -1 | sort)
)

echo "== bundle $OUT"
ls -lh "$OUT"
