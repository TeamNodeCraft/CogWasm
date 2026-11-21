# Stage A results

Date: 2026-11-18. Host: macOS arm64 (Apple silicon), 14 CPUs, 48 GB RAM.
Rust: `nightly-2026-10-19` (1.101.0-nightly, `b6eeb93c9`), isolated under
`toolchain/`. All timings are median of three warm runs except "cold", which is
one build in a fresh target directory. Sizes are bytes of the linked `.wasm`.
Every artifact below was validated with `wasmparser 0.263` (all features) and
executed by Wasmtime 50.0.2.

## Verdict

| Program | What it proves | Result |
|---|---|---|
| (a) `a_add` | `no_std`, pure-Rust core, exported `add(i32,i32)` | **PASS** `add(20,22)=42` |
| (b) `b_alloc` | `no_std`+`alloc`: `Vec`, `String`, `format!`, closures, trait objects, f64/`libm` | **PASS**, `bench` runs and returns finite results |
| (c) `c_plugin` | NodeCraft plugin: `wasm_import_module = "nodecraft"`, f64 parameter list, allocation, result through memory, `panic=abort` | **PASS**, import appears as `nodecraft::host_bias`, `eval` returns `2.25` for params 1,2,3,4 |
| (d) `d_hello_std` | `std` on `wasm32-wasip1`, LLVM comparison | **PASS**, prints `hello from std wasm; sum=14` |

Strict builds (`CG_CLIF_WASM_STRICT=1`, which turns clif-to-wasm "stub with a
trap" into an error) pass for (a), (b) and (c). No function was replaced by a
trap in these programs, including `core`, `alloc`, `compiler_builtins` and
`libm` compiled by cg_clif.

## Compile + link time and size (O2)

`clif-lld` uses the **pure-Rust sysroot** built by `-Zbuild-std` with cg_clif.
Its "cold" column includes building `core`, `alloc` and `compiler_builtins`
(that is the ~6 s). Once built, the warm column is a normal plugin rebuild.
`clif-prebuilt` is the same backend against rustup's **LLVM-built** sysroot,
which tests ABI interoperability, not the pure-Rust shipping path.

| program | config | cold s | warm s | bytes | LLVM O2 warm s | LLVM O2 bytes |
|---|---|---:|---:|---:|---:|---:|
| a_add | clif-lld (pure sysroot) | 6.366 | 0.127 | 236 | 0.074 | 306 |
| b_alloc | clif-lld (pure sysroot) | 7.056 | 0.153 | 89,739 | 0.117 | 18,920 |
| c_plugin | clif-lld (pure sysroot) | 6.855 | 0.145 | 34,502 | 0.100 | 7,274 |
| a_add | clif-prebuilt (LLVM std) | 0.166 | 0.075 | 236 | — | — |
| b_alloc | clif-prebuilt (LLVM std) | 1.070 | 0.100 | 60,741 | — | — |
| c_plugin | clif-prebuilt (LLVM std) | 0.184 | 0.089 | 18,225 | — | — |

The pure-Rust `core` + `alloc` + `compiler_builtins` rlibs total **1.8 MB**
(873 KB + 195 KB + 765 KB), versus 7.6 MB for rustup's LLVM-built set. The 6–7 s
cold number is paid once when producing the shipped sysroot; plugin builds after
that are 0.13–0.15 s. Against the 0.41 s target, **warm compile+link is already
under the target by ~3×**.

Why the pure-Rust build is larger: object granularity. cg_clif's wasm objects
appear not to be split as finely as LLVM's, so `--gc-sections` retains more of
each CGU. The same plugin is 34.5 KB with the pure sysroot and 18.2 KB against
the LLVM sysroot; both are small enough to ship.

## Opt-level 0 vs 2

| program | backend | cold s | warm s | bytes | notes |
|---|---|---:|---:|---:|---|
| a_add | clif O0 | 0.170 | 0.074 | 304 | |
| b_alloc | clif O0 | 1.344 | 0.100 | 80,026 | |
| c_plugin | clif O0 | 0.179 | 0.086 | 25,299 | |
| a_add | LLVM O0 | 0.166 | 0.073 | 307 | |
| b_alloc | LLVM O0 | 0.746 | 0.100 | 60,221 | |
| c_plugin | LLVM O0 | 0.169 | 0.091 | 16,442 | |

The O0 cg_clif numbers use rustup's prebuilt (LLVM, O2) sysroot. Building the
sysroot itself at O0 fails; see blockers.

## Runtime in Wasmtime

`b_alloc::bench(n)` is a dependent f64 loop with `sqrt` + `sin`, a trait-object
dispatch per iteration and a 64-element `Vec`; n = 2,000,000, median of 3 calls.

| binary | ns/iteration |
|---|---:|
| cg_clif O2, rust-lld | 98.19 |
| cg_clif O2, wild | 98.62 |
| LLVM O2, rust-lld | 25.28 |
| LLVM O2, wild | 24.93 |
| cg_clif O0 | 238.87 |
| LLVM O0 | 206.51 |

At O2 cg_clif is **3.9× slower** than LLVM on this benchmark; at O0 it is 1.16×
slower. The plugin host-call path itself is ~1.5 µs per `eval` (dominated by the
Wasmtime store/instance and call boundary), versus ~2 µs for the LLVM build —
not meaningfully different at that scale.

## Wild vs rust-lld

Wild 0.11.0 (`fd79578`) links all three cg_clif programs with
`-C linker-flavor=wasm-ld`, and the resulting modules validate and run with
identical outputs. It is a drop-in replacement for this pipeline.

One concrete difference: **Wild emits `.bss` as literal data**. Our 1 MiB heap
is zero-initialized static memory; rust-lld represents it in the memory's initial
size (no file bytes), while Wild writes a ~1 MiB data segment:

| program | rust-lld bytes | wild bytes | delta |
|---|---:|---:|---:|
| a_add | 236 | 267 | +31 |
| b_alloc | 89,739 | 1,139,729 | +1,049,990 |
| c_plugin | 34,502 | 1,083,560 | +1,049,058 |

The code sections are the same size; only `DATA` differs. For plugin distribution,
either keep rust-lld (which is allowed as the comparison/reference linker) or move
the plugin heap out of `.bss` (host-allocated / `__heap_base` plus a fixed initial
memory). This is a Wild behavior to track, not a codegen bug.

## (d) std / wasm32-wasip1

Built by LLVM with rustup's prebuilt std, **95,055 bytes**, first build ~0.32 s,
runs under Wasmtime's WASI preview-1 host. Rust's `wasm32-wasip1` std links
prebuilt `crt1-command.o` and `libc.a` from the rustup `self-contained/`
directory (no wasi-sdk was installed). That C code is exactly what the plugin path
avoids: (a)–(c) are `no_std` + `alloc`, have no WASI imports, and only import the
explicit `nodecraft` host function.

**std is not needed for these plugins.** (b) covers the containers, formatting,
closures, traits and f64 maths a maths/geometry plugin needs; (c) covers the host
boundary. A plugin that truly needs files/network/clock would violate the
determinism requirement anyway.

## What clif-to-wasm rejected or struggled with

1. **`memory.size` / `memory.grow` intrinsics.** cg_clif emits
   `llvm.wasm.memory.size.i32` / `grow.i32` for
   `core::arch::wasm32::{memory_size, memory_grow}`; clif-to-wasm replaces them
   with trap stubs and warns. A growable heap is therefore unusable. Workaround:
   a fixed `.bss` heap (used here) or a host-managed heap. This is the main
   practical limitation found.
2. **f128 soft-float at O0.** Building `compiler_builtins` with `opt-level=0`
   fails with a libcall signature conflict:
   `__floatsitf` declared once as `(i32) -> f128` and once as
   `(i32 sret, i32) -> ()`. clif-to-wasm does not support f128. At O2 the
   offending functions are eliminated and the build succeeds. Consequence: build
   the shipped sysroot at O2; plugins themselves can be O0 against it (verified).
3. **Atomics.** An `AtomicUsize` allocator counter lowered to wasm atomic
   instructions while the module did not declare the `atomics` feature; Wasmtime
   refused to compile it. Plugins are single-threaded per instance, so a plain
   `UnsafeCell` counter is sufficient and removes the issue. Atomics/threads were
   not otherwise exercised; `wasm32-wasip1-threads` is untested here.
4. **No debug info.** cg_clif's wasm objects carry no DWARF (the native path still
   does). Panic messages are gone with `panic=abort` anyway.

Not hit: i128 `mulhi`, f128 arithmetic in user code, vector big-endian ops,
`try_call`, dynamic stack slots, `get_return_address`.

## Linker/validation details

* Every artifact passes `wasmparser::Validator` with all features enabled.
* Module exports `memory`, `wasm_alloc`, and the program's exports; (c) imports
  exactly `nodecraft::host_bias (f64) -> f64`. No WASI imports.
* `wasm_opt` was not run. It would reduce size further but is another LLVM tool;
  not part of the pure-Rust path.

## Stage B feasibility (rustc.wasm with no LLVM codegen)

**High, and already demonstrated by the project this patch comes from**, not just
inferred from stage A:

* The playground at `ryancbuckley/rust-playground-inbrowser` builds `rustc.wasm`
  for `wasm32-wasip1-threads` from the same cg_clif patch series and the same
  nightly, statically linking cg_clif + clif-to-wasm, and runs it under a WASI
  kernel. It reports a ~98 MB `rustc.wasm` (23 MB gzipped), ~0.3 s for a hello
  program, and 2,640/2,650 differential tests passing against LLVM.
* Stage A confirms the backend is not tied to WASI: the same patched cg_clif
  produces clean `wasm32-unknown-unknown` objects. Inside rustc.wasm the target
  is just a flag.
* **What must ship:** `rustc.wasm` (LLVM is used once to build rustc itself, not
  to compile plugins), the patched cg_clif statically linked (codegen backends
  cannot be dlopen'd inside wasm), the 1.8 MB pure-Rust plugin sysroot, the
  prebuilt SDK rlib, and Wild. Rustc.wasm can call Wild through a host import
  (`playground.spawn`) as the playground does, or the host can link the emitted
  `.o` files natively with the 13 MB Wild binary.
* **Threads:** rustc.wasm uses wasi-threads for its own parallelism; Wasmtime on
  desktop supports this without browser COOP/COEP constraints. A single-threaded
  build is possible if it simplifies the host, at a compile-time cost.
* **Proc macros:** plugins must not need them. Our SDK is a prebuilt rlib with
  pregenerated bindings, matching the plan. If a dependency insists on a proc
  macro, the playground has a wasmi-based `-Zwasm-proc-macros` escape hatch, but
  it is complexity we should avoid.
* **Sysroot determinism:** rlibs must match the exact rustc nightly and the
  cg_clif backend revision; ship them as one versioned bundle. Build them O2.
  The O0 f128 failure means do not let a user opt-level leak into the sysroot
  build.
* **Latency:** native warm plugin compile is 0.13–0.15 s. Even allowing 2–3× for
  rustc.wasm, this is at or under the 0.41 s LLVM-rustc.wasm baseline. This is the
  number stage B must measure on the real rustc.wasm.
* **Memory:** the playground budgets 384 MB for rustc.wasm. Desktop-only stage B
  is fine; a browser-hosted compiler needs that budget and threads.

Stage B risks are integration (WASI syscalls, spawn/linker plumbing, rustc's
thread usage, packaging size), not codegen. Stage A removes the codegen unknown.

## Stage C notes (NodeCraft)

The raw ABI proven by (c) is enough for a first integration:

```
wasm_alloc(size, align) -> ptr
eval(params_ptr: *const f64, len: usize, out_ptr: *mut f64) -> i32
import nodecraft::host_bias(f64) -> f64
```

* One `Store`/instance per evaluation or per worker, memory limit sized to the
  plugin's heap; epoch/fuel interruption; no WASI imports at all.
* `panic=abort` becomes a wasm trap; Wasmtime returns it as an error, so the host
  keeps the last good plugin and reports a node error.
* Determinism: the only imports are explicit host functions; no clock, files,
  network or unseeded RNG. Time and seeds are parameters, as the plan requires.
* WIT/components can come later: wrap this core module host-side, or regenerate
  the SDK bindings with `wit-bindgen` at SDK build time. Nothing in stage A
  depends on proc macros at plugin compile time.
* Hot reload: cache by BLAKE3(source + SDK + compiler revision), compile on a
  dedicated thread, swap the `InstancePre`; do not re-enter the kernel pool from
  host functions.

---

# Stage B results (rustc.wasm with cg_clif only)

Date: 2026-11-18. Host: macOS arm64 (Apple silicon), 14 CPUs, 48 GB RAM.
Isolated rustup/cargo under `toolchain/`. The compiler source is
`rust-lang/rust@b6eeb93c992027a03e1fd120b6652546396b142c` with the
`ryancbuckley/rust-playground-inbrowser@f529863` patch series (23 patches).
Details and reproduce steps: [`stage-b/README.md`](stage-b/README.md).

## Verdict

| Check | Result |
|---|---|
| rustc.wasm builds for `wasm32-wasip1-threads` with `codegen-backends=["cranelift"]` | **PASS** |
| No LLVM codegen inside rustc.wasm (imports/symbols/sections) | **PASS** (0 LLVM imports, 0 `rustc_codegen_llvm`/`rustc_llvm` symbols) |
| Wasmtime runs rustc.wasm, including its `wasi.thread-spawn` compiler thread | **PASS** (Wasmtime 46.0.3) |
| rustc.wasm compiles stage A plugin (c) to a relocatable wasm object | **PASS** |
| Native Wild links it through the `playground.spawn` host import | **PASS**, 23 args, exit 0 |
| Plugin loads in Wasmtime; `eval(1,2,3,4)` | **PASS**, `2.25` |
| Warm edit-to-result well under 1 s | **PASS**, **98.1 ms** in a hot host; **~0.74 s** CLI with warm module cache |

## Build

| Item | Value |
|---|---|
| `./x.py install` | 580.9 s wall (9m41s), x.py reported 9m08s; +31 s for the stage1 library |
| toolchain disk | **12 GB** (source+build 10 GB, build 6.6 GB, artifacts/sysroots/cache ~1 GB) |
| rustc.wasm | **143,715,574 B** (137 MiB), SHA-256 `88d30a79…b734f089` |
| rustc.wasm imports | 26 WASI p1 + `env.memory` + `playground.spawn` + `wasi.thread-spawn` |
| rustc.wasm exports | `memory`, `_start`, `wasi_thread_start` |
| rustc.wasm module compile (cold, no cache) | **39,649.97 ms** median [39,547.03–40,148.85] |
| rustc.wasm module load (Wasmtime cache hit) | **557.54 ms** median [556.58–561.48] |

Wasmtime 50.0.2 was the intended "latest" host, but it removed
`wasi-threads` in 47.0.0 and rustc always runs compilation on a spawned thread.
Stage B pins **Wasmtime 46.0.3**, the last release with
`wasmtime-wasi-threads`; the compiler itself is unmodified. See
[`stage-b/research-wasmtime.md`](stage-b/research-wasmtime.md).

## Warm edit-to-result (median of 5)

Stage A's plugin (c) source gets a real one-line `TWEAK` edit each run; the
result stays 2.25. "Module resident" keeps one Wasmtime engine + compiled
rustc.wasm and instantiates a fresh rustc per edit, as a hot-reload host would.
The CLI number is the whole process wall time with a warm Wasmtime module cache.

| Phase | Median | Min–max |
|---|---:|---:|
| instantiate rustc.wasm | **0.79 ms** | 0.77–1.12 |
| compile inside rustc.wasm (frontend + cg_clif + clif-to-wasm) | **85.36 ms** | 83.43–110.84 |
| link with native Wild (host `playground.spawn`) | **5.36 ms** | 5.11–6.50 |
| load linked plugin (1,066,803 B) | **6.97 ms** | 6.81–7.46 |
| `eval(1,2,3,4)` call | **~0.001 ms** | — |
| **edit-to-result, module resident** | **98.05 ms** | 96.4–125.6 |
| **CLI warm run (cache hit)** | **~0.74 s wall** | — |

Raw data: [`stage-b/results/stage-b.json`](stage-b/results/stage-b.json).

## No LLVM inside rustc.wasm

Generated by `stage-b-host inspect --wasm rustc.wasm` plus `strings`; raw output is [`stage-b/results/rustc-inspect.json`](stage-b/results/rustc-inspect.json).

| Probe | Count |
|---|---:|
| LLVM imports (module/name contains `llvm`) | **0** |
| LLVM exports | **0** |
| function names containing `rustc_codegen_llvm` | **0** |
| function names containing `rustc_llvm` | **0** |
| function names containing uppercase `LLVM` | **0** |
| `LLVMInitialize` / `LLVMRust` / `LLVMContext` strings | **0** |
| `libLLVM` string | 1 (linker search-path table, not code) |

The 129 function names containing `llvm` are cg_clif's
`rustc_codegen_cranelift::intrinsics::llvm_aarch64::…` helpers, i.e. Cranelift
lowerings of LLVM-named intrinsics, not the LLVM library. `rustc_llvm` and
`rustc_codegen_llvm` are not linked.

## Plugin sysroots

| sysroot | core / alloc / compiler_builtins | accepted by rustc.wasm? | plugin result |
|---|---|---|---|
| `sysroot-pure` (Stage A cg_clif, rustup nightly) | cg_clif / cg_clif / cg_clif | **No**, E0514: `1.101.0-nightly` vs `1.101.0-dev` | — |
| `sysroot-stage1` (native stage1 from the rustc.wasm build) | LLVM / LLVM / LLVM | **Yes**, exact version | **2.25** |
| `sysroot-wasm-hybrid` (rustc.wasm core+alloc, stage1 compiler_builtins) | **cg_clif / cg_clif** / LLVM | **Yes** | **2.25** (compile 133.6 ms, link 29.5 ms, e2e 144.5 ms) |

The hybrid sysroot is the self-hosting proof: **rustc.wasm built `core`
(856 KiB) and `alloc` (195 KiB) with cg_clif**, then `alloc` was rebuilt against
a `compiler_builtins` built by the native stage1 LLVM compiler against
rustc.wasm's core metadata. Building `compiler_builtins` with rustc.wasm fails
on the known f128 libcall conflict (`__multf3`/`__subtf3` declared both
f128-return and sret). The plugin path therefore still contains LLVM-generated
`compiler_builtins`, but no LLVM code generator is in rustc.wasm.

## Stage A.2: realistic plugins on the native Stage A toolchain

Date: 2026-11-18. Medians of 5 warm runs. Both plugins are `no_std`+`alloc`,
1 MiB `.bss` heap, linked with Wild for cg_clif and rust-lld for LLVM.
(e) 100k f64 points: `sin`, `cos`, `sqrt`, reduce. (f) 40k iterations × 5
imported host calls = 200,000 calls. Full report:
[`stage-b/a2/RESULTS.md`](stage-b/a2/RESULTS.md); raw:
[`stage-b/results/a2.json`](stage-b/results/a2.json).

| program / metric | LLVM O2 | cg_clif O2 | cg_clif / LLVM |
|---|---:|---:|---:|
| (e) warm compile+link | 0.100 s | 0.131 s | **1.30×** |
| (e) Wasmtime module load | 6.77 ms | 17.18 ms | **2.54×** |
| (e) run 100k points | 2.30 ms / 23.0 ns·pt | 8.78 ms / 87.8 ns·pt | **3.82×** |
| (e) code size | 13,173 B | 39,709 B | **3.01×** |
| (f) warm compile+link | 0.090 s | 0.129 s | **1.44×** |
| (f) Wasmtime module load | 4.08 ms | 10.15 ms | **2.49×** |
| (f) run 200k host calls | 0.622 ms / 3.11 ns·call | 0.570 ms / 2.85 ns·call | **0.92×** |
| (f) code size | 4,638 B | 21,679 B | **4.67×** |

MIR inlining:

* `-Zinline-mir -Zmir-opt-level=3` (the requested config) **fails** building
  `compiler_builtins` on the f128 (`__floatsitf`) signature conflict.
* `-Zinline-mir -Zmir-opt-level=4` builds but saves only **79 B** (e) / **207 B**
  (f) of code and changes run time by <2%.
* `-Zinline-mir` alone at MIR opt level 2 is **byte-identical** to the default.
* `-Zinline-mir=no -Zmir-opt-level=2` also fails (`__umodti3` i128 ABI), so
  default `clif_o2` is the meaningful MIR-disabled baseline.

Interpretation: plugin compile is 1.3–1.4× LLVM (still well under the 0.41 s
target); Wasmtime load is ~2.5×; maths throughput is 3.8× slower with cg_clif,
while host-call-heavy geometry is at parity because the host boundary dominates.
Total artifact size is inflated by Wild's 1 MiB `.bss` data segment; code size
is 3–4.7× LLVM's.

## What is still not Rust

1. **Build-time LLVM**: stage0's LLVM backend builds the native stage1 rustc;
   the stage1 rustc builds rustc.wasm and (for the main path) the plugin
   sysroot. LLVM is not inside rustc.wasm.
2. **`compiler_builtins` in the hybrid sysroot** is LLVM-generated (core/alloc
   are cg_clif). The main `run.sh` path uses the fully stage1-built sysroot for
   reproducibility and version matching.
3. **Wasmtime** is Rust with C pieces (signals/unwinding). Wild, cg_clif and
   clif-to-wasm are pure Rust.
4. **No DWARF** in cg_clif wasm objects; `panic=abort`.

## Blockers and stage C recommendation

* Wasmtime ≥47 cannot run unmodified rustc.wasm (wasi-threads removed). Pin
  46.0.3, or patch rustc to run the compiler on the main thread and use latest
  Wasmtime. The latter is a small compiler patch but a larger semantic change.
* Cold module compile is ~40 s at 137 MiB. Ship a precompiled `.cwasm` / browser
  cache; 98 ms is the edit budget once resident.
* Sysroot version lock: rustc.wasm's `1.101.0-dev` rejects a rustup-nightly
  `1.101.0-nightly` sysroot. Ship rustc.wasm + sysroot + Wild + host as one
  versioned bundle, or build the sysroot with rustc.wasm itself.
* f128 is still unsupported by clif-to-wasm and blocks a fully cg_clif-built
  `compiler_builtins`.
* Wild materialises the 1 MiB `.bss` heap; move the plugin heap out of `.bss`
  (host-allocated or `__heap_base` + fixed initial memory) before shipping.
* cg_clif maths throughput is 3.8× slower than LLVM; host-call-heavy code is at
  parity. For NodeCraft, start with the raw ABI (stage A/B), keep rustc.wasm
  resident, cache the compiled module, and keep LLVM out of the plugin codegen
  path. Only revisit a pure-Rust optimizing backend if maths-heavy nodes become
  the bottleneck.
* Retire the LLVM-inside-wasm path for plugin codegen now. Keep LLVM only for
  the build-time stage1/sysroot until a self-hosting stage2 exists; the hybrid
  sysroot shows the next concrete step (rustc.wasm building core/alloc), with
  compiler_builtins/f128 as the remaining piece.

---

# Stage C results (latest Wasmtime, no wasi-threads, pure sysroot, growable heap)

Date: 2026-11-18/19. Stage C is in [`stage-c/`](../../); details are in
[`stage-c/README.md`](../../README.md) and
[`stage-c/results.md`](../../results.md). Summary:

* **rustc.wasm** is now `wasm32-wasip1` (non-threads), 143,849,756 B, SHA-256
  `3c7ca043…c0302f`. The compiler and the codegen coordinator run on the
  calling thread; the guest stack is 32 MiB via link args. It imports only the
  26 WASI p1 functions and `playground.spawn` — **no `wasi.thread-spawn`** —
  and runs on **Wasmtime 50.0.2** with plain WASI p1.
* **Pure sysroot**: `core`, `alloc` and `compiler_builtins` are all built by
  rustc.wasm itself (no E0514). The blocking f128 libcall signature conflict
  is fixed in cg_clif by returning f128 through an sret pointer on wasm.
  Strict (`CG_CLIF_WASM_STRICT=1`) builds the whole sysroot with no trap stubs.
* **`memory.size` / `memory.grow`** are implemented via reserved helper calls
  recognized by clif-to-wasm; plugin (c) now has a growable bump allocator.
  Plugin (c) is **34,921 B** with Wild and **34,940 B** with rust-lld (was
  1,083,560 B with Wild because the 1 MiB `.bss` heap was emitted as data).
* **Numbers on the latest Wasmtime** (`stage-c/results/stage-c.json`, median
  of 5): module cold compile 38.47 s, cached load 440.5 ms, instantiate
  0.755 ms, compile inside rustc.wasm 83.6 ms, link 6.46 ms, plugin load
  9.95 ms, **edit-to-result 101.2 ms**, `eval(1,2,3,4) == 2.25`.
* **A.2 with the pure sysroot**: (e) 0.120 s compile+link, 17.07 ms load,
  89.5 ns/point (LLVM 23.0); (f) 0.111 s, 9.75 ms, 2.92 ns/host-call (LLVM
  3.11). Code size is unchanged from stage B (39,995 / 21,965 B); total size
  falls to 62,124 / 34,059 B because the heap is no longer in `.bss`.
* **Still not Rust**: build-time LLVM for the native stage1 rustc (and the CI
  LLVM used to build it). Nothing in the shipped plugin codegen path uses
  LLVM. A wasm self-hosting stage is plausible but needs a cg_clif-built
  wasm32-wasip1 host std, proc-macro/build-script execution under wasm, and a
  wasm-hosted Wild; it is a multi-week project and still needs a seed
  compiler, not a blocker for NodeCraft integration.
* **Patches**: `stage-c/patches/rust/0001-…calling-thread.patch` (rustc),
  `stage-c/patches/cg_clif/0002-…f128-sret-return.patch`,
  `stage-c/patches/clif-to-wasm/0001-…memory-grow-and-size.patch` and
  `stage-c/patches/cg_clif/0001-…memory-grow-and-size.patch`. No Wild patch:
  the growable heap removes the 1 MiB data segment.

---

# Speed batch (2026-11-19): precompiled compiler, incremental builds, better wasm

Date: 2026-11-19. Host: macOS arm64 (Apple M4 Pro, 14 CPUs, 48 GB RAM).
rustc `nightly-2026-10-19`, Wasmtime 50.0.2. Every number is a median of 5 on
the same machine and build profile. Final artifact: `rustc.wasm`
**144,051,631 B**, SHA-256
`d2404a865ef00149cc922bd6b9d15548e02d07890391eef3af4587de9376fb91`. The same
`experiments/cranelift-wasm/ci/build-and-test.sh` runs locally and in CI; the
local full run passed in **135 s**, including the two new checks below.

## 1. Precompiled `rustc.wasm` (`.cwasm`)

`stage-c-precompile` writes a host artifact with `Engine::precompile_module`
and a sidecar recording the Wasmtime version (50.0.2), the engine's
`precompile_compatibility_hash()`, the target triple, the engine configuration
and the sizes/SHA-256 of both files. The host's `--precompiled PATH` tries the
artifact and falls back to `Module::new(rustc.wasm)` on any mismatch, with a
warning naming the reason.

| how the host gets `rustc.wasm` | median | size |
|---|---:|---:|
| compile the `.wasm` from scratch (no cache) | **37,561 ms** | 144,051,631 B |
| Wasmtime's compiled-module cache | **461.5 ms** | — |
| deserialize the per-host `.cwasm` (default checks) | **18.7 ms** | 296,372,016 B |
| `.cwasm` with `--verify-precompiled` (full SHA-256 of 419 MB) | **759.0 ms** | — |
| `Engine::precompile_module` (paid once per host/release) | **38,247 ms** | — |

The 282 MiB `.cwasm` is native code and only loads on the exact Wasmtime
version, engine configuration and target. The full SHA-256 check is opt-in: it
costs far more than the deserialization it protects. Fallbacks are tested in CI
(`run.sh precompiled-check`): a mismatched compatibility hash and a truncated
artifact both print the reason and still compile plugin (c), which returns
`2.25`.

The release workflow builds one `.cwasm` per host (macOS arm64, Linux x86_64)
from the same `rustc.wasm`; macOS was built and measured locally, and the Linux
path is exercised by CI's precompiled check on `ubuntu-latest`. Windows is not a
host target yet: the reference host preopens native paths as WASI `/`, and
neither the drive-path translation nor Wild's Windows behaviour is tested.

## 2. Incremental compilation inside a resident `rustc.wasm`

The host keeps its Wasmtime engine and compiled `rustc.wasm` resident across
edits and passes a persistent, WASI-preopened `-Cincremental=DIR`. Two wasm
gaps had to be fixed at the source:

* `patches/rust/0003-wasm-file-locks.patch`: WASI p1 has no file locks, so
  rustc's fallback rejected the session lock. A process-local lock registry
  gives the same exclusive/shared semantics as the Unix fallback (WASI cannot
  coordinate processes; the host contract is one compiler instance per
  directory).
* `patches/rust/0004-wasm-sync-cached-cgus.patch`: the stage-C synchronous
  coordinator handled fresh CGUs but dropped cached-CGU messages on the second
  incremental run, so reused objects never reached Wild (undefined `drop_glue`
  and `alloc` symbols). The synchronous path now copies cached work products
  inline, exactly as the coordinator thread does.

mmap, fsync, timestamps, rename and hard links needed no change: `memmap`
already falls back to `Vec<u8>` on wasm, and WASI p1 implements the rest.

| program | cold compile | first incremental | steady incremental | cold → incremental edit-to-result | incr. dir |
|---|---:|---:|---:|---:|---:|
| plugin (c), one-line edit | 96.1 ms | 94.6 ms | **79.9 ms** | 113.7 → **97.7 ms** (−14.1%) | 590 KB / 11 files |
| A.2 (e), one-line edit | 105.9 ms | 106.3 ms | **88.2 ms** | 136.7 → **119.8 ms** (−12.4%) | 652 KB / 11 files |

`difftest/incremental-check.sh` compiles corpus `smoke.rs` cold and twice with
one persistent directory and requires identical `run(seed)` results and output
buffers (4 seeds). It passes. Cold and incremental output are **not**
byte-identical, and that is a real cost: incremental uses a finer CGU/shared-
generics split, so fewer tiny helpers are inlined. For `smoke.rs` the module
grows 64,616 → 75,555 B (+16.9%, code 30,687 → 35,452 B, 131 → 190 functions);
plugin (c) grows 34,583 → 40,783 B (+18%) and A.2 (e) 58,252 → 65,299 B (+12%).
The incremental A.2 module still runs within ~1% of the cold one (6.91 vs
6.83 ms). Compile-time is the win: about 16–17% less compile and 12–14% less
edit-to-result.

**Recommendation:** keep incremental **opt-in** (`--incremental`, off by
default). It is a measurable win for repeated edits of one crate and it is
correct, but +12–18% output size for 12–14% edit-to-result is not a default
worth imposing until the cold and incremental codegen paths inline the same
helpers.

## 3. Better wasm from clif-to-wasm: inline tiny leaf functions

Profiling A.2 (e) by operation isolated the gap: the loop itself and `sin`/`cos`
were near parity, while the software `sqrt` cost 69.6 ns/point under cg_clif vs
12.3 under LLVM — 57 of the 65 ns gap. LLVM inlines the one- and two-instruction
`core` operator/cast helpers inside libm's `sqrt_round`; cg_clif has no inliner,
and rustc's MIR inliner runs before monomorphization, so every helper stayed a
wasm `call` plus argument moves and stack traffic.

`patches/clif-to-wasm/0002-inline-small-leaf-functions.patch` adds a narrow
inliner built on Cranelift's `Context::inline`: direct calls to a leaf of at
most 12 instructions and 4 blocks, with no calls, memory, globals, tables,
traps or conditional branches (a `nop`, a straight-line `jump` and the final
`return` are allowed). `patches/cg_clif/0003-wasm-inline-small-functions.patch`
collects the candidate bodies of a wasm codegen unit before compiling and runs
the pass, so call order in the unit does not matter. Only the wasm object path
is changed.

| metric | before | after | change |
|---|---:|---:|---:|
| (e) run, 100k points | 86.45 ns/pt | **68.99 ns/pt** | **1.25× faster** |
| (e) code size / functions | 39,995 B / 180 | **39,152 B / 150** | −2.1% / −30 |
| (e) total size | 62,124 B | **58,252 B** | −6.2% |
| (f) run, 200k host calls | 3.799 ns/call | **3.781 ns/call** | unchanged (host-bound) |
| (f) code size | 21,965 B | **21,937 B** | unchanged |
| isolated `sqrt` probe | 69–75 ns/pt | **~52 ns/pt** | 1.3–1.4× faster |
| sqrt probe total size | 28,181 B | **24,870 B** | −11.8% |

Against LLVM `-O2`, (e) moves from **3.76×** slower to **3.00×**; (f) stays at
parity. The inliner is one linear walk per function and the final stage-B
compile median did not regress (79.4 ms vs 81.9 ms before). The full difftest
corpus plus 24 seeded random programs passes on the final stack (72 programs,
288 calls, 0 mismatches), and the no-LLVM audit still reports 0 imports and 0
forbidden names.

## Reproduce

```sh
experiments/cranelift-wasm/ci/build-and-test.sh   # includes incremental + cwasm checks

cd experiments/cranelift-wasm
stage-c/run.sh bench            # stage-B table (writes stage-c/results/stage-c.json)
stage-c/run.sh a2               # A.2 (e)/(f)
stage-c/run.sh incremental      # cold vs incremental, JSON in stage-c/results/
stage-c/run.sh precompiled-check
difftest/run.sh --seeds 24
stage-c/precompile/package.sh --tag v0.0.0-dryrun --skip-build --out release/out
```

## What did not pan out

* `-Ccodegen-units=16` with incremental did not recover the cold build's
  inlining: 16 and 256 incremental CGUs both produced 120 functions for plugin
  (c), vs 92 cold. The extra functions come from incremental's shared-generics
  split, not the CGU count.
* The `--verify-precompiled` digest check is too slow to be the default
  (759 ms vs 18.7 ms); it stays opt-in and `SHA256SUMS` covers install-time
  verification.
* Windows remains unsupported for the precompiled bundle (WASI path mapping and
  Wild untested); it is documented rather than half-supported.
