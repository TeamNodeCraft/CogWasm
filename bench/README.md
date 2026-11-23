# Performance suite

`bench/` measures the product, not one experiment: the cogwasm-host starts
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
| **E. Deterministic counts** | executed wasm instructions per program and the geomean over LLVM `-O2`; compiler work per compile as Wasmtime fuel, median across programs. |

Compile time is the `_start` call inside `rustc.wasm` minus the native Wild
subprocess time; module load and instantiation are reported separately. Peak
memory is the largest linear-memory size requested, tracked by a Wasmtime
`ResourceLimiter`, so it includes the compiler's own heap and stack.

The wall-clock rows are noisy on shared CI runners. Group E adds two counters
that are deterministic by construction; see
[Deterministic instruction counts](#deterministic-instruction-counts).

## Programs

All programs use the difftest ABI: `run(seed: u64) -> u64`, `out_ptr()`,
`out_len()` and the exported `memory` (see
[`programs/prelude.rs`](programs/prelude.rs)). The same source is compiled by
both pipelines, so a result mismatch is a suite failure, never a speed-up.

| Class | Program | Lines | What it exercises |
| --- | --- | ---: | --- |
| small | `a_add` | 26 | no_std arithmetic, mirrors the original `a_add` |
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
| large | `generated_stress` | 6,290 generated | 1,024 generic instantiations in 1,024 small functions across 8 modules |

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
5 runs and five cold module compiles. Quick runs also measure the precompiled
`.cwasm` compiler load, but keep the program compiles on the cached path so
quick points stay comparable across the change that added it. `--fuel` also
measures deterministic compiler work (two fuel runs per program) and is what
the CI quick job uses. `--programs` accepts a name or `class/name`. Other
switches: `--runs N`, `--out FILE`, `--no-precompiled`.

Requirements: the compiler artifacts must already exist under
`toolchain/` (`artifacts/rustc.wasm`, `sysroot`, Wild,
the isolated rustup). Build them with
`ci/build-and-test.sh`. The script builds
`bench/host` into the same Cargo target directory as the cogwasm-host, so CI
reuses the compiled Wasmtime units. The behaviour-identical helpers it shares
with the product host (the Wasmtime error adapter and the `playground.spawn`
summary type) come from [`crates/cogwasm-runtime`](../crates/cogwasm-runtime);
engine setup, stderr capture, fuel and resource-limit tracking stay in
`bench/host` because they differ from the product host.

The suite passes the compiler's `COGWASM_*` variables to the wrapper itself;
the old `STAGE_C_*` spellings still work as a fallback for this release. The
`BENCH_*` variables above remain the benchmark-specific overrides.

Baseline runs against a different checkout (for example a detached worktree at
an older commit) use the same suite with overridden paths:

```sh
BENCH_TOOLCHAIN=/path/to/old/toolchain \
BENCH_LLVM_RUSTC=/path/to/old/.../rustup/toolchains/nightly-2026-10-19-.../bin/rustc \
bench/run.sh --full --out bench/history/<machine>-<commit>.json
```

## Rust benchmark tool (`cogwasm-bench`)

The V2 benchmark driver is a Rust binary in
[`crates/cogwasm-bench`](../crates/cogwasm-bench). It needs no Python: it
prepares the suite sources, builds the ~100k-line `large` program with a Rust
port of `gen_large.py`, and writes the machine-readable result and Markdown
report itself. Toolchain paths default to `toolchain/` and every one can be
overridden (`--toolchain`, `--rustc`, `--precompiled`, `--sysroot`, `--wild`,
`--native-rustc`, `--cache-dir`, `--work-root`, `--deps-root`).

```sh
# CI-sized run with the default and capture configs side by side
cargo run -p cogwasm-bench -- run --suite quick --runs 3 \
    --config default --config capture

# All bench/programs, or only the four plugin-sized medium programs
cargo run -p cogwasm-bench -- run --suite full --runs 5
cargo run -p cogwasm-bench -- run --suite plugin --runs 5

# Generate and measure the ~100k-line (98k-line) program
cargo run -p cogwasm-bench -- run --suite large --runs 3

# Spread-aware comparison; exit 1 only with --fail-on-regression
cargo run -p cogwasm-bench -- compare baseline.json current.json
cargo run -p cogwasm-bench -- compare --fail-on-regression base.json new.json

# Re-render the Markdown, or measure the toolchain footprint only (fast)
cargo run -p cogwasm-bench -- report result.json
cargo run -p cogwasm-bench -- toolchain
```

`run` writes `result.json` and `report.md` under
`toolchain/work/bench/<timestamp>/` by default (git-ignored; use `--out` to
choose another directory). The JSON has a versioned schema (`schema: 1`) with
the machine and load average, compiler identity (rustc.wasm SHA-256 and git
commit), the toolchain footprint, and per program x config measurements. Every
numeric measurement carries `min`, `median`, `max`, `runs` and a `deterministic`
flag; instruction counts, compiler fuel and sizes are exact, wall-clock
metrics are not. The report has one table per section, suite geomeans and the
V2 target table from [`agents/plans/v2.md`](../agents/plans/v2.md), filled only
where the run collected a value.

Configs are `default`, `inline` (`-Zmir-inline-wasm`), `capture`
(`-Zmir-inline-wasm -Zmir-capture-hoist`) and `cgu-per-fn`
(`-Zcogwasm-cgu-per-fn`, incremental only). `--runs N` is both the execution
samples per seed and the number of measured edits. Compiler fuel uses a
separate fuel-enabled engine and is measured twice per program x config; the
run fails if the two samples differ. The edit loop verifies every warm edit
against a fresh cold compile byte for byte, using the same artifact basename so
Wild's module-name byte cannot create a false mismatch.

Builds run in one fixed directory, `toolchain/work/bench/.work/`, cleared at
the start of each run, because rustc's fuel depends on the length of the paths
it handles: with a fixed path, fuel is exact across runs and output
directories. Run one benchmark at a time. Edit-loop swaps use the supervisor's
swap engine (parallel compilation and the in-memory incremental cache), so
`edit swap ms` matches `cogwasm-supervisor edit-loop`. `compare` treats a
single-sample reading (host RSS, peak memory) as changed only beyond 5%;
`--runs 5` or more gives tighter timing spreads on a shared machine.

The tool intentionally does not read or call the Python scripts or write into
`bench/results` or `bench/gap/results`; all of its own output lives under
`toolchain/work/`. Local unit tests need no toolchain
(`cargo test -p cogwasm-bench`). The end-to-end test is opt-in:

```sh
COGWASM_REQUIRE_TOOLCHAIN=1 cargo test -p cogwasm-bench --test toolchain
```

## Correctness gate

Before a timing is recorded, the suite runs the Cranelift module and both LLVM
modules over the fixed seed list and compares `run(seed)` and the output bytes
bit-for-bit. A mismatch aborts the whole run with the differing outcomes; it
does not publish a JSON file. This is the same ABI and comparison rule as
[`difftest`](../difftest/).

The incremental metric edits a constant added by the harness from `0u64` to
`0x0_u64` (both zero) between runs, so it measures a real source edit and a
real incremental recompile without changing the answer.

## Deterministic instruction counts

Wall-clock numbers move on shared runners even when the compiler did not: six
no-op commits shifted the execution ratio between 2.2x and 2.7x and warm
compile between 200 and 340 ms. The quick CI job therefore also records two
counters, which have no clock in them.

**Executed instructions.** `bench/gap` instruments every basic block of the
linked program with a counter, runs the suite seeds, and sums
`executions x instructions in the block`. The quick job runs that step on the
modules the quick suite just built, for ours and LLVM `-O2`'s, and stores
per-program totals under `programs.<name>.instructions` in the history point
(`clif`, `llvm_o2`, and their ratio). The geomean ratio uses the same
`0.1 ms` LLVM `-O2` execution-time floor as the time ratio, so both ratios are
over exactly the same programs.

The two ratios measure different things and should not be expected to match.
Instructions say how much work the program executes; time also includes the
cost of each instruction. For example, `mandelbrot` executes **1.44x** LLVM
`-O2`'s instructions while taking only **1.06x** its time: our instructions are
cheaper per operation on that kernel. Conversely, a small instruction gap can
hide a large time gap when one side has expensive calls. Showing both makes
that distinction visible instead of forcing one number to stand for both.

**Compiler work.** Compiling inside `rustc.wasm` is measured with Wasmtime
fuel on a separate, fuel-enabled engine. Fuel is charged per wasm operator
from a fixed cost table, so it is compiler work, not host time. The host
subtracts the fuel used by instantiation and reports the `_start` call; each
quick program gets two fuel samples, and the host fails the run if they
differ. The history point stores the per-program median under
`programs.<name>.clif.compiler_fuel` and the median across programs under
`deterministic.compiler_fuel_median`.

Determinism evidence and variation found:

* Two independent processes with the same fixed work path produced identical
  fuel for `a_add` (405,448,969) and `b_alloc` (806,451,911), and every
  per-program pair of samples inside a process was identical too.
* The first check did **not** agree: changing the work/output path between
  runs moved `a_add` by a few hundred fuel units (405,525,972 vs 405,448,969).
  rustc hashes the input path into its metadata, so path bytes are part of the
  input. The harness already uses the stable
  `bench/work/compile/<program>` and `bench/work/wasm/...` paths; the fuel
  measurement now relies on that stability instead of creating a new directory
  per sample.
* rustc's standard-library hash seeds come from WASI `random_get`, and fresh
  per-context entropy made two samples in one process differ by 84 fuel units.
  Fuel measurement now installs a deterministic WASI random source; the timed
  runs keep the product's real entropy.
* A compile that finds its output file already present does a few units less
  work than one that creates it. Fuel runs write a separate
  `<program>.fuel.wasm` and remove it before every sample, so both samples
  start from the same state and the benchmark's own module is never replaced
  by a fuel-run build.
* rustc formats the wall clock into the name of its ICE-report file:
  `rustc_driver_impl::ice_path_with_config` calls `jiff::Zoned::now()` and
  `strftime`. The formatter's fuel cost depends on the civil date and time it
  is given, so samples that crossed a second boundary differed by a few fuel
  units (5 in the failure that prompted this note), and separate invocations
  moved with the wall clock. Fuel runs now give WASI a fixed wall and
  monotonic clock and pin `TZ=UTC`; the timed runs keep the real clock and
  time zone. A WASI trace of a fuel compile shows exactly one realtime
  `clock_time_get`, from `ice_path_with_config`, and the rest monotonic reads
  from Cranelift's timing profiler. Ten invocations with eight samples each
  then produced one value for every sample.
* No allocator dependence was observed after those fixes.
* Fuel is a fixed per-operator cost charged by the interpreter instrumentation,
  not a native instruction count, so it does not depend on the CPU or on
  Cranelift's register allocation. We verified repeated runs on the development
  machine; the first CI point on the Linux runner is the cross-runner check.

Counts are exact for the seed list and module that produced them, and two
measurements of the same instrumented module are compared before publishing.
Older history points have no counts and simply show no value for the two lines.
We did not backfill them: the same commits' linked modules are not retained, so
a backfill would not have been exact. Counts start with the next CI point.

CI cost: the count step itself is seconds for the 12 quick programs (two
measurements each). The one-time cost is compiling `rustc.wasm` once on the
fuel-enabled engine, because fuel instrumentation changes the compiled code:
about 39 s on the development machine and roughly 1-2 minutes on the CI runner,
then about 0.4 s per program from the Wasmtime cache. It reuses the quick
modules and never rebuilds the compiler.

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

## Baseline: did the speed batch help?

The suite was run on the same Apple M4 Pro (14 cores, 48 GB, macOS arm64) at
`59eef93` (the commit before the speed batch) and at `92f976a`, with 5 runs per
metric, the same seeds and the same machine. The baseline is a detached
worktree built by its own pinned scripts; the comparison is
[`history/2026-11-19-darwin-arm64-59eef93.json`](history/2026-11-19-darwin-arm64-59eef93.json)
against
[`history/2026-11-19-darwin-arm64-92f976a.json`](history/2026-11-19-darwin-arm64-92f976a.json).
`59eef93` predates the precompiled and incremental work, so those two rows only
exist on the current side.

| compiler start | 59eef93 | current |
| --- | ---: | ---: |
| module compile, no cache | 38,604 ms | 38,345 ms (0.99×) |
| Wasmtime cache load | 448.1 ms | 459.5 ms (1.03×) |
| precompiled `.cwasm` load | — | **17.7 ms** |

The speed batch is a clear compile-time win: the median program compiles in
**104.9 ms instead of 120.2 ms (0.87×)**, and every measured program improved
(0.83–0.98×, outside the run-to-run spread for 17 of them). The incremental
path (new in this batch) brings the same programs to 69–141 ms, 7–47% below
their own warm compile. Executing the compiled programs, the inliner shows up
where it was aimed: A.2 maths (e) **8.55 → 6.83 ms (1.25×)** and the libm
driver **3.33 → 2.76 ms (1.21×)**; the other kernels are within noise. Output
is smaller too: median total size 65,748 → 63,737 B (0.97×), with `math_e`
−6.6% and `libm_driver` −6.0% total and code size essentially unchanged. No
metric regressed beyond its run-to-run spread.

| program | warm compile (ms) | incremental (ms) | exec (ms) | total size (B) | code size (B) |
| --- | ---: | ---: | ---: | ---: | ---: |
| a_add | 90.8 → 75.2 (0.83×) | 69.1 | 0.0008 → 0.0007 (0.87×) | 35,180 → 34,622 (0.98×) | 12,089 → 12,059 (1.00×) |
| b_alloc | 122.6 → 104.9 (0.86×) | 86.0 | 0.0149 → 0.0154 (1.04×) | 82,230 → 80,699 (0.98×) | 41,643 → 41,363 (0.99×) |
| c_plugin | 97.0 → 80.5 (0.83×) | 74.0 | 0.0007 → 0.0008 (1.13×) | 36,853 → 36,295 (0.98×) | 13,205 → 13,175 (1.00×) |
| smoke | 126.1 → 106.6 (0.85×) | 85.0 | 0.0106 → 0.0117 (1.11×) | 65,748 → 64,638 (0.98×) | 30,932 → 30,902 (1.00×) |
| int_loops | 95.8 → 81.6 (0.85×) | 73.2 | 0.0031 → 0.0031 (1.00×) | 36,189 → 35,631 (0.98×) | 13,082 → 13,052 (1.00×) |
| float_sum | 112.6 → 99.9 (0.89×) | 81.5 | 0.0049 → 0.0045 (0.92×) | 51,003 → 50,335 (0.99×) | 24,167 → 24,137 (1.00×) |
| expr | 156.1 → 142.8 (0.91×) | 102.8 | 0.0411 → 0.0416 (1.01×) | 78,879 → 77,986 (0.99×) | 43,398 → 43,349 (1.00×) |
| json | 197.8 → 175.0 (0.88×) | 121.8 | 0.0297 → 0.0283 (0.95×) | 113,637 → 111,556 (0.98×) | 63,229 → 62,980 (1.00×) |
| collections | 281.6 → 264.9 (0.94×) | 141.2 | 0.0876 → 0.0880 (1.00×) | 249,521 → 243,982 (0.98×) | 146,684 → 146,029 (1.00×) |
| scene | 160.0 → 148.5 (0.93×) | 108.1 | 0.0085 → 0.0080 (0.95×) | 86,100 → 85,375 (0.99×) | 47,751 → 47,721 (1.00×) |
| nbody | 123.9 → 112.0 (0.90×) | 82.8 | 70.4032 → 70.5451 (1.00×) | 59,737 → 59,069 (0.99×) | 31,338 → 31,308 (1.00×) |
| mandelbrot | 94.6 → 81.4 (0.86×) | 70.2 | 7.3370 → 7.0487 (0.96×) | 36,783 → 36,225 (0.98×) | 13,498 → 13,468 (1.00×) |
| sha256 | 108.3 → 93.3 (0.86×) | 79.2 | 2.8287 → 2.7779 (0.98×) | 50,725 → 50,057 (0.99×) | 23,198 → 23,168 (1.00×) |
| sort | 128.3 → 112.6 (0.88×) | 87.3 | 11.8387 → 11.8971 (1.00×) | 69,931 → 68,709 (0.98×) | 33,835 → 33,786 (1.00×) |
| matmul | 108.9 → 93.4 (0.86×) | 77.1 | 1.7688 → 1.7860 (1.01×) | 49,750 → 49,082 (0.99×) | 22,878 → 22,848 (1.00×) |
| strings | 127.3 → 113.1 (0.89×) | 91.1 | 1.6890 → 1.4975 (0.89×) | 70,519 → 69,576 (0.99×) | 36,075 → 36,045 (1.00×) |
| math_e | 100.5 → 85.1 (0.85×) | 75.3 | 8.5498 → 6.8254 (0.80×) | 61,611 → 57,549 (0.93×) | 29,204 → 28,330 (0.97×) |
| hostcall_f | 88.5 → 77.6 (0.88×) | 69.8 | 0.5264 → 0.5096 (0.97×) | 36,063 → 35,505 (0.98×) | 12,485 → 12,455 (1.00×) |
| libm_driver | 96.4 → 85.3 (0.88×) | 76.5 | 3.3348 → 2.7643 (0.83×) | 67,802 → 63,737 (0.94×) | 34,975 → 34,101 (0.98×) |
| miniz_driver | 120.2 → 105.9 (0.88×) | 92.8 | 1.4413 → 1.4521 (1.01×) | 257,061 → 253,510 (0.99×) | 159,551 → 159,063 (1.00×) |
| generated_stress | 401.8 → 392.0 (0.98×) | — | 0.0644 → 0.0664 (1.03×) | 165,557 → 164,997 (1.00×) | 83,477 → 83,447 (1.00×) |

The last line of defence is correctness: all 21 programs returned the same
`run(seed)` values and output bytes in the baseline and current runs, and both
LLVM builds. The numbers above are therefore not a speed-up from a wrong
answer. Run-to-run spreads are in the JSON; `bench/compare.py` prints only the
changes larger than that spread.

## Gap attribution

[`gap/REPORT.md`](gap/REPORT.md) explains, per program, why our wasm is slower
and larger than LLVM `-O2`'s. `bench/gap/run.sh` reproduces it: it builds the
same modules as this suite, instruments every basic block with a counter,
executes the suite seeds, and writes `bench/gap/results/analysis.json` plus the
hot-function WAT. The MIR-optimisation experiment is included by default
(`--no-mir` skips it); it measured no execution win, so the report ranks the
backend SROA/inliner and the memory-intrinsic lowering above roadmap step 3.

## CI

* `.github/workflows/cranelift_wasm.yml` runs `bench/run.sh --quick --fuel`
  after the compiler build, then `bench/gap/count-quick.sh` on the same quick
  modules, uploads the JSON, writes the table to the job summary and publishes
  both files on the rolling `ci-logs` release. It uses the artifacts that job
  already built and never rebuilds the compiler.
* On a successful main run, the same job appends the quick result to the
  dedicated, append-only `bench-history` release as
  `<commit-date>-<sha7>.json` (the quick JSON plus commit sha, subject, date,
  run id and runner kind). A re-run replaces only its own commit's asset; the
  script never deletes one. It also backfills quick results that only exist on
  `ci-logs` (starting with `8476eeb`), then rebuilds
  [`chart.svg`](chart/chart.py) and `data.json` from every point and uploads
  them with stable names. The interactive chart is part of the project
  site's Benchmarks section: [`bench/chart/chart.js`](chart/chart.js) and
  [`chart.css`](chart/chart.css), copied by `site/build.py`, read
  `data.json`, which `deploy_pages.yml` downloads next to them. The README
  image links to that section.
* `.github/workflows/bench.yml` runs `--full` on a weekly schedule and on
  `workflow_dispatch` (optional `base` commit and `record_history` switch). It
  publishes the JSON + Markdown as one asset per run on the `bench-results`
  release and writes a comparison against the previous published full run (or
  the given base commit) to the job summary.

## Trend chart

[`bench/chart/chart.py`](chart/chart.py) reads the `bench-history` points and
writes `chart.svg` (the 800x420 README image) and `data.json` (the interactive
page). Both plots are per-commit, not per-time. Panel 1 is the geometric mean
of our execution time over LLVM `-O2` (programs whose LLVM time is under
0.1 ms excluded, the same floor for every commit), the geometric mean of
executed instructions over LLVM `-O2` on that same subset, and the geometric
mean of module size over LLVM `-O2`. Panel 2 is the median warm compile, median
incremental compile when present, precompiled compiler load when present, and
the median compiler instructions (Wasmtime fuel) on a right-hand axis. The
right axis keeps the compile story in one panel; a third small panel would
have made the fixed 800x420 README image cramped. The interactive chart uses
the same layout and shows the exact count in its tooltip.

Deterministic instruction lines are **solid**; wall-clock lines are **dotted**.
The legend says so once. Points without counts (everything published before
this change) simply have no value for those lines. A dashed vertical rule
marks a commit whose quick program subset differs from the previous one, so
changed subsets are visible instead of silently mixed.

Local check:

```sh
python3 bench/chart/chart.py \
  --history bench/chart/tests/fixtures/history --output /tmp/chart
python3 -m unittest discover -s bench/chart/tests -v
```

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
