# Stage B: rustc.wasm with Cranelift as its only codegen backend

This is the stage-B result for the NodeCraft pure-Rust wasm compiler: a real
`rustc` compiled to WebAssembly, running under **Wasmtime on macOS**, with
`rustc_codegen_cranelift` (cg_clif) + [`clif-to-wasm`](https://github.com/ryancbuckley/clif-to-wasm)
as the **only** code generator inside `rustc.wasm`. It compiles stage A's plugin
(c), links the relocatable object with **Wild**, and the resulting plugin runs in
Wasmtime and returns `2.25` for `eval(1,2,3,4)`.

**Verdict: end-to-end PASS.** No LLVM code generator is linked into
`rustc.wasm`; LLVM is used only at build time to compile the native stage1
compiler and the `compiler_builtins` part of the plugin sysroot (see
[What is still not Rust](#what-is-still-not-rust)).

```
plugin.rs ──rustc.wasm (cg_clif)──▶ CLIF ──clif-to-wasm──▶ relocatable .o
          │                                                │
          │ playground.spawn (host import)                 ▼
          └──────────────────────────────▶ native Wild ──▶ plugin.wasm
                                                           │
                                              Wasmtime ────┘ eval == 2.25
```

## Pins

Everything downloaded or built lives under `../toolchain/` (gitignored).

| Component | Source | Revision / version |
|---|---|---|
| rustc source | https://github.com/rust-lang/rust | `b6eeb93c992027a03e1fd120b6652546396b142c` (`nightly-2026-10-19`, rustc 1.101.0) |
| prior art | https://github.com/ryancbuckley/rust-playground-inbrowser | `f5298630dda0f2dc2b44f6ae1b54add7a77d8316` |
| rust patch series | prior art `patches/rust/0001…0023` | applied verbatim; 23 patches |
| cg_clif | in-tree `compiler/rustc_codegen_cranelift` from that patch series | upstream `4f78fd3e…`, clif-to-wasm 0.3.0 |
| clif-to-wasm | crates.io | 0.3.0 |
| wasi-sdk | https://github.com/WebAssembly/wasi-sdk | 34.0 arm64-macOS |
| Native Wild (linker) | Stage A build of https://github.com/wild-linker/wild | `fd79578abb…`, 0.11.0 |
| Wasmtime host | crates.io | **46.0.3** |
| wasmtime-wasi | crates.io | 46.0.3, `p1` |
| wasmtime-wasi-threads | crates.io | 46.0.3 |
| wasmparser (inspect) | crates.io | 0.263 |

Wasmtime **46.0.3 is the last release with `wasi-threads`**. Wasmtime 47 removed
the host implementation (bytecodealliance/wasmtime#13558), and 50.0.2 has no
`wasi:thread-spawn`. `rustc` always runs the non-parallel compiler on a spawned
`rustc` thread (`compiler/rustc_interface/src/util.rs`), so an unmodified
`rustc.wasm` calls `wasi.thread-spawn` on every compile. Pinning 46.0.3 keeps
the compiler unmodified; `Config::wasm_threads(true)` only enables shared
memory/atomics and is not enough on 47+. See
[`research-wasmtime.md`](research-wasmtime.md) for the citations and the
alternatives (patch rustc to run on the main thread, or write a shared-fd WASI
host).

## Build

```sh
# 1. rustc.wasm (isolated rustup/cargo, ~10 min, ~12 GB under toolchain/)
stage-b/scripts/build-rustc-wasm.sh

# 2. matching wasm32-unknown-unknown sysroot for plugin (c)
stage-b/scripts/build-plugin-sysroot.sh stage1

# 3. Wasmtime 46 host
stage-b/run.sh build

# 4. full end-to-end check
stage-b/run.sh check
```

Measured on the 14-CPU Apple-silicon host:

| Item | Value |
|---|---|
| `./x.py install` wall time | **580.9 s** (9m41s); x.py reported 9m08s |
| toolchain disk after build | **12 GB** (`rust/` 10 GB, build 6.6 GB, artifacts/sysroots/cache ~1 GB) |
| `rustc.wasm` size | **143,715,574 B** (137 MiB) |
| `rustc.wasm` SHA-256 | `88d30a796bcf734206154f903fec012ffc2f1d5a5c89f240e67d1201b734f089` |
| rustc.wasm imports | WASI p1 (26), `env.memory`, `playground.spawn`, `wasi.thread-spawn` |
| rustc.wasm exports | `memory`, `_start`, `wasi_thread_start` |

The build uses the prior-art bootstrap config: `host=target=wasm32-wasip1-threads`,
`codegen-backends=["cranelift"]`, `panic=abort`, `-Ztranslate-remapped-path-to-local-path=no`.
CI LLVM is downloaded for the native stage1 compiler (one build-time use);
`./x.py install` never builds LLVM from source here.

### Is any LLVM inside rustc.wasm?

| Check | Result |
|---|---|
| LLVM imports / exports | **0** ([`results/rustc-inspect.json`](results/rustc-inspect.json)) |
| Function names containing `rustc_codegen_llvm` | **0** of 129 `llvm*` names |
| Function names containing `rustc_llvm` | **0** |
| Function names containing uppercase `LLVM` | **0** |
| `LLVMInitialize`, `LLVMRust`, `LLVMContext` strings | **0** |
| `libLLVM` string | 1 occurrence, in a linker search-path table (`…/self-containedlibLLVM.dylib…`), not code |
| Codegen backend list | `cranelift` only; the `llvm` feature is not linked |

The 129 `llvm_*` function names are cg_clif's own intrinsics shims
(`rustc_codegen_cranelift::intrinsics::llvm_aarch64::…`), i.e. Cranelift
implementations of LLVM-named intrinsics, not the LLVM library.

## Host design

`stage-b/host/` is a small Rust program (`stage-b-host`, Wasmtime 46.0.3):

* **WASI p1** via `wasmtime-wasi::p1`, with `/` preopened read-write so rustc
  sees the host filesystem at the same absolute paths (source, sysroot, output).
* **`playground.spawn`** `(argv_ptr, argc) -> i32`: reads the guest argv array
  from rustc's **shared** memory (`Extern::SharedMemory`, not `Extern::Memory`),
  then runs native Wild with the same arguments, inheriting stdio. Return 0 on
  success, the child exit code otherwise. Time spent here is the link time.
* **wasi-threads** via `wasmtime-wasi-threads`: `WasiThreadsCtx` instantiates a
  second instance over shared memory for rustc's compiler thread.
  `HostState: Clone` is required; each clone rebuilds a fresh `WasiP1Ctx` with
  the same args/env/preopens (`WasiP1Ctx` is not cloneable). The host filesystem
  is shared, so the compiler thread can open source/sysroot paths.
* **Module cache**: Wasmtime's on-disk cache (`--cache-dir`), with compression
  disabled because the 137 MiB module is faster to map than to decompress.
* **Fast teardown**: a 137 MiB compiled module + 1 GiB shared memory costs
  seconds to free; the CLI calls `process::exit` after printing, and the bench
  keeps one engine/module resident across edits.

## Sysroots

The plugin target is `wasm32-unknown-unknown` (no WASI imports, only
`nodecraft::host_bias`). Two sysroots were built:

| mode | builder | core / alloc / compiler_builtins | works with rustc.wasm? |
|---|---|---|---|
| `pure` (`sysroot-pure`) | Stage A patched cg_clif (native) | cg_clif / cg_clif / cg_clif | **No**: rlibs are `1.101.0-nightly`, rustc.wasm is `1.101.0-dev`; E0514 |
| `stage1` (`sysroot-stage1`) | native stage1 compiler from the rustc.wasm build | LLVM / LLVM / LLVM | **Yes**, exact version match; used by `run.sh` |
| `hybrid` (`sysroot-wasm-hybrid`) | rustc.wasm for core+alloc, stage1 LLVM for compiler_builtins | **cg_clif / cg_clif** / LLVM | **Yes**; optional proof of self-hosting |

`pure` is the ideal, but rustc's crate metadata includes the compiler version
string (`1.101.0-nightly (b6eeb93c9)` vs `1.101.0-dev`), so a sysroot built by
rustup's nightly is rejected by rustc.wasm even though the source commit is the
same. Building the sysroot with the native stage1 compiler from the same source
fixes that; it is LLVM-built code but **outside** rustc.wasm.

The optional `stage-b/scripts/build-sysroot-with-rustc-wasm.sh` runs
`rustc.wasm` as Cargo's Rust compiler for wasm crates (native stage1 for build
scripts) and successfully builds **core (856 KiB) and alloc (195 KiB) with
cg_clif inside rustc.wasm**. It then fails on `compiler_builtins` with the known
f128 libcall conflict:

```
attempt to declare `__multf3` with signature (f128,f128)->f128
but it was already declared with signature (sret i32,f128,f128)->()
```

A `compiler_builtins` without f128 is not available; the hybrid sysroot builds
that one crate with the native stage1 LLVM compiler *against rustc.wasm's core
metadata*, then rebuilds `alloc` with rustc.wasm, and the plugin compiles and
returns 2.25. This is the only LLVM-generated code in the plugin path, and it is
not inside `rustc.wasm`.

## Results

Full tables are appended to
[`../stage-a-results.md`](../stage-a-results.md); raw data is in
[`results/stage-b.json`](results/stage-b.json) and
[`results/a2.json`](results/a2.json).

The headline numbers (median of 5, warm, stage1 sysroot, macOS arm64):

| Phase | Median | Notes |
|---|---:|---|
| rustc.wasm module compile — cold, no cache | **39,650 ms** | 137 MiB, Wasmtime Cranelift |
| rustc.wasm module load — cache hit | **557.5 ms** | Wasmtime on-disk cache |
| instantiate rustc.wasm | **0.79 ms** | per edit; module resident |
| plugin compile inside rustc.wasm | **85.4 ms** | frontend + cg_clif + clif-to-wasm |
| Wild link (native, via spawn) | **5.36 ms** | 23 args, one invocation |
| load linked plugin | **6.97 ms** | 1.07 MB module |
| `eval(1,2,3,4)` | **~0.001 ms** | result 2.25 |
| **edit-to-result, module resident** | **98.1 ms** | instantiate + compile + link + load + eval |
| **CLI warm run, cache hit** | **~0.74 s wall** | includes 0.56 s module load + process start |

Target "well under 1 s warm": met both ways (98 ms in a hot host, 0.74 s for a
cold process with a warm module cache). The 39.7 s cold module compile is the
reason the cache matters; a hot-reload host should keep the module resident.

## What is still not Rust

1. **LLVM compiles the native stage1 rustc and the stage1 sysroot.** Stage0's
   LLVM backend compiles the native stage1 compiler, which then cross-compiles
   `rustc.wasm` and the matching sysroot. No LLVM is linked into `rustc.wasm`.
2. **`compiler_builtins` in the hybrid sysroot** is still LLVM-generated (core
   and alloc are cg_clif). Removing it needs either (a) clif-to-wasm support for
   f128 libcalls / the multi-result demotion used by compiler_builtins, or
   (b) rustc.wasm building its own stage2 from a pure sysroot, which is the
   self-hosting step.
3. **Wasmtime** is Rust with some C dependencies (signals, unwinding). Wild is
   pure Rust; `clif-to-wasm` and cg_clif are pure Rust.
4. **No debug info** in cg_clif wasm objects; `panic=abort` means traps.

The remaining non-Rust step is therefore *build-time only*: LLVM compiling the
stage1 compiler and, for now, `compiler_builtins`. A true bootstrap (stage0 wasm
built by cg_clif itself) would remove it; that is stage C's long pole, not a
blocker for shipping a plugin compiler.

## Blockers and caveats

* **Wasmtime 50 cannot run rustc.wasm** (wasi-threads removed). Pin 46.0.3, or
  patch rustc to avoid the compiler thread and use Wasmtime 50. The research
  note documents both.
* **Cold module compile is ~40 s.** Not a per-edit cost with a resident module
  or a warm Wasmtime cache, but a browser/Node host must ship a precompiled
  `.cwasm` or accept it.
* **`rustc.wasm` is 137 MiB** (98 MiB in the prior-art report). The extra size
  is largely the name section and the statically linked clippy/rustfmt
  entrypoints from the prior-art patch series; stripping symbols is possible if
  the LLVM-symbol check is done on a separate copy. Browser delivery should use
  Brotli/gzip (prior art: ~23 MiB gzipped).
* **Wild writes the 1 MiB `.bss` heap as a data segment** (1.07 MB plugin vs
  34.5 KB for the same plugin with rust-lld in stage A). The code section is only
  tens of KB; move the plugin heap out of `.bss` or use a host-allocated heap
  for distribution.
* **f128** remains unsupported by clif-to-wasm; it blocks a fully cg_clif-built
  `compiler_builtins` and is the same class of failure stage A saw at O0.
* **The host is desktop-only in this stage.** A browser/Node host needs the
  same WASI p1 + wasi-threads + spawn plumbing; the browser-kernel prior art
  already has it in JS, but that is stage C.

## Stage C recommendation (NodeCraft)

1. **Ship the raw ABI first**: `wasm_alloc(size, align)`, `eval(params_ptr, len,
   out_ptr) -> i32`, imports only `nodecraft::*`; no WASI, no clock, no RNG.
   Stage A and B both prove this path.
2. **Use a host-resident compiler process**, not a per-edit CLI: keep
   `rustc.wasm` compiled/instantiated, compile on a dedicated thread, keep the
   last good `InstancePre`, and cap plugin memory with `StoreLimits`. The 98 ms
   edit-to-result is the budget to preserve.
3. **Precompile and cache `rustc.wasm`** (Wasmtime `.cwasm` / browser
   `WebAssembly.compileStreaming` + Cache Storage). Do not pay the 40 s cold
   compile per session.
4. **Pin one bundle**: rustc.wasm + matching sysroot + Wild + host. The
   version-string trap (nightly vs `-dev`) is real; ship them as one versioned
   artifact and check the hash.
5. **Retire the LLVM-inside-wasm path now** for plugin codegen; keep LLVM only
   for the build-time stage1/sysroot until a self-hosting stage2 exists. For
   plugin codegen, cg_clif's 3–4× runtime gap on maths-heavy code is the main
   quality risk; host-call-heavy geometry is already at parity. If maths
   throughput matters, either accept it, add a small hand-written wasm SIMD
   kernel, or revisit a pure-Rust optimizing backend later.
6. **Do not require proc macros** in plugin builds; the prior art's
   `-Zwasm-proc-macros` path is complexity the SDK does not need. Prebuild the
   SDK rlib with pregenerated bindings.

## Layout

```
stage-b/
├── README.md                     # this file
├── research-wasmtime.md          # wasi-threads/spawn research, citations
├── run.sh                        # build/check/bench/inspect
├── host/                         # Wasmtime host (Cargo.lock committed)
├── scripts/
│   ├── build-rustc-wasm.sh       # prior-art patches + x.py install
│   ├── build-plugin-sysroot.sh   # pure | stage1
│   ├── build-sysroot-with-rustc-wasm.sh  # optional self-hosted core/alloc
│   └── rustc-wasm-wrapper.sh     # Cargo RUSTC wrapper for that experiment
├── a2/                           # A.2 plugins (e) maths, (f) geometry + harness
└── results/                      # stage-b.json, a2.json (committed)
```

## Licences

The prior-art repository is Apache-2.0; `clif-to-wasm` is Apache-2.0 WITH
LLVM-exception; Wild is Apache-2.0/MIT. This repository's own code is MIT OR
Apache-2.0 (see the repository root). The patch series is reused under the
prior art's Apache-2.0 licence; the `rustc.wasm` build output is a derived
artifact of rust-lang/rust (MIT OR Apache-2.0) and the prior-art patches.
