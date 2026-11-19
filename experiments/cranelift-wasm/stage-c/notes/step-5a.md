# Goal 1 step 5a: where optimisation lives

Status: **done**. Decision: build step 5b as **our own MIR passes** (a
cost-model inliner followed by SROA), not as stack-slot promotion on Cranelift
IR. The measurements below are from the product pipeline at batch-1
(`4e9873…` rustc.wasm, `e3b3358` source) on the Apple M4 Pro development
machine.

## What was compared

Both prototypes targeted the same nbody measurement: 128 bodies, 300 steps,
seeds `0, 1, 42, 0x9e3779b97f4a7c15`. The baseline is
`bench/run.sh --programs nbody --runs 5 --fuel`:

| | baseline |
|---|---:|
| execution, own wasm (ms) | 67.73 |
| LLVM `-O2` (ms) | 13.29 |
| ours/O2 | 5.098 |
| warm compile (ms) | 105.34 |
| compiler fuel | 1,004,176,758 |
| module bytes | 41,858 |
| executed instructions (gap) | 6,204,746,456 |
| — stack load/store | 1,012,060,384 |
| — local moves | 3,319,425,040 |
| — bounds checks | 277,328,064 |

### MIR prototype: loop-invariant aggregate field reads

The `sroa` pass excludes any local whose address is taken, so a `Vec` local is
a stack slot and every `vec[i]` reloads its pointer and length:
`_502 = copy ((_2.0: RawVec).0: NonNull)` and `_503 = copy (_2.1: usize)`.
On nbody those two loads are inside the inner `i`/`j` loops.

The prototype adds a MIR pass after GVN. It finds natural loops with a unique
preheader, then moves scalar `Assign(dest, Use(Copy(place)))` statements out
of the loop when:

- `place` is a field projection of a local, with no dereference or indexing;
- that base local is not written and has no `Ref`/`RawPtr` in the loop, and is
  not passed by value to a call in the loop;
- the base has an assignment in a block that dominates the preheader;
- `dest` is scalar and is not otherwise assigned in the loop;
- the statement is not in a cleanup block.

The prototype was scoped to the `nbody` crate for the experiment; it is a
narrow first SROA increment, not a general inliner or full SROA. It is 230
lines including the new pass file and its registration.

Result (same command, 5 runs):

| | baseline | MIR hoist | change |
|---|---:|---:|---:|
| execution, own wasm (ms) | 67.73 | 70.67 | +4.3% raw* |
| LLVM `-O2` (ms) | 13.29 | 14.01 | +5.4% raw* |
| ours/O2 | 5.098 | **5.046** | **-1.0%** |
| warm compile (ms) | 105.34 | 110.50 | +4.9% |
| compiler fuel | 1,004,176,758 | 1,009,643,147 | +0.5% |
| module bytes | 41,858 | 41,450 | -1.0% |
| executed instructions | 6,204,746,456 | **5,550,101,348** | **-10.6%** |
| — stack load/store | 1,012,060,384 | 873,395,276 | -13.7% |
| — local moves | 3,319,425,040 | 3,002,088,688 | -9.6% |
| — bounds checks | 277,328,064 | 39,144,640 | -85.9% |
| — other | 1,517,176,156 | 1,556,715,932 | +2.6% |

\* The raw wall-clock moved on both sides in the same direction, so the
machine-independent ratio is the useful number: the pass removes **10.6% of
the executed instructions** but the nbody wall time is dominated by work the
pass does not remove; the observed execution change is within run noise. The
compile-time cost is real: about +5% warm compile on this program.

The large bounds-check drop is a side effect of making the `Vec` pointer and
length loop-invariant before the later simplification/GVN-adjacent passes; it
is not a bounds-check elimination pass. The remaining 1.56G `other` bucket
shows why one SROA increment is not parity.

### Cranelift prototype: loop-invariant stack loads

The CLIF prototype found natural loops on Cranelift IR and moved
`stack_addr` + `load` pairs whose stack slot had no store, call or address
escape in the loop to the preheader. On nbody it moved **120** static loads,
and the compiler built. It then **miscompiled nbody**: the module trapped in
`core::panicking::panic_bounds_check` because a hoisted pointer/length read a
value that had been written through an address the pass did not see (a stack
address stored in a block argument or through a call argument, or a store
derived from such an address). That prototype has no valid performance
numbers.

A sound narrow version was also tried: a stack slot may be hoisted only if no
stack address for it ever escapes to a store, call or block argument anywhere
in the function. That version moved **0** loads on nbody: every hot slot is
initialised through `Vec::push`/call arguments, so the type and alias facts
that MIR has are exactly what the narrow CLIF version lacks.

Making the CLIF version generally correct needs:

- full escape/alias analysis for stack slots (including pointers stored in
  memory and block arguments), and
- a real mem2reg with block parameters/phi placement at CFG joins, not just
  loop-invariant hoisting.

That is the same work as a general SROA pass, on a representation that has
already lost Rust's `&mut` no-alias and aggregate type information. The
prototype was ~300 lines and still unsound for one benchmark; a sound version
is substantially larger than the MIR pass.

## Decision

**Step 5b is MIR work: a cost-model inliner, then SROA/mem2reg, repeated.**
Reasons:

1. The MIR prototype cuts 10.6% of nbody's instructions with 230 lines and
   type/alias facts rustc already has. The CLIF prototype needs full
   escape/alias analysis and phi construction to be sound, and its safe slice
   found no opportunity on nbody.
2. MIR is where inlining and SROA compose: inlining exposes the aggregates and
   the no-alias facts that make field promotion valid; doing the same on CLIF
   would first have to reconstruct that information.
3. The compile-time cost belongs in the tier-2 build (Goal 1's second tier):
   the MIR prototype costs about +5% warm compile on nbody. The fast tier is
   unchanged.
4. Keep clif-to-wasm's existing tiny-leaf inliner and address folding; they are
   local clean-ups, not the structural optimiser.

Step 5b should not claim parity from this prototype. The nbody instruction
count is still 3.08x LLVM `-O2` and the execution ratio is still 5.05x; the
prototype only proves that the MIR route has a measurable first win and that
the CLIF route needs a much larger, less informed pass.

## Reproduce

```sh
# baseline (product compiler)
bench/run.sh --programs nbody --runs 5 --fuel --out bench/results/step5a-baseline.json

# MIR prototype (patch from the experiment, not applied by the product build)
experiments/cranelift-wasm/stage-c/scripts/build-rustc-wasm.sh
bench/run.sh --programs nbody --runs 5 --fuel --out bench/results/step5a-mir.json
bench/gap/run.sh --no-mir nbody
```

The prototype pass is a scratch experiment: it was applied to the toolchain
rust tree, built, measured, and then removed. The product build and this
repository remain unchanged. The numbers above come from
`bench/results/step5a-baseline.json` and `step5a-mir5.json` on the machine
recorded there.
