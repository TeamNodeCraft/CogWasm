#!/usr/bin/env bash
# Apply the rustc.wasm compiler patch series to a rust-lang/rust checkout and
# only then initialize the submodules.
#
# Usage: legacy/llvm/ci/apply-rust-patches.sh <rust-checkout> <patch-dir>
#
# The ordering is intentional: patch 0002 rewrites .gitmodules and the
# src/llvm-project gitlink to YoWASP/llvm-project llvmorg-22.1.0+wasm, so the
# submodules must be fetched after the series is applied, not before.
set -euo pipefail

rust_dir=${1:?usage: apply-rust-patches.sh <rust-checkout> <patch-dir>}
patch_dir=${2:?usage: apply-rust-patches.sh <rust-checkout> <patch-dir>}

if [ ! -d "$patch_dir" ]; then
  echo "ERROR: patch directory not found: $patch_dir" >&2
  exit 1
fi

# Resolve the patch directory before changing into the rust checkout, so a
# relative argument is relative to the caller (the legacy patch directory is
# next to this script) rather than to the checkout.
patch_dir="$(cd "$patch_dir" && pwd)"

cd "$rust_dir"

patches=$(find "$patch_dir" -maxdepth 1 -type f -name '*.patch' | sort)
if [ -z "$patches" ]; then
  echo "ERROR: no *.patch files in $patch_dir" >&2
  exit 1
fi

count=$(printf '%s\n' "$patches" | wc -l | tr -d ' ')
if [ "$count" -ne 7 ]; then
  echo "ERROR: expected exactly 7 patches in $patch_dir, found $count" >&2
  printf '%s\n' "$patches" >&2
  exit 1
fi

base=$(git rev-parse HEAD)
echo "Applying $count compiler patches on top of $base ($(git log -1 --format=%s))"
printf '%s\n' "$patches"

# Throwaway committer identity; the author is preserved from each patch.
# shellcheck disable=SC2086
GIT_COMMITTER_NAME=cogwasm-ci \
GIT_COMMITTER_EMAIL=cogwasm-ci@users.noreply.github.com \
  git am $patches

applied=$(git rev-list --count "$base..HEAD")
if [ "$applied" -ne "$count" ]; then
  echo "ERROR: expected $count commits after git am, found $applied" >&2
  exit 1
fi

echo "Patch series applied; HEAD is $(git rev-parse --short HEAD) ($(git log -1 --format=%s))"

# Match what actions/checkout's `submodules: true` used to do for the compiler
# fork: shallowly fetch every submodule at the gitlink recorded in the tree.
echo "Initializing submodules at the patched gitlinks (shallow)..."
git submodule sync --recursive
git submodule update --init --recursive --depth 1

# The pinned YoWASP LLVM fork predates wasi-sdk 34's stricter libc++
# headers: its __wasi__ branches call abort() without including <cstdlib>, so
# LLVMSupport no longer compiles. Apply the compatibility fix to the submodule.
llvm_patch_dir="$(cd "$(dirname "$patch_dir")" && pwd)/llvm-22.1.0-wasip1"
if [ -d "$llvm_patch_dir" ]; then
  for p in "$llvm_patch_dir"/*.patch; do
    [ -e "$p" ] || continue
    echo "Applying LLVM patch $(basename "$p")"
    git -C src/llvm-project apply "$p"
  done
fi

echo "src/llvm-project: $(git submodule status src/llvm-project)"
echo "Submodules ready."
