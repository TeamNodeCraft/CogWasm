# Performance suite

`bench/` measures the product, not one experiment: the stage-C host starts
`rustc.wasm`, compiles Rust to WebAssembly, links it with Wild, and the linked
program runs in Wasmtime. Every number is a median of N measured runs (default
5) after a warm-up, with min and max, and every program's result is checked
against LLVM `-O0` and `-O2` before the suite accepts a timing.

The suite answers the owner's question directly: *do the changes we make alter
the final CogWasm in-wasm compile time and execution?*

## What is measured

| Group | Metrics |
| --- | --- |
| **A. Compiler start** | `rustc.wasm` module compile with no cache, Wasmtime cached load, per-host precompiled `.cwasm` load, instantiation. Measured in child processes so teardown is never part of a sample. |
| **B. Compile inside `rustc.wasm`** | warm compile (module resident, fresh compiler instance), incremental compile with a one-line edit, instantiation, linker time, peak guest memory. Phases come from `-Ztime-passes`: frontend = total − codegen − link, plus the raw phase map. |
| **C. Execution** | `run(seed)` in Wasmtime for the Cranelift pipeline and for LLVM `-O0` and `-O2` builds of the same source. |
| **D. Output size** | total, code and data bytes for every build. |

Compile time is the `_start` call inside `rustc.wasm` minus the native Wild
subprocess time; module load and instantiation are reported separately. Peak
memory is the largest linear-memory size requested, tracked by a Wasmtime
`ResourceLimiter`, so it includes the compiler's own heap and stack.

## Programs

All programs use the difftest ABI: `run(seed: u64) -> u64`, `out_ptr()`,
`out_len()` and the exported `memory` (see
[`programs/prelude.rs`](programs/prelude.rs)). The same source is compiled by
both pipelines, so a result mismatch is a suite failure, never a speed-up.

| Class | Program | Lines | What it exercises |
| --- | --- | ---: | --- |
| small | `a_add` | 26 | no_std arithmetic, mirrors stage-C `a_add` |
| small | `b_alloc` | 63 | `Vec`, `String`, `format!`, closures, trait objects |
| small | `c_plugin` | 36 | `nodecraft::host_bias`, f64 parameters, growable heap |
| small | `smoke` | 29 | difftest smoke: sort, Vec, shifts, output buffer |
| small | `int_loops` | 65 | integer loop codegen |
| small | `float_sum` | 109 | f64 summation orders (naive, Kahan, Neumaier, pairwise) |
| medium | `expr` | 481 | tokenizer, recursive-descent parser, interpreter |
| medium | `json` | 562 | JSON-like tokenizer/parser and traversal |
| medium | `collections` | 309 | `Vec`/`BTreeMap`/`BTreeSet`/`BinaryHeap`/iterators |
| medium | `scene` | 499 | trait-object-heavy scene/graph walk |
| kernels | `nbody` | 154 | 128-body f64 simulation, 300 steps |
| kernels | `mandelbrot` | 72 | 320×240 f64 render |
| kernels | `sha256` | 194 | SHA-256 over 64 KiB |
| kernels | `sort` | 114 | hand-written heapsort over 20k `u64` |
| kernels | `matmul` | 88 | 128×128 f64 matrix multiply |
| kernels | `strings` | 165 | split/count/case-convert over 512 lines |
| kernels | `math_e` | 34 | A.2 (e): 100k libm `sin`/`cos`/`sqrt` points |
| kernels | `hostcall_f` | 41 | A.2 (f): 200k `nodecraft` host calls |
| large | `libm_driver` | 31 + libm 0.2.16 | real no_std crate built as an rlib, driver linked against it |
| large | `miniz_driver` | 34 + miniz_oxide 0.9.1 + adler2 2.0.1 | second substantial no_std crate |
| large | `generated_stress` | 6,266 generated | 1,024 generic instantiations in 1,024 small functions |

The three vendored crates live in [`vendor/`](vendor/) with their licences and
a pinned `Cargo.lock`. `bench/scripts/gen_stress.py` generates the stress
program deterministically into `bench/work/`; it is never committed.

## Running

```sh
# CI-sized subset, about a minute after the compiler exists
bench/run.sh --quick

# Everything, default 5 runs; writes bench/results/full-<commit>-<time>.json/.md
bench/run.sh --full

# A specific subset and more runs
bench/run.sh --programs a_add,expr,sha256 --runs 7 --out bench/results/local.json
```

`--quick` is 12 programs (all small programs, `expr`, `nbody`, `sha256`,
`matmul`, `math_e`, `hostcall_f`) with 3 runs; `--full` is all 21 programs with
5 runs and five cold module compiles. `--programs` accepts a name or
`class/name`. Other switches: `--runs N`, `--out FILE`, `--no-precompiled`.

Requirements: the stage-C artifacts must already exist under
`experiments/cranelift-wasm/toolchain/` (`rustc.wasm`, `sysroot-pure`, Wild,
the isolated rustup). Build them with
`experiments/cranelift-wasm/ci/build-and-test.sh`. The script builds
`bench/host` into the same Cargo target directory as the stage-C host, so CI
reuses the compiled Wasmtime units.

Baseline runs against a different checkout (for example a detached worktree at
an older commit) use the same suite with overridden paths:

```sh
BENCH_TOOLCHAIN=/path/to/old/experiments/cranelift-wasm/toolchain \
BENCH_LLVM_RUSTC=/path/to/old/.../rustup/toolchains/nightly-2026-10-19-.../bin/rustc \
bench/run.sh --full --out bench/history/<machine>-<commit>.json
```

## Correctness gate

Before a timing is recorded, the suite runs the Cranelift module and both LLVM
modules over the fixed seed list and compares `run(seed)` and the output bytes
bit-for-bit. A mismatch aborts the whole run with the differing outcomes; it
does not publish a JSON file. This is the same ABI and comparison rule as
[`experiments/cranelift-wasm/difftest`](../experiments/cranelift-wasm/difftest/).

The incremental metric edits a constant added by the harness from `0u64` to
`0x0_u64` (both zero) between runs, so it measures a real source edit and a
real incremental recompile without changing the answer.

## Results and comparison

Each run writes one JSON and one Markdown file. The JSON records the machine
kind, OS, CPU, cores, RAM, commit, pins (rustc, Wasmtime, vendored crate
versions, Wild hash), settings, per-program compile/execution/size statistics
(median/min/max/runs) and the correctness outcomes.

```sh
bench/compare.py base.json new.json
bench/compare.py --fail-on-regression base.json new.json   # blocking later
```

`compare.py` flags a change only when it is larger than both the run-to-run
spread and `--threshold` (default 5%). It refuses to compare different machine
kinds unless `--force` is given: CI numbers belong with CI numbers and local
numbers with local numbers. The regression threshold is advisory unless
`--fail-on-regression` is passed (and then `--fail-threshold`, default 10%).
To make CI blocking later, add that flag to the `bench.yml` comparison step;
keeping it off while the first CI baselines accumulate is deliberate.

Committed points live in [`history/`](history/), one JSON per machine and
commit. They are written locally or, for CI, only when the `bench.yml`
`record_history` input is true.

## CI

* `.github/workflows/cranelift_wasm.yml` runs `bench/run.sh --quick` after the
  compiler build, uploads the JSON, writes the table to the job summary and
  publishes both files on the rolling `ci-logs` release. It uses the artifacts
  that job already built.
* `.github/workflows/bench.yml` runs `--full` on a weekly schedule and on
  `workflow_dispatch` (optional `base` commit and `record_history` switch). It
  publishes the JSON + Markdown as one asset per run on the `bench-results`
  release and writes a comparison against the previous published full run (or
  the given base commit) to the job summary.

## Adding a program

1. Add `bench/programs/<class>/<name>.rs` with `#![no_std]`,
   `include!("../prelude.rs");` and a private `fn body(seed: u64) -> u64`.
   Emit values with the prelude's `emit_*` helpers; derive every input from
   `seed`. No clock, filesystem, network, unseeded randomness or addresses.
2. Add it to [`programs.json`](programs.json) with `class`, `source`, `quick`
   and, if it links a vendored crate, `externs`. Use `"generated": true` for a
   program produced by a script under `bench/scripts/`.
3. Run `bench/run.sh --programs <name> --runs 1`; the suite compiles it with
   both backends and fails if the outputs differ.

## Limitations

* The LLVM `-O0`/`-O2` builds link the same O2-built `libm`/`miniz_oxide`
  rlibs, so those large-crate numbers isolate driver codegen, not the
  dependency's own optimisation level.
* Peak memory is the linear-memory maximum, not host RSS.
* `-Ztime-passes` is measured in a separate phase run; the headline compile
  number does not pay for the extra instrumentation.
* CI runners are slower and noisier; the JSON records the runner kind and
  `compare.py` rejects mixed-machine comparisons.
