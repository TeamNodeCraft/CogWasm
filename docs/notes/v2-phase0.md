# V2 phase 0: what an edit actually rebuilds

Status: **phase 0 complete; phase 1's first experiment done and negative.** The
measurements are from a local macOS arm64 machine (14 cores, shared: the
compiler rebuild and some timing runs overlapped), 23 November 2026. The
generated program, the measurement scripts and the opt-in `0009` patch are in
the tree; the default compiler build and pipeline do not pass the flag, and
flag-off behaviour is unchanged.

## What phase 0 asked

1. A large, realistic program that `rustc.wasm` can compile: about 100k lines,
   `no_std + alloc`, many modules, generics, iterators, traits, structs and
   enums, deterministic output from an exported `run`.
2. The existing resident edit loop at that size, plus the four plugin-sized
   programs, so V1 has a number at every size.
3. For five realistic edits, how many functions' optimised MIR changed, how
   many functions rustc recompiled because of codegen-unit (CGU) granularity,
   and where the incremental time goes.

The answer decides whether phase 1's function-granular store is worth building:
if most of an edit's codegen is unchanged functions that only share a CGU, the
payoff is large; if the edit is dominated by the front end, incremental
save/load and link, a per-function store saves much less.

## The large program

[`bench/scripts/gen_large.py`](../../bench/scripts/gen_large.py) generates the
program from a seed (default `20261123`; output is not checked in):

- 144 inline modules, 98,246 lines, 4.32 MB of Rust.
- Every module has a struct, an enum, a trait impl, a trait object, a
  module-local key/value type, `Table<KeyN, u64>` and `Window<ValueN>`
  instantiations (so each module owns its monomorphised copies), twelve
  `#[inline(never)]` stage functions with iterator chains and `match`, one
  `#[inline]` local-copy helper, and a `run(seed)` that dispatches through the
  prelude's exported `run`.
- The crate root has a `static MODULES: [fn(u64) -> u64; 144]`, a `body` that
  calls every module and emits bytes, and the usual `BENCH_EDIT_TWEAK` probe.
- The program takes about 2.7 ms to run on the four suite seeds and emits
  1,232 deterministic bytes.

`bench/scripts/v2_phase0.py` applies the five edits below at fixed source
locations, always to the same file path so spans do not move, and compares the
final optimised MIR (`-Zdump-mir=SimplifyCfg-final`) of a cold compile of the
base with a cold compile of the edited source. Allocation, promoted-constant
and static ids are normalised before hashing: they are compiler numbering, and
an edit that adds one allocation renumbers everything after it.

| edit | what changes | MIR definitions changed | monomorphised instances affected |
| --- | --- | ---: | ---: |
| `one_function` | one instruction in `m064::hot_edit` | 1 | 1 |
| `add_function` | a new used function at the end of the file, called from `m032::run` | 2 | 2 |
| `signature` | `m016::stage_06`'s parameter type, plus its one caller | 3 | 3 |
| `struct_layout` | a field added to `m048::Packet48`, plus its one construction | 1 | 1 |
| `generic_insert` | the body of `support::Table::insert`, instantiated once per module | 1 definition | about 144 instances |

The monomorphised-instance count comes from `-Zquery-dep-graph
-Zprint-mono-items`: the MIR dump is per generic definition, so the cold dump
sees `Table::insert` once, while every module has its own `KeyN` instance.

**Difftest.** LLVM `rustc -O2` and the CogWasm pipeline produce the same
`run(seed)` and output buffer on seeds `0, 1, 42, 2^64-1` (1,595,579-byte LLVM
module, 7,089,876-byte flag-off Cranelift module; reproduce with
[`bench/scripts/v2_difftest_large.sh`](../../bench/scripts/v2_difftest_large.sh)).
The quick difftest corpus is green below.

## V1 edit loop at every size

`cogwasm-supervisor edit-loop --backend resident --verify-cold`, serving guest,
incremental, cached swap, precompiled `rustc.wasm`. Five edits per source; every
warm edit was byte-identical to a fresh cold compile. Peak RSS is the whole
supervisor process (`/usr/bin/time -l`). The four plugin sources are
`bench/programs/medium/*.rs` plus the shared prelude and the benchmark probe,
prepared the same way as `ci/edit-loop-sources.sh`.

| source | lines | cold build | output | peak RSS | edit→result p50 | p90 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| large (generated) | 98,246 | 19,830 ms | 12,754,344 B | 2,017 MB | 2,191.5 ms | 2,223.9 ms |
| `json` | 562 | 219 ms | 115,979 B | 424 MB | 92.2 ms | 98.6 ms |
| `collections` | 309 | 295 ms | 252,788 B | 433 MB | 110.3 ms | 117.9 ms |
| `scene` | 499 | 155 ms | 79,344 B | 423 MB | 80.2 ms | 86.4 ms |
| `expr` | 475 | 153 ms | 75,560 B | 422 MB | 76.3 ms | 84.4 ms |

For the large program the 2.19 s median splits into 1.99 s build, 0.19 s swap
(Wasmtime compiling the 12.75 MB output) and 0.01 s call; Wild's link is 24 ms.
The four plugin edits split into 74–106 ms build, 2–5 ms swap, 0.1 ms call and
6–8 ms link. The large program is the first case where the edit loop is far
from the 100 ms target: the build is dominated by rustc re-optimising and
re-codegenning a CGU that holds a whole module's functions.

The numbers above are from the V1 compiler (before `0009`). The same flag-off
run on the rebuilt compiler measured 18,724 ms cold and 2,243.0 ms p50
(2,255.9 ms p90), within run-to-run noise; all phase-1 comparisons below use
the rebuilt compiler's flag-off run.

## Cacheability: what the edit actually rebuilds

Native stage-1 compiler (the build's `--host aarch64-apple-darwin
--target wasm32-wasip1` stage 1), `-Copt-level=2 -Zmir-opt-level=2`, against
`wasm32-wasip1`; the partitioner and incremental machinery are target
independent and wasm32-wasip1 has the same pointer width and layouts as
wasm32-unknown-unknown. The product pipeline uses Cranelift, not this stage-1
compiler's LLVM backend, so the *shape* of the split is what transfers, not the
absolute codegen milliseconds. `-Zprint-mono-items -Zquery-dep-graph` gave the
rebuilt CGUs and the functions in them; `-Ztime-passes` and `-Zself-profile`
gave the split; the product link/swap numbers are the edit loop's.
`-Zincremental-info` logs for the one-function edit, flag off and on, are in
`toolchain/work/v2/incremental-info/`; the per-CGU reuse counts below come from
the `CGU_REUSE` lines, because the query table does not expose the CGU decision
directly.

| edit | CGUs rebuilt / total | function bodies in them (distinct) | unchanged share | LLVM codegen of rebuilt CGUs | edit wall |
| --- | ---: | ---: | ---: | ---: | ---: |
| `one_function` | 1 / 153 | 98 (98) | 99% | 26 ms | 2,765 ms |
| `add_function` | 2 / 153 | 145 (127) | 99% | 31 ms | 3,573 ms |
| `signature` | 3 / 153 | 15,699 (15,678) | 99.98% | 2,498 ms | 5,924 ms |
| `struct_layout` | 2 / 153 | 143 (125) | 99% | 31 ms | 2,937 ms |
| `generic_insert` | 107 / 256 | 2,935 (2,931) | about 95% | 270 ms | 3,853 ms |

The one-function edit is exactly the case phase 1 targets: **one** function's
MIR changed, but rustc rebuilt a CGU containing **98** function bodies; 97 of
them were unchanged. The signature edit is worse than its own function count:
the rebuilt set includes a shared fallback/volatile CGU holding 15,556 inlined
bodies, and optimising that CGU costs 2.47 s of the 5.92 s edit. The
widely-used generic is the only edit that invalidates many CGUs (107 of 256,
because adding the `Vec::swap` call also adds monomorphisations), and even there
about 95% of the rebuilt bodies are unchanged.

Time split of the same incremental compiles (ms; front end = parse, expand,
resolve, type check, borrow check; mono = monomorphisation walk and partition;
save = result-cache and dep-graph serialisation plus work-product copy; link is
the native `rlib` link, not Wild):

| edit | front end | mono/partition | to LLVM IR | LLVM passes | incremental save | link | total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `one_function` | 446 | 312 | 10 | 36 | 499 | 30 | 2,765 |
| `add_function` | 468 | 317 | 10 | 42 | 495 | 31 | 3,573 |
| `signature` | 468 | 321 | 567 | 2,466 | 486 | 30 | 5,924 |
| `struct_layout` | 462 | 314 | 10 | 48 | 505 | 37 | 2,937 |
| `generic_insert` | 456 | 357 | 96 | 303 | 518 | 41 | 3,853 |

What this says:

- **The cacheable share is real but not where the time is for the good case.**
  A one-function edit has 97/98 unchanged bodies, yet codegen is only 36 ms of
  a 2.8 s edit. The rest is the fixed per-edit work: about 0.45 s front end,
  0.31 s mono/partition, 0.50 s incremental save, plus the rest of rustc's
  query and metadata work. A per-function store can remove most of the 36 ms,
  not most of the 2.8 s.
- **The signature case is the one with a big codegen payoff.** 2.47 s of LLVM
  optimisation is spent on a shared fallback/volatile CGU holding 15,556
  bodies; a
  per-function store/stitcher that never forms that CGU would recover almost
  all of it. But phase 1 has to avoid the CGU machinery's own fixed cost to
  realise that.
- **Fixed costs dominate the plugin-sized edits too.** Their builds are
  74–106 ms, of which codegen is a small part; the resident backend's guest,
  incremental save/load and Wild link are the rest.

## Phase 1, first step: one function per CGU

`compiler/patches/rust/0009-cgu-per-function.patch` adds
`-Zcogwasm-cgu-per-fn` (off by default, incremental mode only). Root
(`GloballyShared`) functions are named after their mangled symbol and placed in
their own CGU; the existing `LocalCopy` inlined-copy handling is unchanged;
statics and global assembly keep their module-based CGUs; the
`-Ccodegen-units` merging loop is skipped when the flag is on, because merging
would undo the experiment.

The native stage-1 compiler with the flag gives the deterministic counts. The
CGU total is the headline: the crate has **7,661** root CGUs, not 153. The
number includes GloballyShared monomorphisations from `core` and `alloc` as
well as the user functions, which is why it is much larger than the module
count.

| edit | CGUs rebuilt / total | function bodies in rebuilt CGUs | codegen of rebuilt CGUs | work-product copy | link | incremental save | edit wall |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `one_function` | 1 / 7,661 | 7 | 1 ms | 1,832 ms | 1,293 ms | 608 ms | 8,156 ms |
| `add_function` | 6 / 7,662 | 57 | 11 ms | 1,806 ms | 1,277 ms | 613 ms | 8,258 ms |
| `signature` | 9 / 7,661 | 120 | 21 ms | 1,808 ms | 1,241 ms | 535 ms | 8,187 ms |
| `struct_layout` | 5 / 7,661 | 54 | 10 ms | 1,828 ms | 1,280 ms | 577 ms | 8,307 ms |
| `generic_insert` | 292 / 7,805 | 1,880 | 305 ms | 1,850 ms | 1,307 ms | 573 ms | 8,464 ms |

The flag does exactly what it says at the codegen level: the one-function edit
rebuilds 7 bodies instead of 98, the signature edit rebuilds 120 instead of
15,699, and LLVM optimisation falls from 2.47 s to 21 ms. **But every edit is
slower**, because rustc now copies 7,661 CGU work products into the incremental
cache (1.8 s per edit, against 33 ms), links 7,661 objects/rlib members (1.2 s,
against 31 ms), and pays a little more incremental save/load. The per-edit floor
is about 8.1–8.5 s, against 2.8–5.9 s flag-off. This is a negative result for
"one function per CGU implemented through rustc's CGU machinery".

### The product pipeline confirms it

Plugin-sized programs, same rebuilt compiler, `edit-loop --verify-cold`, five
edits, flag-off vs `--cgu-per-fn`. Every warm edit was byte-identical to a cold
compile with the same flag.

| program | p50 off | p50 on | change | output off | output on | peak RSS off/on | cold off/on |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `json` | 94.8 ms | 145.0 ms | **+53%** | 116,083 B | 128,862 B | 425 / 1,072 MB | 209 / 286 ms |
| `collections` | 106.7 ms | 290.9 ms | **+173%** | 252,820 B | 275,417 B | 433 / 1,081 MB | 295 / 457 ms |
| `scene` | 78.3 ms | 151.9 ms | **+94%** | 79,376 B | 84,957 B | 423 / 1,070 MB | 159 / 236 ms |
| `expr` | 76.9 ms | 110.6 ms | **+44%** | 75,656 B | 86,971 B | 420 / 1,069 MB | 154 / 206 ms |

Peak RSS goes from about 425 MB to about 1.07 GB even for these small programs.
Codegen is not the only cost; the 7,661-CGU bookkeeping, work-product copying
and a larger link dominate.

Executed instructions and module size from `bench/gap` (`--no-mir`, the
incremental per-fn artifact; the gap runner's own `sizes` field reads a stale
side file, so the sizes below are the module files):

| program | module off | module on | change | instructions off | instructions on | change | ours/LLVM-O2 off → on |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `json` | 94,384 B | 128,130 B | +35.8% | 1,785,050 | 2,580,670 | **+44.6%** | 3.871 → 5.690 |
| `collections` | 223,702 B | 275,196 B | +23.0% | 4,816,227 | 5,236,492 | +8.7% | 4.771 → 5.209 |
| `scene` | 67,620 B | 84,730 B | +25.3% | 398,549 | 593,956 | **+49.0%** | 3.652 → 5.560 |
| `expr` | 60,583 B | 86,295 B | +42.4% | 2,567,021 | 3,769,458 | **+46.8%** | 4.037 → 6.014 |

Per-function CGUs also prevent inlining across functions that used to share a
module CGU, so the output gets both larger and slower. That is a second,
independent cost on top of the edit-loop overhead.

### The large program cannot be linked this way

`edit-loop --cgu-per-fn` on the 98k-line program compiled and then failed at
the link step:

```text
[cogwasm-host] cannot run .../wild: Argument list too long (os error 7)
[cogwasm-host] spawn: .../wild (7687 args) -> -1 in 1.1 ms
error: linker ... not found
```

7,687 linker arguments exceed the host's limit. The attempt took 33.75 s
(including 6.34 s module load) and peaked at 4.38 GB RSS; the flag-off cold
build is 18.72 s and 2.03 GB. There is therefore no edit→result and no
warm-vs-cold byte-identity result for the large program with the flag. The
native measurement above is the evidence for the large program.

### Correctness gates

- `bench/gap` completed for both configurations (module build, instrumentation
  and execution); `bench/gap/results` was restored with
  `git checkout -- bench/gap/results` afterwards.
- `difftest/run.sh --quick` (56 handwritten + 8 generated): **PASS** flag off
  (46 s) and **PASS** with `DIFFTEST_CGU_PER_FN=1` (49 s, with
  `-Cincremental` so the flag actually applies), zero mismatches in both.
- Large program, flag off: LLVM vs pipeline outcomes match on all four seeds.
- Plugin programs, flag on: every edit of every program passed
  `edit-loop --verify-cold`, i.e. the warm incremental artifact was
  byte-identical to a fresh cold compile of the same source with the same flag.
- Large program, flag on: the link failure above prevents the check.
- Default pipeline: flag-off difftest is green and the flag-off code path is
  unchanged. Module sizes are unchanged (94,396 B for `json`; about 7.09 MB
  for the large program), and the pipeline's outputs match LLVM.
- **Correction (review):** the first draft of this note said the pipeline is
  nondeterministic run to run. It is not. Three compiles of `json` to
  `json-1.wasm`, `json-2.wasm` and `json-3.wasm` differ in exactly one byte:
  the module name Wild writes into the wasm `name` section, which is the output
  file's name (`1`, `2`, `3`). The same source compiled to the same output name
  gives identical bytes. This still matters for V2: the content-addressed store
  must not hash the output file name (or the stitcher should write a fixed
  module name).

## What phase 1 should do next

The numbers say the goal is right and this implementation is wrong.

1. **Do not use rustc's CGU machinery for per-function granularity.** 7,661
   CGUs turn a 25 ms codegen win into a 5.4 s loss. The phase 1 store and
   stitcher must bypass CGU work products and the object linker entirely, as the
   plan's copy-and-patch stitcher intends; otherwise the fixed costs swamp the
   cacheable work. The large-program link failure is the same problem in its
   starkest form: one object per function is not a link strategy.
2. **The codegen payoff is real in the signature case.** Rebuilding a shared
   fallback/volatile CGU holding 15,556 bodies costs 2.47 s; avoiding it
   is worth doing. A first version should split only *local user functions*
   (or only CGUs that actually contain changed bodies), keep shared generics
   and inlined copies in coarser units, and cap the number of CGUs. That keeps
   the 2.5 s win without 7,661 work products.
3. **Attack the fixed per-edit cost next.** Even with a perfect codegen cache,
   the one-function edit is 2.7 s, of which about 0.44 s is the front end,
   0.30 s mono/partition, 0.48 s incremental save/load and the rest other
   query/metadata work. The plan's resident front end (phase 2) and in-memory
   incremental state are the levers; the per-function store alone cannot reach
   50 ms on this program.
4. **Keep the flag opt-in and do not ship it.** It is a measurement tool. The
   default pipeline, compiler build and sysroot do not pass it, and the
   correctness gates above hold with it on.
5. **Re-measure the split on Cranelift.** The native stage-1 split is the right
   shape but uses LLVM for codegen; the product's Cranelift backend and the
   Wild link have different absolute costs. The plugin edit-loop numbers above
   are the product-level check; the large program needs a linker that accepts
   thousands of inputs (or the stitcher) before it can be measured end to end.

## Reproduce

```sh
# 1. Generate the program (not checked in) and check it against LLVM.
python3 bench/scripts/gen_large.py \
    --prelude bench/programs/prelude.rs \
    --out toolchain/work/v2/large/large.rs
bench/scripts/v2_difftest_large.sh

# 2. V1 edit loop, large program.
toolchain/target/cogwasm-host/release/cogwasm-supervisor edit-loop \
    --backend resident \
    --host toolchain/target/cogwasm-host/release/cogwasm-host \
    --rustc toolchain/artifacts/rustc.wasm --sysroot toolchain/sysroot \
    --linker toolchain/target/wild/release/wild \
    --precompiled toolchain/work/precompiled/rustc.cwasm \
    --cache-dir toolchain/cache \
    --work toolchain/work/v2/large/editloop/work \
    --store toolchain/work/v2/large/editloop/store \
    --compiler-id v2-phase0-large \
    --source toolchain/work/v2/large/large.rs --edits 5 --verify-cold

# 3. Cacheability (native stage 1; flag off, then flag on).
python3 bench/scripts/v2_phase0.py \
    --large toolchain/work/v2/large/large.rs \
    --out toolchain/work/v2/cacheability --rounds 2
python3 bench/scripts/v2_phase0.py \
    --large toolchain/work/v2/large/large.rs \
    --out toolchain/work/v2/cacheability-perfn --rounds 2 --cgu-per-fn

# 4. Product per-fn plugin measurement.
#    (edit-loop with --cgu-per-fn, as in toolchain/work/v2/plugin-perfn.sh)

# 5. Difftest with the flag on and off.
difftest/run.sh --quick
DIFFTEST_CGU_PER_FN=1 difftest/run.sh --quick

# 6. Instructions and size with the flag on.
BENCH_CGU_PER_FN=1 BENCH_KEEP_INCREMENTAL_ARTIFACT=1 \
    bench/gap/run.sh --no-mir json collections scene expr
git checkout -- bench/gap/results
```

`BENCH_KEEP_INCREMENTAL_ARTIFACT=1` is needed because `bench-host` normally
rebuilds the measured module without incremental mode; the per-function flag
only applies to the incremental compile.

## Files changed

- `bench/scripts/gen_large.py` (new generator), `bench/scripts/v2_phase0.py`
  (new measurement script), `bench/scripts/v2_difftest_large.sh` (LLVM-vs-
  pipeline check for the large program).
- `docs/notes/v2-phase0.md` (this note), `agents/plans/v2.md` (V1 numbers).
- `compiler/patches/rust/0009-cgu-per-function.patch` (new),
  `compiler/upstreams.toml` (lists it after `0008`),
  `toolchain/rust/compiler/rustc_session/src/options.rs`,
  `toolchain/rust/compiler/rustc_monomorphize/src/partitioning.rs`,
  `toolchain/rust/.cogwasm-applied-patches` (the new hash).
- `crates/cogwasm-supervisor/src/job.rs` and `src/main.rs` (the opt-in
  `cgu_per_fn` option and `edit-loop --cgu-per-fn`).
- `bench/run.py`, `bench/host/src/main.rs`, `difftest/run.sh` (experiment-only
  plumbing; defaults unchanged).

The patch series check: a pristine `git archive` of `toolchain/rust` HEAD with
`0002`–`0009` applied reproduces all 21 files the series touches byte for byte,
and `0009` reverse-applies to the working tree. `compiler/scripts/upstream.py
check` reports the manifest OK.
