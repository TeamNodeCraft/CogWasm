# Differential testing: LLVM rustc vs the Cranelift → wasm pipeline

`difftest/` compiles the same Rust source twice — once with rustup's prebuilt
LLVM rustc for `wasm32-unknown-unknown`, once with the stage-C `rustc.wasm`
(cg_clif + clif-to-wasm + Wild) — loads both wasm modules in Wasmtime 50.0.2,
and compares everything observable through the corpus ABI: the `u64` return
value and the bytes in an exported output buffer. It runs in CI on every push
that touches the product directories and fails the job on any mismatch.

```
corpus/*.rs ──┬─ LLVM rustc ──────────────► work/llvm/<name>.wasm ─┐
              │                                                   ├─ Wasmtime runner ─► PASS/FAIL
              └─ rustc.wasm + cg_clif ────► work/clif/<name>.wasm ─┘
generator ────┘  (seeded random programs, same two paths)
```

No LLVM code generator runs in the Cranelift path. LLVM is only the reference
compiler; the pure sysroot (`core` + `alloc` + `compiler_builtins`, built by
`rustc.wasm`) is used for the Cranelift compile.

## Layout

```
difftest/
├── run.sh                    # one entry point (build, compile, run, report)
├── corpus/
│   ├── prelude.rs            # shared ABI, allocator, panic handler, output
│   ├── expect_trap.txt       # programs expected to trap in both backends
│   └── *.rs                  # 49 handwritten programs (one category each)
├── results/                  # run.log + details.log (gitignored)
├── FINDINGS.md               # mismatches, non-guarantees, triage
└── FEASIBILITY.md            # running cg_clif/rustc tests through this path
```

The runner and generator live in the host workspace at
[`crates/difftest-host/`](../crates/difftest-host/) (two bins,
`difftest-host` and `difftest-gen`).

Generated programs and compiled wasm live under `toolchain/difftest-work/`
(gitignored); generated output is never committed.

## ABI and shared prelude

Every corpus program is a `#![no_std]` cdylib that textually includes
`corpus/prelude.rs`:

```rust
#![no_std]
include!("prelude.rs");

fn body(seed: u64) -> u64 { /* deterministic computation */ }
```

The prelude exports exactly:

| Export | Signature | Meaning |
|---|---|---|
| `run` | `(u64) -> u64` | calls the program's `body(seed)` |
| `out_ptr` | `() -> u32` | address of the output byte buffer |
| `out_len` | `() -> u32` | bytes written by `body` |
| `memory` | wasm memory | linear memory used by the above |

There are **no imports**: no WASI, no host functions, no clock, no randomness.
Both backends run under the same Wasmtime engine without a linker import list.

The prelude also provides:

* **`#[panic_handler]`** → `core::arch::wasm32::unreachable()`. Builds use
  `-C panic=abort`, so panics, bounds checks and division-by-zero checks become
  a wasm trap.
* **Growable bump `GlobalAlloc`**, starting at `memory.size() * 65536` and
  calling `memory.grow` by whole pages; `dealloc` is a no-op. The same source is
  compiled by both backends, so allocation sizes/order are identical. It starts
  at a page boundary in both modules, so alignment and layout are identical even
  though the modules' static-data sizes differ. **No address is ever emitted** —
  programs emit values only; the one address-like check (`alloc` returns a
  pointer aligned to the requested `Layout`) is emitted as a boolean.
* **Output buffer** of 16,384 bytes with little-endian `emit_u8/u16/u32/u64`,
  `emit_i*`, `emit_u128/i128`, `emit_f32/f64` (as bit patterns), `emit_bytes`,
  `emit_str`, `emit_bool`. Writes beyond the cap are truncated equally on both
  sides.
* **`Rng`** (splitmix64) so seed-derived inputs are deterministic and identical.

Compile flags are the same for both paths:

```
--edition=2021 --target wasm32-unknown-unknown --crate-type cdylib
-C panic=abort -C opt-level=2
-Zcodegen-backend=cranelift -Clinker=<wild> -Clinker-flavor=wasm-ld   # Cranelift only
```

## Handwritten corpus (49 programs)

| Category | Programs |
|---|---|
| Integer arithmetic | `int_wrapping`, `int_checked`, `int_saturating`, `int_divmod`, `int_casts`, `int_loops` |
| Wide integers | `int_i128`, `int_i128_div` |
| Bits and shifts | `int_bits`, `int_shifts` |
| Floats | `float_bits`, `float_arith`, `float_convert`, `float_round`, `float_sqrt`, `float_nan_props`, `float_sum` |
| Slices, arrays, memory | `slices`, `arrays`, `memcpy_large`, `ptr_ops`, `struct_layout` |
| In-memory field loops | `loop_field_reads` (step 5b's loop hoist) |
| Allocation | `alloc_growth`, `alloc_patterns`, `box_rc` |
| Collections and strings | `vec_ops`, `string_ops`, `format_specs`, `btreemap`, `collections_extra` |
| Algorithms | `sort_impl`, `hash_fnv`, `matrix`, `checksum`, `fixed_point` |
| Language features | `closures`, `generics`, `trait_objects`, `iterators`, `recursion`, `enum_match`, `control_flow`, `result_option` |
| ABI smoke test | `smoke` (sort + Vec + shifts + output buffer) |
| Expected traps | `panic_trap`, `trap_div_zero`, `trap_oob`, `trap_unreachable` |

Coverage highlights: wrapping/checked/saturating/overflowing ops; signed
division and `MIN / -1`; `i128`/`u128` mul/div/shifts; masked runtime shifts;
f32/f64 infinities, signed zeros, subnormals, NaN classification, saturating
float→int casts, f64↔f32 rounding; software sqrt/sin/cos; `Vec`, `String`,
`format!`, `BTreeMap`/`BTreeSet`, `VecDeque`, `BinaryHeap`, `Rc`/`RefCell`;
closures, trait objects, generics/const generics, iterators, recursion, enums;
8 KiB memcpys and multi-megabyte bump-allocation growth.

### Expected traps

`corpus/expect_trap.txt` lists the four trap programs. For those, **a trap in
both backends is a match**; a trap in one backend only, or a normal return from
either, is a mismatch. Trap *messages* are not compared (the two backends name
functions differently); only the both-trapped/one-trapped classification is.

## Seeded generator

`difftest-gen` writes one program per generation seed (`gen_00001` … default
`gen_00024`). Each program is deterministic and bounded:

* 10–19 straight-line statements chosen from wrapping add/sub/mul, rotates,
  variable shifts, conditional branches, `match`, i32/i64/i128 widening,
  f64 division and array/slice reads;
* a 16–63 iteration loop with 4–7 statements and periodic `emit_u64`;
* a `Vec<u64>` of 8–31 elements, sorted and folded;
* the runtime `seed` is mixed into all four locals, so every `--run-seeds`
  value exercises a different path;
* constants and structure come from a splitmix64 RNG keyed by the generation
  seed; the source FNV-1a checksum is recorded in the generated
  `manifest.tsv` for reproducibility.

The runner uses four fixed run seeds by default:
`0, 1, 42, 0x9e3779b97f4a7c15`. `--quick` uses eight generated programs and two
run seeds.

## Runner comparison rules

For every `(program, seed)` the runner creates a **fresh store and instance**
for each backend (module compilation is shared across seeds). It then:

1. calls `run(seed)`; a trap is recorded with its Wasmtime message;
2. on success reads `out_ptr`/`out_len`, bounds-checks them against `memory`,
   and reads the bytes;
3. compares the `u64` return and the output bytes **bit-for-bit**;
4. treats a missing export, bad memory range or failed instantiation as an
   invalid/mismatched program (never silently skipped).

A trap in both is a match only for `expect_trap` programs. A trap in one is
always a mismatch. On mismatch it prints program, seed, both outcomes, the
first differing byte and hex windows, writes the same to
`results/details.log`, and exits nonzero. A per-program PASS/FAIL line and a
summary (`programs`, `calls`, `value_matches`, `trap_matches`, `mismatches`) are
printed to stdout, so the CI log contains the full record.

## Running

```sh
cd CogWasm

# Default: 48 handwritten + 24 generated, 4 run seeds (288 calls)
difftest/run.sh

# More/fewer generated programs
difftest/run.sh --seeds 64
difftest/run.sh --quick          # 8 generated, 2 run seeds

# CI (what build-and-test.sh adds after the A.2 checks)
difftest/run.sh --seeds 24
```

The script builds the runner/generator itself (reusing
`toolchain/target/stage-c-host`, where Wasmtime is already compiled), then
compiles both backends for every program. It needs the stage-C artifacts:
`toolchain/stage-c/artifacts/rustc.wasm`,
`toolchain/stage-c/sysroot-pure`,
`toolchain/target/stage-c-host/release/stage-c-host`,
`toolchain/target/wild/release/wild`, and the isolated nightly rustup. Missing
artifacts produce a clear error; run `compiler/scripts/setup-sources.sh` and
`compiler/scripts/build-*.sh` first.

### Incremental-compilation check

`incremental-check.sh` compiles `corpus/smoke.rs` cold, then twice with one
persistent `-C incremental` directory, and requires all three linked modules to
be byte-identical. It is the regression check for the wasm file-lock fallback
and the host's `--incremental` path; `compiler/run.sh incremental` checks the
real programs. It needs the same stage-C artifacts as `run.sh`:

```sh
difftest/incremental-check.sh
```

### Environment overrides

| Variable | Default |
|---|---|
| `DIFFTEST_LLVM_RUSTC` | first `toolchain/rustup/toolchains/nightly-*-*/bin/rustc` |
| `DIFFTEST_RUSTC_WASM` | `toolchain/stage-c/artifacts/rustc.wasm` |
| `DIFFTEST_SYSROOT` | `toolchain/stage-c/sysroot-pure` |
| `DIFFTEST_STAGE_C_HOST` | `toolchain/target/stage-c-host/release/stage-c-host` |
| `DIFFTEST_WILD` | `toolchain/target/wild/release/wild` |
| `DIFFTEST_CACHE_DIR` | `toolchain/stage-c/cache` |
| `DIFFTEST_TARGET_DIR` | `toolchain/target/stage-c-host` |
| `DIFFTEST_WORK` | `toolchain/difftest-work` |
| `DIFFTEST_RESULTS` | `difftest/results` |
| `DIFFTEST_CARGO` | isolated `toolchain/cargo/bin/cargo`, else `cargo` |
| `DIFFTEST_HOST_TOOLCHAIN` | `1.99.1` (same as the stage-C host build; falls back to the LLVM nightly) |
| `DIFFTEST_GEN_START` | `1` |

Host discovery is derived from the isolated rustup toolchain directory names, so
macOS arm64 and Linux x86_64 both work without editing the script.

## Measured results

Local macOS arm64 (Apple silicon, 14 CPUs), 2026-11-19, existing artifacts:

| Configuration | Programs | Calls | Wall time |
|---|---:|---:|---:|
| `--quick` (8 generated, 2 run seeds) | 56 | 112 | **47.5 s** |
| default (24 generated, 4 run seeds) | 72 | 288 | **56.6 s** |
| `--seeds 32` (32 generated, 4 run seeds) | 80 | 320 | **62 s** |

Breakdown for the default run: 49 handwritten programs compiled in 36–38 s,
24 generated in 18 s, the Wasmtime comparison itself ~1 s; the rest is the
tiny runner build (Wasmtime was already compiled for the stage-C host).
All runs above were green: `value_matches` + `trap_matches` = `calls`,
`mismatches=0`.

CI uses `--seeds 24`. The Linux runner's `rustc.wasm` is ~2–2.3× slower per
compile than this Mac (see `docs/results.md`), so the step should take
roughly 2–3 minutes; it is 72 wasm compiles of small programs and stays well
inside the 5-minute budget.

## CI wiring

`ci/build-and-test.sh` adds, after the A.2 `(e)/(f)` checks and before the
no-LLVM audit:

```sh
step "differential test: LLVM rustc vs Cranelift->wasm"
"$REPO/difftest/run.sh" --seeds 24
```

`set -e` makes any compile failure or mismatch fail the job. Full details are
on stdout (captured by the published CI log) and in `difftest/results/` on the
runner.

## Adding a case

1. Create `corpus/<name>.rs`:

   ```rust
   #![no_std]
   #![allow(dead_code)]
   include!("prelude.rs");

   fn body(seed: u64) -> u64 {
       let mut rng = Rng::new(seed);
       let x = rng.next_u64();
       emit_u64(x.rotate_left(7));
       x ^ 0x9e37_79b9_7f4a_7c15
   }
   ```

2. Keep it **deterministic for a given seed** and free of host input. Emit
   values, never addresses. For computed NaNs compare properties
   (`is_nan`/classification), not payload bits — see `FINDINGS.md`.
3. If it must trap in both backends for every run seed, add its stem to
   `corpus/expect_trap.txt`; otherwise leave it out and it must return normally.
4. Run `difftest/run.sh --quick`. No other registration is needed: the script
   globs `corpus/*.rs` and derives the manifest from the filename and
   `expect_trap.txt`.
5. Keep new corpus programs small enough to compile in well under a second (the
   CI budget is ~5 minutes for 72 programs).

## Limitations / non-goals

* Target is `wasm32-unknown-unknown` with `core` + `alloc` only. Programs using
  `std` (files, threads, `println!`, `f64::sqrt`, …) cannot be compiled by the
  Cranelift path, whose sysroot intentionally has no `std`. The feasibility note
  covers what it would take to use rustc/cg_clif's own suites.
* Single-threaded, no atomics, no WASI, no host imports.
* The bump allocator never frees; corpus programs stay in the low MiB range.
* The output buffer is capped at 16 KiB. Overflow is truncated identically on
  both sides, so it cannot create a false PASS for compared bytes, but a
  program that emits more than 16 KiB is not fully compared.
* NaN payloads from arithmetic are not part of the contract (Rust does not
  guarantee them); explicit `from_bits`/`to_bits` payload round-trips *are*
  compared bit-for-bit.
* Trap messages/backtraces are not compared; only trap-vs-trap classification
  for `expect_trap` programs.
* This is differential testing, not a proof: a bug present in both compilers or
  outside the exercised category can pass.

See [`FINDINGS.md`](FINDINGS.md) for the current result and triage notes, and
[`FEASIBILITY.md`](FEASIBILITY.md) for running external test suites.
