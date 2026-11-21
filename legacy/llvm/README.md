# Legacy LLVM build path

This directory keeps the original path that compiles rustc **with LLVM and
LLD** to WebAssembly. It is **manual-only**: nothing here is built on push.

The workflows that still use it are all `workflow_dispatch` only:

- [`build_llvm.yml`](../../.github/workflows/build_llvm.yml) builds the Wasm
  LLVM/Clang tools.
- [`rustc_minimal.yml`](../../.github/workflows/rustc_minimal.yml) builds the
  minimal rustc.wasm.
- [`rustc_llvm_with_lld.yml`](../../.github/workflows/rustc_llvm_with_lld.yml)
  builds the LLVM/LLD-based compiler and dist jobs.
- [`create_release.yml`](../../.github/workflows/create_release.yml) packages
  their artifacts.

The shipped CogWasm compiler does not use this path. The A/B baselines in
`difftest` and `bench` use the native LLVM rustc from rustup, not this build.

## Layout

- `patches/rust-1.99.1/` — the `rustc.wasm` compiler patch series applied to
  `rust-lang/rust` 1.99.1.
- `patches/llvm-22.1.0-wasip1/` — the compatibility patch for the YoWASP LLVM
  fork used by that series.
- `config_rewriter/` — the standalone `config_rewriter` / `wasm_optimizer`
  crate; it keeps its own empty `[workspace]` table.
- `ci/apply-rust-patches.sh` — applies the compiler patch series and then
  initializes submodules.
- `docs/update-1.99.1/` — rebase notes and original patch inventory for the
  1.99.1 series.
- `toolchain-scripts/` — the musl, crosstool-ng, NetBSD and Solaris toolchain
  scripts plus the `*.defconfig` files.
