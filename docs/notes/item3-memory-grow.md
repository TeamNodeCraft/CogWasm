# Item 3: wasm `memory.size` / `memory.grow` and a growable plugin allocator

**Status: PASS.** `core::arch::wasm32::{memory_size, memory_grow}` now lower to
real `memory.size` / `memory.grow` instructions through cg_clif and
clif-to-wasm, with no Cranelift change. Tiny end-to-end programs linked by both
rust-lld and Wild grow memory and use the new pages. Plugin (c) now has a
growable bump allocator instead of a fixed 1 MiB `.bss` heap; it still returns
`2.25` for `eval(1,2,3,4)`, and the Wild artifact is 35,258 bytes rather than
1,083,560.

## Design

Cranelift's ISA-neutral IR has no opcode for either instruction, and the task
forbids patching Cranelift. The two crates therefore agree on a tiny calling
convention:

* **cg_clif** recognizes the LLVM intrinsic symbols
  `llvm.wasm.memory.grow.i32` / `llvm.wasm.memory.size.i32` before the generic
  LLVM-intrinsic fallback. It builds a direct call to an external function with
  a reserved `ExternalName::TestCase` name and the helper signature
  `(i32) -> i32` (grow) or `() -> i32` (size), then writes the call result to
  the intrinsic's return place. The LLVM intrinsics carry the memory index as a
  first argument; stdarch statically requires it to be zero, so cg_clif drops
  it and forwards only the value operand.
* **clif-to-wasm** sees the direct call, matches the exact reserved test-case
  name, validates the call signature (i32 memory only, non-tail), and emits
  `memory.grow` / `memory.size` for `ModuleLayout::memory_index` instead of a
  call. No `Env` lookup and no object symbol are involved, so nothing leaks into
  the relocatable object or the linker's import list.

Reserved names (public as `clif_to_wasm::WasmIntrinsic::symbol_name`):

| wasm instruction | reserved call name | signature |
|---|---|---|
| `memory.grow` | `__clif_to_wasm_memory_grow` | `(i32) -> i32` |
| `memory.size` | `__clif_to_wasm_memory_size` | `() -> i32` |

Why `ExternalName::TestCase`: it is the only Cranelift external-name form that
carries a string without requiring a module declaration/symbol. A normal
`declare_function(..., Linkage::Import)` would also declare an undefined symbol
in the object, which the linker would then report (clif-to-wasm only prunes
unused globals/tables, not functions). Using a reserved name keeps the mechanism
inside `compile_function` and keeps the object clean.

The clif-to-wasm side is a new public `WasmIntrinsic` enum plus a check in the
direct-call lowering. Invalid signatures, tail calls, and memory64 return
`Error::Unsupported` instead of miscompiling.

## Files changed

Toolchain sources (left with the patches applied, so `git diff`/`git status`
shows them):

| file | change |
|---|---|
| `toolchain/clif-to-wasm/src/wasm_intrinsic.rs` | new: `WasmIntrinsic`, reserved names, expected signatures |
| `toolchain/clif-to-wasm/src/env.rs` | document that `WasmIntrinsic` reserves a few `TestCase` names |
| `toolchain/clif-to-wasm/src/lib.rs` | declare/export `WasmIntrinsic` |
| `toolchain/clif-to-wasm/src/lower.rs` | recognize reserved direct calls, emit `memory.grow`/`memory.size`, validate |
| `toolchain/clif-to-wasm/tests/memory.rs` | new: compile CLIF, check emitted operators, run in Wasmtime, negative tests |
| `toolchain/cg_clif/src/intrinsics/llvm.rs` | lower the two LLVM intrinsics to the reserved helper calls |

Stage-C files:

| file | purpose |
|---|---|
| `compiler/patches/clif-to-wasm/0001-lower-wasm-memory-grow-and-size.patch` | clif-to-wasm patch (includes test) |
| `compiler/patches/cg_clif/0001-wasm-lower-memory-grow-and-size.patch` | cg_clif patch, on top of stage-A |
| `guests/mem_grow/` | tiny no_std grow/size/probe program |
| `guests/mem_grow_harness/` | Wasmtime checker for that program |
| `guests/c_plugin/` | stage-A plugin (c) with a growable bump allocator |
| `guests/c_plugin_harness/` | checks `eval == 2.25` and that memory grew |
| `compiler/scripts/run-item3.sh` | builds/tests everything, both linkers |
| `docs/results/item3.log` | complete output of `run-item3.sh all` |
| `docs/results/item3-sizes.txt` | section-level sizes for the table below |

## Patches and bases

* `clif-to-wasm`: base `0e4e0458f5a190a7fcdbb2298d9ad780678c44bc` (v0.3.0).
  The patch adds two files and edits `src/env.rs`, `src/lib.rs`, and
  `src/lower.rs`; it applies
  with `git am` or `git apply` to a clean pinned checkout.
* `cg_clif`: base `4f78fd3e5f2ce8a688865b4a6197e6d33211c09c` **plus**
  `docs/notes/history/stage-a/0001-cg_clif-wasm-backend.patch` (stage A).
  The item-3 patch touches only `src/intrinsics/llvm.rs`, which the stage-A
  patch does not modify, so it also applies to the upstream revision directly.
  `git am`/`git apply` both work.

Both patches were generated with `git format-patch` and carry a commit message.
Verified by extracting clean `git archive HEAD` trees, applying the stage-A patch
to cg_clif, and running `git apply --check` for both item-3 patches.

## How to reproduce

All commands from the repository root:

```sh
./compiler/scripts/run-item3.sh all
# or individually:
./compiler/scripts/run-item3.sh build     # rebuild toolchain/target/cg_clif/...dylib
./compiler/scripts/run-item3.sh test      # clif-to-wasm tests, including tests/memory.rs
./compiler/scripts/run-item3.sh harness   # build the two Wasmtime harnesses
./compiler/scripts/run-item3.sh programs  # build both programs with rust-lld and Wild
./compiler/scripts/run-item3.sh run       # execute both programs from both linkers
```

The script sets the isolated toolchain:

```sh
export RUSTUP_HOME=$PWD/toolchain/rustup
export CARGO_HOME=$PWD/toolchain/cargo
export PATH=$CARGO_HOME/bin:$PATH
export RUSTUP_TOOLCHAIN=nightly-2026-10-19
export CARGO_TARGET_DIR=$PWD/toolchain/target/lld-plugins
DYLIB=$PWD/toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib
WILD=$PWD/toolchain/target/wild/release/wild
```

The dylib itself (as in the task statement; from the repository root):

```sh
cd toolchain/cg_clif
RUSTUP_TOOLCHAIN=nightly-2026-10-19 \
CARGO_TARGET_DIR=$PWD/../target/cg_clif cargo build --release
# -> toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib
```

The exact program build (both programs, rust-lld; add `-Clinker=$WILD
-Clinker-flavor=wasm-ld` and a different target dir for Wild):

```sh
cd guests/mem_grow   # or c_plugin
CG_CLIF_WASM_STRICT=1 \
RUSTFLAGS="-Zcodegen-backend=$DYLIB -Cpanic=abort" \
cargo build --release --target wasm32-unknown-unknown \
    -Zbuild-std=core,alloc,compiler_builtins
```

The harnesses build with stable `1.99.1` and Wasmtime 50.0.2:

```sh
cd guests/mem_grow_harness   # and c_plugin_harness
RUSTUP_TOOLCHAIN=1.99.1 CARGO_TARGET_DIR=$PWD/toolchain/target/harness \
    cargo build --release
```

## Test results

### clif-to-wasm

`cargo test --features module --test memory --test clif --test object`
(with `RUST_LLD` pointing at the nightly `rust-lld`): **15 tests pass**.

* `tests/memory.rs::memory_size_and_grow_instructions`: parses CLIF that calls
  both reserved names, validates the wasm, asserts exactly two `memory.size`
  and two `memory.grow` operators, then instantiates and checks:
  initial size `32` pages (the harness's `MEMORY_PAGES`), `grow(2)` returns
  `32`, size becomes `34`, `grow(1)` returns `34`, size becomes `35`, and a
  host write/read in the newest page round-trips.
* `wrong_signature_is_rejected`: a `(i64) -> i32` call is `Unsupported` (the
  harness substitutes a valid stub, so the module still validates).
* `memory64_is_rejected`: with `AddressType::I64`, all three helper calls are
  `Unsupported` with "64-bit memory".
* Existing `clif_runtests` (all `.clif` files, five configs) and all 10
  `tests/object.rs` tests still pass.

### `mem_grow` end to end

`mem_grow` exports `pages()`, `grow(delta)`, and `probe(addr, value)`; the
harness runs one instance so growth persists across calls. Both linkers give
the same results:

```
initial memory.size: 16 pages          # linker-provided minimum
memory.grow(2) returns old size: 16
memory.size after grow: 18
host memory.size after grow: 18
host read/write at 0x100000: ok        # first new page
guest write/read at 0x10fff0: 0x12345678, host sees it
memory.grow(0) returns 18
memory.grow(65536) returns -1, size stays 18
PASS mem_grow: initial=16 pages, grew to 18 pages
```

Artifact sizes: **308 bytes** with rust-lld, **339 bytes** with Wild.

### Plugin (c) with the growable allocator

`c_plugin.wasm` is 34,787 bytes (rust-lld) / 35,258 bytes (Wild). Both pass:

```
initial memory: 17 pages
alignment: in=0x110000 out=0x110020 aligned64=0x110040
after wasm_alloc: 18 pages
after eval: 18 pages
eval rc=0 result=2.25
second eval: rc=0 result=2.25, memory 18 pages
PASS c_plugin: eval(1,2,3,4) = 2.25, memory 17 -> 18 -> 18 pages
```

The harness also checks 8-byte and 64-byte alignment of allocator returns and
runs `eval` twice (the second call reuses the bump heap).

The existing stage-A host also still passes:
`host plugin <wasm> 1 2 3 4` prints `eval rc=0 result=2.25` for both.

## Sizes and the Wild `.bss` problem

Stage-C section sizes (all measured, not estimated):

| artifact | total | code | data | memory min |
|---|---:|---:|---:|---:|
| c_plugin, rust-lld, stage A fixed `.bss` | 34,502 | 22,877 | 2,772 | **33 pages** |
| c_plugin, Wild, stage A fixed `.bss` | **1,083,560** | 22,877 | **1,051,866** | 33 pages |
| c_plugin, rust-lld, stage C growable | 34,787 | 23,162 | 2,772 | **17 pages** |
| c_plugin, Wild, stage C growable | **35,258** | 23,162 | **3,280** | **17 pages** |
| LLVM O2, rust-lld (reference) | 7,274 | — | — | — |

The fixed heap was 1 MiB of zero-initialized static memory. rust-lld represented
it as memory minimum pages (hence 33 = 17 + 16 pages) with no file bytes; Wild
materialized it as a literal data segment, hence the ~1.05 MB `DATA`. With the
growable allocator the data section is 2,772 bytes (rust-lld) / 3,280 bytes
(Wild), memory minimum is 17 pages for both, and Wild is within 471 bytes of
rust-lld instead of 1,049,058 bytes larger. The remaining 23,162-byte code
section is the same order as stage A's 22,877 and comes from the pure-Rust
`core`/`alloc` built by cg_clif (the stage-A note: object granularity), not from
the allocator. Against LLVM's 7,274-byte plugin it is ~4.8x, but the 1 MiB
blow-up is gone.

## Limitations

* **32-bit memories only.** `llvm.wasm.memory.grow.i64`/`size.i64` (wasm64) are
  not handled; clif-to-wasm rejects the i32 helpers with a 64-bit memory and
  cg_clif does not match the i64 names. `memory.size`/`grow` use
  `ModuleLayout::memory_index`; cg_clif uses 0.
* **Direct calls only.** `call_indirect`/`return_call` to a reserved name is not
  lowered (the latter is explicitly rejected). These are compiler intrinsics,
  not user-callable APIs.
* **The allocator is a bump allocator.** It grows by whole pages and `dealloc`
  is a no-op; each instance leaks until it is dropped. That is sufficient for
  one `eval` and for the cogwasm-host. A long-lived instance that allocates
  repeatedly should use dlmalloc-style free lists (the vendored
  `dlmalloc-0.2.14/src/wasm.rs` is the natural reference) or a free-list bump.
  The allocator starts at the current memory end, after static data and the
  linker-provided shadow stack, so it never reuses stack/static pages.
* **`ExternalName::TestCase` is documented as test-only.** It is the only
  string-keyed external name that needs no module symbol. An upstream version
  of clif-to-wasm might prefer an explicit `Env`/declaration hook instead; the
  instruction lowering itself would not change.
* cg_clif's own test harness (`test.sh` + rustc test suite) was not run; the
  end-to-end programs and harnesses are the cg_clif-side test. A cg_clif unit
  test would need the rustc test harness, which is not set up in this
  experiment.
* `CG_CLIF_WASM_STRICT=1` is used for every program build, so no function was
  silently replaced by a trap.

## Upstream-ready?

* **clif-to-wasm: yes, with one design discussion.** The lowering is ~110 lines
  including the public enum and docs, adds a focused integration test, has no
  new dependency, and follows the crate's existing `Error::Unsupported` style.
  The open question is whether maintainers accept a reserved-name convention
  (and specifically a `TestCase` name) versus extending `Env`. The alternative
  is a small `Env::wasm_intrinsic(FuncIndex)` method, but that requires the
  object module to track symbol names and complicates the non-relocatable
  builder. The current mechanism is the smallest one that works in both layouts.
* **cg_clif: yes.** The patch is ~55 lines, local to `src/intrinsics/llvm.rs`,
  uses only public Cranelift APIs, and leaves the native path untouched. It
  depends on the clif-to-wasm patch for `WasmIntrinsic`, so the two should land
  together (or cg_clif can spell the two strings itself).
