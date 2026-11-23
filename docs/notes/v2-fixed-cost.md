# V2 fixed-cost profile: where one warm edit goes

Status: **M1 complete; gate green.** This note profiles one warm one-function
edit of the 98k-line generated program in the resident serving guest with
`-Zself-profile`. It is the input to the phase 2 work: the numbers say the
fixed cost is **incremental compilation machinery** (51% of the edit), not
code generation (2%) and not crate metadata (0.1%).

Measurements are from a local macOS arm64 machine (14 cores, shared; the
baseline run recorded load average 7.51), 23 November 2026. The generated
program is not checked in; `cogwasm-bench` regenerates it.

## Method

`cogwasm-bench profile --suite large` does what the edit loop does: an
incremental seed compile of the prepared large source, one unprofiled edit,
then a second edit with `-Zself-profile=<dir>`. It swaps each artifact into a
live instance and calls `run(seed)`, so the measured edit -> result includes
the Wasmtime swap and the call. One further cheap request after the profiled
edit retires the profiler session; without it the serving guest keeps the
session alive and measureme never flushes its string table (the first attempt
produced an event-only `*.mm_profdata` that `analyzeme` rejects). The tool
then reads the raw events with `analyzeme` 12.0.3 (the same measureme version
rustc 1.101 links) and adds up each event's self time.

Buckets are assigned by event **kind** first and label second: a
`lower_to_hir` event of kind `IncrementalResultHashing` is incremental work,
not front-end work. The buckets plus the measured swap, call and the host time
outside rustc's instrumented events must sum to the measured edit -> result.
The gate is a 5% difference; `profile.json` carries the exact numbers.

`-Zself-profile` needs a process id to name its file, and wasm32-wasip1 has
none: `std::process::id()` used to panic inside measureme. Patch
`0010-wasm-placeholder-process-id` (opt-in only through using the profiler)
returns 1 on wasm; every other platform keeps the panic. It changes only the
compiler's own WASI std, not the `wasm32-unknown-unknown` sysroot that user
programs link.

## Baseline before the profile

`cargo run --release -p cogwasm-bench -- run --suite large --runs 3 --no-fuel`
(`toolchain/work/v2/m1/baseline/result.json`):

| metric | value |
| --- | ---: |
| edit -> result p50 | 2,207.0 ms |
| edit -> result p90 | 2,311.5 ms |
| warm compile median | 16,570.5 ms |
| cold compile | 17,704.6 ms |
| output | 12,754,344 B |

The profiled edit measured 2,498.8 ms and 2,518.4 ms in two runs; the
unprofiled edit in the same run measured 2,256.3 ms. The profiler adds about
11% instrumentation overhead. The bucket shares below are stable across the
two runs to within one percentage point.

## The fixed cost of one warm edit

Two runs, median and range in ms; shares are of the measured edit.

| bucket | median ms | range | share |
| --- | ---: | ---: | ---: |
| incremental save/load (hashing, dep graph, query cache) | 1,285.9 | 1,279.5-1,292.2 | **51.2%** |
| front end (parse, expand, resolve, typeck, borrowck) | 578.6 | 576.2-581.0 | 23.1% |
| swap (Wasmtime compile + instantiate) | 201.1 | 200.7-201.5 | 8.0% |
| monomorphisation / partitioning | 197.6 | 196.2-199.0 | 7.9% |
| query overhead / other rustc work | 141.2 | 141.0-141.4 | 5.6% |
| codegen | 43.6 | 43.5-43.8 | 1.7% |
| link (rustc activity; Wild is a separate 19-21 ms spawn) | 29.7 | 28.4-30.9 | 1.2% |
| unprofiled host/rustc time | 17.8 | 17.4-18.1 | 0.7% |
| call | 9.9 | 9.7-10.1 | 0.4% |
| crate metadata / rmeta | 3.2 | 3.2-3.3 | 0.1% |

Gate: for both runs the buckets sum to the measured edit exactly (0.00%
difference; the only floor is that the unprofiled remainder is floored at
zero). **PASS.**

## Top 10 activities by self time

| # | activity | kind | self ms | total ms | calls |
| ---: | --- | --- | ---: | ---: | ---: |
| 1 | `lower_to_hir` | IncrementalResultHashing | 239.7 | 345.1 | 15,954 |
| 2 | `parse_crate` | GenericActivity | 209.8 | 209.8 | 1 |
| 3 | `incr_comp_encode_dep_graph` | GenericActivity | 175.8 | 175.8 | 1,229,572 |
| 4 | `encode_query_results_for` | GenericActivity | 154.3 | 154.3 | 58 |
| 5 | `typeck_root` | IncrementalResultHashing | 133.3 | 133.3 | 1 |
| 6 | `late_resolve_crate` | GenericActivity | 131.7 | 133.0 | 1 |
| 7 | `<unknown>` | IncrementalResultHashing | 128.6 | 128.6 | 0 |
| 8 | `monomorphization_collector_graph_walk` | GenericActivity | 116.3 | 225.0 | 1 |
| 9 | `expand_crate` | GenericActivity | 66.4 | 79.7 | 1 |
| 10 | `cgu_partitioning_place_items` | GenericActivity | 65.8 | 94.4 | 1 |

`lower_to_hir`, `typeck_root` and `<unknown>` are all
`IncrementalResultHashing`: rustc is hashing query results to key the dep
graph. `incr_comp_encode_dep_graph` writes the new dep graph, and
`encode_query_results_for` writes the query result cache. Hashing alone is
about 0.50 s; encoding the two files is about 0.33 s; the remaining
incremental time is save/load plumbing, query-cache promotion and work-product
copying.

The front end's real work is parse (0.21 s), late resolution (0.13 s), macro
expansion (0.08 s), type checking and the rest. It is not the same 1.06 s the
first draft reported: that draft classified by label alone and moved the
0.50 s of result hashing into "front end".

## What this means for phase 2

1. **Incremental state is the lever.** 1.29 s per edit is rustc hashing
   results, encoding the dep graph and query cache, and loading them back.
   Keeping that state alive in the resident guest is the only candidate that
   can remove a large share of the 2.5 s. It cannot remove all of it: the
   result hashing is part of deciding what is dirty, so an in-memory
   serialization/load path should be expected to recover the encode/load half
   (roughly 0.4-0.6 s here), not the 0.5 s of hashing.
2. **Crate metadata is already gone.** `needs_metadata` is false for a
   `cdylib`, so the "skip rmeta a binary never needs" candidate is worth 3 ms,
   not 0.5 s. It is not worth a patch.
3. **Codegen is not the one-function problem.** Optimising the 98 rebuilt
   bodies costs 44 ms. Function granularity matters for the signature edit
   (2.47 s of optimisation in one shared CGU) and for the link, not for this
   edit's fixed cost. Phase 1 targets the link/object side, not this number.
4. **The swap is 201 ms and scales with output size.** It is the second
   largest single line after incremental. A smaller module (or a stitcher that
   avoids rebuilding it) is worth more on the large program than a codegen
   change.
5. **Monomorphisation and partitioning are 198 ms.** A partitioner that keeps
   shared generics in coarser units must not add much here; the current walk
   is already 116 ms self.

## Reproduce

```sh
export PATH="$PWD/toolchain/cargo/bin:$PATH"
export CARGO_HOME="$PWD/toolchain/cargo" RUSTUP_HOME="$PWD/toolchain/rustup"

cargo run --release -p cogwasm-bench -- run --suite large --runs 3 --no-fuel \
    --out toolchain/work/v2/m1/baseline
cargo run --release -p cogwasm-bench -- profile --suite large \
    --out toolchain/work/v2/m1/profile-large2
cat toolchain/work/v2/m1/profile-large2/profile.md
```

Raw profile data is under `toolchain/work/v2/m1/profile-large2/work/self-profile/`
(git-ignored; about 1.6 MB per run).

## Files changed

- `crates/cogwasm-bench/src/profile.rs` (new): the `profile` subcommand, its
  bucket/gate summary and unit tests.
- `crates/cogwasm-bench/src/main.rs`, `src/build.rs`, `Cargo.toml`: the
  subcommand, the shared artifact helper and `analyzeme`.
- `compiler/patches/rust/0010-wasm-placeholder-process-id.patch` (new),
  `compiler/upstreams.toml` (lists it after `0009`),
  `toolchain/rust/.cogwasm-applied-patches` (its SHA-256).
- `toolchain/rust/library/std/src/sys/process/unsupported.rs` (the wasm
  placeholder).
- `docs/notes/v2-fixed-cost.md` (this note).

## M2 result: in-memory incremental artifacts (negative)

Status: **implemented behind `-Zcogwasm-incremental-memory`, correct, but a
negative result for the fixed-cost target.** The flag keeps the dep graph,
query cache and work-product index in the serving guest's memory instead of
writing and re-reading them under the incremental directory. It removes the
file I/O but not the work the profile actually found: rustc still hashes every
query result and still encodes the dep graph and the query cache. The measured
win is about 2-3%, below the 5% this milestone wanted, so the flag stays
opt-in and nothing about the default pipeline changes.

### What the flag does

- `rustc_serialize` gains a memory-backed `FileEncoder`. It publishes its
  finished bytes in a process-local map keyed by the incremental crate
  directory and file name, not by the per-session directory rustc renames
  after every compile.
- `rustc_incremental` asks for a memory encoder for the dep graph, query
  cache and work-product index when the flag is on. `open_incremental_file`
  takes those bytes instead of opening the file; if there is no memory entry
  it falls back to the normal file path.
- The dep graph's staging-to-final rename happens before its encoder finishes.
  `memory_files` records that pending rename and publishes the bytes at the
  final key.
- Work-product object files stay on disk. Only the index is in memory.
- The flag is `[UNTRACKED]` and is never passed by the default pipeline, the
  large/plugin benchmark defaults, the sysroot build or difftest.

### Gates

- `cogwasm-supervisor edit-loop --verify-cold --incremental-memory`: passed
  on all four plugin programs and on the large program; every warm edit was
  byte-identical to a fresh cold compile of the same source.
- `difftest/run.sh --quick`: **PASS** flag off (60 s) and **PASS** with
  `DIFFTEST_INCREMENTAL_MEMORY=1` (68 s), 56 handwritten + 8 generated
  programs, zero mismatches.
- Default pipeline unchanged: the large program still emits **7,092,923 B**
  and executes **318,573,438** instructions in the default config, exactly
  the M1 baseline numbers.
- The `toolchain/sysroot` was not rebuilt or changed.

### Measured effect

`cogwasm-bench run`, 5 edits per program for the plugin suite and 3 for the
large program, flag off vs on. The machine was heavily loaded (load average
20-32 on 14 cores), so these are p50s with spread, not a quiet-machine claim.

| program | edit p50 off | edit p50 on | change |
| --- | ---: | ---: | ---: |
| `collections` | 111.6 ms | 107.9 ms | -3.4% |
| `expr` | 76.3 ms | 75.3 ms | -1.4% |
| `json` | 91.8 ms | 89.5 ms | -2.5% |
| `scene` | 76.7 ms | 75.1 ms | -2.1% |
| plugin geomean | 84.3 ms | 82.4 ms | **-2.2%** |
| large | 2,215.0 ms | 2,152.3 ms | **-2.8%** |

The profile predicted that an in-memory serialization/load path could recover
the encode/load half of the 1.28 s incremental bucket, roughly 0.4-0.6 s. The
measurement says the file I/O itself was only about 60 ms of that on the large
program; the hashing (`lower_to_hir`, `typeck_root`, `<unknown>`, about 0.50 s)
and encoding (`incr_comp_encode_dep_graph`, `encode_query_results_for`, about
0.33 s) dominate and remain. Removing those needs a different design: retain
the previous session's `OnDiskCache`/`SerializedDepGraph` objects rather than
re-encode them, which is a query-system change, not a file-backend change.

### Decision

Keep the flag as a measurement tool. Do not make it default. The next lever
with a measurable payoff on this program is phase 1's CGU/link work (M3), not
another incremental file backend.

### Files changed in M2

- `compiler/rustc_serialize/src/{lib,opaque,memory_files}.rs`,
  `compiler/rustc_data_structures/src/memmap.rs`,
  `compiler/rustc_incremental/src/persist/{file_format,load,save}.rs`,
  `compiler/rustc_session/src/options.rs`.
- `compiler/patches/rust/0011-wasm-incremental-memory.patch` (new),
  `compiler/upstreams.toml` (lists it after `0010`),
  `toolchain/rust/.cogwasm-applied-patches`.
- `crates/cogwasm-supervisor/src/{job,main}.rs` (the opt-in flag),
  `crates/cogwasm-bench/src/build.rs` (`incremental` and
  `incremental-memory` configs), `difftest/run.sh` (the opt-in
  `DIFFTEST_INCREMENTAL_MEMORY` path).
