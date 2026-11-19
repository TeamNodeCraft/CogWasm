# Roadmap

The aim: a Rust compiler that runs anywhere WebAssembly runs, starts
instantly, compiles the whole language, contains no LLVM, is built without
LLVM, and produces code that matches or beats LLVM where it matters.

Four goals. Each item says how we will know it is done. Pick an unclaimed item,
open an issue saying you are working on it, and follow
[GUIDELINES.md](GUIDELINES.md).

Status keys: **done**, **in progress**, **open**, **research** (needs
investigation before it can be planned).

## Already done

| | |
| --- | --- |
| **done** | `rustc.wasm` with Cranelift as its only code generator, compiling a `no_std` + `alloc` program and linking it with Wild (stages A–C, [`experiments/cranelift-wasm/`](../experiments/cranelift-wasm/)) |
| **done** | Single-threaded compiler on plain WASI preview 1 and the latest Wasmtime |
| **done** | `core`, `alloc` and `compiler_builtins` built by `rustc.wasm` itself |
| **done** | Growable heap (`memory.grow` / `memory.size` in clif-to-wasm) |
| **done** | CI that builds and tests the whole pipeline and audits `rustc.wasm` for LLVM |
| **done** | Differential tests: the same programs built with LLVM and with our pipeline must produce identical output ([`difftest/`](../experiments/cranelift-wasm/difftest/)) |

## Goal 1 — Generated code as fast as LLVM, faster where it counts

Today: tight numeric loops run about 3× slower than LLVM `-O2` output; code
that mostly calls its host runs at parity.

**What parity means.** The geometric mean over the benchmark suite's programs,
against LLVM `-O2` built for the same wasm features and run in the same
Wasmtime. The first target is **scalar parity: within 1.1×, with `simd128` off
on both sides**. Matching LLVM's auto-vectorisation is a separate, later goal.

**Where the gap is.** Both pipelines end in Wasmtime, which compiles the wasm
to native code with the same Cranelift backend. So the whole gap is in the wasm
we hand it. Roughly in order of cost for idiomatic Rust:

1. **Operations done in software.** Most of A.2 (e)'s gap was `sqrt` (about
   52 ns per point against LLVM's 12): LLVM emits wasm's `f64.sqrt`, we call a
   routine. The same may hold for `min`/`max`/`abs`/`copysign`/`floor`/`ceil`/
   `trunc`, bit counts and rotates.
2. **Values in memory instead of registers.** Wasm has no addressable stack,
   so any local whose address is taken (structs, tuples, `Option`, iterator
   state) lives in linear memory and every use is a load or store. LLVM's SROA
   and mem2reg turn nearly all of these back into registers; Cranelift has no
   equivalent.
3. **Too little inlining.** Iterators, closures and small methods are only
   free once inlined, which then also removes most of point 2.
4. **Bounds checks** that LLVM proves away with range analysis.
5. **Loop optimisation:** unrolling, and vectorisation when SIMD is allowed.

**Compile time.** Every pass costs time, and Goal 2 wants fast compiles. So
optimisation goes in a second tier, as in a JIT: compile fast for the edit
loop, rebuild optimised in the background, swap the faster plugin in when it
is ready.

Work in this order; each step is measured with the benchmark suite before the
next starts.

| Status | Item | Done when |
| --- | --- | --- |
| **done** | First code-generation wins in clif-to-wasm (tiny-leaf inlining; 1.25× on A.2 (e), LLVM gap 3.76× → 3.00×) | Measured improvement on the A.2 maths benchmark; difftest green |
| **in progress** | A benchmark suite against LLVM `-O0` and `-O2` (small, medium and large programs plus compute kernels), run in CI, results published | A table in the README updated by CI |
| **done** | 1. Gap attribution: for each benchmark's hot loops, diff our wasm against LLVM's and assign every extra instruction to one of the five causes above | A published per-benchmark breakdown that names the next fix |
| **open** | 2. Map built-in operations 1:1 to wasm instructions (`sqrt` first, then the rest of the list in point 1) | No software routine where wasm has an instruction; A.2 (e) gap measured |
| **open** | 3. Turn up rustc's own pure-Rust MIR optimisations (`-Zmir-opt-level`, `-Zinline-mir`: MIR inlining, SROA, GVN, copy propagation) for the optimising tier | Speed and compile-time change measured per benchmark; difftest green |
| **open** | 4. Tidy loads and stores: fold constant offsets into wasm's load/store offset, drop redundant extends and address arithmetic | Instruction counts in hot loops measured before/after |
| **open** | 5. A cost-model inliner plus SROA/mem2reg on Cranelift IR (inline, promote non-escaping stack slots to SSA values, repeat) | Iterator-heavy benchmarks within 1.2× of LLVM `-O2` |
| **open** | 6. Bounds-check removal from simple range facts (loop indices against slice lengths) | Measured on the benchmarks that index slices in loops |
| **open** | Tier 2: run steps 3–6 in a background optimising build and swap the result in when ready, keeping the fast tier for the edit loop | Scalar parity (within 1.1× of LLVM `-O2`, `simd128` off) with edit-to-result unchanged |
| **open** | Explicit SIMD (wasm `simd128`) supported end to end through clif-to-wasm, with SDK helpers | A `simd128` program in the difftest corpus; measured speed-up |
| **research** | 7. Loop unrolling, then auto-vectorisation in tier 2 for simple loops | Matches LLVM on the vectorisable benchmarks with `simd128` on |
| **research** | Profile-guided recompilation from live profiles | Measured win over the static tier-2 build |
| **research** | Specialisation of hot functions on runtime-constant inputs | Measured win, with a fallback when inputs change |
| **research** | Bulk maths offloaded to the GPU (host-compiled kernels via wgpu) | Beats LLVM-built wasm by an order of magnitude on large arrays |

Step 1 is done: [bench/gap/REPORT.md](../bench/gap/REPORT.md) attributes every program over 0.1 ms, with dynamic counts, hot-function WAT and the size breakdown. The next fix it names is small/constant `memcpy`/`memmove` lowering (84% of sort's and 91% of sha256's extra instructions), then SROA/mem2reg plus a cost-model inliner.

## Goal 2 — Near-instant compiler start

Today: the first load compiles the 137 MB `rustc.wasm` to native code (about
38 s); later loads come from Wasmtime's cache in about 0.46 s, or in about
19 ms from a per-host precompiled `.cwasm`.

| Status | Item | Done when |
| --- | --- | --- |
| **done** | Precompiled native `rustc.wasm` per host, with version checks and a fallback (18.7 ms load; release workflow and dry-run bundle) | First compile after install in well under 1 s |
| **done** | Incremental compilation kept across edits (opt-in; 12–14% faster edit-to-result, +12–18% output) | One-line edits measurably faster than a cold compile |
| **open** | Snapshot an already-initialised compiler, so starting skips set-up; copy-on-write memory instantiation | Instantiation in about 10 ms |
| **open** | A smaller `rustc.wasm` (drop Clippy, rustfmt and unused targets from the build) | Size and load time measured before/after |
| **open** | A resident compiler service in the host (keep `rustc.wasm` loaded, reuse its caches) | Edit → result under 100 ms on a CI runner |

## Goal 3 — The whole language and ecosystem

| Status | Item | Done when |
| --- | --- | --- |
| **open** | Procedural macros: proc-macro crates compiled to wasm and run inside the compiler (host-provided Wasmtime instance, at native speed) | `serde`'s derive works |
| **open** | `std` for WASI preview 1 and 2, built by Cranelift | A `std` program in the difftest corpus |
| **open** | Multiple crates and dependencies: a small resolver in the host plus a cache of prebuilt popular crates | A program using several crates.io crates builds |
| **open** | Component model output (WIT, WASI 0.2) | A component runs in Wasmtime |
| **open** | Debug info: DWARF through the Cranelift backend, clif-to-wasm and Wild | Stack traces with source lines |
| **open** | Panics that unwind, via wasm exception handling | Unwinding tests pass |
| **open** | Threads: host-provided `thread-spawn` now, shared-everything-threads when it ships (see the README) | Parallel code generation measured |
| **in progress** | Versioned releases: `rustc.wasm`, matching standard library, Wild and third-party notices (workflow + local dry-run bundle; not yet published) | A published, reproducible release |

## Goal 4 — No LLVM anywhere, not even to build

Today LLVM is used once at build time to compile rustc itself into
`rustc.wasm`, and `rustc.wasm`'s own `std` contains wasi-libc, which is C.

| Status | Item | Done when |
| --- | --- | --- |
| **research** | Self-hosting: `rustc.wasm` compiles rustc into `rustc.wasm` | A `rustc.wasm` built only by `rustc.wasm` passes CI and difftest |
| **research** | Replace wasi-libc in the compiler's `std` with Rust (or a `std` that targets WASI directly) | No C in the build |
| **open** | Reproducible builds: the same inputs give a byte-identical `rustc.wasm` | Two independent builds compare equal |

## Quality work (always open)

| Status | Item | Done when |
| --- | --- | --- |
| **open** | Make the review scan a blocking CI gate (fix existing findings first) | `quality.yml` fails on any finding |
| **open** | Fix the cause of the tolerated `generate-copyright` packaging failure | CI no longer matches a failure message |
| **open** | Run rustc's run-pass tests and rustc_codegen_cranelift's own tests through our pipeline | Pass rate published and tracked |
| **open** | Fuzzing with a random program generator in every CI run | Seeds and findings recorded |

## Good first tasks

- Add programs to the difftest corpus for areas it doesn't cover yet.
- Fix review-scan findings in one file at a time.
- Improve error messages in the host when compilation fails.
- Measure and document something nobody has measured yet.
