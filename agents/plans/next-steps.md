# Next steps after the layout migration

Written 2026-11-21, after the layout migration landed on `main` (`81e1049`).
Ordered by priority. Each task has a size (S under a day, M a few days, L a
week or more), what it depends on, and when it is done. Goal numbers refer to
[`ROADMAP.md`](../ROADMAP.md); the migration record is in
[`layout-migration.md`](layout-migration.md).

## Now: land the migration safely

### N1. Watch the first cold CI run on the new layout (S)

`cranelift_wasm` #39 on `81e1049` is the first run with empty caches and the
first cold build from the new paths. It exercises what was not verified
locally: `ci/setup-toolchain.sh` reading `compiler/upstreams.toml`, a fresh
`setup-sources.sh`, and `rustc.wasm` built from source under `toolchain/`.

- [ ] If it fails, fix forward on `main`. Likely places: toolchain paths in
      `ci/setup-toolchain.sh`, the `build/host` stage1 path, cache `path:`
      entries, `release/gen-notices.py` manifest paths.
- [ ] Push a trivial change that touches a cached input's neighbour (for
      example a doc) and confirm run #40 restores the compiler cache (the
      log says "build reused").
- [ ] Record the cold and warm durations in `docs/notes/release.md` (CI section).
- [ ] Delete the old `cranelift-wasm-*-v1` compiler and source cache entries in
      Actions → Caches; they can never match again.

Done when: one cold and one warm green run on `main`.

### N2. Run the workflows the migration touched but did not run (S)

- [ ] `release.yml` with `dry_run: true`: bundle names changed
      (`cogwasm-host-<tag>-<host>`, `sysroot-<tag>.tar.gz`) and notices now
      come from `release/gen-notices.py --root`.
- [ ] `bench.yml` (manual): new toolchain and cache paths.
- [ ] `quality.yml` ran on the push; check it is green.
- [ ] Legacy LLVM workflows are manual-only: run `rustc_minimal.yml` once or
      record them as frozen in `legacy/llvm/README.md`.

## Next: finish the migration's loose ends

### M1. Decide and unify the drifted host helpers (M)

`cogwasm-host` and `bench-host` define the same helpers with different
behaviour, so only `Wt` and `SpawnSummary` moved into `cogwasm-runtime`. Each
needs a decision, then one shared implementation with a test that fails
before and passes after.

| Helper | Difference today | Decision to make |
| --- | --- | --- |
| `read_cstr` | host silently stops at 1 MiB; bench errors past 1 MiB | Error (a truncated path is a silent bug) |
| `median` | `(a+b)/2` vs `a+(b-a)/2` | One formula; check bench history is unaffected |
| `guest_memory` | `GuestMem` wrapper vs raw `Memory` | Use the wrapper everywhere |
| `new_store` / `prepare_instance` | bench adds captured stderr, fixed WASI randomness, fuel, limits | Shared builder with options; host keeps defaults |
| `spawn_impl` / `add_playground_spawn` | debug env var, log prefix, argv vs count | Shared impl, per-host recorder |
| `startup_once`, `prepare_source`, `main` | genuinely different CLIs | Leave separate |

- [ ] One PR per row, each with `bench/run.sh --quick` identical apart from
      timings and difftest green.

Unblocks: G2.1 (the supervisor needs a library API, not copies).

### M2. Put the remaining pins in the manifest (S)

`compiler/upstreams.toml` covers the compiler build. These still hard-code
`nightly-2026-10-19`, `1.99.1`, `50.0.2` or `wasi-sdk-34`: `bench/run.sh`,
`bench/run.py`, `bench/gap/*`, `difftest/*.sh`, `release/package.sh`,
`release/gen-notices.py`, the `env:` and cache keys in `cranelift_wasm.yml`,
`release.yml`, `bench.yml` and `quality.yml`.

- [ ] Read them through `compiler/scripts/upstream.py get`. In workflows, a
      first step that writes them to `$GITHUB_ENV`.
- [ ] Add to `upstream.py check`: the root `Cargo.toml` Wasmtime pin equals
      `upstream.wasmtime.version`.

Done when: `git grep nightly-2026-10-19` hits only the manifest and history.

### M3. Refresh or retire the upstream PR bundles (S)

All four bundles in `compiler/upstream/` differ from the live patches
(`upstream.py check` warns). Either regenerate each bundle's `.patch` from the
live series and re-run its `test-results.txt`, or mark it superseded in
`compiler/upstream/README.md`. Then make the warning a failure.

### M4. Decide where generated evidence lives (S, owner decision)

`bench/gap/results/` (about 130 WAT dumps plus `analysis.json`),
`docs/results/*.log` and `bench/history/*.json` are committed and grow with
every run. Options: keep; move the bulky parts to a release asset like
`bench-history` and rewrite the links in `bench/gap/REPORT.md`,
`docs/notes/item3-memory-grow.md` and `docs/notes/release.md`.

### M5. Remove the `STAGE_C_*` fallbacks (S, after one release)

Every `COGWASM_*` read still accepts `STAGE_C_*`. Remove the fallback in the
first release after the migration, and the note in `docs/compiler.md`.

### M6. Local housekeeping (S)

- [ ] `toolchain/stage-b/` (1.4 GB of stage-B scratch sysroots and work dirs)
      and `toolchain/target/stage-c-pure-plugins` are unused by any script.
      The disk has about 20 GB free.
- [ ] The next local `rustc.wasm` rebuild is likely a full one, because the
      toolchain moved (bootstrap records absolute paths).
- [ ] Make pushes use the account key: `git config core.sshCommand "ssh -i
      ~/.ssh/github_alex -o IdentitiesOnly=yes"` in this clone.

## Then: roadmap work, by leverage

### G2.1. Resident compiler service — migration plan P6 (L)

Goal 2: edit → result under 100 ms on a CI runner. Today the supervisor
shells out to `cogwasm-host raw` per job (`crates/cogwasm-supervisor/src/real.rs`),
paying the module load each time; the in-process path
(`wasmtime_runtime.rs`) only runs demo modules.

1. [x] Extract the compile path from `crates/cogwasm-host/src/main.rs`
       (`run_rustc_with`, `Opts`, spawn handling) into the `cogwasm-host`
       library, with the CLI as a thin wrapper. Depends on M1.
       Lives in `crates/cogwasm-host/src/compile.rs`, re-exported as `cogwasm_host::compile` (`CompileRequest`, `rustc_args`, `CompileOutcome`, `compile`, `Compiler`).
2. [x] A `Resident` backend in the supervisor that owns one `Engine`, the
       loaded (precompiled) `rustc.wasm` `Module` and the incremental cache,
       and instantiates per job.
       Lives in `crates/cogwasm-supervisor/src/resident.rs` (`ResidentCompiler`,
       selected with `--backend resident`).
3. [x] Golden test: two jobs in one process load the module once.
       Lives in `crates/cogwasm-supervisor/tests/resident.rs`
       (`two_jobs_share_one_loaded_module_and_survive_a_rejection`).
4. [ ] Measure edit → result on the CI runner (`bench` gains a resident
       mode); record against the 100 ms target.
       Measured by `cogwasm-supervisor edit-loop` (CI step in `ci/build-and-test.sh`; results in `docs/notes/supervisor.md`; CI number pending).
5. [ ] If instantiation dominates, this hands over to G2.2.

### G1.1. Finish step 5b: SROA / mem2reg after the inliner (L)

The cost-model inliner (patch `0006`) and the opt-in loop-field hoist
(`0005`) are in. Next is the half that 5a measured as the win.

1. [ ] Escape/alias analysis for locals whose address is passed to a callee
       that the inliner has removed.
2. [ ] SROA of non-escaping aggregates, then mem2reg; iterate with the
       inliner.
3. [ ] Make the loop-field hoist sound enough to be default-on, or drop it.
4. [ ] Acceptance: iterator-heavy benchmarks within 1.2× of LLVM `-O2`;
       difftest green; warm compile time within 5%.

### G1.2. Bench table in the README updated by CI (M, in progress)

The last open part of the benchmark row in Goal 1. `bench.yml` already
publishes results; add a generated table between markers in `README.md`
(or link the site page) and have CI open the update.

### Q1. Make the review scan a blocking gate (M)

- [ ] Install `clippy` for the pinned nightly in `ci/setup-toolchain.sh` and in
      `quality.yml` (it is skipped locally today).
- [ ] Fix findings one file at a time (the remaining ones are pedantic and
      `unwrap` findings in `cogwasm-host` and `difftest-host`, plus
      `allow-needs-expect` in the difftest corpus).
- [ ] Switch `review-scan.sh --strict` on in `quality.yml`.

### G2.2. Smaller `rustc.wasm` (M)

Drop Clippy, rustfmt and unused targets from `compiler/bootstrap.toml`.
Measure size, first load, cache load and `.cwasm` size before and after.
Cheap, and it helps every load path including G2.1.

### G4.1. Reproducible `rustc.wasm` (M)

Object files embed the source path (`docs/notes/supervisor.md`), and the
toolchain now lives at a different absolute path on every machine. Add
`--remap-path-prefix` for the toolchain and repo roots, build twice in
different directories, and compare. This also stabilises the supervisor's
cache keys.

### G3.1. Procedural macros (L, research spike first)

The largest ecosystem gap (`serde` derive). Spike: compile one proc-macro
crate to wasm, run it in a host-provided Wasmtime instance from
`rustc.wasm`'s proc-macro bridge, and expand one derive. Write the result up
in `docs/notes/` before committing to a design.

### Q2. Fuzzing and wider test runs (M)

`difftest` already generates 24 deterministic random programs. Make the seed
rotate per CI run, record failing seeds in `difftest/FINDINGS.md`, then start
running rust-lang's run-pass tests through the pipeline (pass rate tracked).

## Suggested order

1. N1, N2: confirm the migration in CI (this week).
2. M2, M3, M6, then M1: small, and M1 unblocks G2.1.
3. G2.1 and G1.1 in parallel: they touch different code (supervisor/host vs
   rustc MIR patches).
4. Q1, G2.2, G1.2 as filler work between the big items.
5. G4.1, then the G3.1 spike.
