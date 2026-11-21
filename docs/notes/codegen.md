# Codegen profiling: why A.2 (e) is slower than LLVM

Track C, 2026-11-19. Host: macOS arm64 (Apple M4 Pro, 14 cores). All numbers
are Wasmtime 50.0.2, median of five warm calls, same machine and build profile
as `docs/results.md`.

## Method

`wasm2wat` is not installed, so `toolchain/tracks/c/` (gitignored) holds a
throwaway printer/analyzer built against the local `wasmprinter` 0.263 and
`wasmparser` 0.263. It prints WAT and counts, per function and in total,
`local.get`/`set`/`tee`, loads and stores, calls, conversions, arithmetic and
control flow.

To split the (e) gap by operation instead of guessing, the benchmark was rebuilt
with one operation at a time. Each probe is `guests/e_math` with only
`transform` changed, compiled with the same two backends and timed with the same
`a2-harness bench_math` (100k points):

| `transform(x)` | LLVM O2 ns/point | cg_clif O2 ns/point |
|---|---:|---:|
| `x` (loop only) | 0.75 | 2.95 |
| `libm::sin(x)` | 4.82 | 6.63 |
| `libm::cos(x * 0.25)` | 4.27 | 6.16 |
| `libm::sqrt(abs(x))` | 12.33 | 69.58 |
| the full (e) transform | 22.99 | 87.79 |

The parts add up to the whole on both backends (LLVM 22.2 vs 23.0, cg_clif 85.3
vs 87.8), so the decomposition is sound. `sin` and `cos` are at parity (0.9x
LLVM); the **software `sqrt` alone accounts for 57 of the 65 ns/point gap**
(5.6x LLVM), and the remaining ~8 ns is loop bookkeeping (2.2 ns) plus `sin` and
`cos` (3.6 ns).

## What the emitted wasm shows

### `sqrt`: tiny generic calls that only LLVM inlines

`e_math` depends on `libm` with `default-features = false`, which turns off
libm's `arch` feature, so `libm::sqrt` is the portable Goldschmidt software
algorithm on **both** backends (LLVM's wasm build has no `f64.sqrt` either).
The difference is what happens to the helpers around it:

* LLVM: one 490-byte `sqrt` function, 0 stores, 3 calls, everything inlined by
  LLVM's own inliner on already-monomorphized IR.
* clif-to-wasm: `sqrt` (73 B) calls `sqrt_round` (1829 B), which calls
  `goldschmidt` twice (415 B + 338 B) and contains **67 calls**, 35 stores and
  27 loads in the wasm body. The call targets are one- to four-instruction
  `core` helpers: `<u64 as Shl<u32>>::shl`, `Shr::shr`, `BitAnd::bitand`,
  `PartialEq::eq`, `wrapping_sub`, `CastInto::cast`, `Float::to_bits`…

The first of those calls in `sqrt_round` compiles to:

```wat
local.get 4
i64.load             ;; `ix` lives in a stack slot
i32.const 52
i32.const 1049840    ;; #[track_caller] location
call $<u64 as Shr<u32>>::shr
```

while the callee is just:

```wat
(func $Shr::shr (param i64 i32 i32) (result i64)
  local.get 0
  local.get 1
  i64.extend_i32_u
  i64.shl
)
```

Rust's MIR inliner cannot remove these: it runs **before monomorphization**, so
a call to `<F::Int as Shl<u32>>::shl` inside `sqrt_round<F>` is still
polymorphic when the pass runs. `-Zinline-mir -Zmir-opt-level=4` leaves the
call count unchanged (241 calls in the whole module) and measured slightly
slower (74–80 ns/point), confirming the earlier A.2 result from a different
angle. Cranelift has no inliner of its own, and clif-to-wasm cannot add one
while compiling one function at a time.

### Loop bookkeeping: locals held in stack slots

The loop-only probe is 4x LLVM (2.95 vs 0.75 ns/point). The hot loop is
`reduce`; `slice::Iter::next` is inlined into it by MIR, but the iterator
(`ptr`, `end`) is address-taken, so cg_clif keeps it in a stack slot and every
iteration re-stores and re-loads it:

```wat
local.get 1          ;; stack slot A
local.get 3
i32.load             ;; iter.ptr
i32.store
local.get 4
i32.load             ;; iter.end
local.set 12
local.get 5
local.get 12
i32.store
local.get 1
i32.load             ;; the same iter.ptr again
local.tee 13
local.get 12
i32.eq
```

LLVM's SROA and GVN promote the whole iterator to SSA registers and emit about
half the instructions. Cranelift's egraph pass has no rules for `stack_load` /
`stack_store`, so the traffic survives; clif-to-wasm only lowers it to linear
memory. This is the same root cause as the `sqrt` calls (no cross-function or
memory-to-SSA optimization) and is left as future work: a sound stack-slot SROA
needs alias/escape analysis, which is too large for this batch.

### Size and dead code

The linked (e) module is 39,995 B of code against LLVM's 13,173 B, and contains
180 functions against 34. Most extra functions are allocation, formatting and
panic paths that are never called by `eval`; they make the module larger and
slower to compile but not slower to run. The hot path is `reduce`, `transform`,
`sin`, `cos`, `sqrt` and their helpers.

### (f) is not affected

`f_geometry` spends its time in imported host calls; the backend emits almost
nothing between them. It is at parity (2.85 vs 3.11 ns/call in the earlier
stage-B table) and this patch does not touch the call ABI, so no change is
expected.

## Patch: inline tiny leaf functions

Two patches, one concern:

1. `compiler/patches/clif-to-wasm/0002-inline-small-leaf-functions.patch` adds
   `is_small_leaf` and `inline_small_leaf_functions`, built on Cranelift's
   `Context::inline` hook. A direct call is replaced by its body only when the
   callee is a single-block leaf of at most eight instructions with no stack
   slots, calls, memory, global or table accesses, branches or traps.
2. `compiler/patches/cg_clif/0003-wasm-inline-small-functions.patch` applies it
   to every codegen unit's wasm object before compilation. A codegen unit's
   bodies are all collected before any is compiled, so a call is inlined
   whether the callee is compiled before or after the caller. The native path
   is unchanged.

Correctness: an inlined callee has no observable operation that could be
duplicated, reordered or skipped. Arguments and results are the same values;
`#[track_caller]` locations are ordinary arguments; an integer division trap
still traps at the same point. The leaves that qualify are the operator and
cast wrappers whose bodies are pure `iadd`/`ishl`/`band`/`ireduce`/`icmp`.
Calls that are not tiny, are tail calls, or whose body is not in the unit are
kept. The tests in `toolchain/clif-to-wasm/tests/inline.rs` cover the inlined
result, a body with a load, an oversized body, and an unresolved callee.

## Results

### Native cg_clif A/B (pre-combined-build)

The same probe kernels were compiled with the unpatched and the patched cg_clif
dylib (both against a rustup LLVM-built wasm sysroot, so only the plugin and
libm are compiled by cg_clif) and timed with `a2-harness bench_math`:

| kernel | unpatched ns/pt | patched ns/pt | code bytes |
|---|---:|---:|---:|
| `sqrt` | 69.0 | **52.1** | 16,529 -> 15,766 |
| `sin` | 6.65 | 6.66 | unchanged |
| `cos` | 6.18 | 6.24 | unchanged |
| loop only | 2.9-3.0 | 2.9-3.0 | byte-identical artifact |

In the `sqrt` probe, total wasm `call` instructions fall from 163 to 103 and
`sqrt_round` shrinks from 1829 B / 67 calls to 1690 B / 49 calls. The remaining
calls are to helpers in another libm codegen unit (cg_clif sees one codegen
unit at a time) or to branchy `*Assign` methods that take `&mut self` and are
not leaves. Raising the instruction limit from 12 to 16 changes nothing, so 12
is kept.

### Final A.2 on the combined-build rustc.wasm

Artifact 144,051,631 B SHA-256 `d2404a86…6fb91` (0004 + both codegen patches),
plugins rebuilt by it, `compiler/run.sh a2`, median of 5:

| metric | before | after | change |
|---|---:|---:|---:|
| (e) run, 100k points | 86.45 ns/pt | **68.99 ns/pt** | 1.25x faster |
| (e) code size | 39,995 B | **39,152 B** | -2.1% |
| (e) functions | 180 | **150** | -30 |
| (e) total size | 62,124 B | **58,252 B** | -6.2% |
| (f) run, 200k host calls | 3.799 ns/call | **3.781 ns/call** | unchanged |
| (f) code size | 21,965 B | **21,937 B** | unchanged |
| (f) total size | 34,059 B | **33,721 B** | -1.0% |

Against LLVM -O2, (e) improves from 3.76x to **3.00x** slower; (f) remains at
parity. `compiler/run.sh check` still returns 2.25, `a2check` passes both
programs, and `difftest/run.sh --seeds 24` passes 72 programs / 288 calls / 0
mismatches. The final stage-B compile median was 79.4 ms (81.9 ms before), so
the extra pass did not show a compile-time regression at this size. The main
missed opportunity is cross-CGU inlining: cg_clif gives the pass only the
bodies in the current codegen unit, so libm's generic helpers in other CGUs
remain calls.
