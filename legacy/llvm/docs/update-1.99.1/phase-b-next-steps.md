# Phase B next steps (not performed in phase A)

Phase A did **not** push, create repos, or run CI.

## 1. Create the rust fork and push the rebased branch

The recipe now points at `AlexMitchellMus/rust` branch
`compile_rustc_for_wasm-1.99.1`; **that fork does not exist yet**.

```sh
# from .../rust-wasm-compiler/rust
git remote add fork https://github.com/AlexMitchellMus/rust.git
git push -u fork compile_rustc_for_wasm-1.99.1
```

Create the fork first (GitHub UI or `gh repo fork oligamiq/rust --clone=false`).
Forking `oligamiq/rust` preserves the original patch history. The branch head
must be `f5e1d56ab21` (or the current local head).

## 2. Push the recipe branch

```sh
# from .../rust-wasm-compiler/rust_wasm
git push -u origin update-1.99.1
```

Commit: `1ff915a Update build recipe to Rust 1.99.1`.

The workflows' inner `actions/checkout` steps still use
`ref: 'gh-actions'` for `rust_wasm` itself (config_rewriter). Either merge
`update-1.99.1` into `gh-actions`, or change those refs to `update-1.99.1`
before running.

## 3. Run the build

1. Run `rustc_llvm_with_lld.yml` with `job=install` first. This builds
   `rustc.wasm` (abort + unwind) and is the expensive step.
2. If it succeeds, run `job=dist-linux`, `dist-macos`, `dist-windows`.
3. Then run `create_release.yml` with the new version (e.g. `v4.0.0`).

Expected first failure points, in order:

- **LLVM 22.1.0+wasm vs rustc 1.99.1.** Rust 1.99.1 supports LLVM >= 21 and has
  LLVM-22/23 guards, but this combination has not been built. If it fails, port
  YoWASP's wasm patches to LLVM 23.1 (the version Rust 1.99.1 pins) or switch
  to a newer wasm LLVM fork.
- **wasi-sdk 34 layout.** The recipes assume `wasi-sdk-34.0-*` extracts to
  `wasi-sdk-34.0-<host>` and that `bin/clang` + `share/wasi-sysroot` exist.
  The compiler branch's `config.llvm.toml` uses relative `wasi-sdk/...`; the
  recipe still overwrites it with the toolchain generator's config and rewrites
  the `wasi-sdk-*` paths with a version-agnostic sed.
- **Main-thread stack.** rustc runs on the wasm main thread; the target spec
  does not set `-z stack-size` for rustc.wasm. If the compiler overflows,
  add `-Wl,-z,stack-size=<N>` / `-C link-arg` and raise wasmtime's
  `max_wasm_stack`/`async_stack_size`.
- **panic=unwind.** The unwind job may need wasm EH changes in 1.99.1.

## 4. Verify the artifact

With wasmtime 46.0.3 + `wasmtime-wasi-threads` (the current host pin):

```sh
wasmtime run -Sthreads=y -Spreview2=n \
  --dir work::/ --dir dist --dir sysroot \
  dist/bin/rustc.wasm --sysroot /sysroot --version
```

Expected: `rustc 1.99.1`. Then repeat the spike's plugin compile / diagnostics
/ hot-reload cases.

## 5. After the first release

- Keep `wasm32-wasip1-threads` for v4.0.0. The single-threaded host
  (`wasm32-wasip1` + latest wasmtime) is a separate spike; see
  `threads-assessment.md`.
- Consider making the compiler branch's `config.llvm.toml` authoritative and
  dropping the toolchain generator's config overwrite.
- Add the `wasi-sdk`/LLVM versions to the release notes and README.
