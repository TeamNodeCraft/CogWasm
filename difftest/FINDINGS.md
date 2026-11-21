# Findings

Last updated: 2026-11-19, local macOS arm64 (Apple silicon), stage-C artifacts
as shipped in `toolchain/stage-c/` (rustc.wasm 143,849,756 B, sysroot-pure built
by rustc.wasm, Wild 0.11.0, Wasmtime 50.0.2).

## Current status: no mismatches

| Run | Handwritten | Generated | Run seeds | Calls | Value matches | Trap matches | Mismatches |
|---|---:|---:|---:|---:|---:|---:|---:|
| `--quick` | 48 | 8 | 2 | 112 | 104 | 8 | **0** |
| default | 48 | 24 | 4 | 288 | 272 | 16 | **0** |
| `--seeds 32` | 48 | 32 | 4 | 320 | 304 | 16 | **0** |

"Value matches" means the `u64` return and every output byte agreed; "trap
matches" are the four `expect_trap` programs trapping in both backends for
every run seed. No compile failure occurred on either path for any corpus
program.

No real miscompile was found, so there is no cg_clif / clif-to-wasm reproducer
to file. The corpus does cover the areas that usually differ (i128 mul/div,
masked shifts, float→int saturation, f64↔f32 rounding, `min`/`max` NaN
semantics, trait-object dispatch, `BTreeMap` ordering, bulk memory and
`memory.grow`), and they agree on this toolchain.

## NaN payloads: not a guarantee, but no divergence observed

Rust does not guarantee the payload bits of a NaN produced by arithmetic (the
wasm spec allows nondeterministic NaN payload propagation). The corpus
therefore does **not** compare payloads of computed NaNs:

* `float_arith` / `float_convert` fold any NaN result to a one-byte marker and
  compare only "is NaN"; finite results are compared bit-for-bit.
* `float_nan_props` compares explicit `from_bits`/`to_bits` patterns (bitwise
  round-trip *is* guaranteed), `is_nan`/`is_infinite`/`is_sign_negative`,
  classification, `total_cmp`, and the documented `min`/`max` rule that returns
  the non-NaN operand.
* `float_bits` compares explicit bit patterns and arithmetic on finite values
  (including signed zeros and subnormals), not computed NaN payloads.

To check whether the exclusion hides a real difference, I separately compiled
and ran this probe through both paths:

```rust
#![no_std]
include!("prelude.rs");

fn body(_seed: u64) -> u64 {
    let inf = f64::INFINITY;
    emit_u64((inf - inf).to_bits());
    emit_u64((0.0f64 / 0.0).to_bits());
    emit_u64((inf * 0.0).to_bits());
    emit_u64((f64::NAN + 1.0).to_bits());
    emit_u64((-f64::NAN).to_bits());
    let p = f64::from_bits(0x7ff8_dead_beef_cafe);
    emit_u64(p.to_bits());
    emit_u64((p + 1.0).to_bits());
    emit_u64((p * 1.0).to_bits());
    emit_u64((-p).to_bits());
    emit_u64(p.abs().to_bits());
    emit_u64(p.min(1.0).to_bits());
    emit_u64(p.max(1.0).to_bits());
    emit_u64((p as f32).to_bits() as u64);
    let q = f32::from_bits(0x7fc0_dead);
    emit_u64((q + 1.0).to_bits() as u64);
    emit_u64((q as f64).to_bits());
    0
}
```

Result: the 128 output bytes were **identical** between the LLVM module and the
Cranelift module (all 16 NaN/propagation values, including payload
`0xdeadbeefcafe`, matched). That is stronger than the corpus contract requires,
but it is toolchain-specific: a future compiler or Wasmtime may canonicalise
differently, so the committed corpus deliberately keeps payloads out of the
hard assertions. If a future run diverges only in payload bits, it must be
recorded here and not promoted to a mismatch; a divergence in `is_nan`/sign/
classification is a real mismatch.

## Trap behaviour

The four `expect_trap` programs (`panic_trap`, `trap_div_zero`, `trap_oob`,
`trap_unreachable`) trap in both modules for every run seed. The runner does not
compare trap *messages*: the two backends emit different function names, and
Wasmtime's message includes a backtrace. It compares only the classification
(both trapped, one trapped, or a normal return). One-sided traps and unexpected
normal returns are mismatches.

## Known non-guarantees / limitations that are intentionally not asserted

| Observable | Why it is not compared |
|---|---|
| NaN payloads from arithmetic | not guaranteed by Rust or wasm |
| Trap message/backtrace text | implementation detail, includes function names |
| Module size, data layout, function count | not part of the ABI contract; Wild/cg_clif sizes differ |
| Addresses / allocation base | `memory.size()` at start can differ; only values are emitted |
| `Vec`/`String` capacity | implementation detail; programs emit `len()` only |
| Timing | not deterministic in CI |

Everything else that the corpus emits — return values, integer/float bit
patterns (except computed NaNs), formatted strings, collection order, trap-
vs-return — is compared bit-for-bit. The only truncation is the 16 KiB output
buffer cap; it applies identically to both sides and is documented in the
README.

## Procedure if a mismatch appears

1. The runner prints `program`, `seed`, both outcomes, and the first differing
   byte; the same is in `results/details.log`. Save that output.
2. Re-run that pair alone with `--run-seeds <seed>` to confirm determinism.
3. Minimise: keep the prelude and the smallest `body` that reproduces; reduce
   operations one at a time. Record the minimal source here.
4. Classify: real miscompile (wrong value/order/trap), missing feature
   (compile error), or a documented non-guarantee. Only a real miscompile is a
   failure of the pipeline; do **not** remove or weaken the expectation to make
   CI green. File a cg_clif / clif-to-wasm issue with the minimal wasm/CLIF and
   link it here.
5. If it is a non-guarantee, document it in the table above and explain why the
   corpus contract still tests the guaranteed observable behaviour.
