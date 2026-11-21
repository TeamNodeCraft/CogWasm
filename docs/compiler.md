# The compiler build: no LLVM in the plugin path, no wasi-threads, growable heap

The CogWasm compiler closes the remaining non-Rust pieces of the stage-A/B plugin pipeline:

1. **rustc.wasm runs on the latest stable Wasmtime with plain WASI p1.** It is
   now built for `wasm32-wasip1` (non-threads): rustc's compiler thread and
   rustc_codegen_ssa's coordinator thread are replaced by direct execution on
   the calling thread, and the guest stack is sized at link time. There is no
   `wasi:thread-spawn` import. Host: **Wasmtime 50.0.2** (latest stable;
   51.0.0 is still an RC), no `wasmtime-wasi-threads`.
2. **The plugin sysroot is built by rustc.wasm itself.** `core`, `alloc` and
   `compiler_builtins` for `wasm32-unknown-unknown` are all compiled by
   rustc.wasm + cg_clif, so their metadata matches rustc.wasm exactly. The
   E0514 "nightly vs -dev" trap is gone. The f128 ABI conflict that blocked
   `compiler_builtins` is fixed in cg_clif: wasm returns f128 through an sret
   pointer, matching LLVM's wasm C ABI.
3. **`memory.size` / `memory.grow` work, and the plugin heap grows.** cg_clif
   and clif-to-wasm agree on two reserved helper calls, which clif-to-wasm
   turns into real wasm instructions. Plugin (c) uses a bump/free-list over
   `memory.grow` instead of a fixed 1 MiB `.bss` heap; Wild and rust-lld now
   produce the same 35 KB artifact, down from 1.08 MB with Wild.
4. **Re-measured on the latest Wasmtime**, median of 5, against the earlier
   stage-B (Wasmtime 46) and A.2 tables. After the 2026-11-19 speed batch the
   headline is: plugin compile inside rustc.wasm **79.4 ms**, link **6.5 ms**,
   edit-to-result **96.2 ms**, `eval(1,2,3,4) == 2.25`. A per-host precompiled
   `.cwasm` loads in **18.7 ms** (vs 37.6 s cold, 461 ms from Wasmtime's
   cache), and a one-line edit with `--incremental` takes **97.7 ms**
   steady-state (14.1% less than a cold compile). See the speed-batch section
   of [`notes/history/stage-a-results.md`](notes/history/stage-a-results.md).

Full numbers and comparisons: [`results.md`](results.md) and the appended
[`notes/history/stage-a-results.md`](notes/history/stage-a-results.md) stage-C
section.

## Pins

The existing pinned toolchain/downloads under `../toolchain/` (gitignored) are
reused. New or changed pins:

| Component | Source | Revision / version |
|---|---|---|
| rustc source | https://github.com/rust-lang/rust | `b6eeb93c992027a03e1fd120b6652546396b142c` (`nightly-2026-10-19`) |
| prior-art rust series | https://github.com/ryancbuckley/rust-playground-inbrowser | `f5298630dda0f2dc2b44f6ae1b54add7a77d8316`, 23 patches |
| cg_clif | in-tree, from that series | upstream `4f78fd3e…` |
| clif-to-wasm | https://github.com/ryancbuckley/clif-to-wasm (`toolchain/clif-to-wasm`) | `0e4e0458f5a190a7fcdbb2298d9ad780678c44bc` (0.3.0) + compiler patch |
| wild | https://github.com/wild-linker/wild | `fd79578abb…`, 0.11.0 |
| host | crates.io | **wasmtime / wasmtime-wasi 50.0.2**, `p1` feature only |
| wasmparser | crates.io | 0.263 |
| wasi-sdk | https://github.com/WebAssembly/wasi-sdk | 34.0 arm64-macOS |

`rustc.wasm`: **143,849,756 bytes**, SHA-256
`3c7ca043ac35adcc994de4f335aa71ded3270dc2aa92ce8e21ceee7a93c0302f`.
It imports 26 WASI p1 functions plus `playground.spawn`, exports `memory` and
`_start`, and has **no** `wasi.thread-spawn`, `env.memory`, LLVM import,
`rustc_codegen_llvm` or `rustc_llvm` symbol.

## Layout

```
compiler/                           # build rustc.wasm and the pure sysroot
├── bootstrap.toml                  # wasm32-wasip1 build config (LLVM only for stage1)
├── run.sh                          # plugins/check/bench/a2
├── scripts/
│   ├── setup-sources.sh            # rust checkout, prior-art patches, wasi-sdk
│   ├── build-rustc-wasm.sh         # apply compiler patches + x.py install
│   ├── build-host.sh               # Wasmtime 50.0.2 p1 host
│   ├── build-pure-sysroot.sh       # core/alloc/compiler_builtins via rustc.wasm
│   ├── rustc-wasm-wrapper.sh       # Cargo RUSTC wrapper
│   └── run-item3.sh                # memory.grow regression suite
├── patches/                        # rust, cg_clif, clif-to-wasm, wild patches
└── upstream/                       # upstream-ready PR drafts
crates/
├── cogwasm-host/                   # Wasmtime 50.0.2 p1 host (no wasi-threads)
├── cogwasm-runtime/                # helpers shared with the benchmark host
├── cogwasm-supervisor/             # reference compile supervisor
└── difftest-host/                  # differential-test runner
guests/                             # wasm32-unknown-unknown programs + harnesses
├── a_add, b_alloc, c_plugin        # stage-A programs, (c) with a growable heap
├── e_math, f_geometry              # A.2 programs, growable heap
├── a2_harness                      # A.2 load/bench harness (standalone crate)
└── mem_grow, *_harness             # item-3 end-to-end tests
docs/
├── compiler.md (this file), results.md
├── notes/                          # design notes
├── notes/history/                  # retired stage-A/B notes and raw results
└── results/                        # measurements + logs
```

## Reproduce

```sh
# 0. Fetch the pinned rust source, prior-art patches and wasi-sdk (idempotent).
compiler/scripts/setup-sources.sh

# 1. Apply the compiler patches and build rustc.wasm (~3.5 min incremental).
compiler/scripts/build-rustc-wasm.sh

# 2. Build the CogWasm Wasmtime 50.0.2 host. The sysroot builder needs it,
#    because rustc.wasm is executed through this host while Cargo compiles
#    core/alloc/compiler_builtins.
compiler/scripts/build-host.sh

# 3. Build the plugin sysroot with rustc.wasm itself (strict, no trap stubs).
CG_CLIF_WASM_STRICT=1 compiler/scripts/build-pure-sysroot.sh

# 4. Build plugins with rustc.wasm, link with Wild, eval (c).
compiler/run.sh plugins
compiler/run.sh check
compiler/run.sh a2check   # A.2 (e)/(f) correctness, no timing assertions

# 5. Stage-B comparison table (history) on Wasmtime 50.0.2 (median of 5).
compiler/run.sh bench

# 6. A.2 load/run measurements.
compiler/run.sh a2

# 7. Item-3 regression suite (clif-to-wasm tests + mem_grow + plugin).
compiler/scripts/run-item3.sh all
```

`build-rustc-wasm.sh` reuses the pinned source/build tree under
`toolchain/rust` and applies every patch under `compiler/patches/rust/`,
`compiler/patches/cg_clif/` and `compiler/patches/clif-to-wasm/` in sorted order, idempotently.
`x.py install` must succeed: the former
`generate-copyright` failure for the out-of-tree clif-to-wasm path dependency
is fixed by `compiler/patches/rust/0002-generate-copyright-path-deps.patch`. On any
non-zero exit the script prints the tail of the log and stops; it only copies
`build/wasm32-wasip1/stage2/bin/rustc.wasm` after a successful install.

## Environment variables

The scripts and the Rust host read the `COGWASM_*` names below. For one
release, every read site also accepts the old `STAGE_C_*` spelling as a
fallback, so an existing shell or CI environment keeps working; exports and
sets always use the new name. The fallback is removed in the next release.

| New name | Old fallback | Meaning |
|---|---|---|
| `COGWASM_STAGE1` | `STAGE_C_STAGE1` | native stage1 `rustc` (default: `toolchain/rust/build/host/stage1/bin/rustc`) |
| `COGWASM_SYSROOT` | `STAGE_C_SYSROOT` | plugin sysroot added to target-crate compiles |
| `COGWASM_STD_SRC` | `STAGE_C_STD_SRC` | stage1 `rust-src` checkout |
| `COGWASM_REAL_SRC` | `STAGE_C_REAL_SRC` | real rust checkout the `rust-src` symlink points at |
| `COGWASM_RUSTC_WASM` | `STAGE_C_RUSTC_WASM` | `rustc.wasm` to run |
| `COGWASM_WILD` | `STAGE_C_WILD` | Wild linker binary |
| `COGWASM_HOST` | `STAGE_C_HOST` | `cogwasm-host` binary |
| `COGWASM_CACHE` | `STAGE_C_CACHE` | Wasmtime compiled-module cache |
| `COGWASM_HOST_TRIPLE` | `STAGE_C_HOST_TRIPLE` | host triple for the stage1 `rust-src` path |
| `COGWASM_HOST_TOOLCHAIN` | `STAGE_C_HOST_TOOLCHAIN` | stable toolchain that builds the host (default `1.99.1`) |
| `COGWASM_TIME` | `STAGE_C_TIME` | print `TIMING` lines from the host |
| `COGWASM_SPAWN_DEBUG` | `STAGE_C_SPAWN_DEBUG` | print the linker spawn argv |

`CG_CLIF_WASM_STRICT` is cg_clif's own variable and is unchanged.

## Item 1: run on the calling thread, latest Wasmtime

Two forced spawns were removed for `cfg(target_arch = "wasm32")`:

* `rustc_interface::run_in_thread_with_globals` runs the compiler closure
  directly instead of `thread::Builder::spawn_scoped(...).join()`. The wasm
  guest stack is fixed at link time with
  `-Clink-arg=-Wl,-z,stack-size=33554432` (32 MiB); `RUST_MIN_STACK` no longer
  applies.
* `rustc_codegen_ssa::back::write` runs the codegen coordinator
  synchronously: each submitted CGU is optimized/codegenned immediately and
  `join` finalizes the collected modules. The blocking
  `SharedEmitterMain::check` is skipped before the join (the emitter is still
  owned by the coordinator state at that point).

Other spawns were checked: the parallel path (`rustc_thread_pool`), the
jobserver helper and codegen worker threads are only reached with
`parallel = Some(..)`; the wasm build has `-Zthreads=1`/no
`parallel-compiler` feature, so they are not called. Linking still goes
through the existing `playground.spawn` host import.

The host is [`crates/cogwasm-host/`](../crates/cogwasm-host/), a port of the earlier Wasmtime 46 host to
Wasmtime 50.0.2:
plain `wasmtime_wasi::p1`, guest memory is a normal `Memory`, no
`wasmtime-wasi-threads`. `check` still returns **2.25**.

The host code that is identical to the benchmark host's copy — the Wasmtime
error adapter and the `playground.spawn` summary type — lives in
[`crates/cogwasm-runtime`](../crates/cogwasm-runtime). Engine setup, the
WASI store, spawn recording and startup reporting stay here because those
differ between the two hosts in observable ways.

## Item 2: a sysroot built by rustc.wasm

`guests/c_plugin` is built with `-Zbuild-std=core,alloc,compiler_builtins`
under `RUSTC=compiler/scripts/rustc-wasm-wrapper.sh`; `core`, `alloc` and
`compiler_builtins` are produced by **rustc.wasm** and copied to
`toolchain/sysroot`. The metadata version matches rustc.wasm
(`rustc 1.101.0-dev`), so rustc.wasm accepts the rlibs.

The one source fix needed was the **f128 ABI**. cg_clif's generated f128
libcall used `(f128, f128) -> f128`, while the same symbol declared by Rust
`extern "C"` on wasm used the wasm C ABI's hidden sret pointer. cg_clif now
treats wasm32/wasm64 like s390x and x86_64 Windows for f128 returns: pass a
hidden return pointer, call, and load the result. `__multf3`, `__floatsitf`,
`__subtf3`, … all agree.

Strict mode (`CG_CLIF_WASM_STRICT=1`, which makes clif-to-wasm's
"replace with trap" an error) builds the whole sysroot with **zero**
unsupported/trap-stub diagnostics. Plugins (a), (b), (c), (e), (f) also build
strict with this sysroot.

## Item 3: memory.grow and a growable heap

Cranelift has no `memory.grow`/`memory.size` opcode, so the two crates agree on
reserved direct-call names (`clif_to_wasm::WasmIntrinsic`):

| wasm | reserved call | signature |
|---|---|---|
| `memory.grow` | `__clif_to_wasm_memory_grow` | `(i32) -> i32` |
| `memory.size` | `__clif_to_wasm_memory_size` | `() -> i32` |

cg_clif intercepts `llvm.wasm.memory.grow.i32` / `size.i32` before its generic
LLVM-intrinsic trap fallback; clif-to-wasm validates the call and emits the
real instruction for `ModuleLayout::memory_index`. `ExternalName::TestCase`
keeps the helper out of the object's symbol/import tables. Tests cover
semantics, wrong signature and memory64 rejection.

Plugin (c) replaces its 1 MiB `.bss` array with a bump allocator that starts at
`memory.size() * 65536` and calls `memory.grow` by whole pages. `dealloc` is a
no-op (instance lifetime), which is fine for a plugin that is instantiated,
evaluated and dropped. A.2 programs (e) and (f) use the same allocator.

**Wild result:** with the fixed `.bss` heap, Wild wrote `.bss` as literal data
(1,051,866 data bytes for plugin c, 1.08 MB total). With the growable heap,
plugin c's data is 2,420 bytes for both Wild and rust-lld, memory minimum is
17 pages, and Wild is actually 19 bytes *smaller* than rust-lld (34,921 vs
34,940). At this scale Wild's `.bss` handling is no longer a problem; no Wild
patch is needed. Exact numbers are in
[`notes/item3-memory-grow.md`](notes/item3-memory-grow.md).

## Patch list

| # | Patch (against pinned upstream) | One line | Upstream-ready? |
|---|---|---|---|
| 1 | [`compiler/patches/rust/0001-…calling-thread.patch`](../compiler/patches/rust/0001-wasm-run-compiler-and-codegen-on-calling-thread.patch) | wasm: run the compiler and codegen coordinator on the calling thread | **Yes, with review.** Two-file rustc change; needs a wasm-host test, which this repo provides as an end-to-end script rather than an in-tree rustc test. |
| 2 | [`compiler/patches/cg_clif/0002-…f128-sret-return.patch`](../compiler/patches/cg_clif/0002-wasm-f128-sret-return.patch) | wasm: return f128 through an sret pointer, matching the C ABI | **Yes.** Includes an example regression and a cg_clif test-suite case. |
| 3 | [`compiler/patches/clif-to-wasm/0001-…memory-grow-and-size.patch`](../compiler/patches/clif-to-wasm/0001-lower-wasm-memory-grow-and-size.patch) | lower wasm `memory.grow` / `memory.size` helper calls | **Yes.** Public `WasmIntrinsic`, validation, tests. Needs a paired cg_clif patch/release. |
| 4 | [`compiler/patches/cg_clif/0001-…memory-grow-and-size.patch`](../compiler/patches/cg_clif/0001-wasm-lower-memory-grow-and-size.patch) | wasm: lower `memory.grow` / `memory.size` LLVM intrinsics | **Yes, paired with #3.** Requires the clif-to-wasm `WasmIntrinsic` API; upstream as a two-PR series. |
| 5 | [`compiler/patches/rust/0002-…generate-copyright-path-deps.patch`](../compiler/patches/rust/0002-generate-copyright-path-deps.patch) | generate-copyright: read notices from an out-of-tree path dependency's own source directory | **Yes.** Small bootstrap-tool fix with a unit test; unblocks `x.py install` with the path-dependency clif-to-wasm. |
| 6 | [`compiler/patches/rust/0003-…wasm-file-locks.patch`](../compiler/patches/rust/0003-wasm-file-locks.patch) | wasm: process-local file-lock fallback for incremental compilation | **Yes, with review.** WASI p1 has no lock API; one-instance-per-directory is a host contract. |
| 7 | [`compiler/patches/rust/0004-…sync-cached-cgus.patch`](../compiler/patches/rust/0004-wasm-sync-cached-cgus.patch) | wasm: feed cached CGUs to synchronous codegen so incremental reuse links | **Yes, paired with #1.** Fixes the second-incremental-run undefined symbols. |
| 8 | [`compiler/patches/clif-to-wasm/0002-…inline-small-leaf-functions.patch`](../compiler/patches/clif-to-wasm/0002-inline-small-leaf-functions.patch) | inline tiny leaf functions before wasm codegen | **Yes.** Narrow policy with tests; A.2 (e) 1.25× faster. |
| 9 | [`compiler/patches/cg_clif/0003-…wasm-inline-small-functions.patch`](../compiler/patches/cg_clif/0003-wasm-inline-small-functions.patch) | wasm: give clif-to-wasm the codegen unit's small leaf bodies | **Yes, paired with #8.** |
| — | Wild | no patch: growable heap removes the 1 MiB data segment | n/a (see notes) |

The rust patch applies on top of the 23-patch prior-art series; the cg_clif
patches apply to `4f78fd3e…` plus the stage-A wasm patch (cg_clif #2 also
applies to plain upstream). clif-to-wasm applies to `0e4e0458…`.

## What is now pure Rust

* Plugin codegen: rustc frontend → cg_clif → clif-to-wasm → wasm object. All
  Rust; no LLVM library or code generator in rustc.wasm.
* Plugin sysroot: `core`, `alloc`, `compiler_builtins` all compiled by
  rustc.wasm with cg_clif (no LLVM-built rlibs on the plugin path).
* Linker: Wild (pure Rust) for the plugin; rust-lld is the LLVM-toolchain
  reference and produces an almost identical file.
* Host: the CogWasm Wasmtime host is Rust; Wasmtime itself contains C for
  signals/unwinding, but no plugin codegen depends on it.

## What is still not Rust

1. **Build-time LLVM for rustc itself.** Stage 0's LLVM backend builds the
   native stage1 rustc, which cross-compiles rustc.wasm. `x.py` also uses a
   downloaded CI LLVM to build that native compiler. Nothing in the shipped
   plugin path runs LLVM, but producing the compiler still does.
2. **A self-hosting compiler is not proven.** In principle rustc.wasm could
   compile the next rustc.wasm: the frontend is pure Rust, cg_clif can compile
   it, this stage supplies a pure sysroot, and Wild can link wasm. What is
   missing is a wasm32-wasip1 *host* std built by cg_clif, proc-macro/build-
   script execution under wasm (or avoiding them), and linking a 137 MB wasm
   with a wasm-hosted Wild. It is a multi-week project, not a blocker for
   shipping plugins, and it still needs one seed compiler to break the
   bootstrap cycle. "No LLVM at all" cannot be bootstrapped from nothing.
3. **Wild's `.bss` representation** is still not what rust-lld does in
   general; at the current (small) `.bss` size it is byte-for-byte equal to
   rust-lld's data, so no patch is warranted yet. Re-check if a future plugin
   puts a large zeroed static in `.bss`.

## Recommendation

Ship the NodeCraft integration now: the raw ABI (`wasm_alloc`, `eval`), the
pure sysroot, rustc.wasm and Wild as one versioned bundle, with one resident
Wasmtime engine/instance and the compiled module cached. Edit-to-result is
101 ms and compile is 83.6 ms, both well inside the 0.41 s target. More
performance work (cg_clif maths is ~3.9× LLVM on (e); host-call geometry is at
parity) is optional and only worth it if maths-heavy nodes become the
bottleneck; a hand-written wasm SIMD kernel or an optimizing pure-Rust backend
is a later project. The remaining engineering risk is integration (memory
caps, epoch/fuel interruption, cache invalidation), not codegen.
