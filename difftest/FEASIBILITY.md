# Feasibility: running cg_clif's / rustc's own tests through the wasm path

Time-boxed investigation (~25 minutes). The short answer: a **compile-only**
differential over `#![no_std]` rustc UI tests is easy and already works at
~69% on a small sample, but running cg_clif's real run-pass suite through
`rustc.wasm` → wasm needs a `wasm32-wasip1` `std` built by cg_clif and a
compiletest-style harness. That is a multi-week project, not a CI add-on.

## How cg_clif tests are organised

The pinned cg_clif checkout has no `tests/` directory of its own. `test.sh`
runs `./y.sh test`, whose `config.txt` selects suites such as:

* `testsuite.no_sysroot` — `example/*.rs` compiled with a mini core, AOT and JIT;
* `testsuite.base_sysroot` — `example/*.rs` with a real sysroot, plus rustc UI
  tests (`aot.<name>`);
* `testsuite.extended_sysroot` — crates (`rust-random/rand`, `regex`,
  `graviola`, `portable-simd`, stdarch) built and tested.

The actual rustc tests live in the pinned rust checkout
(`toolchain/stage-b/rust/tests/`), mostly `tests/ui/**` with `//@ run-pass` /
`//@ check-pass` directives, plus `tests/codegen*`, `tests/assembly*`, etc.
`y.sh` drives a `compiletest`/custom harness and expects **native executables**
with stdout/stderr comparisons, panic messages and unwinding. None of that maps
directly to a `no_std` wasm cdylib exporting `run(seed)`.

## What was actually attempted

Using the pinned rust source and our existing artifacts, I ran a bounded
compile-only differential over the first 40 UI test files containing
`#![no_std]` (selected by file order, not by expected outcome):

```
for each file:
  LLVM:  rustc --edition=2021 --target wasm32-unknown-unknown --crate-type lib \
                 -C panic=abort -C opt-level=2 file.rs
  ours:  stage-c-host raw ... -Zcodegen-backend=cranelift ... file.rs
```

Results:

| Step | Count |
|---|---:|
| files selected | 40 |
| LLVM compiled successfully under these flags | 16 |
| of those, rustc.wasm + cg_clif also compiled | **11 (69%)** |
| failed under cg_clif | 5 |

All five failures were **missing sysroot crates**, not codegen:
`E0463 can't find crate for std` (3), `libc` (1), `unwind` (1). Their
`#![no_std]` is conditional or their auxiliary crate pulls in std; LLVM gets
those from rustup's prebuilt sysroot, while our sysroot deliberately contains
only `core` + `alloc` + `compiler_builtins`.

Separately, cg_clif's own wasm regression
`example/f128-wasm-abi.rs` (`#![no_std]`, compile-only, f128 sret ABI) compiles
successfully with **both** compilers. `mini_core_hello_world.rs` needs an
extern `mini_core` crate in both paths, so it was not a meaningful comparison.

This is not a pass rate for cg_clif's suite: the sample is small, filtered only
by `#![no_std]`, and excludes anything needing flags/aux crates/special
targets. It does show the compiler frontend + backend can ingest real rustc UI
test sources where the sysroot is sufficient.

## What a real run-pass run would take

1. **A `wasm32-wasip1` `std` built by cg_clif.** Today `sysroot-pure` has only
   `core`/`alloc`/`compiler_builtins`; the only `std` available for
   `wasm32-wasip1` in `toolchain/` was built by LLVM for the native stage1
   (`toolchain/stage-b/rust/build/host/stage1/lib/rustlib/wasm32-wasip1/lib`).
   Mixing that with `rustc.wasm` is an ABI/metadata experiment, and most tests
   expect the exact same compiler build. Building `std` with cg_clif would hit
   the usual wasm blockers: libc/wasi-libc C code is LLVM-built (or must be
   cross-compiled), unwinding is unsupported, and threads/atomics are out.
2. **A `_start`/`main` adapter.** cg_clif tests are `fn main()` programs
   (or `#[test]` harnesses), not cdylibs with `run(seed)`. A wrapper would need
   to call `main`, handle argc/argv, exit codes and `std::process::exit`.
3. **A WASI runner with expectations.** Our difftest runner calls exports and
   compares values. The suite compares stdout/stderr and exit status; running it
   needs `wasmtime run` (or a WASI p1 host) with preopened dirs, env, and
   normalised panic output. `stage-c-host` is a rustc driver, not a general
   WASI CLI.
4. **Filtering.** Exclude tests using `std::thread`, `catch_unwind`,
   backtraces, `/proc`, network, subprocesses, `#[ignore]` platform quirks,
   LLVM-specific assembly/codegen checks, and tests whose expected output
   embeds native pointer sizes. The wasm32 C ABI (sret f128, 32-bit pointers)
   already differs.
5. **Compiletest integration.** `//@ compile-flags`, revisions, aux crates and
   `//@ normalize-*` directives are not honoured by a simple `for f in *.rs`
   loop; a real run needs rustc's `compiletest` or a reimplementation of the
   directives that matter.

## Rough estimate

* **Compile-only no_std smoke subset** (what was attempted): already feasible
  in ~20 s for dozens of files; could be added as an optional CI job, but it
  mostly re-tests the rustc frontend and sysroot availability. Low value next to
  the 48-program runtime differential.
* **cg_clif `example/*.rs` compile-only**: 1 of 15 is a clean no_std wasm
  comparison (`f128-wasm-abi`); the rest need std/mini_core/features. Low
  signal.
* **Full cg_clif run-pass suite under wasm**: multi-week. The gating item is a
  trustworthy cg_clif-built `wasm32-wasip1` std (plus wasi-libc), then a
  compiletest-compatible harness. A realistic first milestone is
  "100 no_std run-pass tests compiled by both compilers and executed in
  Wasmtime with stdout/exit comparison"; that would still be a separate,
  slower job from this ~1-minute difftest.

## Recommendation

Keep the current corpus differential as the CI gate. If more coverage is
wanted cheaply, grow the handwritten corpus and generator; if external suites
are wanted, fund the `wasm32-wasip1` std + compiletest adapter as its own
project, starting with compile-only UI tests and a curated no_std run-pass
subset.
