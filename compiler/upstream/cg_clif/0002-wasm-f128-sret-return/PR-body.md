# wasm: return f128 through an sret pointer, matching the C ABI

> **Draft research note, not submission text.** A human owns any future
> upstream contribution and must write the issue/PR in their own words under
> the project's current AI policy. See `../../README.md` and
> `CogWasm/CONTRIBUTING.md`.

## Problem

WebAssembly's basic C ABI returns `f128` through a hidden sret pointer, and
rustc's `extern "C"` ABI on wasm does the same. cg_clif's compiler-generated
libcall declarations used a direct `f128` return instead.

`compiler_builtins` both defines and calls `f128` libcalls such as `__multf3`.
When its own Rust declaration was seen next to a generated call, module
declaration failed with:

```text
attempt to declare `__multf3` with signature ... f128 ...
but it was already declared with signature (sret ...)
```

This blocks building `core`, `alloc` and `compiler_builtins` for wasm with a
wasm-hosted cg_clif, and therefore blocks a pure-cg_clif wasm sysroot.

## Fix

Treat `wasm32`/`wasm64` like s390x and x86_64 Windows in
`FunctionCx::lib_call`: pass and return `f128` indirectly, and load the result
from the return slot. The existing s390x/Windows path already handles the
argument and result ABI, so this only extends the `indirect_f128` condition.

Add `example/f128-wasm-abi.rs`, a compile-only regression that both emits a
compiler-generated `__multf3` call (`a * b`) and declares the same symbol with
`extern "C"`. Add `aot.f128_wasm_abi` to the base sysroot suite, and enable it
in `config.txt` so a wasm-target run actually executes it.

## Tests

Against a scratch tree containing cg_clif `4f78fd3` + the wasm backend series
+ this patch (nightly-2026-10-19):

```text
TARGET_TUPLE=wasm32-wasip1 ./test.sh
...
[AOT] f128_wasm_abi
[AOT] wasm_memory_intrinsics
...
EXIT=0
real 79.62
```

The run built `core`, `compiler_builtins`, `std` and `alloc` for
wasm32-wasip1 with cg_clif and then ran the two compile-only cases. Only those
two entries were left enabled in the scratch `config.txt`.

Direct backend compile checks, using the built
`librustc_codegen_cranelift.dylib`:

```sh
rustc --target wasm32-wasip1 \
  -Zcodegen-backend=$DYLIB -Cpanic=abort --crate-type lib \
  example/f128-wasm-abi.rs          # exit 0
rustc --target wasm32-unknown-unknown \
  -Zcodegen-backend=$DYLIB -Cpanic=abort --crate-type lib \
  example/f128-wasm-abi.rs          # exit 0
```

Negative check: with the wasm backend but **without** this patch, the same
compile fails with the incompatible-signature error. That confirms the test
exercises the fix.

`rustfmt --check` on the touched files exits 0. Full details:
`test-results.txt`.

## How found

While building `compiler_builtins` for wasm with cg_clif. The f128 libcalls
were the remaining blocker for a sysroot built entirely by cg_clif; the
signature mismatch was reproducible with a small example.

## Upstream status

No equivalent change exists on current `main`:

* `src/abi/mod.rs` at
  `a4af0a9fd1ce26e03364c07ab0c482d084fa97fc` still has
  `indirect_f128 = arch == S390x || (is_like_windows && arch == X86_64)`;
  wasm is not included.
* `git log -S'indirect_f128' -- src/abi/mod.rs` finds
  `6a6f2419` ("fix f128 libcall abi in the cranelift backend", 2026-07-10),
  which introduced the `indirect_f128` condition for s390x/x86_64 Windows and
  is not about wasm. Earlier ABI work is
  https://github.com/rust-lang/rustc_codegen_cranelift/pull/1546
  ("Fix the ABI for libcalls"), also not wasm-specific.
* GitHub issue/PR searches for f128+wasm in cg_clif return no equivalent.
  Related, but not the same: https://github.com/rust-lang/rustc_codegen_cranelift/issues/1717
  (`compiler_builtins` cannot call functions through upstream
  monomorphizations).

Recommended disposition: keep.

## Conflicts

Rebased from `4f78fd3e5f2ce8a688865b4a6197e6d33211c09c` to
`a4af0a9fd1ce26e03364c07ab0c482d084fa97fc`:

* `src/abi/mod.rs` is unchanged upstream; the code hunk applies as-is.
* `build_system/tests.rs` gained an unrelated `aot.tls_conflicting_declarations`
  test and `config.txt` gained its entry. The new test and config insertion
  are placed before the existing `powi` case; the only effect is a line
  offset. Default 3-line context also picks up the new TLS line in
  `config.txt`, so applying this patch to the older pinned tree needs
  `git apply -C1`; all code hunks apply.
* `rustfmt` made no changes.

* The f128 and memory-intrinsics patches both add a test entry immediately
  after `aot.float-minmax-pass` in `build_system/tests.rs` and `config.txt`.
  They are independent PRs; if they are stacked, the second one needs a
  trivial rebase of those two lines. The code changes do not conflict.

## Licence

MIT OR Apache-2.0, as a contribution to the Rust project.

## Notes

* **Dependency:** the test is only reachable with a wasm codegen backend.
  Upstream cg_clif does not support wasm targets yet (`build_isa` rejects
  them), so this patch is a prerequisite/companion of the wasm backend series,
  not a standalone user-visible feature. The ABI change itself is a two-line
  extension of an existing s390x/Windows path and is inert on native targets.
* The test is skipped on non-wasm targets by design. For upstream CI it would
  only run in a job that sets `TARGET_TUPLE` to a wasm target and has the wasm
  backend; document that when the backend series lands.
* LLVM/Clang's WebAssembly basic C ABI is the reference for the hidden sret
  pointer. A cross-check against `clang --target=wasm32` returning `__float128`
  would be useful in review but is not required for the Rust-side fix.
* Best venue: cg_clif PR, coordinated on `#wg-cranelift`; stack it with the
  wasm backend and the memory-intrinsics patch.
