# Goal 1 step 5b: a MIR optimiser

Status: **in progress**. The cost-model inliner and the third increment's
mutable-borrow rule are landed. The fourth increment measured the inliner on
the full 21-program suite and dropped the loop-field hoist. The fifth increment
builds the missing interprocedural non-capture summary, brings the hoist back
as patch `0008` behind `-Zmir-capture-hoist`, and recovers the `nbody` win
(6,204,745,840 -> 5,550,101,348 executed instructions, -10.55%) while the full
diff test stays green against a sysroot built with both passes on. The inliner
and the hoist stay opt-in. B, C and D are met; A is not (`strings` 2.98x,
`collections` 4.50x, `sort` 1.99x, `json` 3.68x of LLVM `-O2`). The default,
fast and pure sysroot builds are unchanged.

> The pass code, patch header and this note are LLM-generated under the Rust
> project's personal-use exemption. This is a local CogWasm experiment, not an
> upstream contribution; a human must review and own it before it goes
> anywhere else.

## What landed

> Historical: the `0005` pass described in this section was dropped in the
> fourth increment (see below). The section is kept as the record of the first
> three increments.

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

> Historical commands for the first increment. The hoist flag and patch `0005`
> no longer exist; the fourth increment's commands are in its own section.

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
`-Zmir-inline-wasm-threshold` (default 200). It reuses rustc's inlining
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
it owns byte-for-byte; it also reverse-applies to the working tree. (This was
the check at the time; `0005` has since been dropped and the fourth increment
rechecks the series without it.) The
scratch check skips `0001` because the prior-art series already contains it,
which is what the build script's reverse check does on a fresh checkout. The
only difference from a full `toolchain/rust` copy is the build script's
`clif-to-wasm` path rewrite in `compiler/rustc_codegen_cranelift/Cargo.toml`
and the `setup-sources.sh` changes; neither is part of a patch. The
`.cogwasm-applied-patches` 0006 entry was updated to the new file hash.

### What this does not fix

`nbody`'s 1.01G stack accesses are unchanged: a `Vec` needs a drop and is
borrowed by `#[inline(never)] grow_one`, so SROA cannot see it and the
inliner must not override `inline(never)`. That was the loop-field hoist's job
(patch 0005); the fourth increment ran the composition and then dropped the
hoist because no cheap sound version exists. `json`'s hot locals are behind recursive calls, which cannot
inline. `collections` still has 34 address-taken locals, mostly iterator
state behind non-inlined `next`.

## Fourth increment: composition, the full suite, and dropping the hoist

Status: **the inliner is measured on the full 21-program suite and stays
opt-in; the loop-field hoist is dropped.** The composition run shows exactly
what the two patches do: `0006` moves the iterator-heavy programs, `0005` moves
`nbody` and a few small kernels, and neither substitutes for the other. The
inliner does not reach the 1.2x target. The hoist's only cheap sound form hoists
nothing, so the patch, its flag and its workflow switch are gone; the default
and pure sysroot builds are unchanged.

### Composition

Both flags were run on one rebuilt `rustc.wasm` that still contained `0005`
(commit `6d1a14d` plus the third increment), 21 programs, 3 runs x 4 seeds,
counts from `bench/gap`. Only the programs whose instruction count moved at
either stage are shown; `smoke` moved +0.01% and the other nine are
byte-identical.

| program | off | inline only (0006) | both (0006 + 0005) | both vs off | ratio both |
|---|---:|---:|---:|---:|---:|
| `nbody` | 6,204,745,840 | 6,204,745,840 | 5,550,101,348 | **-10.55%** | 3.082 |
| `b_alloc` | 1,634,448 | 1,634,448 | 1,606,154 | -1.73% | 4.617 |
| `scene` | 415,667 | 409,913 | 406,055 | -2.31% | 3.567 |
| `matmul` | 380,350,388 | 380,352,296 | 380,155,700 | -0.05% | 1.096 |
| `strings` | 107,244,701 | 66,008,021 | 66,008,021 | -38.45% | 2.978 |
| `collections` | 5,061,326 | 4,772,355 | 4,772,355 | -5.71% | 4.498 |
| `sort` | 321,207,695 | 299,459,431 | 299,459,431 | -6.77% | 1.988 |
| `json` | 1,871,062 | 1,777,296 | 1,777,296 | -5.01% | 3.677 |
| `float_sum` | 243,236 | 222,388 | 222,388 | -8.57% | 3.129 |
| `expr` | 2,700,640 | 2,647,871 | 2,647,871 | -1.95% | 3.958 |
| `math_e` | 774,674,908 | 773,874,908 | 773,874,908 | -0.10% | 2.207 |

`nbody`'s stack load/store count falls 1,012,060,384 -> 873,395,276 (-13.7%),
the same figure the 5a prototype reported. The composition is additive: `0006`
adds nothing to `nbody`, and `0005` adds nothing to `strings`, `collections`,
`sort` or `json`. The hoist is the only part that touches `nbody`, and it is
also the part with the soundness problem.

### D: the hoist cannot be made sound cheaply, so it is dropped

The missing fact in `0005` is a pointer the pass cannot see. It already skips a
function that creates a raw pointer to a local, and relies on borrow checking
for references, but a call can return a raw pointer into a local. The only
local dataflow rule that closes that hole without interprocedural analysis is
to treat a local as escaped for the whole function as soon as any `&mut`/`*mut`
to it exists anywhere: an opaque callee that receives the borrow may keep the
address as a raw pointer and hand it back later. Shared borrows of a `Freeze`
base are safe (a write through `&` would need `UnsafeCell` or be UB), so only
mutable borrows need to disqualify a local.

That rule was implemented as a scratch variant of the pass and built into a
native stage-1 compiler. Its final optimized MIR was compared with the hoist
off, per program, after normalizing the source paths:

| program | final MIR with the sound guard |
| --- | --- |
| `strings`, `sort`, `expr`, `nbody` | identical to flag off |
| `collections`, `json`, `scene` | identical except allocation-ID numbering |

So the sound version hoists **zero** reads on every hot program in the suite.
The reason is visible in `nbody`'s final MIR: each of the four `Vec` locals is
passed as `&mut` to `RawVec::grow_one` before the hot loop --

```text
bb40:
    _389 = &mut (_2.0: alloc::raw_vec::RawVec<[f64; 3]>);
    _388 = alloc::raw_vec::RawVec::<[f64; 3]>::grow_one(move _389) -> ...
```

-- and `grow_one` is `#[inline(never)]`, so neither inlining nor a local rule
can remove the escape. Hoisting those fields is exactly what produced the
10.55% `nbody` win, and proving it safe needs an interprocedural escape/alias
analysis of the callee, not a cheap guard. The pass also previously
miscompiled rustc's own crates when it ran by default (a dataflow bounds panic
while building the pure sysroot in CI), which is the bar item D sets.

Decision: **drop `0005`**. The patch file, the `-Zmir-hoist-loop-field-reads`
flag, the `BENCH_MIR_HOIST`/`DIFFTEST_MIR_HOIST` plumbing and the `mir_hoist`
workflow dispatch input are removed. No pass in the tree can now miscompile
unsafe code in pursuit of that win. The measured composition above is kept as
evidence of what a sound escape analysis would have to recover.

### Full-suite measurement of the inliner

The instruction counts were measured with the pre-drop compiler
(`rustc.wasm` `7dac7e2b3e10ff7a…`) in both configurations; the compile-time
pairs were measured with the final, hoist-free compiler
(`3a79116e1c7568be…`), as was the re-count of the hot programs below. The compiler was then rebuilt with only `0006` (the
hoist removed), and the flag-off and inline modules for `strings` and `nbody`
were byte-identical to the pre-drop compiler, as they must be: `0005` was off
in both configurations. Re-counting `collections`, `sort` and `json` with the
final compiler gives exactly 4,772,355, 299,459,431 and 1,777,296 instructions,
the same numbers as the table. The tables below therefore describe the final
compiler as well as the one that produced them.

### A: not met

The four required benchmarks are not within 1.2x of LLVM `-O2`; the inliner
gets `strings` from 4.84x to 2.98x and `collections` from 4.77x to 4.50x, and
moves `sort` and `json` much less:

| benchmark | instructions off | instructions on | ratio on | target | over target |
|---|---:|---:|---:|---:|---:|
| `strings` | 107,244,701 | 66,008,021 | 2.978 | 1.2 | 2.48x |
| `collections` | 5,061,326 | 4,772,355 | 4.498 | 1.2 | 3.75x |
| `sort` | 321,207,695 | 299,459,431 | 1.988 | 1.2 | 1.66x |
| `json` | 1,871,062 | 1,777,296 | 3.677 | 1.2 | 3.06x |
| `nbody` | 6,204,745,840 | 6,204,745,840 | 3.446 | 1.2 | 2.87x |
| `nbody` + hoist | 6,204,745,840 | 5,550,101,348 | 3.082 | 1.2 | 2.57x |

Reaching the target needs removing 60% of `strings`' remaining instructions,
73% of `collections`', 40% of `sort`'s and 67% of `json`'s. The inliner's win
is real but it is the easy half: the remaining cost is calls it cannot remove
(`build_lines`, `String::push`, `fmt`, recursive `walk`, `#[inline(never)]`
`grow_one`), allocations, and the memory traffic of locals whose address is
observed. Threshold tuning does not close a 2.5-3.8x gap (next section), and
the hoist never touched these four programs even before it was dropped. A is
**not met**.

### Full suite: instructions and code size

The tables below are the full 21-program suite. "on" is `-Zmir-inline-wasm`
with the 200/400 thresholds. The composition table in the first section is a
separate, earlier run with both patches; it is not the shipped configuration.

### Instructions and LLVM ratio

| program | instructions off | instructions on | delta | ours/O2 off | ours/O2 on |
|---|---:|---:|---:|---:|---:|
| `a_add` | 804 | 804 | +0.00% | 3.045 | 3.045 |
| `b_alloc` | 1,634,448 | 1,634,448 | +0.00% | 4.699 | 4.699 |
| `c_plugin` | 4,124 | 4,124 | +0.00% | 12.274 | 12.274 |
| `smoke` | 405,976 | 406,016 | +0.01% | 5.045 | 5.045 |
| `int_loops` | 213,915 | 213,915 | +0.00% | 1.735 | 1.735 |
| `float_sum` | 243,236 | 222,388 | -8.57% | 3.422 | 3.129 |
| `expr` | 2,700,640 | 2,647,871 | -1.95% | 4.037 | 3.958 |
| `json` | 1,871,062 | 1,777,296 | -5.01% | 3.871 | 3.677 |
| `collections` | 5,061,326 | 4,772,355 | -5.71% | 4.771 | 4.498 |
| `scene` | 415,667 | 409,913 | -1.38% | 3.652 | 3.601 |
| `nbody` | 6,204,745,840 | 6,204,745,840 | +0.00% | 3.446 | 3.446 |
| `mandelbrot` | 611,294,724 | 611,294,724 | +0.00% | 1.439 | 1.439 |
| `sha256` | 70,417,340 | 70,417,340 | +0.00% | 1.663 | 1.663 |
| `sort` | 321,207,695 | 299,459,431 | -6.77% | 2.132 | 1.988 |
| `matmul` | 380,350,388 | 380,352,296 | +0.00% | 1.096 | 1.096 |
| `strings` | 107,244,701 | 66,008,021 | -38.45% | 4.838 | 2.978 |
| `math_e` | 774,674,908 | 773,874,908 | -0.10% | 2.209 | 2.207 |
| `hostcall_f` | 10,881,056 | 10,881,056 | +0.00% | 1.546 | 1.546 |
| `libm_driver` | 302,693,800 | 302,693,800 | +0.00% | 2.461 | 2.461 |
| `miniz_driver` | 216,101,124 | 216,101,124 | +0.00% | 2.295 | 2.295 |
| `generated_stress` | 671,468 | 671,468 | +0.00% | 1.913 | 1.913 |

### Module size and warm compile

| program | module off | module on | delta | warm off ms | warm on ms | delta |
|---|---:|---:|---:|---:|---:|---:|
| `a_add` | 18,163 | 18,163 | +0.00% | 58.70 | 60.28 | +2.69% |
| `b_alloc` | 63,757 | 63,757 | +0.00% | 92.58 | 91.45 | -1.23% |
| `c_plugin` | 19,830 | 19,830 | +0.00% | 62.97 | 65.72 | +4.36% |
| `smoke` | 46,496 | 46,780 | +0.61% | 91.19 | 93.08 | +2.08% |
| `int_loops` | 19,172 | 19,172 | +0.00% | 67.77 | 66.35 | -2.09% |
| `float_sum` | 33,122 | 33,770 | +1.96% | 83.17 | 84.07 | +1.09% |
| `expr` | 60,583 | 60,783 | +0.33% | 130.20 | 133.21 | +2.32% |
| `json` | 94,384 | 96,151 | +1.87% | 162.70 | 171.97 | +5.70% |
| `collections` | 223,702 | 218,702 | -2.24% | 250.36 | 254.71 | +1.74% |
| `scene` | 67,620 | 67,518 | -0.15% | 133.87 | 137.48 | +2.70% |
| `nbody` | 41,858 | 41,858 | +0.00% | 96.54 | 95.63 | -0.94% |
| `mandelbrot` | 19,766 | 19,766 | +0.00% | 65.85 | 63.80 | -3.11% |
| `sha256` | 33,347 | 33,347 | +0.00% | 76.19 | 78.56 | +3.11% |
| `sort` | 50,535 | 50,560 | +0.05% | 97.04 | 102.06 | +5.17% |
| `matmul` | 31,881 | 31,649 | -0.73% | 76.16 | 75.66 | -0.65% |
| `strings` | 52,002 | 53,077 | +2.07% | 96.43 | 106.04 | +9.97% |
| `math_e` | 41,090 | 41,039 | -0.12% | 65.50 | 69.06 | +5.44% |
| `hostcall_f` | 19,046 | 19,046 | +0.00% | 60.59 | 62.32 | +2.86% |
| `libm_driver` | 47,278 | 47,278 | +0.00% | 65.93 | 68.02 | +3.17% |
| `miniz_driver` | 236,262 | 236,262 | +0.00% | 82.63 | 81.54 | -1.32% |
| `generated_stress` | 153,659 | 153,659 | +0.00% | 388.62 | 390.31 | +0.43% |

Suite summary: total executed instructions 9,012,834,242 -> 8,948,589,138
(-0.71%), dominated by `nbody`, which the inliner does not change; the
per-program geomean ratio is 2.894 -> 2.787 (ten modules change, eleven are
byte-identical). Module-size median change 0.00%, mean
+0.17%; the largest growth is `strings` +2.07%, `float_sum` +1.96% and `json`
+1.87%, the largest shrink `collections` -2.24%.

### C: met, with a caveat for the hot programs

Three interleaved full-suite pairs, each with five warm-compile samples per
program and `--cold-runs 1`; the only difference between the two sides is the
`BENCH_INLINE_WASM` environment variable.

| pair | suite median warm delta | mean | worst program |
|---|---:|---:|---:|
| 1 | +1.16% | +1.65% | +10.7% (`strings`) |
| 2 | +1.29% | +2.12% | +9.9% (`strings`) |
| 3 | +1.59% | +2.38% | +10.0% (`strings`) |
| **median of pairs** | **+1.29%** | | |

The suite-level median is stable (83.07, 83.17 and 83.83 ms across the three
flag-off runs, a 0.9% spread). It is also insensitive to per-program noise: the
median across-run range of a single program is 3.07% (max 7.85%), and the
eleven byte-identical modules show a compile delta median of +0.43% with a
range of -3.11% to +4.36%. Against that noise floor, the median delta is about
+1.3% and the geomean is +2.03%; total suite compile time is +2.01%.

The caveat is that the median hides the programs the inliner actually changes:
`strings` +9.97%, `json` +5.70%, `sort` +5.17%, `math_e` +5.44% (whose module
changed by -0.12%, so that one is noise), and `collections` +1.74%. Four of
twenty-one programs exceed 5% individually. Criterion C is met as written
(median over the suite), not as "every program is within 5%". The cost is only
paid by the opt-in optimising compile; the default and fast tiers never run the
pass.

### B: difftest is green with the pass on and off

Full corpus, 24 generated programs, four run seeds:

- flags off: `PASS: 49 handwritten + 24 generated programs`, 292 calls,
  276 value matches, 16 trap matches, 0 mismatches, 63 s;
- `-Zmir-inline-wasm`: same 73 programs, 0 mismatches, 56 s.

### Threshold recheck on the full suite

The 200/400 constants were re-tested at 300 on the full suite (same compiler,
`-Zmir-inline-wasm-threshold=300`, the mutable-borrow floor stays 400). The
higher threshold buys a slightly better suite instruction geomean (2.7875 ->
2.7553) but loses on the required benchmarks and on size:

| program | instructions 200 | instructions 300 | delta | module 200 | module 300 |
|---|---:|---:|---:|---:|---:|
| `strings` | 66,008,021 | 67,905,889 | +2.88% | 53,077 | 53,508 |
| `collections` | 4,772,355 | 4,847,727 | +1.58% | 218,702 | 225,537 |
| `sort` | 299,459,431 | 299,457,763 | -0.00% | 50,560 | 52,327 |
| `json` | 1,777,296 | 1,776,505 | -0.04% | 96,151 | 97,241 |
| `int_loops` | 213,915 | 198,719 | -7.10% | 19,172 | 20,213 |
| `nbody` | 6,204,745,840 | 6,204,743,664 | -0.00% | 41,858 | 46,304 |
| `mandelbrot` | 611,294,724 | 611,283,416 | -0.00% | 19,766 | 22,085 |
| `matmul` | 380,352,296 | 380,350,620 | -0.00% | 31,649 | 34,167 |
| `scene` | 409,913 | 407,949 | -0.48% | 67,518 | 71,545 |

`nbody`, `mandelbrot` and `matmul` grow their modules 8-12% for no instruction
change; `strings` and `collections` -- the two programs the pass exists for --
get worse. Module size over the suite is +1.21% median / +3.00% mean at 300,
against +0.00% / +0.17% at 200. One full compile-time run at 300 measured a
suite median warm-compile delta of +11.3% against the flag-off medians (+7.9%
against the 200 runs), so the higher threshold also costs much more. 300 is
rejected; the default stays 200. The mutable-borrow floor
stays 400 because it is what admits `next_code_point` (cost 370) in `strings`;
lowering it to 300 would undo the third increment, and the full-suite data
above does not justify raising it.

### SROA beyond rustc's: not built, and the measurements say why

The remaining plan item was partial aggregate splitting and drop expansion for
`Vec`/`String`-like locals. Nothing was landed. The third increment already
showed why a repeat `ReferencePropagation -> SROA -> GVN` round is a no-op:
`sroa`'s `excluded_locals` marks any local whose field address is observed, and
`Vec`/`String` also need a drop, which `sroa` does not expand. The full-suite
measurement adds a second constraint: the locals that still matter are kept in
memory by calls the inliner deliberately does not remove (`#[inline(never)]`
`grow_one`, recursive `walk`, `fmt`). Splitting such a local would have to
rewrite the call to operate on the split fields, which is an interprocedural
change, not a local SROA extension. That is the same escape/alias problem as
`0005`, just on a larger scale, and it is not a cheap increment. The honest
next step for A is a different representation change (for example, promoting
the `Vec` pointer/length around a known non-retaining call), not a threshold.

### Patch reproduction

`0006-wasm-cost-model-inliner.patch` was regenerated without `0005`'s option
context. A fresh export of `toolchain/rust` at `2f59a94fd` with `0002`, `0003`,
`0004`, `0006` and `0007` applied reproduces all 18 files the series touches
byte-for-byte, including the four owned by `0006` (`inline.rs`,
`inline_wasm.rs`, `lib.rs`, `options.rs`); `0006` also reverse-applies to the
working tree. The `.cogwasm-applied-patches` entry was updated to the new file
hash.

### Reproduce

```sh
compiler/scripts/build-rustc-wasm.sh
BENCH_INLINE_WASM=1 bench/gap/run.sh --no-mir
BENCH_INLINE_WASM=1 bench/run.sh --full --runs 5 --cold-runs 1 \
    --out toolchain/work/step5b/bench/on.json
DIFFTEST_INLINE_WASM=1 difftest/run.sh
```

## Fifth increment: interprocedural non-capture summaries, and the hoist returns

Status: **implemented and measured, opt-in only**. Patch
`0008-capture-aware-loop-field-hoist.patch` adds `CaptureHoist` behind
`-Zmir-capture-hoist`. It computes a per-argument "not captured" summary from
callee MIR and uses it to decide whether a borrow of a local before a loop can
be ignored for the loop-field hoist. The default compiler, the default sysroot
and the default benchmark compiles do not pass the flag.

### What "not captured" means

For argument `i` of a function, the summary says **not captured** when the
callee does not keep the argument's address, or an address derived from it by a
pointer operation, anywhere that outlives the call. Concretely, the callee must
not:

- store the address in memory (through a pointer, into a static, into a union,
  or into a local whose drop can run code that observes it);
- return the address;
- pass it to a callee that captures it (an unknown callee is capturing);
- convert it to an integer;
- put it in a closure or coroutine that can run later;
- use it as an inline-asm operand.

"Derived" means a field projection behind the pointer, a reference or
raw-pointer cast, a reborrow, pointer arithmetic, or a transmute that preserves
the pointer. A value **loaded** through the pointer is data, not a derived
address. This is the important boundary for `RawVec::grow_one`: it reads
`self.ptr` and hands that heap pointer to the allocator, but it never keeps the
`&mut RawVec` itself. The caller's `Vec` fields cannot be reached through the
heap pointer unless something stored an address of the `Vec` local into it
first, and that first store is itself a capture and is rejected where it is
created (see "values loaded through the argument" below).

### How the summary is computed

`compiler/rustc_mir_transform/src/capture.rs` runs a may-analysis over pointer
taint. Each pointer-typed argument seeds one taint bit in its argument local.
The walker transfers taint through `Use`/`Move`, `Ref`/`RawPtr`/`Reborrow`,
pointer casts, pointer `Offset`, aggregate construction and `RawPtr` aggregates.
A transfer that stores taint into a local records containment; a transfer that
stores it through a pointer, into the return place, into a union, or into a
type that needs a drop marks the source argument captured. Each call argument
is checked against the callee's summary, recursively. The analysis uses
`tcx.optimized_mir` for both local and external callees (external bodies are
already optimised and cached); a thread-local stack of definitions being
summarised or optimised turns a recursive query into "capturing" instead of a
query cycle.

Only reference- and raw-pointer-shaped arguments get a meaningful summary.
Aggregate arguments are conservatively capturing: a per-field summary would be
needed to admit them, and the hoist only asks about borrows.

`const_eval_select((a, b, ..), const_fn, runtime_fn)` is classified as a call
to `runtime_fn` with the tuple's fields as positional arguments. Every other
intrinsic, shim, virtual call, function pointer and foreign item is unknown and
therefore capturing. `RawVec::grow_one`, its `grow_amortized`/`finish_grow`
helpers, the `Global` allocator methods and the `const_eval_select` runtime
branches all resolve to non-capturing for the receiver/borrow argument on the
suite's wasm builds.

### The hoist rule

`compiler/rustc_mir_transform/src/capture_hoist.rs` restores the fourth
increment's pass shape: natural loops, a unique `Goto` preheader, only loops in
a nest, a `Freeze` base, no in-loop write to the base, no in-loop borrow of it,
no inline asm, and a base initialized in a block that dominates the preheader.
The old whole-function raw-pointer bail-out is replaced by the summary: a base
is admitted only when **every** `Ref`/`RawPtr` rooted at it is either a call
argument whose summary marks that argument non-capturing or is otherwise
consumed locally without an escape sink, and no tainted pointer is used inside
the loop. A use inside the loop rejects the base, so a borrow that is live in
the loop cannot be hoisted past.

The preheader read is a new temporary; the original assignment remains in the
loop and copies from the temporary, so a zero-trip loop leaves the destination
untouched.

### Soundness argument

The walker is a may-analysis: every store of a tainted pointer to a sink marks
the source argument captured, and taint is never killed. Over-approximation is
sound; the interesting cases are the constructs where a pointer can leave.

**Assignments.** An assignment of a tainted pointer to a local records
containment in that local; it is not an escape by itself because the local dies
with the function. The escape cases are: a destination with a `Deref` in its
projection (a store through a pointer, including `Box`, `&mut` and raw
pointers), the return place, a union-typed place (another field can later be
read as the pointer), and a local whose type needs a drop (the drop can run
arbitrary code while the pointer is in it). A move is treated like a copy for
taint, which over-approximates.

**Returns.** A tainted pointer in `_0` at a `Return`, or an assignment to
`_0`, marks the argument captured. This covers a returned reference, a raw
pointer, a struct or tuple carrying either, and a `Box`/`Vec` whose element
points at the argument.

**Calls.** The function operand is checked first: calling through a tainted
function pointer invokes unknown code that has the address. Each tainted
argument is then checked against the callee's summary at that position. A
missing summary (foreign, no MIR, shim, virtual, unclassified intrinsic) and a
recursive occurrence are capturing. There is no result-taint rule for a
non-capturing callee: by definition it does not return the argument, so its
result cannot carry the argument's address. A capturing callee already marks
the corresponding argument.

**Pointer-to-integer casts.** `PointerExposeProvenance` from a tainted pointer
marks the argument captured. A pointer-width integer can be turned back into a
pointer and stored, and the analysis does not track provenance through integer
arithmetic. `PointerWithExposedProvenance` starts from an integer with no
taint, so it cannot reintroduce a tracked argument by itself.

**Inline asm.** Any tainted input or output operand marks the argument
captured; the assembly can store it anywhere. A loop containing inline asm is
rejected by the hoist regardless.

**Unions.** A store of a tainted pointer into a union marks the argument
captured because the pointer can be read back through another field. An
aggregate whose `AggregateKind::Adt` names a union field is treated the same
way.

**Transmute.** `CastKind::Transmute`, `BoxDerefTransmute`, `Subtype`,
`PtrToPtr`, `FnPtrToPtr` and pointer coercions keep the taint when the target is
a pointer type and mark the argument captured when the target is not. A
`transmute` from a pointer to a struct or integer is therefore a capture.

**Statics.** MIR reaches a mutable static through a pointer, so the store is a
`Deref` store and is covered by the assignment rule. A `static` that is only
read cannot receive the address. `Rvalue::ThreadLocalRef` is not a pointer to
the argument.

**Box and raw-pointer stores.** They are `Deref` stores to the analysis and are
capturing. A `Box` constructed from a tainted aggregate is an aggregate, not a
raw pointer store; it is captured when the boxed value is returned, stored, or
put in a type that needs a drop.

**Closures and coroutines.** Constructing a closure, coroutine or coroutine
closure with a tainted capture marks the argument captured immediately: the
closure can outlive the call and run later. A closure that is only called
inside the callee still gets this treatment, which is conservative.

**Drops.** A `Drop` terminator on a tainted local whose type needs a drop (or is
a union) marks the argument captured, because drop glue is arbitrary code and
can read the local. Storing a tainted pointer into a type that needs a drop is
also a capture even if the drop is not visible in this body, for the same
reason.

**Panics and unwinding.** A panic or unwind is not a capture by itself: the
function does not return, so the caller does not resume at the call. If a
cleanup path stores the address (through a pointer, into a static, into a
drop-typed local, or by calling a capturing callee), the cleanup blocks are
ordinary reachable MIR and the same sinks fire there. The blanket "any unwind
edge captures" rule was deliberately not used: pre-optimisation MIR still has
cleanup edges under `panic=abort`, and that rule rejected every body and made
the summary useless. The recursive walker visits unwind-reachable blocks from
the entry, including cleanup chains.

**Recursion and cycles.** The thread-local stack marks a definition that is
already being summarised or optimised. A call back into it is treated as
capturing for that occurrence. The resulting summary is an over-approximation
of a fixpoint (arguments not involved in the cycle can still come out
non-capturing), which is the sound direction. Query cycles are prevented
because the check happens before `optimized_mir` is requested.

**Values loaded through the argument.** A load through a `Deref` does not carry
the pointer's address; it carries data. The analysis therefore does not taint
`(*self).field`. If a callee stores such a loaded pointer into a location that
outlives the call, the stored value can only alias the caller local if some
earlier code put the local's address into that memory. Creating that
self-reference requires a store of an address derived from the local, and every
such store is a capture in the function that performs it (a raw-pointer
creation plus a `Deref` store, a pointer-to-integer cast, or a call to a
capturing callee). A callee that merely relays a pre-existing pointer field
does not create a new alias; this is the one place where the summary relies on
the creation site, not on the load. It is stated here explicitly so the
boundary is reviewable.

**What is not modelled.** There is no points-to analysis, no integer provenance
tracking beyond "pointer-to-integer is capturing", no per-field summary for
aggregate arguments, and no attempt to prove a recursive function's fixpoint.
All of those are conservative in the summary direction.

### Two write bugs the sysroot run found

The first version of the restored hoist passed the suite's program-local
difftest but miscompiled `vec_ops` when the pure sysroot had been built with
the pass enabled. Dumping `CaptureHoist` before/after for `core` and `alloc`
found exactly one changed function, `core::slice::sort::stable::drift::sort`:

- the loop reassigns the whole `DriftsortRun` local (`_29 = move _79`) between
  reads of `_29.0`, but the old `loop_effects` recorded a local as written only
  when the assignment had a non-empty projection, so the direct assignment did
  not invalidate the hoist;
- a call inside a loop writes its result into a destination place, and the old
  rule did not record the destination local as written either.

Both are fixed: every assignment destination local is in `written`, and a call
destination and `Yield` resume place are recorded as writes. Two new corpus
programs (`capture_reassign`, `capture_call_dest`) pin the two cases. After the
fix, the same dump comparison shows **zero** changed functions in `core` and
`alloc` with both flags on, and the full difftest against the passes-on sysroot
is green.

### Adversarial difftest corpus

Seven new handwritten programs would miscompile under a hoist without the
summary or without the write rules. `loop_field_reads` is the positive case:
`blend(&mut p)` only reads and writes through the borrow, its summary is
non-capturing, and the hoist applies to the nested loop.

| program | what would go wrong under a naive hoist |
| --- | --- |
| `capture_static` | callee stores `&mut local` in a `static mut`; the loop writes the field through it |
| `capture_field` | callee stores the address in a field of another borrowed struct |
| `capture_return` | callee returns a pointer to a field of its argument |
| `capture_usize` | address is converted to `usize` and back |
| `capture_closure` | a closure captures a raw pointer derived from the local |
| `capture_reassign` | the loop reassigns the whole aggregate between field reads |
| `capture_call_dest` | a call in the loop writes its result into the whole aggregate |

All seven pass with LLVM, with our pipeline flag-off and with
`-Zmir-inline-wasm -Zmir-capture-hoist`; the complete corpus (56 handwritten +
24 generated) is green in all four configurations below.

### Measurements

One full-suite run per configuration: 21 programs, executed wasm instructions
from `bench/gap` over the suite seeds, module bytes and warm compile (median of
5) from `bench/run.sh --full --runs 5 --cold-runs 1`. "off" is the default
pipeline, "inline" is `-Zmir-inline-wasm`, "both" adds `-Zmir-capture-hoist`.

| program | instructions off | inline | both | both/off | both/LLVM-O2 |
|---|---:|---:|---:|---:|---:|
| `a_add` | 804 | 804 | 804 | 1.000 | 3.045 |
| `b_alloc` | 1,634,448 | 1,634,448 | 1,606,154 | 0.983 | 4.617 |
| `c_plugin` | 4,124 | 4,124 | 4,124 | 1.000 | 12.274 |
| `smoke` | 405,976 | 406,016 | 406,016 | 1.000 | 5.045 |
| `int_loops` | 213,915 | 213,915 | 213,915 | 1.000 | 1.735 |
| `float_sum` | 243,236 | 222,388 | 222,388 | 0.914 | 3.129 |
| `expr` | 2,700,640 | 2,647,871 | 2,647,871 | 0.981 | 3.958 |
| `json` | 1,871,062 | 1,777,296 | 1,777,296 | 0.950 | 3.677 |
| `collections` | 5,061,326 | 4,772,355 | 4,772,355 | 0.943 | 4.498 |
| `scene` | 415,667 | 409,913 | 409,421 | 0.985 | 3.597 |
| `nbody` | 6,204,745,840 | 6,204,745,840 | 5,550,101,348 | **0.895** | 3.082 |
| `mandelbrot` | 611,294,724 | 611,294,724 | 611,294,724 | 1.000 | 1.439 |
| `sha256` | 70,417,340 | 70,417,340 | 70,417,340 | 1.000 | 1.663 |
| `sort` | 321,207,695 | 299,459,431 | 299,459,431 | 0.932 | 1.988 |
| `matmul` | 380,350,388 | 380,352,296 | 380,155,700 | 1.000 | 1.096 |
| `strings` | 107,244,701 | 66,008,021 | 66,008,021 | 0.615 | 2.978 |
| `math_e` | 774,674,908 | 773,874,908 | 773,874,908 | 0.999 | 2.207 |
| `hostcall_f` | 10,881,056 | 10,881,056 | 10,881,056 | 1.000 | 1.546 |
| `libm_driver` | 302,693,800 | 302,693,800 | 302,693,800 | 1.000 | 2.461 |
| `miniz_driver` | 216,101,124 | 216,101,124 | 216,101,124 | 1.000 | 2.295 |
| `generated_stress` | 671,468 | 671,468 | 671,468 | 1.000 | 1.913 |

Suite totals: executed instructions 9,012,834,242 -> 8,948,589,138 with the
inliner (-0.71%) and 8,293,719,264 with both (-7.98%); the per-program
instruction geomean ratio to LLVM `-O2` is 2.894 -> 2.787 (inline) -> 2.770
(both). `nbody` is the whole of the extra total: 6,204,745,840 ->
5,550,101,348 (-10.55%), with stack loads/stores 1,012,060,384 -> 873,395,276
(-13.7%). That is exactly the number the dropped `0005` composition reported.
`b_alloc` moves -1.73%, `matmul` -0.05%, `scene` -1.50%; the other programs are
unchanged by the hoist (`collections`, `strings`, `sort`, `json` are identical
to the inliner-only modules).

Module size: median change both/off 0.00%, geomean +0.12%; `nbody` -0.98%,
`strings` +2.07% and `json` +1.87% are the inliner's, not the hoist's. The hoist
alone changes `nbody` 41,858 -> 41,450 bytes and `b_alloc` 63,757 -> 63,745.

Warm compile: median both/off **-1.54%**, geomean -1.51%; inline/off median
-0.90%, geomean +0.18%; hoist-only relative to inline median -0.59%, geomean
-1.68%. The single-pair numbers have the same per-program noise the fourth
increment measured (median across-run range 3.07%); what they establish is that
the hoist adds no measurable compile cost of its own and that C is inside the
5% bound.

### Criteria with this increment

- **A: still not met.** The required benchmarks are `strings` 2.978x,
  `collections` 4.498x, `sort` 1.988x and `json` 3.677x of LLVM `-O2`;
  `nbody` is 3.082x with the hoist. Recovering the 10.55% `nbody` win does not
  change the four iterator-heavy programs. A needs a representation change,
  not another threshold or hoist.
- **B: met.** Full corpus, 56 handwritten + 24 generated, four run seeds:
  default sysroot flags off and on are both green (320 calls, 304 value + 16
  trap matches, 0 mismatches), and the pure sysroot built with
  `-Zmir-inline-wasm -Zmir-capture-hoist` is green both with the program flags
  off and with them on.
- **C: met.** Suite-median warm compile is -1.54% (both/off) and -0.59%
  (both/inline), inside the 5% bound.
- **D: sound for the optimising build, still flag-gated.** The sysroot built
  with both passes on changes zero `core`/`alloc` functions under the hoist
  after the write-rule fix (the earlier build changed `drift::sort` and
  miscompiled `vec_ops`; see above), and the seven adversarial programs pass.
  The pass is not made default-on because the task keeps every new pass behind
  `-Z` until a tier decides to ship it; the evidence is that an optimising tier
  could turn it on without the old unsound guard.

### Reproduce

```sh
compiler/scripts/build-rustc-wasm.sh
BENCH_INLINE_WASM=1 BENCH_CAPTURE_HOIST=1 bench/gap/run.sh --no-mir
BENCH_INLINE_WASM=1 BENCH_CAPTURE_HOIST=1 bench/run.sh --full --runs 5 --cold-runs 1
DIFFTEST_INLINE_WASM=1 DIFFTEST_CAPTURE_HOIST=1 difftest/run.sh --seeds 24
```

For the sysroot check, back up `toolchain/sysroot`, then

```sh
PURE_SYSROOT_RUSTFLAGS="-Zmir-inline-wasm -Zmir-capture-hoist" \
    compiler/scripts/build-pure-sysroot.sh
DIFFTEST_INLINE_WASM=1 DIFFTEST_CAPTURE_HOIST=1 difftest/run.sh --seeds 24
```

and restore the default sysroot afterwards. The script appends
`PURE_SYSROOT_RUSTFLAGS` to `-Cpanic=abort`; its default build is unchanged.

## What remains for step 5b

Step 5b is still **in progress**. Of its four acceptance criteria, B (difftest),
C (suite-median warm compile) and D (no unsound hoist) are met; A (1.2x of
LLVM `-O2` on the iterator-heavy benchmarks) is not, and the SROA/mem2reg half
that was supposed to deliver it is not built.

1. **A: not met.** `strings` is 2.98x, `collections` 4.50x, `sort` 1.99x and
   `json` 3.68x of LLVM `-O2` in executed instructions after the inliner
   (`nbody` 3.45x, or 3.08x with the capture-aware hoist). The gap is 40-73% of
   the remaining instructions, not a threshold. The measured options are
   exhausted: 300 inlines more but makes `strings`/`collections` worse and costs
   +11% median warm compile; the hoist now recovers `nbody` but is not the four
   iterator-heavy programs. The next work is a real representation change, not
   a knob.
2. **SROA beyond rustc's: not built.** `sroa`'s `excluded_locals` still excludes
   any local whose field address is observed, and `Vec`/`String` need drop
   expansion. Partial aggregate splitting would have to rewrite the calls that
   take `&mut` of the aggregate (`RawVec::grow_one`, `String::push`, recursive
   `walk`). The capture summary now answers the "does the callee keep the
   address" half for those calls, but `sroa` still needs a drop story and a
   rewrite of the call to operate on split fields; no cheap version was found.
3. **The hoist is back, opt-in.** Patch `0008` and `-Zmir-capture-hoist`; the
   `nbody` win is recovered and the sysroot/adversarial evidence is in this
   note. It stays behind the flag because the task keeps new passes out of the
   default pipeline; turning it on in an optimising tier is a policy decision
   with this soundness evidence, not a missing analysis.
4. **The inliner stays opt-in.** The default, fast and pure-sysroot builds are
   unchanged. The suite-median compile cost in this increment's single pair was
   -0.90% (inline/off) and -0.59% (both/inline); the fourth increment's three
   interleaved pairs measured +1.29% for the inliner, with `strings` +10.0%,
   `json` +5.7% and `sort` +5.2%. Before it becomes default in an optimising
   tier, either fix the `strings` body or decide that a per-program >5% cost is
   acceptable there. Tier 2 itself is still not built.
5. **Fuel was not re-measured this increment.** The hoist changes MIR shape, so
   a later experiment that reports compiler fuel should re-run the two-sample
   determinism check rather than inherit the fourth increment's numbers.
6. **The full suite remains the measurement unit.** The instruction table above
   is 21 programs x 4 seeds with exact basic-block instrumentation; module size
   and warm compile come from `bench/run.sh --full --runs 5 --cold-runs 1`.
   The per-program compile noise is large (median across-run range 3.07%, max
   7.85%), so the -1.5% / -0.6% compile deltas are evidence of "well inside
   5%", not of a speed-up.
