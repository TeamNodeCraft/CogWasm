#!/usr/bin/env bash
# Package the in-browser REPL's compiler (site/repl/): rustc.wasm, a
# single-threaded wild.wasm and the wasm32-unknown-unknown sysroot files a
# no_std program needs, each without custom sections and gzipped, plus
# manifest.json. The page downloads these only when a visitor asks for them.
#
#   ci/build-repl-assets.sh [output-dir]   (default toolchain/work/repl-assets)
#
# Expects the toolchain ci/build-and-test.sh builds (rustc.wasm, the sysroot,
# the patched Wild checkout and the stable toolchain). Builds wild.wasm for
# wasm32-wasip1 with wasi-sdk's clang for Wild's C dependencies; Wild patch
# 0002 makes that build run without wasm threads.
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TOOL="$REPO/toolchain"
OUT=${1:-$TOOL/work/repl-assets}
UPSTREAM="$REPO/compiler/scripts/upstream.py"
STABLE=$(python3 "$UPSTREAM" get rust-stable version)
WASI_SDK="$TOOL/wasi-sdk"
LIB="$TOOL/sysroot/lib/rustlib/wasm32-unknown-unknown/lib"
export RUSTUP_HOME="$TOOL/rustup" CARGO_HOME="$TOOL/cargo"

[ -f "$TOOL/artifacts/rustc.wasm" ] || { echo "missing $TOOL/artifacts/rustc.wasm" >&2; exit 1; }
[ -d "$TOOL/wild/.git" ] || { echo "missing Wild checkout at $TOOL/wild; run ci/setup-toolchain.sh" >&2; exit 1; }
[ -x "$WASI_SDK/bin/clang" ] || { echo "missing wasi-sdk at $WASI_SDK" >&2; exit 1; }

echo "== building wild.wasm (wasm32-wasip1, $STABLE)"
"$TOOL/cargo/bin/rustup" target add wasm32-wasip1 --toolchain "$STABLE" >/dev/null
( cd "$TOOL/wild" && RUSTUP_TOOLCHAIN="$STABLE" \
    CARGO_TARGET_DIR="$TOOL/target/wild-wasm" \
    CC_wasm32_wasip1="$WASI_SDK/bin/clang" \
    AR_wasm32_wasip1="$WASI_SDK/bin/llvm-ar" \
    CFLAGS_wasm32_wasip1="--sysroot=$WASI_SDK/share/wasi-sysroot" \
    "$TOOL/cargo/bin/cargo" build --release --target wasm32-wasip1 -p wild-linker )
WILD_WASM="$TOOL/target/wild-wasm/wasm32-wasip1/release/wild.wasm"

rm -rf "${OUT:?}"
mkdir -p "$OUT"
work=$(mktemp -d)
trap 'rm -rf "${work:?}"' EXIT

pack() { # source-wasm name
    python3 "$REPO/ci/strip-wasm.py" "$1" "$work/$2"
    gzip -9 -n -c "$work/$2" >"$OUT/$2.gz"
}
pack "$TOOL/artifacts/rustc.wasm" rustc.wasm
pack "$WILD_WASM" wild.wasm

# A no_std program links core and compiler_builtins; alloc is left out (the
# page's programs have no allocator), which saves about 2 MB. Release assets
# are a flat list, so the files are named sysroot-<file>.gz.
for file in "$LIB"/libcore-*.rmeta "$LIB"/libcore-*.rlib \
    "$LIB"/libcompiler_builtins-*.rmeta "$LIB"/libcompiler_builtins-*.rlib; do
    [ -f "$file" ] || { echo "missing sysroot file $file" >&2; exit 1; }
    gzip -9 -n -c "$file" >"$OUT/sysroot-$(basename "$file").gz"
done

python3 - "$OUT" "$work" "$LIB" "$(git -C "$REPO" rev-parse HEAD)" <<'PY'
import json, os, sys
out, work, lib, commit = sys.argv[1:]
def entry(name, raw_path):
    return {"file": name, "size": os.path.getsize(os.path.join(out, name)),
            "raw_size": os.path.getsize(raw_path)}
sysroot = []
for name in sorted(n for n in os.listdir(out) if n.startswith("sysroot-")):
    raw = name[len("sysroot-"):-3]
    item = entry(name, os.path.join(lib, raw))
    item["path"] = f"lib/rustlib/wasm32-unknown-unknown/lib/{raw}"
    sysroot.append(item)
manifest = {
    "schema": 1,
    "commit": commit,
    "rustc": entry("rustc.wasm.gz", os.path.join(work, "rustc.wasm")),
    "wild": entry("wild.wasm.gz", os.path.join(work, "wild.wasm")),
    "sysroot": sysroot,
}
manifest["download_bytes"] = (manifest["rustc"]["size"] + manifest["wild"]["size"]
                              + sum(item["size"] for item in sysroot))
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w"), indent=2)
print(f"repl assets in {out}: {manifest['download_bytes'] / 1e6:.1f} MB to download")
PY
