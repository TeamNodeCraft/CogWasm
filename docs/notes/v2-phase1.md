# V2 phase 1: local-function codegen units

Status: **M3 negative.** Splitting only local user functions into their own
codegen units and capping the total with rustc's existing merge does exactly
what phase 0 asked at the CGU level: the large program goes from 153 CGUs to a
capped 256, and a one-function edit rebuilds one CGU in both configurations.
It is nevertheless **slower** in the product, because the rewritten CGU is a
shared/volatile one holding 43 inlined bodies that the local split never
touches, while 256 work products, more linker arguments and a larger module
add fixed cost. The flag stays opt-in and the default pipeline is unchanged.

Measurements are from the local macOS arm64 machine on 23 November 2026. The
machine was heavily loaded (load average 28-47 on 14 cores); the plugin and
large timings are p50s with that caveat, and the deterministic size,
instruction and CGU counts are the stronger evidence.

## What the flag does

`compiler/patches/rust/0012-wasm-local-cgu.patch` adds:

- `-Zcogwasm-cgu-local`: in incremental builds, a `GloballyShared`
  `MonoItem::Fn` whose `DefId` is in the local crate gets a CGU named after
  its mangled symbol. Instances of shared generics (`core`, `alloc`, other
  crates) keep their existing characteristic CGU, and inlined (`LocalCopy`)
  bodies stay attached to every root that uses them, exactly as in V1.
- `-Zcogwasm-cgu-local-cap=N`: the merge pass that rustc already has caps the
  total CGU count. The default is `-Ccodegen-units` (256 for an incremental
  build), so the linker never sees one object per function.

The full one-function-per-CGU experiment, `-Zcogwasm-cgu-per-fn` (patch
0009), is unchanged and remains the measurement that showed 7,661 CGUs and a
failed large-program link. The new flag is the phase-0 recommendation's
"local only, capped" form.

## CGU counts (native stage-1 compiler)

The partitioner is target independent, so the counts come from the stage-1
host compiler with `--target wasm32-wasip1 --crate-type lib`, an incremental
base compile and the same one-function edit phase 0 used
(`m064::hot_edit`), with `-Zprint-mono-items -Zquery-dep-graph
-Zincremental-info`:

| configuration | CGUs total | CGUs rebuilt | distinct bodies in rebuilt CGUs |
| --- | ---: | ---: | ---: |
| default | 153 | 1 | 53 |
| `-Zcogwasm-cgu-local` (cap 256) | 256 | 1 | 43 |

The split works at the root-function level, but the edit's dirty set still
contains one shared/volatile CGU with 43 inlined copies. That CGU, not the
changed function's own CGU, is where the rebuild time goes. A partitioner
cannot see "which bodies changed" before it partitions, so it cannot give that
CGU the changed-body treatment without the function store this milestone does
not build.

## Product edit loop

`cogwasm-supervisor edit-loop --verify-cold`, and `cogwasm-bench run` with the
same source, flag off vs `--cgu-local`:

| measurement | incremental | cgu-local | change |
| --- | ---: | ---: | ---: |
| large edit -> result p50 | 2,609.5 ms | 4,028.4 ms | **+54.4%** |
| large build p50 | 2,378.2 ms | 3,825.4 ms | +60.9% |
| large link p50 | 123.9 ms | 47.6 ms | link args 180 -> 283 |
| large swap p50 | 221.3 ms | 192.9 ms | module 12,710,511 -> 12,860,878 B |
| large executed instructions | 420,090,882 | 420,090,882 | unchanged |
| plugin edit p50 geomean | 87.8 ms | 107.7 ms | **+22.7%** |
| plugin size geomean | 4.15x O2 | 4.43x O2 | +2-13% per program |
| plugin executed instructions | unchanged | unchanged | unchanged |

The large program **links and runs** with the flag: Wild receives 283
arguments (under the host limit), the module instantiates, `run(seed)`
returns the expected value, and every warm edit is byte-identical to a fresh
cold compile. The plugin programs pass the same check.

The one lower line, large link p50, is not a win: the link is shorter than the
incremental baseline in this run, but the total is 1.4 s worse because the
build and the extra CGU bookkeeping dominate. Plugin link medians are 7.9 ms
(cgu-local) against 6.5-7.2 ms (incremental).

## Gates

- `edit-loop --verify-cold --cgu-local`: passed on the large program and on
  `json`; warm and cold bytes identical.
- `difftest/run.sh --quick`: **PASS** flag off (53 s) and **PASS** with
  `DIFFTEST_CGU_LOCAL=1` (51 s), 56 handwritten + 8 generated programs, zero
  mismatches.
- Default pipeline unchanged: the large default output is still
  **7,092,923 B** and **318,573,438** executed instructions, exactly the M1
  baseline. The sysroot was not rebuilt.
- Large program with the flag: links (283 arguments) and runs; module
  12,860,878 B; instructions 420,090,882 over the four seeds.

## Why it is negative, and what phase 1 actually needs

1. **The dirty CGU is not a local root CGU.** The one rewritten CGU exists
   because rustc puts the reachable inlined copies into a shared/volatile
   unit; splitting local roots does not change its contents. The 53 -> 43
   body change is too small to matter.
2. **CGU count is a cost, not a free cache key.** 256 objects mean 256 work
   products to copy, 283 linker arguments and a larger module to compile for
   the swap. Phase 0 measured the same effect at 7,661 objects; 256 only makes
   it smaller, not free.
3. **Cross-function inlining gets worse.** The output is 2-13% larger on the
   plugin suite at identical executed instructions, which is exactly the
   inlining loss phase 0 saw in the full per-function experiment.
4. **A different mechanism is required.** The plan's copy-and-patch stitcher
   and function store bypass work products and the object linker entirely;
   that is the only way to get per-function granularity without paying the
   256-object fixed cost. Rustc's CGU machinery cannot do it, and this
   milestone confirms that locally-capped CGUs do not either.

## Files changed

- `compiler/patches/rust/0012-wasm-local-cgu.patch` (new),
  `compiler/upstreams.toml` (lists it after `0011`),
  `toolchain/rust/.cogwasm-applied-patches`.
- `toolchain/rust/compiler/rustc_session/src/options.rs` (the two options),
  `toolchain/rust/compiler/rustc_monomorphize/src/partitioning.rs` (local
  split and cap).
- `crates/cogwasm-supervisor/src/{job,main}.rs` (`--cgu-local`,
  `--cgu-local-cap`), `crates/cogwasm-bench/src/build.rs` (`cgu-local`
  config), `difftest/run.sh` (`DIFFTEST_CGU_LOCAL`).
- `docs/notes/v2-phase1.md` (this note), `agents/plans/v2.md` (measured V2
  numbers), `bench/README.md` (the config list).
