# V2 fixed-cost profile: where one warm edit goes

Status: **M1 complete; gate green.** This note profiles one warm one-function
edit of the 98k-line generated program in the resident serving guest with
`-Zself-profile`. It is the input to the phase 2 work: the numbers say the
fixed cost is **incremental compilation machinery** (51% of the edit), not
code generation (2%) and not crate metadata (0.1%).

Measurements are from a local macOS arm64 machine (14 cores, shared; the
baseline run recorded load average 7.51), 23 November 2026. The generated
program is not checked in; `cogwasm-bench` regenerates it.

> The profiler, its summary and this note are LLM-generated under the Rust
> project's personal-use exemption. This is a local CogWasm experiment, not an
> upstream contribution; a human must review and own it before it goes
> anywhere else.

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
