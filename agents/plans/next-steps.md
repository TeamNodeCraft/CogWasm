# Next steps after the layout migration

Written 2026-11-21, after the layout migration landed on `main` (`81e1049`).
Last updated 2026-11-22: N1 and M1–M3 done, G2.1 done, G2.3 in progress.
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

- [x] If it fails, fix forward on `main`. Fixed forward in four commits:
      `toolchain/` excluded from the root workspace (`6858879`), cg_clif's
      clif-to-wasm path computed (`33e614e`), a frozen guest clock for fuel
      samples (`73ea336`), setup run when a reused compiler has no cargo
      (`50aa738`). The cold path passed build and all checks in run #41.
- [x] Confirm a later run restores the compiler cache: run 44898130401
      (`50aa738`) and run 44912655095 (`6934ed5`) both logged "build reused"
      and were green; all caches are now saved.
- [ ] Record the cold and warm durations in `docs/notes/release.md` (CI section).
      Known so far: warm (compiler reused) run 44912655095 took 35 min wall,
      986 s inside `build-and-test.sh`.
- [ ] Delete the old `cranelift-wasm-*-v1` compiler and source cache entries in
      Actions → Caches; they can never match again.

Done when: one cold and one warm green run on `main`. **Met** (remaining
boxes are bookkeeping).

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

- [x] Done in `cba2435` with the decisions in the table: `read_cstr` errors
      past 1 MiB, `median` uses `a+(b-a)/2` with `total_cmp`, `GuestMem`
      everywhere, `prepare_instance` with `StoreOptions`, one spawn
      implementation with per-host recorders. Bench and difftest unchanged.

Unblocks: G2.1 (the supervisor needs a library API, not copies).

### M2. Put the remaining pins in the manifest (S)

`compiler/upstreams.toml` covers the compiler build. These still hard-code
`nightly-2026-10-19`, `1.99.1`, `50.0.2` or `wasi-sdk-34`: `bench/run.sh`,
`bench/run.py`, `bench/gap/*`, `difftest/*.sh`, `release/package.sh`,
`release/gen-notices.py`, the `env:` and cache keys in `cranelift_wasm.yml`,
`release.yml`, `bench.yml` and `quality.yml`.

- [x] Read them through `compiler/scripts/upstream.py get`. In workflows, a
      first step that writes them to `$GITHUB_ENV`. (`3b8d675`)
- [x] Check that the root `Cargo.toml` Wasmtime pin equals
      `upstream.wasmtime.version`: in `compiler/scripts/check-upstreams.py`
      (kept out of `upstream.py` so the compiler cache key did not change).

Done when: `git grep nightly-2026-10-19` hits only the manifest and history.

### M3. Refresh or retire the upstream PR bundles (S)

All four bundles in `compiler/upstream/` differ from the live patches
(`upstream.py check` warns). Either regenerate each bundle's `.patch` from the
live series and re-run its `test-results.txt`, or mark it superseded in
`compiler/upstream/README.md`. Then make the warning a failure.

**Done** (`3b8d675`): the bundles are deliberate refreshed-for-upstream
versions, not stale copies. Each has a `DIVERGES.md`, the README has a table,
and `check-upstreams.py` fails on a differing bundle without one.

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
- [x] Make pushes use the account key: `core.sshCommand` is set in this
      clone.

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
4. [x] Measure edit → result on the CI runner (`bench` gains a resident
       mode); record against the 100 ms target.
       `cogwasm-supervisor edit-loop`, run by `ci/build-and-test.sh` on every
       push. CI run 44912655095: median 174.1 ms, **target missed**; local
       macOS arm64 41 ms. Breakdown in `docs/notes/supervisor.md`.
5. [x] If instantiation dominates, this hands over to G2.2.
       It does not: instantiation 0.2 ms, swap 0.8 ms, link 4 ms; ~170 ms is
       rustc inside the guest. Next for Goal 2 is cutting rustc.wasm's
       per-invocation set-up (snapshot after session/prelude load), not G2.2.

### G2.3. Warm rustc session across compiles (L, design first)

Profiling (`docs/notes/supervisor.md`, "Fixed versus per-source cost") shows
~90% of a small compile is fixed per invocation: ≈ 19 ms loading sysroot
crate metadata, ≈ 8 ms session floor, ≈ 8 ms link (macOS; ×4.2 on the CI
runner, where the edit is 174 ms against the 100 ms target).

1. [x] Design note: how a guest keeps one session's crate store warm across
       compiles (driver loop over WASI vs. snapshotting an instance after
       sysroot load; what query/incremental state must be reset per request).
       Drafted in `docs/notes/warm-session.md`: options A (long-lived guest +
       metadata cache), C (warm session template), D (cheaper fixed work,
       in-process Wild); experiments E1–E4 decide between them.
1a. [x] E1: the rmeta read is ≈ 7 ms of ≈ 19 ms crate loading (≈ 80% of
        `core`'s expansion cost); half of it, and an estimated ~10 ms over the
        whole compile, is first-touch of fresh guest memory.
1b. [x] A0: Wasmtime pooling allocator (host-only), opt-in `COGWASM_POOLING=1`.
        macOS: no gain (40 ms default vs 41–45 ms pooled). **Linux CI: 131.3 →
        96.5 ms median (−27%), under the 100 ms target** (run 44969263935,
        keep resident 1 GiB).
1d. [x] A0 confirmed on CI (run 44972190861, interleaved, two sources):
        keeping 256 MiB resident saves ≈ 31–36 ms per job (fixture 125 → 93 ms,
        `json` 539 → 503 ms); pooling with nothing resident saves nothing.
        Now the resident backend's default (`Allocation::RESIDENT_DEFAULT`);
        temporary CI step removed, `ci/edit-loop-ab.sh` kept for reruns.
1c. [x] E2 and E4 on the runner. E4: expansion + resolution were 146 of
        166 ms; E1 on the runner: the rmeta read 71 ms. Cause: WASI reads
        through tokio's blocking pool with a fresh host buffer per read.
        Fixed by `allow_blocking_current_thread` (`833d995`): **CI edit →
        result 134.9 → 53.6 ms**, plugin (c) incremental ≈ 300 → 120–130 ms.
        E2: 143 MiB peak guest memory; restore 8–19 ms into resident memory.
2. [ ] Option A (long-lived guest keeping sysroot metadata) for the
       remaining ≈ 40 ms of crate loading and session set-up; measure on a
       plugin-sized source (plugin (c), `json`), where CI is still over 100 ms.
       Phase 1 done (`0007-wasm-serve-loop.patch`, `ServeSession`,
       `--verify-cold`): **CI edit → result 53.6 → 23.6 ms** (run
       44997648351), byte-identical to cold compiles; locally −11 to −14 ms
       per job. Plugin-sized sources (`ci/edit-loop-sources.sh`, four
       medium programs, in CI): serving saves a fixed ≈ 15–17 ms locally
       (−4 to −9%), 10–27 ms on CI (run 45000755486: collections 464, expr
       220, json 304, scene 232 ms per edit); found and fixed address-keyed hash caches leaking across
       sessions. Next for these sizes: per-source compile time (G1/G2.2) and
       the output module's native compile in the swap (≈ 57 ms for a 250 KB
       module).
3. [x] Acceptance for the small edit: CI edit → result under 100 ms (53.6 ms,
       run 44978461782); difftest green. Plugin-sized edits are the next bar.

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
