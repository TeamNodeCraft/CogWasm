# Stage C results

Date: 2026-11-18/19. Host: macOS arm64 (Apple silicon, 14 CPUs, 48 GB RAM).
Isolated rustup/cargo under `../toolchain/`. rustc source
`b6eeb93c992027a03e1fd120b6652546396b142c` (`nightly-2026-10-19`,
`rustc 1.101.0-dev` as built), prior-art 23-patch series, cg_clif
`4f78fd3e…`, clif-to-wasm `0e4e0458…` + stage-C patch, Wild `fd79578…`
0.11.0, **Wasmtime 50.0.2** with plain WASI p1. Medians are of five warm runs
unless stated; "cold" is a fresh Wasmtime module compile with no cache.

## Verdict

| Check | Result |
|---|---|
| rustc.wasm built for **wasm32-wasip1** (non-threads), cg_clif only | **PASS** |
| No `wasi:thread-spawn` import; no wasi-threads host | **PASS** (27 imports: 26 WASI p1 + `playground.spawn`) |
| Runs on latest stable Wasmtime 50.0.2, plain WASI p1 | **PASS** |
| Pure sysroot (`core` + `alloc` + `compiler_builtins`) built by rustc.wasm | **PASS**, no E0514 |
| f128 ABI conflict fixed in cg_clif | **PASS** |
| Strict sysroot build, no trap stubs | **PASS** (0 warnings) |
| Plugins (a), (b), (c), (e), (f) strict with the pure sysroot | **PASS** |
| `memory.size` / `memory.grow` implemented; growable heap | **PASS** |
| Plugin (c) `eval(1,2,3,4)` under Wasmtime | **PASS**, **2.25** |
| Plugin (c) small with Wild and rust-lld | **PASS**, 34,921 / 34,940 B (was 1,083,560 B with Wild) |

## rustc.wasm

| Item | Value |
|---|---|
| Size | **143,849,756 B** (137.2 MiB) |
| SHA-256 | `3c7ca043ac35adcc994de4f335aa71ded3270dc2aa92ce8e21ceee7a93c0302f` |
| Imports | 26 WASI p1 + `playground.spawn`; **no** `wasi.thread-spawn`, **no** `env.memory` |
| Exports | `memory`, `_start` |
| LLVM imports / exports | 0 / 0 |
| `rustc_codegen_llvm` / `rustc_llvm` symbols | 0 / 0 |
| `LLVMInitialize` strings | 0 |
| `libLLVM` strings | 1 (linker search-path table, not code) |
| Build | incremental `./x.py install`, **3 min 27 s**; stage-B initial build remains 580.9 s |

Raw inspection: [`results/rustc-inspect.json`](results/rustc-inspect.json).

## Stage B table, latest Wasmtime (median of 5)

Plugin (c) source, pure sysroot, total edit-to-result. The stage-B column is
the Wasmtime 46.0.3 + wasi-threads result from
`notes/history/stage-b/results/stage-b.json`.

| Phase | Stage B (Wasmtime 46, threads) | **Stage C (Wasmtime 50.0.2, p1)** | C / B |
|---|---:|---:|---:|
| rustc.wasm module compile, cold, no cache | 39,649.97 ms | **38,467.66 ms** | 0.97× |
| rustc.wasm module load, cache hit | 557.54 ms | **440.52 ms** | 0.79× |
| instantiate rustc.wasm (per edit, module resident) | 0.79 ms | **0.755 ms** | 0.96× |
| compile inside rustc.wasm (frontend + cg_clif + clif-to-wasm) | 85.36 ms | **83.64 ms** | 0.98× |
| Wild link (host `playground.spawn`) | 5.36 ms | **6.46 ms** | 1.21× |
| load linked plugin | 6.97 ms | **9.95 ms** | 1.43× |
| `eval(1,2,3,4)` | ~0.001 ms | **~0.001 ms** | — |
| **edit-to-result, module resident** | 98.05 ms | **101.23 ms** | 1.03× |
| plugin bytes | 1,066,803 | **34,945** | 0.033× (30.5× smaller) |
| result | 2.25 | **2.25** | — |

Warm edit-to-result is flat (101 ms vs 98 ms): faster compiled-module load and
slightly faster compile, offset by a slower link/plugin load. The big change
is artifact size: the 1 MiB Wild `.bss` segment is gone.

Raw: [`results/stage-c.json`](results/stage-c.json),
[`results/stage-c-bench.log`](results/stage-c-bench.log).

## A.2 table with the pure sysroot (median of 5)

(e) 100k f64 points: `sin`, `cos`, `sqrt`, reduce. (f) 40k iterations × 5
imported host calls = 200,000 calls. Stage-C "warm" is the compile+link time
reported inside rustc.wasm (`STAGE_C_TIME=1`); the whole Cargo process wall
with a warm Wasmtime cache is 0.675 s (e) / 0.609 s (f), dominated by the
~0.44 s rustc.wasm module load.

| program / metric | LLVM O2 | stage-B cg_clif | **stage-C cg_clif (pure sysroot)** | C / A |
|---|---:|---:|---:|---:|
| (e) warm compile+link | 0.100 s | 0.131 s | **0.1196 s** (compile 0.1133 + link 0.0067) | 0.91× |
| (e) Wasmtime module load | 6.77 ms | 17.18 ms | **17.07 ms** | 0.99× |
| (e) run 100k points | 2.30 ms / 23.0 ns·pt | 8.78 ms / 87.8 ns·pt | **8.95 ms / 89.5 ns·pt** | 1.02× |
| (e) code / total size | 13,173 / 16,773 B | 39,709 / 1,110,860 B | **39,995 / 62,124 B** | 1.01× / 0.056× |
| (f) warm compile+link | 0.090 s | 0.129 s | **0.1110 s** (compile 0.1048 + link 0.0067) | 0.86× |
| (f) Wasmtime module load | 4.08 ms | 10.15 ms | **9.75 ms** | 0.96× |
| (f) run 200k host calls | 0.622 ms / 3.11 ns·call | 0.570 ms / 2.85 ns·call | **0.585 ms / 2.92 ns·call** | 1.03× |
| (f) code / total size | 4,638 / 7,815 B | 21,679 / 1,082,687 B | **21,965 / 34,059 B** | 1.01× / 0.031× |

The cg_clif runtime and code-size gap versus LLVM is unchanged: maths is
**3.9×** slower, host-call-heavy geometry is at parity, code is 3.0× / 4.7×
larger. The total artifact size drops by 18–32× because the growable heap no
longer lives in `.bss` (data: 5,865 B for e, 2,542 B for f, vs ~1.05 MB each).

Raw: [`results/a2-pure-load.log`](results/a2-pure-load.log),
[`results/a2-pure-timing.log`](results/a2-pure-timing.log),
[`results/a2-pure-timing-e.log`](results/a2-pure-timing-e.log),
[`results/a2-pure-compile.json`](results/a2-pure-compile.json).

## Plugin (c) sizes and the Wild `.bss` problem

| artifact | total | code | data | memory min |
|---|---:|---:|---:|---:|
| stage-A c_plugin, rust-lld, fixed `.bss` | 34,502 | 22,877 | 2,772 | 33 pages |
| stage-A c_plugin, Wild, fixed `.bss` | **1,083,560** | 22,877 | **1,051,866** | 33 pages |
| stage-B c_plugin, Wild, fixed `.bss` | 1,066,803 | — | — | — |
| **stage-C c_plugin, rust-lld, growable** | **34,940** | 23,018 | 2,420 | **17 pages** |
| **stage-C c_plugin, Wild, growable** | **34,921** | 23,018 | 2,420 | **17 pages** |
| LLVM O2 reference, rust-lld | 7,274 | — | — | — |

Wild and rust-lld now agree on data (2,420 B) and memory minimum (17 pages);
Wild is 19 bytes smaller. The previous behaviour — Wild writing the entire
zero-initialised `.bss` (1,051,866 B) as a data segment — is documented in
[`notes/item3-memory-grow.md`](notes/item3-memory-grow.md). It does not need a
Wild patch at this scale; a future large zeroed static should be re-checked.

## Strict mode and failures

* `CG_CLIF_WASM_STRICT=1` builds `core`, `alloc`, `compiler_builtins` with
  rustc.wasm: **no unsupported/trap-stub diagnostics**.
* `CG_CLIF_WASM_STRICT=1` builds a_add, b_alloc, c_plugin (growable), e_math,
  f_geometry with the pure sysroot: all pass.
* `x.py install` exits non-zero in the final `generate-copyright` step when
  clif-to-wasm is a path dependency. The `rustc.wasm` artifact is produced
  before that step; the stage-C build script copies it. This is a build
  packaging quirk, not a compiler failure.
* Wasmtime 50.0.2 no longer has wasi-threads; the tested module does not need
  it. No `-Sthreads`/`wasi-threads` setting is used.

## Patch list

| # | Patch | One line | Upstream-ready? |
|---|---|---|---|
| 1 | `compiler/patches/rust/0001-wasm-run-compiler-and-codegen-on-calling-thread.patch` | run rustc and codegen on the calling thread on wasm | yes, with upstream review/test harness |
| 2 | `compiler/patches/cg_clif/0002-wasm-f128-sret-return.patch` | f128 returns through sret on wasm (LLVM C ABI) | yes, has example + test-suite case |
| 3 | `compiler/patches/clif-to-wasm/0001-lower-wasm-memory-grow-and-size.patch` | lower reserved memory.grow/size helper calls | yes, tests included |
| 4 | `compiler/patches/cg_clif/0001-wasm-lower-memory-grow-and-size.patch` | map `llvm.wasm.memory.*` to those helpers | yes, paired with #3 |
| — | Wild | no patch; growable heap removes the 1 MiB data segment | n/a |

## Raw measurement notes

* Stage-C module cold compile samples: 39,061.7, 38,467.7, 38,266.0,
  38,348.0, 38,881.9 ms. Cached loads: 441.2, 443.8, 440.5, 439.9, 439.1 ms.
* Stage-C edit-to-result samples: 129.2, 104.9, 99.5, 100.0, 101.2 ms
  (first run is the one-off compile of a fresh target directory).
* A.2 compile samples and load samples are in the raw logs above; both
  use one `Engine`/`Module` for the runtime calls and a fresh engine for the
  five load measurements, matching the stage-B A.2 harness.
* All linked artifacts pass `wasmparser 0.263` validation with all features.
* Disk: toolchain 28 GB, 76 GB free after the stage; well above the 15 GB stop.
