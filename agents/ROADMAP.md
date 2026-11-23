# Roadmap

The aim: a Rust compiler that runs anywhere WebAssembly runs, starts
instantly, compiles the whole language, contains no LLVM, is built without
LLVM, and produces code that matches or beats LLVM where it matters.

Four goals. Each item says how we will know it is done. Pick an unclaimed item,
open an issue saying you are working on it, and follow
[GUIDELINES.md](GUIDELINES.md).

Status keys: **done**, **in progress**, **open**, **research** (needs
investigation before it can be planned).

The next generation is drafted in [plans/v2.md](plans/v2.md): a
function-granular, always-optimising compiler (one optimised build per edit,
a size-only tier 2, a global store of optimised functions, table-slot hot
patching), with per-axis targets.

## Already done

| | |
| --- | --- |
| **done** | `rustc.wasm` with Cranelift as its only code generator, compiling a `no_std` + `alloc` program and linking it with Wild (stages A–C, [`compiler/`](../compiler/)) |
| **done** | Single-threaded compiler on plain WASI preview 1 and the latest Wasmtime |
| **done** | `core`, `alloc` and `compiler_builtins` built by `rustc.wasm` itself |
| **done** | Growable heap (`memory.grow` / `memory.size` in clif-to-wasm) |
| **done** | CI that builds and tests the whole pipeline and audits `rustc.wasm` for LLVM |
| **done** | Differential tests: the same programs built with LLVM and with our pipeline must produce identical output ([`difftest/`](../difftest/)) |
| **done** | A live REPL on the site: `rustc.wasm` and a single-threaded `wild.wasm` compile the visitor's Rust in their browser (≈ 50 ms per edit once warm) and hot-swap it into a running program; 40 MB, downloaded on request; CI packages it and checks it in Node's V8 ([`site/repl/`](../site/repl/)) |

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
   52 ns per point against LLVM's 12), but the attribution in
   [bench/gap/REPORT.md](../bench/gap/REPORT.md) found that LLVM also calls
   the software `libm::sqrt` there. A wasm `f64.sqrt` instruction therefore
   would not close that gap; the first real memory win is the constant-copy /
   `memcpy` lowering below.
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

**Where optimisation lives.** We keep the parts that give CogWasm its breadth:
rustc's frontend (it is what makes this real Rust), `rustc_codegen_cranelift`
(every intrinsic, the ABI, layouts, drop glue, panics, SIMD), clif-to-wasm and
Wild. What we may move is the *optimisation*. Today it can only run on Cranelift
IR, after Rust's guarantees are gone: that `&mut` never aliases, the types, and
which values are whole structs. Those are exactly what inlining and SROA need,
and MIR still has them. The final CLIF → wasm → CLIF round trip through Wasmtime
costs little on its own (Wasmtime's Cranelift re-runs value numbering, hoisting
and register allocation); what it cannot recover is what we put in linear
memory. Step 5a measured both places: a narrow MIR field-hoist cut nbody's
executed instructions by 10.6%, while the Cranelift stack-load prototype was
unsound without full escape/alias analysis and its sound slice found no
opportunity. **Step 5b therefore builds the optimiser in MIR.** See
[`step-5a.md`](../docs/notes/step-5a.md).

Techniques we expect to use: SSA construction directly from MIR (Braun et al.,
2013, "Simple and Efficient Construction of SSA Form"); inlining then SROA in a
loop; on-demand bounds-check elimination (ABCD, Bodík et al., 2000) for step 6;
and e-graph rewriting (Cranelift's acyclic e-graphs, the `egg` crate) for
algebraic clean-up once the structural gaps are closed. For the fast tier's
compile time (Goal 2), copy-and-patch compilation (Xu and Kjolstad, 2021) is
worth evaluating.

Work in this order; each step is measured with the benchmark suite before the
next starts.

| Status | Item | Done when |
| --- | --- | --- |
| **done** | First code-generation wins in clif-to-wasm (tiny-leaf inlining; 1.25× on A.2 (e), LLVM gap 3.76× → 3.00×) | Measured improvement on the A.2 maths benchmark; difftest green |
| **in progress** | A benchmark suite against LLVM `-O0` and `-O2` (small, medium and large programs plus compute kernels), run in CI, results published | A table in the README updated by CI |
| **done** | 1. Gap attribution: for each benchmark's hot loops, diff our wasm against LLVM's and assign every extra instruction to one of the five causes above | A published per-benchmark breakdown that names the next fix |
| **done** | 2. Lower constant and small `memcpy`/`memmove`/`memset` (the real memory gap; A.2 (e) is software `libm`, not a missing wasm `sqrt`): address read-only constant aggregates in place, inline constant copies up to 64 bytes, use wasm bulk-memory for larger or runtime lengths, and omit all-zero data segments in Wild | `sha256` 12.33× → 1.50×, `sort` 12.22× → 3.51×; no software `copy_forward` in their hot functions; difftest green |
| **open** | 3. Turn up rustc's own pure-Rust MIR optimisations (`-Zmir-opt-level`, `-Zinline-mir`: MIR inlining, SROA, GVN, copy propagation) for the optimising tier. **Measured no win:** 0.997× / 1.012× execution geomean, +1% compile, −0.4% size, outputs green; rustc's existing MIR passes are too conservative (little inlining; SROA gives up on aggregates whose address is passed on). Our own MIR passes are step 5a's other candidate | Speed and compile-time change measured per benchmark; difftest green |
| **done** | 4. Tidy loads and stores: fold constant offsets into wasm's load/store offset, drop redundant extends and address arithmetic where the 32-bit wrap cannot change the address | `strings` 111.2M → 107.2M and `miniz_driver` 216.4M → 216.1M executed instructions, no program regressed; suite geomean unchanged at 2.49× |
| **done** | 5a. Decide where optimisation lives: prototype inlining + SROA on `nbody` both as our own MIR passes (patched into rustc, using Rust's no-alias and type information) and as stack-slot → SSA promotion on Cranelift IR; compare executed instructions, speed, compile-time cost and the effort to generalise. **Decision: MIR.** The MIR field-hoist cut nbody 6.20G → 5.55G executed instructions (−10.6%) for +4.9% warm compile; the CLIF stack-load prototype moved 120 loads but miscompiled nbody, and its sound slice moved none. [Notes](../docs/notes/step-5a.md) | A written comparison with numbers, and a decision recorded here |
| **in progress** | 5b. Build the MIR winner of 5a: a cost-model inliner, then SROA/mem2reg, repeated (inline, promote non-escaping locals, repeat). The inliner (`-Zmir-inline-wasm`, patch `0006`) is measured on the full suite: instruction geomean 2.894× → 2.787× (`strings` 4.84× → 2.98×), difftest green, suite-median warm compile +1.29%; `strings`/`collections`/`sort`/`json` are still 2.98×/4.50×/1.99×/3.68× of LLVM `-O2`. The fifth increment adds the interprocedural non-capture summary and brings the loop-field hoist back as `-Zmir-capture-hoist` (patch `0008`): `nbody` 6.20G → 5.55G executed instructions (−10.55%) and 1.01G → 0.87G stack accesses, with the sysroot built by both passes green. SROA/mem2reg is not built | Iterator-heavy benchmarks within 1.2× of LLVM `-O2` (**not met: 2.0–4.5×**) |
| **open** | 6. Bounds-check removal from simple range facts (loop indices against slice lengths) | Measured on the benchmarks that index slices in loops |
| **open** | Tier 2: run steps 3–6 in a background optimising build and swap the result in when ready, keeping the fast tier for the edit loop | Scalar parity (within 1.1× of LLVM `-O2`, `simd128` off) with edit-to-result unchanged |
| **open** | Tier 2 in the live REPL: swap in the fast build at once, then build the optimising tier in the background (a second worker) and swap it in too, showing both timings; plus a compute-heavy example where the difference is visible. Works on GitHub Pages with plain workers; worth showing once tier 2 is measurably faster | The page shows the fast → optimised swap with timings and the optimised build runs measurably faster |
| **open** | Explicit SIMD (wasm `simd128`) supported end to end through clif-to-wasm, with SDK helpers | A `simd128` program in the difftest corpus; measured speed-up |
| **research** | 7. Loop unrolling, then auto-vectorisation in tier 2 for simple loops | Matches LLVM on the vectorisable benchmarks with `simd128` on |
| **research** | Profile-guided recompilation from live profiles | Measured win over the static tier-2 build |
| **research** | Specialisation of hot functions on runtime-constant inputs | Measured win, with a fallback when inputs change |
| **research** | Bulk maths offloaded to the GPU (host-compiled kernels via wgpu) | Beats LLVM-built wasm by an order of magnitude on large arrays |

Step 1 is done: [bench/gap/REPORT.md](../bench/gap/REPORT.md) attributes every program over 0.1 ms, with dynamic counts, hot-function WAT and the size breakdown. Batch 1 (19 Nov 2026) then did the fix it ranked first: small/constant `memcpy`/`memmove` lowering, constant aggregates addressed in place, zero-data omission in Wild and safe address folding. The suite geomean went from **3.53× to 2.49×** of LLVM `-O2`; `sha256` 12.33× → 1.50×, `sort` 12.22× → 3.51×, total size median 63.7 KB → 46.5 KB, and the compile-time median stayed flat (104.9 → 102.0 ms). The next step is SROA/mem2reg plus a cost-model inliner (step 5); the MIR-optimisation experiment (step 3) measured no execution win.

Step 5b has an inliner and the capture-aware hoist but not parity. Patch `0006-wasm-cost-model-inliner.patch` adds `-Zmir-inline-wasm`: calls in a cyclic block get a 200 cost threshold, a callee that takes `&mut`/`*mut` gets 400, calls outside loops stay with rustc's inliner, loop-free bodies are skipped, and inlining stops at four times the caller's original block count. On the full 21-program suite it changes ten modules and leaves eleven byte-identical; executed-instruction geomean 2.894× → 2.787×, with `strings` 4.84× → 2.98×, `collections` 4.77× → 4.50×, `sort` 2.13× → 1.99× and `json` 3.87× → 3.68× of LLVM `-O2`. Patch `0008` adds `-Zmir-capture-hoist`: a per-argument non-capture summary computed from callee MIR admits the `RawVec::grow_one` borrows, and the loop-field hoist returns; with both flags the suite is 8.29G executed instructions (off 9.01G), `nbody` 6.20G → 5.55G (−10.55%) and 1.01G → 0.87G stack accesses, and the per-program geomean ratio is 2.770×. The full difftest corpus is green with the flags off and on (56 handwritten + 24 generated, 0 mismatches), and a pure sysroot built with both passes on is green with the program flags off and on. Warm compile in this increment's full-suite pair is −1.54% median (both/off) and −0.59% (both/inline); the fourth increment's three interleaved pairs measured +1.29% for the inliner, with `strings` +9.97%, `json` +5.70% and `sort` +5.17% individually. That leaves criterion A unmet: the four iterator-heavy benchmarks are still 2.0–4.5× of LLVM `-O2`. The inliner, the hoist and the SROA/mem2reg half remain opt-in work; see [step-5b.md](../docs/notes/step-5b.md).

Tier 2's host side is planned in [supervisor.md](../docs/notes/supervisor.md): the compiler stays a pure function and a host-side supervisor owns the tier policy, the artifact cache and the swap. The swap is instance-level at a quiescent point, because WebAssembly cannot replace a function in a live instance and the engine call stack cannot be migrated. The first five milestones, the composable-spec refactor and the empirical selector are done: the reference supervisor (job contract, content-addressed store, fake compiler, supersession, swap seam, eight golden tests in CI) the real-backend spec matrix (an incremental spec is 1.3–2.2× faster to rebuild; a stripped spec is 26–42% smaller at the same compile time) the cold-restart swap demo (a swap costs a wasm-to-native compile, 240–270 ms, unless the module is precompiled, then 0.3 ms) the state ABI for stateful plugins (snapshot/restore with an ABI version check that rejects a mismatched swap), empirical spec selection (the host records compile/exec/size per spec and picks one with hysteresis) and the multi-crate workspace scheduler (dependencies build as cached rlibs; editing the app leaves the library a cache hit, editing the library rebuilds both). Per-codegen-unit reuse is left as a research item gated on a compiler boundary, with the generated-stress experiment as its evidence.

## Goal 2 — Near-instant compiler start

Today: the first load compiles the 137 MB `rustc.wasm` to native code (about
38 s); later loads come from Wasmtime's cache in about 0.46 s, or in about
19 ms from a per-host precompiled `.cwasm`. With `rustc.wasm` kept loaded,
a one-line edit of a small program reaches a running result in about 31 ms on
macOS arm64 and **23.6 ms on the CI runner** (was 174 ms; target 100 ms). The
CI gap was WASI file reads going through tokio's blocking pool with a fresh
host buffer per read (45 MB for libcore's metadata), fixed by letting WASI
block the calling thread; the pooling allocator saved another ≈ 31–36 ms per
job, and one long-lived rustc guest that serves every request (patch 0007,
metadata cached across requests) took the runner from 53.6 to 23.6 ms. Plugin-sized edits (the four 300–560-line medium benchmark programs) take
220–465 ms on CI through the serving guest; locally they then roughly
halved (171–363 → 101–179 ms; CI 191–323 ms) once edits reused rustc's incremental state,
which a cache-key bug had been discarding on every edit. The swap's native
compile of the output then went from 18–53 to 2–5 ms (parallel compilation
plus Wasmtime's incremental cache), so these edits are now 86–134 ms locally
and almost all rustc's incremental rebuild. Preopening the directories the guest
uses (instead of only `/`) made its file calls 3–7× cheaper: on CI these edits
are now 113–146 ms (small fixture 20.8 ms); see
[supervisor.md](../docs/notes/supervisor.md) and
[warm-session.md](../docs/notes/warm-session.md).

| Status | Item | Done when |
| --- | --- | --- |
| **done** | Precompiled native `rustc.wasm` per host, with version checks and a fallback (18.7 ms load; release workflow and dry-run bundle) | First compile after install in well under 1 s |
| **done** | Incremental compilation kept across edits (opt-in; 12–14% faster edit-to-result, +12–18% output) | One-line edits measurably faster than a cold compile |
| **in progress** | Snapshot an already-initialised compiler, so starting skips set-up; copy-on-write memory instantiation. Profiling showed the set-up worth skipping lives inside a rustc session (crate metadata, session state), not in guest start-up (≈ 3 ms), so the work is now "a warm rustc across compiles": design note with options and experiments in [warm-session.md](../docs/notes/warm-session.md). E1 done (reading the 45 MB `libcore.rmeta` costs ≈ 7 ms per compile); the resident backend now uses Wasmtime's pooling allocator with 256 MiB kept resident, which saves ≈ 31–36 ms per job on the CI runner (small fixture 125 → 93 ms; plugin-sized `json` 539 → 503 ms) and nothing on macOS | Instantiation in about 10 ms |
| **open** | A smaller `rustc.wasm` (drop Clippy, rustfmt and unused targets from the build) | Size and load time measured before/after |
| **done** | A resident compiler service in the host (keep `rustc.wasm` loaded, reuse its caches): the compile path is a library (`cogwasm_host::compile`), the supervisor has a resident backend (`--backend resident`: one engine and module, pooled memory, a fresh guest per job), WASI reads go straight into guest memory, and CI measures edit → result with `cogwasm-supervisor edit-loop` on every push. **CI runner: 23.6 ms** for a small edit (run 44997648351, one serving guest; was 174 ms). Plugin-sized edits (≈ 120–130 ms on CI) are the warm-session item's next target | Edit → result under 100 ms on a CI runner |
| **open** | The live REPL's fast path, step 1: rustc's incremental cache kept across edits in the compile worker's in-memory filesystem (a stable incremental directory per program, as the native edit loop does); needs nothing beyond plain Web Workers, so it works on GitHub Pages | Edit → result measured in the browser before and after; every edit's output still byte-identical to a cold compile in the smoke test |
| **open** | The live REPL's fast path, step 2: one `rustc.wasm` instance kept running across edits (the serving loop, with its metadata cache). Between edits the guest waits in `cogwasm.next_request`, which in a worker needs either cross-origin isolation for `SharedArrayBuffer` + `Atomics.wait` (GitHub Pages cannot send the headers, so a service worker adds them, as the in-browser playground does) or JS Promise Integration where the browser has it | Measured win over step 1 in the browser, with a fallback to a fresh instance per edit where neither is available |

The resident service and tier 2 share the supervisor planned in [supervisor.md](../docs/notes/supervisor.md).

## Goal 3 — The whole language and ecosystem

| Status | Item | Done when |
| --- | --- | --- |
| **done** | Libraries built by `rustc.wasm` itself: a `no_std` + `alloc` library compiles to an `rlib` and a program links against it with `--extern`; the supervisor's workspace mode builds a crate graph with a cache per crate (editing the app leaves the library a cache hit) | A program using a library crate, both compiled by `rustc.wasm`, runs correctly |
| **open** | Libraries in the live REPL, step 1: a curated set of pure-Rust `no_std` crates (for example `libm`, `glam` with `libm`, `fastrand`) built by `rustc.wasm` in CI from the same build as the sysroot, published with the REPL's compiler and fetched on demand when the visitor's code names them | A REPL example that `use`s a crate compiles in the browser; the smoke test covers it |
| **open** | Libraries, step 2: `std` for WASI preview 1 and 2, built by `rustc.wasm` and Cranelift, so crates that need `std` build | A `std` program in the difftest corpus |
| **open** | Libraries, step 3: procedural macros: proc-macro crates compiled to wasm and run inside the compiler (host-provided Wasmtime instance, at native speed; an interpreter in the browser, as the in-browser playground does) | `serde`'s derive works |
| **open** | Libraries, step 4: dependencies without cargo: a small resolver in the host that reads `Cargo.toml` and lock files, fetches sources, and feeds the workspace scheduler, plus a cache of prebuilt popular crates | A program using several crates.io crates builds |
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
