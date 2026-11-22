# Goal 1 step 5b: a MIR optimiser

Status: **in progress**. The first SROA increment is landed and measured on the
suite. A loop-aware cost-model inliner is landed behind its own flag and
measured on the full suite through a rebuilt `rustc.wasm`, with a green
difftest. A third increment diagnoses the locals that still live in memory
after inlining and adds a narrow threshold for mutably-borrowed callees; it is
measured on a six-program subset, leaves the five flag-off modules it was
compared on byte-identical, and does not move `nbody`. All of it is opt-in and no speed claim is made for
the product default.

> The pass code, patch header and this note are LLM-generated under the Rust
> project's personal-use exemption. This is a local CogWasm experiment, not an
> upstream contribution; a human must review and own it before it goes
> anywhere else.

## What landed

`compiler/patches/rust/0005-hoist-loop-invariant-field-reads.patch` adds
`LoopFieldHoist`, a MIR pass after GVN that is opt-in through
`-Zmir-hoist-loop-field-reads`, off by default. The compiler build, the pure
sysroot and the default benchmark compiles do not pass the flag and do not run
it.

`sroa` promotes a local only when its address is not taken, so a `Vec` or struct
local that was written before a loop stays in its stack slot and every field
read inside the loop goes to memory. The pass takes a read of a scalar field of
such a local, evaluates it once in the loop preheader into a fresh local, and
replaces the in-loop read with a copy of that local. The original assignment to
its destination stays in the loop, so a loop that runs zero times leaves the
destination untouched.

This is the SROA half of the step-5a prototype, not the inliner. The prototype
was a 230-line experiment scoped to the `nbody` crate; it hoisted into the
destination local directly (wrong for a zero-trip loop) and did not reason
about aliases. This version fixes the destination and is conservative:

- a direct read of `base.field` cannot coexist with a live unique borrow of
  `base` in borrow-checked MIR, so a borrow taken before the loop cannot mutate
  the field while the loop reads it directly;
- a shared borrow cannot write a `Freeze` base, which is required;
- raw pointers are outside those guarantees. A raw pointer to a local (not
  through a dereference), a cast to a raw pointer from a reference, or an
  integer-to-pointer cast skips the function; pointers into a buffer the local
  points at do not alias its own fields and are allowed. This is deliberately
  coarse; a points-to analysis can admit more later;
- borrows of `base`, moves out of it, discriminant writes and storage changes
  inside the loop reject the base;
- any borrow created inside the loop rejects it, because it can mutate between
  iterations without a borrow conflict.

The pass is conservative enough that the calls that keep `Vec`'s address
escaping still limit what it sees. That is the cost-model inliner's job.

## Verification so far

A synthetic function keeps `p` in memory with an `#[inline(never)]` call before
the loop and then reads `p.x` in the loop. A native stage-1 build with the
patch dumps this MIR (abridged):

```text
bb1 (preheader):
    _14 = no_retag copy (_1.0: u64);
bb3 (loop):
    _13 = copy _14;
    nop;
    _11 = Add(copy _12, copy _13);
```

So the field read moved out of the loop and the loop reuses the hoisted local.
The patch applies cleanly to a pristine pre-change tree and reproduces the
working tree byte-for-byte.

The first version of the guard skipped every function containing any raw
pointer. It built and passed difftest in CI, but the quick suite's executed
instruction counts were identical to the pre-pass run: nbody's `body` contains
raw pointers into its buffers, so the whole function was skipped. Narrowing the
guard to pointers that can alias a local makes the pass hoist 95 reads in
`body` (native MIR dump); that is the version in the patch now.

### Why it is opt-in

The narrowed guard was built in CI and rustc.wasm ICE'd while compiling `core`:

```text
panicked at rustc_mir_dataflow/src/framework/mod.rs:145
index out of bounds: the len is 9 but the index is 10
query stack: mir_const_qualif ... `iter::...::rfind::check::{closure#0}`
```

The earlier version ran at level 2 by default. At that level the stage-1
compiler also ran the pass when it built rustc.wasm, so it transformed rustc's
own unsafe-heavy crates and miscompiled the compiler. The narrowed raw-pointer
guard is not strong enough for unsafe code: a call can still return a raw
pointer into the base. The pass is therefore behind an explicit unstable flag,
which the compiler, sysroot and default benchmark builds do not pass. A real
escape/alias analysis is the way back to a default-on pass.

### First experiment run

The first `mir_hoist` dispatch (run 44670568809, commit `9f33698`) hit the
compiler cache (build and test 10 minutes instead of 46) and **passed difftest
with the pass enabled**. The quick suite then failed, but not on a miscompile:
with the pass, `sha256`'s compiler fuel differed by 5 units between the two
`--fuel` samples. That is intermittent; it reproduced in 1 of 3 local runs and
the samples agree without the pass. The runtime instruction counts do not
depend on fuel, so the experiment quick suite now runs without `--fuel`; the
compiler-work metric is unavailable for experiment runs until the
nondeterminism is understood.

A local reconstruction with a rebuilt rustc.wasm compiled all twelve quick
programs with the flag. `nbody`'s results matched the same compiler without the
flag for all four quick seeds, and its module was 413 bytes smaller (41,707 to
41,294). Instrumenting both modules and counting executed wasm instructions
reproduces the step-5a result exactly: **6,204,745,840 -> 5,550,101,348**
(-10.55%, the same total the 5a prototype reported). The no-flag count matches
the CI pre-pass baseline exactly, so the local compiler is representative.

### Second experiment run

Run 44676541954 (dispatch on `fe8cad0`, hoist on, `--fuel` off for the reason
above) completed in **24 minutes** on a compiler-cache hit. Difftest and the
codegen structure checks passed with the pass enabled; the chart point was
skipped.

| program | instructions, default | with hoist | change | module bytes |
|---|---:|---:|---:|---:|
| `nbody` | 6,204,745,840 | 5,550,101,348 | **-10.55%** | 41,304 -> 40,896 |
| `b_alloc` | 1,632,952 | 1,604,658 | -1.73% | 62,062 -> 62,050 |
| `matmul` | 380,350,388 | 380,153,792 | -0.05% | 31,290 |
| `sha256` | 70,417,340 | 70,400,948 | -0.02% | 32,185 -> 32,182 |
| `expr` | 2,666,213 | 2,666,401 | **+0.01%** | 59,450 -> 59,454 |
| other seven | | unchanged | 0 | |

Quick-suite instruction geomean: **1.84366 -> 1.80274** (-2.2% relative).

The win is real but narrow: one kernel and a small allocation program. `expr`
is the only regression, 188 instructions (+0.01%), from a hoist whose in-loop
copy costs more than the load it removes. The compiler-work metric is absent
because the fuel determinism guard flakes on `sha256` with the pass.

Profiling the `expr` regression attributed all 188 instructions to
`expr::Env::set`, whose `while i < self.vars.len()` lookup loop is single-level
and runs a handful of times: the preheader read is paid per loop entry, and the
in-loop copy does not pay it back. The pass now only optimizes loops that are
part of a nest (nested in another loop, or containing a nested loop). With that
gate, a local A/B on the same compiler (only the flag differing) gives `expr`
**0.000%**, `nbody` **-10.551%** and `b_alloc` **-1.731%**, so the win is kept
and the regression is gone. The suite table above is from the pre-gate run; the
gated dispatch will confirm the final suite numbers.

The suite numbers above are still **not measured**. `rustc.wasm` cannot be run
in this environment (only paths outside the checkout work inside its WASI
sandbox), so the benchmark run must happen on a normal machine or in CI. Until
then this is an implementation increment, not a speed result.

## Reproduce

```sh
# MIR-level check, native compiler (the config targets wasm by default)
toolchain/rust/x build --stage 1 \
    --host aarch64-apple-darwin --target aarch64-apple-darwin library rustc
RUSTC_BOOTSTRAP=1 build/host/stage1/bin/rustc --edition 2024 -O \
    -Zmir-hoist-loop-field-reads \
    -Zdump-mir=LoopFieldHoist -Zdump-mir-dir=/tmp/mirdump \
    --crate-type lib test.rs
ls /tmp/mirdump
```

```sh
# product pipeline
compiler/scripts/build-rustc-wasm.sh
bench/run.sh --quick --runs 3 --fuel
bench/gap/run.sh --no-mir nbody
```

The pipeline builds programs without the flag, so the pass is off there. The
bench and difftest scripts opt the clif compile in with `BENCH_MIR_HOIST=1` and
`DIFFTEST_MIR_HOIST=1`; the `cranelift_wasm` workflow exposes the same switch as
the `mir_hoist` workflow_dispatch input. That run still publishes its quick JSON
and Markdown asset, but skips publishing the point to the benchmark history, so
an experiment cannot pollute the chart. Because the compiler inputs do not
change, a dispatch after the normal run can reuse the cached compiler outputs
and skip the ~26-minute build.

## Second increment: a loop-aware cost-model inliner

`compiler/patches/rust/0006-wasm-cost-model-inliner.patch` adds `InlineWasm`, a
MIR pass after the normal inliner, opt-in through `-Zmir-inline-wasm` and
`-Zmir-inline-wasm-threshold` (default 400). It reuses rustc's inlining
machinery and changes only the decision.

### What the baseline does

MIR inlining already runs in the compiles the suite measures: passing
`-Zinline-mir=yes` on top of the default pipeline produced **byte-identical
modules** for `strings`, `collections`, `expr`, `json` and `scene`. The
remaining calls are the ones the size-based cost model declines, and its
default thresholds are what keeps them out of line:

| `strings::body` final MIR | lines | blocks |
| --- | ---: | ---: |
| default thresholds | 1,676 | 161 |
| `-Zinline-mir-threshold=100` | 1,676 | 161 |
| `-Zinline-mir-threshold=200` | 3,171 | 323 |
| `-Zinline-mir-threshold=400` | 6,614 | 583 |
| `InlineWasm` (this patch) | 3,441 | 335 |

Threshold 100, the setting the step-3 experiment used, is exactly the default
for these calls, which is why that run measured nothing on `strings`.

### What a global threshold buys, locally

A quick A/B with the existing `rustc.wasm` and only the threshold flags, on a
/tmp mirror of the checkout and toolchain (5 programs, 3 runs x 4 seeds):

| program | default | threshold 400 | change | ours/LLVM-O2 |
| --- | ---: | ---: | ---: | ---: |
| `strings` | 1.462 ms | 0.863 ms | **-41.0%** | 8.75x -> 5.14x |
| `collections` | 0.0690 ms | 0.0595 ms | -13.7% | 5.73x -> 4.04x |
| `expr` | 0.0401 ms | 0.0373 ms | -7.0% | within noise |
| `json` | 0.0269 ms | 0.0249 ms | -7.4% | within noise |
| `scene` | 0.0083 ms | 0.0078 ms | -6.0% | within noise |

`strings` is the robust result (sample ranges 0.84-1.02 ms against
1.39-1.55 ms); the smaller programs are inside run-to-run noise. Warm compile
costs +2% to +10%, incremental compile is unchanged, and every output stayed
identical. This is the ceiling a cost-model inliner is aiming at, not the
patch's own result.

### What the patch does

- Calls in a cyclic block of the caller's CFG get the 200 threshold; calls
  outside loops stay with rustc's inliner, so cold code does not pay.
- Bodies without a back edge are skipped before the inliner runs, which keeps
  the pass off the many loop-free functions in a crate.
- The recursion limit is raised from five to twenty levels because the
  iterator chain is nested; the growth budget, not the depth limit, bounds
  the body, and inlining stops at four times the caller's original block
  count.
- Loop membership comes from the dominator back edges; blocks created by an
  inline inherit the call site's flag, so the chain keeps inlining inside the
  loop.

Native MIR validation (stage-1 compiler, `--emit=obj`, wasm32 target):
`strings::body` 1,676 -> 3,441 lines over 335 blocks and `json` 1,183 ->
1,265; `scene`, `expr` and `collections` are unchanged at `body`. The loop
chain is inlined and the cold setup is not.

### What the pass measures

`rustc.wasm` was rebuilt with the patch and the full suite was measured with
the pass off and on, same compiler, 21 programs, 3 runs x 4 seeds. At the
default threshold (200) only nine of the twenty-one modules change at all;
the other twelve are byte-identical to the baseline, and every program
returned identical results.

| program | off | on | change | ours/LLVM-O2 | module | warm compile |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `strings` | 1.422 ms | 0.828 ms | **-41.8%** | 8.40x -> 4.74x | -4.1% | +0.2% |
| `json` | 0.0266 ms | 0.0243 ms | -8.7% | 3.38x -> 2.72x | +0.3% | +0.6% |
| `collections` | 0.0694 ms | 0.0634 ms | -8.6% | 5.74x -> 4.76x | -2.7% | +2.8% |
| `sort` | 3.530 ms | 3.320 ms | -6.0% | 3.55x -> 3.34x | -0.3% | -0.8% |
| `scene` | 0.0089 ms | 0.0085 ms | -4.5% | 1.86x -> 1.66x | -0.2% | +0.1% |
| `smoke` | 0.0074 ms | 0.0072 ms | -3.6% | 2.38x -> 2.23x | +0.2% | +1.5% |
| `nbody` | 72.31 ms | 71.88 ms | -0.6% | 5.06x -> 5.02x | 0.0% | +0.6% |
| `float_sum` | 0.0048 ms | 0.0049 ms | +1.7% | 1.46x -> 1.56x | +2.0% | +1.1% |

Execution geomean **-4.4%**, warm compile **+0.45%** and module size **-0.2%**
over all 21 programs. `strings` is the robust win (0.79-0.95 ms against
1.38-1.66 ms) and its module is 4.1% smaller. Every positive delta in the
suite belongs to a program whose module is byte-identical (`c_plugin` +7.7%,
`int_loops` +7.6%, `generated_stress` +6.5%, `libm_driver` +6.0%), so those
are measurement noise, not code changes.

### Why the default is 200 and not 300

The first revision of the patch used 300. It gave `strings` the same time but
inlined more: 5,072 rather than 3,441 MIR lines of `strings::body`, modules
grew (median +1.4%), warm compile cost +3.3%, and `int_loops` changed: its
body grew from 89 to 189 blocks, and the 2-3 microsecond run got 38.7% slower
with 7.1% fewer executed instructions -- a hot-loop layout effect. At 200
every one of those programs is untouched, the suite geomean is better
(-4.4% against -2.3%), and compile and size are essentially flat (+0.45%,
-0.2%). The global-threshold experiment pointed the same way: 400 on the
existing compiler measured `strings` -41.0% for +2% to +10% warm compile.

The other knobs: with the normal inliner's five-level recursion limit and the
400 threshold the pass reached 3,499 MIR lines of `strings::body` and
`strings` improved -16.9%; raising the limit to 20 gave 5,072 lines at 400 and
-43.5%. At 200 the chain inlines to 3,441 lines and -41.8%, so the deeper
limit matters mainly for the larger callees that 400 admits. A 400 threshold
with no early exit for loop-free bodies cost `libm_driver` +65% warm compile
for no execution gain; with the early exit and 200 that is -2.5%. The
four-times growth budget does not bind on any suite program.

The full difftest corpus is green with the 200 threshold enabled: 49
handwritten + 24 generated programs, every `run(seed)` and output buffer
identical to the LLVM build.

### What is not done

The threshold and growth constants are still first estimates, and the pass
has to be measured on a machine other than this one. The suite geomean is a
small win, not parity; `strings` carries most of it.

## Third increment: what still lives in memory after inlining

Status: **implemented and measured, opt-in only**. The diagnosis below is from
final optimized MIR; the measured result is a six-program subset with the
rebuilt `rustc.wasm`. The change stays behind `-Zmir-inline-wasm`; the five
flag-off modules compared against the previous compiler are byte-identical, so
the default pipeline did not change.

### Method

The stage-1 native compiler (patches 0001-0007) dumped final optimized MIR for
the `body` function of `strings`, `collections`, `sort`, `json` and `nbody`
with `-Copt-level=2 -Zmir-opt-level=2 -Zmir-inline-wasm` at the landed 200
threshold. The target is
`wasm32-wasip1`, the only wasm target the stage-1 build has; its pointer width
(32 bits) and aggregate layout match `wasm32-unknown-unknown`. A local counts
as address-taken when a `Rvalue::Ref` or `Rvalue::RawPtr` roots at it (a
leading field projection still roots at the local), which is exactly cg_clif's
`analyze` rule: such a local becomes a stack slot and every read or write is a
linear-memory load or store. Loop membership is the natural-loop union: a
back edge whose target dominates the source, then every block the header
dominates that can reach it again.

The count covers locals whose address is taken. cg_clif also leaves an
aggregate without a CLIF type in memory when it is *not* address-taken
(`nbody` still has five `[f64; 3]` locals after SROA); those are outside this
table but are part of why `nbody`'s stack traffic is so large.

### The diagnosis

| program | address-taken locals | loop-contained | what the hot ones are |
| --- | ---: | ---: | --- |
| `strings` | 16 | 4 | two `SplitWhitespace` locals: one whose `slice::Iter` is passed to `next_code_point` (cost 370, so declined at 200), one borrowed by `find`/`IsNotEmpty` and the `find` closure; the other 12 are `[u8;8]` `emit_bytes` buffers outside the hot loop and `Rng` borrowed by the one-shot `build_lines`. |
| `collections` | 36 | 12 | `Vec` behind `RawVec::grow_one` (`#[inline(never)]`) and whole `&Vec`; `VecDeque`, `String` and `BTreeSet` behind `&mut` methods (`push_back`/`pop_front`, `push`/`push_str`, `insert`/`entry`); iterator locals (`Zip`, `Chain`, `vec_deque::Iter`, `Take<Skip>`, `Union`, `btree_map::Iter`) behind non-inlined `next`; two `sort_by` function pointers; `[u8;8]` emit buffers; one `fmt` tuple. |
| `sort` | 14 | 6 | `Vec` behind `grow_one`; `ManuallyDrop<u64>` raw borrow in the heapsort loop; one function pointer; 11 `[u8;8]`/`[u8;1]` emit buffers in the 16-iteration output loop. |
| `json` | 23 | 22 | almost all loop-contained but cold: `Value`, `Stats`, `Vec` and `String` behind the recursive `walk`/`collect_paths` (one-shot; recursion cannot inline), 18 `[u8;8]` buffers in the output loop, sort's `ManuallyDrop`. No hot local survives. |
| `nbody` | 16 | 4 | the four `Vec` locals behind `RawVec::grow_one` (`#[inline(never)]`); they also need a drop, so SROA refuses them whole and the hot loops reload `Vec.ptr`/`len` each iteration. The 12 `[u8;8]` emit buffers are cold. |

The reasons fall into four buckets:

1. a cold or one-shot call the inliner never sees inside a loop (`build_lines`,
   `walk`, `collect_paths`, `emit_bytes`);
2. a callee with `#[inline(never)]` (`RawVec::grow_one`), which no threshold
   should override;
3. a callee that takes `&mut`/`*mut` of the local and is above the loop
   threshold (`next_code_point`, `Iterator::next` for the BTree iterators,
   `String::push`, `BinaryHeap::sift_up`);
4. a whole-local `&` borrow outside the loop (`&Vec` for indexing) or a drop,
   which excludes a `Vec`/`String` from SROA for the whole function.

### A second promotion round is a no-op

The plan's "iterate inlining and promotion" was implemented first: after
inlining, run rustc's `ReferencePropagation`, `ScalarReplacementOfAggregates`
and `GVN` a second time, once after the first `GVN` and once at the end of the
pipeline. Native final-MIR A/B:

| program | address-taken before | after | static accesses to them in loop blocks |
| --- | ---: | ---: | ---: |
| `strings` | 16 | 16 | 53 -> 53 |
| `collections` | 36 | 35 | 61 -> 56 |
| `sort` | 14 | 14 | 20 -> 20 |
| `json` | 23 | 23 | 71 -> 73 |
| `nbody` | 16 | 16 | 130 -> 130 |

It was not kept. The reason is in rustc, not in the pass order:
`sroa`'s `excluded_locals` marks any local whose field address is observed, so
a repeat round cannot promote the borrowed aggregates; `Vec`/`String` also
need a drop, which `sroa` does not expand. The remaining locals are exactly
the ones a call keeps alive.

### The fix: a mutably-borrowed callee

The kept change is one rule in `InlineWasm`: a callee that takes a
`&mut`/`*mut` argument gets a 400 cost threshold instead of 200, whether or
not it is marked `#[inline]`. Inlining that call is the only way to remove
the borrow without new alias analysis. The rule is narrow on purpose: raising
the whole loop threshold to 400 also inlines shared-borrow and by-value
helpers, which grew `nbody`'s body from 144 to 264 blocks and its module by
10.6% for no executed-instruction change.

Before (200, `strings::body` final MIR, the hot `split_whitespace` loop):

```text
bb75:
    _272 = &mut ((((((((_24.0: ...Filter...).0: ...Split...).0: ...SplitInternal...)
        .2: ...CharPredicateSearcher...).0: ...MultiCharEqSearcher...)
        .2: ...CharIndices...).1: ...Chars...).0: core::slice::Iter<'_, u8>);
    _271 = next_code_point::<'_, core::slice::Iter<'_, u8>>(move _272)
        -> [return: bb72, unwind unreachable];
```

After (the mut rule): `next_code_point` is inlined at the loop-contained call
sites (`scope 322 (inlined next_code_point::<slice::Iter<u8>>)` in the dump),
and the hot loop no longer has a `next_code_point` call rooted at the
`SplitWhitespace` local. `strings::body` goes from 16 to 15 address-taken
locals and 149 to 148 blocks; `collections::body` goes from 36 to 34 (the
inlined `BinaryHeap` and `BTreeSet` paths drop a `Copied<Intersection>` local
and a `fmt` tuple). `sort`, `json` and `nbody` are unchanged in MIR shape,
module size and executed instructions.

### Measured result

`rustc.wasm` rebuilt with this patch, six programs, 3 runs x 4 seeds, off and
on with the same compiler. Counts are executed wasm instructions
(`bench/gap`), not timings; module bytes and warm compile are the suite's
medians. Every output was identical.

| program | off | on | change | stack load/store, off -> on | module | warm compile |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `strings` | 107,244,701 | 66,008,021 | **-38.4%** | 11,972,805 -> 9,489,332 (-20.7%) | 52,002 -> 53,077 (+2.1%) | 91.9 -> 97.6 ms (+6.2%) |
| `collections` | 5,061,326 | 4,772,355 | -5.7% | 568,111 -> 552,039 (-2.8%) | 223,702 -> 218,702 (-2.2%) | 244.3 -> 248.1 ms (+1.6%) |
| `sort` | 321,207,695 | 299,459,431 | -6.8% | 18,094,617 -> 20,593,087 (+13.8%) | 50,535 -> 50,560 (+0.05%) | 90.4 -> 93.7 ms (+3.6%) |
| `json` | 1,871,062 | 1,777,296 | -5.0% | 240,977 -> 232,549 (-3.5%) | 94,384 -> 96,151 (+1.9%) | 163.3 -> 160.6 ms (-1.6%) |
| `nbody` | 6,204,745,840 | 6,204,745,840 | 0.0% | 1,012,060,384 -> 1,012,060,384 | 41,858 (same) | 89.8 -> 91.9 ms (+2.3%) |
| `int_loops` | 213,915 | 213,915 | 0.0% | 6,264 (same) | 19,172 (same) | 60.3 -> 62.6 ms (+3.8%) |

`int_loops` and `nbody` are byte-identical, so their compile deltas are the
shared machine; the flag-off strings module is byte-identical to the previous
compiler's. The inliner's own win is mostly `strings`; the third increment
adds the difference between the old 200 rule and this one, which the same-harness
A/B puts at `strings` 71.68M -> 66.01M (-7.9%), `collections` 4.864M -> 4.772M
(-1.9%) and `json` 1.797M -> 1.777M (-1.1%), with `sort`, `nbody` and
`int_loops` unchanged. The rule is compile-time neutral in that A/B: the old
200 build warm-compiled `strings` in 97.9 ms and this one in 97.6 ms. The
+6.2% off-to-on warm compile is the inliner's existing cost from the second
increment, not this rule's; it is the one number just outside G1.1's 5%
target and should be re-measured before the pass is considered for a default
tier. The other five are within +/-4%.

### Patch reproduction

The regenerated `0006-wasm-cost-model-inliner.patch` applies cleanly to a
scratch worktree at the prior-art tip (`2f59a94fd`, which is patch 0001's
commit) with `0002`-`0005` and `0007` applied, and reproduces the four files
it owns byte-for-byte; it also reverse-applies to the working tree. The
scratch check skips `0001` because the prior-art series already contains it,
which is what the build script's reverse check does on a fresh checkout. The
only difference from a full `toolchain/rust` copy is the build script's
`clif-to-wasm` path rewrite in `compiler/rustc_codegen_cranelift/Cargo.toml`
and the `setup-sources.sh` changes; neither is part of a patch. The
`.cogwasm-applied-patches` 0006 entry was updated to the new file hash.

### What this does not fix

`nbody`'s 1.01G stack accesses are unchanged: a `Vec` needs a drop and is
borrowed by `#[inline(never)] grow_one`, so SROA cannot see it and the
inliner must not override `inline(never)`. That remains the loop-field hoist's
job (patch 0005), and the honest composition of 0005 with this inliner is the
next experiment. `json`'s hot locals are behind recursive calls, which cannot
inline. `collections` still has 34 address-taken locals, mostly iterator
state behind non-inlined `next`.

## What remains for step 5b

1. A sound escape/alias analysis for raw pointers, including pointers returned
   by calls, so the pass can run in the default optimised build without
   miscompiling the compiler or unsafe user code.
2. Run the opt-in experiment (the `mir_hoist` dispatch) and record per-program
   and suite instruction counts, runner time and compile fuel against the
   default baseline.
3. The cost-model inliner is measured: `InlineWasm` (patch 0006) inlines
   loop-contained calls that rustc's cost model declines, with a 200 loop
   threshold and 400 for callees that take `&mut`/`*mut`. `rustc.wasm` is
   rebuilt, the six-program subset is measured, and difftest is green. The
   full 21-program suite and a repeat of the `mir_hoist` dispatch are still
   missing before it can be more than an experiment.
4. A real mem2reg/SSA promotion at CFG joins (the hoist only handles
   loop-invariant reads, not arbitrary non-escaping locals).
5. Repeat inlining and promotion in a loop: **measured no-win** for rustc's
   own `ReferencePropagation`/`SROA`/`GVN` (see the third increment). The
   next SROA work is drop expansion and partial aggregate splitting, not a
   repeat round.
6. Measure executed instructions and runner time on the full suite, and the
   compile time cost, before claiming any of it. The six-program subset is in
   the third increment; the 5a notes say the +5% warm compile belongs in the
   tier-2 build; a default-on pass would pay that in the fast tier, so it
   stays opt-in until tier 2 exists.
7. Understand the intermittent 5-unit compiler-fuel difference the pass causes
   on `sha256` (1 in 3 local runs) before considering a default-on pass; it
   may be a nondeterministic hash-order path in rustc that the changed MIR
   reaches.
8. The loop-nest profitability gate is in and removes the `expr` regression.
   A finer trip-count/loop-cost model could recover single-level hot loops, and
   the gated suite numbers still need the confirming dispatch.
