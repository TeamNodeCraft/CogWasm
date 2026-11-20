# Goal 1 step 5b: a MIR optimiser

Status: **in progress**. The first increment is landed; the cost-model inliner
and the benchmark measurement are not. No performance claim is made yet.

> The pass code, patch header and this note are LLM-generated under the Rust
> project's personal-use exemption. This is a local CogWasm experiment, not an
> upstream contribution; a human must review and own it before it goes
> anywhere else.

## What landed

`stage-c/patches/rust/0005-hoist-loop-invariant-field-reads.patch` adds
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

The suite numbers above are still **not measured**. `rustc.wasm` cannot be run
in this environment (only paths outside the checkout work inside its WASI
sandbox), so the benchmark run must happen on a normal machine or in CI. Until
then this is an implementation increment, not a speed result.

## Reproduce

```sh
# MIR-level check, native compiler (the config targets wasm by default)
experiments/cranelift-wasm/toolchain/stage-b/rust/x build --stage 1 \
    --host aarch64-apple-darwin --target aarch64-apple-darwin library rustc
RUSTC_BOOTSTRAP=1 build/host/stage1/bin/rustc --edition 2024 -O \
    -Zmir-hoist-loop-field-reads \
    -Zdump-mir=LoopFieldHoist -Zdump-mir-dir=/tmp/mirdump \
    --crate-type lib test.rs
ls /tmp/mirdump
```

```sh
# product pipeline
experiments/cranelift-wasm/stage-c/scripts/build-rustc-wasm.sh
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

## What remains for step 5b

1. A sound escape/alias analysis for raw pointers, including pointers returned
   by calls, so the pass can run in the default optimised build without
   miscompiling the compiler or unsafe user code.
2. Run the opt-in experiment (the `mir_hoist` dispatch) and record per-program
   and suite instruction counts, runner time and compile fuel against the
   default baseline.
3. The cost-model inliner, which is the other half of 5a's winner and what
   removes the calls keeping `Vec`'s address escaping. Inlining before this
   pass should widen what the SROA half can see.
4. A real mem2reg/SSA promotion at CFG joins (the hoist only handles
   loop-invariant reads, not arbitrary non-escaping locals).
5. Repeat inlining and promotion in a loop.
6. Measure executed instructions and runner time on the suite, and the compile
   time cost, before claiming any of it. The 5a notes say the +5% warm compile
   belongs in the tier-2 build; a default-on pass would pay that in the fast
   tier, so it stays opt-in until tier 2 exists.
7. Understand the intermittent 5-unit compiler-fuel difference the pass causes
   on `sha256` (1 in 3 local runs) before considering a default-on pass; it
   may be a nondeterministic hash-order path in rustc that the changed MIR
   reaches.
8. Add a profitability cost to the hoist. `expr` regressed by 0.01% because the
   in-loop copy can cost more than the load it removes; the win is only worth
   it when the read is hot enough (nested loops, or a loop body large enough
   that the load is the cheaper part).
