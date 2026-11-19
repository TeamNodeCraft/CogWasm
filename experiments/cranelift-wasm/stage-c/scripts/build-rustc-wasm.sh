#!/usr/bin/env bash
# Build stage-c rustc.wasm: wasm32-wasip1 (non-threads), cg_clif only, with
# every patch under stage-c/patches/{rust,cg_clif,clif-to-wasm}/ applied.
#
# Reuses the stage-B source tree under toolchain/stage-b (incremental when the
# tree was already built). Every patch under stage-c/patches/{rust,cg_clif,
# clif-to-wasm}/ is applied idempotently before the build, in sorted order.
#
# `x.py install` must succeed: the generate-copyright failure caused by the
# out-of-tree clif-to-wasm path dependency is fixed by
# patches/rust/0002-generate-copyright-path-deps.patch. A nonzero exit prints
# the log tail and stops; rustc.wasm is only copied out after a successful
# install.
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)   # experiments/cranelift-wasm
TOOL="$ROOT/toolchain"
WORK="$TOOL/stage-b"
SRC="$WORK/rust"
CG="$SRC/compiler/rustc_codegen_cranelift"
CLIF="$TOOL/clif-to-wasm"
export RUSTUP_HOME="${RUSTUP_HOME:-$TOOL/rustup}"
export CARGO_HOME="${CARGO_HOME:-$TOOL/cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
export RUSTUP_TOOLCHAIN=nightly-2026-10-19
WASI_SDK="$WORK/wasi-sdk"
OUT="$TOOL/stage-c/artifacts/rustc.wasm"
LOG="$ROOT/stage-c/results/build-rustc-wasm.log"
# Patch files are applied in order, and a later patch can change the context
# of an earlier one. Once a patch has been applied successfully, record its
# hash so a rerun in the same checkout skips it instead of trying (and
# failing) to reverse-check a context that later patches have moved. A fresh
# checkout has no state file and applies the whole series.
#
# The state must live with the checkout it describes: CI caches the patched
# clif-to-wasm checkout but not the stage-B work tree, so keeping its hashes in
# $WORK made a restored checkout look unpatched, and re-applying 0001 failed
# on context that 0002-0004 had already moved.
PATCH_STATE="$WORK/.stage-c-applied-patches"
CLIF_PATCH_STATE="$CLIF/.cogwasm-applied-patches"
sha256_file() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}
patch_recorded() { [ -f "$1" ] && grep -qx "$2" "$1"; }
record_patch() { printf '%s\n' "$2" >> "$1"; }

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

[ -d "$SRC/.git" ] || { echo "missing stage-B rust tree; run stage-b/scripts/build-rustc-wasm.sh once" >&2; exit 1; }
[ -d "$WASI_SDK" ] || { echo "missing $WASI_SDK" >&2; exit 1; }
[ -d "$CLIF/.git" ] || { echo "missing $CLIF; run ci/setup-toolchain.sh first" >&2; exit 1; }
mkdir -p "$(dirname "$LOG")" "$(dirname "$OUT")"

# Rust patches. A patch may be applied as a commit (the local experiment tree)
# or left uncommitted (a fresh CI checkout); the reverse check covers both and
# makes reruns idempotent. Loop over every patch so a new one (for example the
# generate-copyright or wasm file-lock fix) cannot be silently skipped.
shopt -s nullglob
RUST_PATCHES=("$ROOT"/stage-c/patches/rust/*.patch)
[ "${#RUST_PATCHES[@]}" -gt 0 ] || {
    echo "no rust patches found under $ROOT/stage-c/patches/rust" >&2; exit 1; }
for p in "${RUST_PATCHES[@]}"; do
    hash=$(sha256_file "$p")
    if patch_recorded "$PATCH_STATE" "$hash"; then
        echo "$(basename "$p") already applied (recorded)"
        continue
    fi
    if git -C "$SRC" apply --reverse --check "$p" 2>/dev/null; then
        echo "$(basename "$p") already applied"
    else
        echo "applying $(basename "$p")"
        git -C "$SRC" apply "$p"
    fi
    git -C "$SRC" apply --reverse --check "$p" || {
        echo "$(basename "$p") is not present after apply" >&2; exit 1; }
    record_patch "$PATCH_STATE" "$hash"
done

# clif-to-wasm patches (this checkout is the repository root, so the patches'
# src/... paths resolve correctly).
CLIF_PATCHES=("$ROOT"/stage-c/patches/clif-to-wasm/*.patch)
[ "${#CLIF_PATCHES[@]}" -gt 0 ] || {
    echo "no clif-to-wasm patches found under $ROOT/stage-c/patches/clif-to-wasm" >&2; exit 1; }
for p in "${CLIF_PATCHES[@]}"; do
    hash=$(sha256_file "$p")
    if patch_recorded "$CLIF_PATCH_STATE" "$hash"; then
        echo "$(basename "$p") already applied (recorded)"
        continue
    fi
    if git -C "$CLIF" apply --reverse --check "$p" 2>/dev/null; then
        echo "$(basename "$p") already applied"
    else
        echo "applying $(basename "$p")"
        git -C "$CLIF" apply "$p"
    fi
    git -C "$CLIF" apply --reverse --check "$p" || {
        echo "$(basename "$p") is not present after apply" >&2; exit 1; }
    record_patch "$CLIF_PATCH_STATE" "$hash"
done

# cg_clif patches (in-tree copy; stage A's wasm patch is already applied there).
# The patch paths are src/... relative to the *rust* repository root, not to
# the cg_clif directory. Running `git -C "$CG" apply` here silently reports
# "Skipped patch" and exits 0 because the path is outside the current
# directory, which left a fresh CI tree unpatched. Apply with --directory from
# the root instead, then assert the content is really there.
CG_PREFIX=compiler/rustc_codegen_cranelift
CG_PATCHES=("$ROOT"/stage-c/patches/cg_clif/*.patch)
[ "${#CG_PATCHES[@]}" -gt 0 ] || {
    echo "no cg_clif patches found under $ROOT/stage-c/patches/cg_clif" >&2; exit 1; }
for p in "${CG_PATCHES[@]}"; do
    hash=$(sha256_file "$p")
    if patch_recorded "$PATCH_STATE" "$hash"; then
        echo "$(basename "$p") already applied (recorded)"
        continue
    fi
    if git -C "$SRC" apply --reverse --check --directory="$CG_PREFIX" "$p" 2>/dev/null; then
        echo "$(basename "$p") already applied"
    else
        echo "applying $(basename "$p")"
        git -C "$SRC" apply --directory="$CG_PREFIX" "$p"
    fi
    git -C "$SRC" apply --reverse --check --directory="$CG_PREFIX" "$p" || {
        echo "$(basename "$p") is not present after apply" >&2; exit 1; }
    record_patch "$PATCH_STATE" "$hash"
done
grep -q 'llvm.wasm.memory.grow.i32' "$CG/src/intrinsics/llvm.rs" || {
    echo "cg_clif memory.grow lowering missing from $CG/src/intrinsics/llvm.rs" >&2; exit 1; }
grep -q 'pub enum WasmIntrinsic' "$CLIF/src/wasm_intrinsic.rs" || {
    echo "clif-to-wasm WasmIntrinsic missing from $CLIF/src/wasm_intrinsic.rs" >&2; exit 1; }
# The in-tree cg_clif must consume the patched clif-to-wasm, not crates.io.
# Fail loudly if the pinned Cargo.toml line changes: a silent no-op would use
# the crates.io release without the memory.grow patch.
python3 - "$CG/Cargo.toml" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read()
old = 'clif-to-wasm = { version = "0.3.0", features = ["module"] }'
new = 'clif-to-wasm = { path = "../../../../clif-to-wasm", features = ["module"] }'
if new in s:
    pass
elif old in s:
    s = s.replace(old, new)
    open(p, 'w').write(s)
else:
    sys.exit(f"unexpected clif-to-wasm dependency in {p}; cannot point it at the patched checkout")
PY

sed "s|@WASI_SDK@|$WASI_SDK|g" "$ROOT/stage-c/bootstrap.toml" > "$SRC/bootstrap.toml"
cd "$SRC"
# On GitHub Actions bootstrap assumes HEAD^1 is an upstream rust-lang/rust
# commit. Here HEAD is the last of the 23 prior-art commits applied with
# `git am` (whose hashes differ from upstream because the committer timestamp
# differs), so download-ci-llvm asks for a nonexistent artifact and 404s.
# --ci=false uses the local freshness logic, which walks back to the pinned
# upstream commit b6eeb93 before downloading LLVM.
set +e
./x.py install --ci=false --set build.jobs="$JOBS" 2>&1 | tee "$LOG"
status=${PIPESTATUS[0]}
set -e

ART="$SRC/build/wasm32-wasip1/stage2/bin/rustc.wasm"
if [ "$status" -ne 0 ]; then
    # No failure is tolerated, including the final generate-copyright step:
    # 0002-generate-copyright-path-deps.patch fixes the out-of-tree clif-to-wasm
    # panic, so a nonzero install is a real problem. Print the tail where the
    # failing command is reported and stop before copying anything.
    tail -n 200 "$LOG" >&2
    echo "x.py install failed with status $status; see $LOG" >&2
    exit "$status"
fi
[ -f "$ART" ] || { echo "rustc.wasm was not produced; see $LOG" >&2; exit 1; }
cp "$ART" "$OUT"
echo "rustc.wasm -> $OUT"
ls -lh "$OUT"
