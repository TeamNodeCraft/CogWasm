# Layout migration plan

Move CogWasm's shipped product out of `experiments/cranelift-wasm/` into a
first-class top-level layout, extract the duplicated host code, consolidate the
toolchain and Cargo workspaces, and retire the stage-A/B scaffolding. The
legacy LLVM path is kept under `legacy/llvm/`. One reviewable PR per phase;
CI stays green throughout.

## Goal and non-goals

**Goal.** The repo reads as a product, not an experiment. One host workspace,
one Wasmtime pin, one patch manifest, one ignored toolchain root, and no
committed build output. Same behaviour, same codegen.

**Non-goals.**

- No behaviour change. Same `rustc.wasm`, same sysroot, same difftest/bench
  results. Every phase is a move or a behaviour-preserving refactor.
- No codegen changes and no new compiler features.
- No change to what CI proves (LLVM audit, difftest, A.2 checks, bench).
- The legacy LLVM path is **not** deleted here; it is moved to `legacy/llvm/`
  or removed only by an explicit owner decision (open question).

## Current state (verified)

| # | Problem |
| --- | --- |
| 1 | Product lives under `experiments/cranelift-wasm/stage-c/`; top-level `patches/`, `config_rewriter/`, `musl*.sh`, `crosstool-ng-build.sh`, `build-{netbsd,solaris}-toolchain.sh`, `*.defconfig`, `ci/apply-rust-patches.sh` belong to the legacy LLVM path (`build_llvm.yml`, `rustc_minimal.yml`, `rustc_llvm_with_lld.yml`). |
| 2 | Stage B is load-bearing: `build-rustc-wasm.sh` sets `WORK=$TOOL/stage-b`; `STAGE_C_STAGE1` defaults to `$TOOL/stage-b/rust/build/host/stage1/bin/rustc`; `run.sh` (lines 70, 182) `cd`s into `stage-b/a2/harness`; `ci/build-and-test.sh:34`; `release/gen-notices.py` reads `toolchain/stage-b/{rust,wasi-sdk}` and `stage-b/a2/harness/Cargo.toml`; three workflows hash `stage-b/a2/harness/Cargo.lock` into cache keys. Stage A leftovers: `programs/{a_add,b_alloc,c_plugin,d_hello_std,host}` + `run.sh`; copies in `stage-c/programs` and `stage-b/a2` have diverged. |
| 3 | Host code is duplicated. `stage-c/host/src/main.rs` (1600 lines), `bench/host/src/main.rs` (1718) and `stage-b/host/src/main.rs` (1069) each define `new_store`, `prepare_instance`, `guest_memory`, `read_cstr`, `add_playground_spawn`, `spawn_impl`, `startup_once`, `median`. `difftest/host` is lean (513 lines) and does **not** duplicate them. `stage-c/host/src/lib.rs` exports only `config` + `precompiled`; `bench/host` depends on it by the cross-tree path `../../experiments/cranelift-wasm/stage-c/host`. `supervisor/src/real.rs` shells out to `stage-c-host raw`, reloading the compiler per job (against Goal 2). |
| 4 | 23 standalone Cargo crates (27 tracked `Cargo.toml` minus 4 under `bench/vendor/`), no workspace. Wasmtime pins drift: `=50.0.2` (`stage-c/host`, `bench/host`, `difftest/host`, `bench/gap`, `stage-c/supervisor`); `50.0.1` non-exact (`c_plugin_harness`, `mem_grow_harness`, `stage-b/a2/harness`, `programs/host`); `=46.0.3` (`stage-b/host`, plus `wasmtime-wasi-threads`). `.cwasm` is tied to the exact Wasmtime version. Guest programs built for `wasm32-unknown-unknown` must not join the host workspace. |
| 5 | Patches in four places: `patches/rust-1.99.1/` + `patches/llvm-22.1.0-wasip1/` (legacy), `experiments/cranelift-wasm/patches/0001-cg_clif-wasm-backend.patch` (check if still applied), `stage-c/patches/{rust,cg_clif,clif-to-wasm,wild}` (live, 16 files), `upstream/` (4 PR bundles duplicating 4 live patches). Pinned revisions live only in prose in `experiments/cranelift-wasm/README.md`. |
| 6 | Generated output committed: `bench/gap/results/` (135 files; `analysis.json` 36 260 lines), `stage-c/results/*.log`, `bench/history/*.json` (3 × ~3.9k lines; a `bench-history` release already exists), `tree.txt` (9 279-line Android NDK listing). |
| 7 | `docs/update-1.99.1/` is legacy LLVM rebase notes. |
| 8 | Ignored `experiments/cranelift-wasm/toolchain/` mixes toolchains, upstream clones, sysroots, `work*/` dirs and loose `build-*.log`. |

## Target tree

```
compiler/                        # build rustc.wasm + pure sysroot (the product build)
├── bootstrap.toml               # wasm32-wasip1 build config
├── upstreams.toml               # machine-readable pins: repo, rev, ordered patch list
├── run.sh                       # plugins / check / a2check / incremental (today stage-c/run.sh)
├── patches/
│   ├── rust/                    # 6 live rust patches
│   ├── cg_clif/                 # 5 live cg_clif patches (+ stage-A patch if still applied)
│   ├── clif-to-wasm/            # 4 live clif-to-wasm patches
│   └── wild/                    # 1 live wild patch
└── scripts/
    ├── build-rustc-wasm.sh      # apply patches + x.py install
    ├── build-pure-sysroot.sh    # core/alloc/compiler_builtins via rustc.wasm
    ├── build-host.sh            # build the Wasmtime host
    ├── rustc-wasm-wrapper.sh    # Cargo RUSTC wrapper
    └── run-item3.sh             # memory.grow regression suite
crates/                          # host workspace: one Cargo.toml, one Wasmtime pin
├── Cargo.toml                   # [workspace] members + [workspace.dependencies]
├── cogwasm-runtime/             # extracted Wasmtime helpers (new_store, prepare_instance,
│                                #   guest_memory, read_cstr, add_playground_spawn, spawn_impl,
│                                #   startup_once, median)
├── cogwasm-host/                # CLI = today's stage-c-host (lib.rs config/precompiled + main.rs)
├── cogwasm-precompile/          # bin = today's stage-c/host/src/bin/precompile.rs
├── cogwasm-supervisor/          # today's stage-c/supervisor
├── difftest-host/               # today's difftest/host (lean; optional runtime later)
├── bench-host/                  # today's bench/host
└── gap-tool/                    # today's bench/gap Rust crate
guests/                          # wasm32-unknown-unknown programs + harnesses (NOT in host workspace)
├── a_add/ b_alloc/ c_plugin/ e_math/ f_geometry/ mem_grow/   # guest programs
├── c_plugin_harness/ mem_grow_harness/ a2-harness/           # host-side harnesses
└── run.sh                       # drive plugins/check (today stage-c/run.sh if kept separate)
difftest/                        # differential-test corpus + driver
├── corpus/  run.sh  incremental-check.sh  README.md  FEASIBILITY.md  FINDINGS.md
bench/                           # suite scripts, programs, vendored deps (host crate moved out)
├── run.sh  run.py  compare.py  programs.json  programs/  chart/  scripts/  vendor/
├── gap/                         # gap analysis scripts + REPORT.md (crate moved to crates/gap-tool)
└── history/                     # OPEN: keep here or drop in favour of the bench-history release
release/                         # gen-notices.py, package.sh (today stage-c/precompile/package.sh)
ci/                              # one CI dir: build-and-test.sh, setup-toolchain.sh, audit-llvm.py,
│                                #   build-with-diagnostics.sh, publish-build-log.sh
site/                            # unchanged (Pages)
docs/                            # design notes from stage-c/notes + a short stages A–C history
legacy/llvm/                     # manual-only LLVM path (kept; removal is an open question)
├── patches/rust-1.99.1/  patches/llvm-22.1.0-wasip1/
├── config_rewriter/  ci/apply-rust-patches.sh  docs/update-1.99.1/  toolchain/
└── musl*.sh  crosstool-ng-build.sh  build-{netbsd,solaris}-toolchain.sh  *.defconfig
```

## Old → new mapping

| Old path | New path |
| --- | --- |
| `experiments/cranelift-wasm/stage-c/bootstrap.toml` | `compiler/bootstrap.toml` |
| `experiments/cranelift-wasm/stage-c/run.sh` | `compiler/run.sh` |
| `experiments/cranelift-wasm/stage-c/scripts/build-rustc-wasm.sh` | `compiler/scripts/build-rustc-wasm.sh` |
| `experiments/cranelift-wasm/stage-c/scripts/build-pure-sysroot.sh` | `compiler/scripts/build-pure-sysroot.sh` |
| `experiments/cranelift-wasm/stage-c/scripts/rustc-wasm-wrapper.sh` | `compiler/scripts/rustc-wasm-wrapper.sh` |
| `experiments/cranelift-wasm/stage-c/scripts/build-host.sh` | `compiler/scripts/build-host.sh` |
| `experiments/cranelift-wasm/stage-c/scripts/run-item3.sh` | `compiler/scripts/run-item3.sh` |
| `experiments/cranelift-wasm/stage-c/patches/{rust,cg_clif,clif-to-wasm,wild}` | `compiler/patches/…` |
| `experiments/cranelift-wasm/patches/0001-cg_clif-wasm-backend.patch` | `compiler/patches/cg_clif/` (if still applied; else delete) |
| `experiments/cranelift-wasm/stage-c/host/` | `crates/cogwasm-host/` (+ `crates/cogwasm-precompile/` from `src/bin/precompile.rs`) |
| `experiments/cranelift-wasm/stage-c/supervisor/` | `crates/cogwasm-supervisor/` |
| `experiments/cranelift-wasm/stage-c/precompile/package.sh` | `release/package.sh` |
| `experiments/cranelift-wasm/release/gen-notices.py` | `release/gen-notices.py` |
| `experiments/cranelift-wasm/difftest/` | `difftest/` (corpus + run.sh + incremental-check.sh + docs) |
| `experiments/cranelift-wasm/difftest/host/` | `crates/difftest-host/` |
| `experiments/cranelift-wasm/stage-c/programs/{a_add,b_alloc,c_plugin,e_math,f_geometry,mem_grow}` | `guests/…` |
| `experiments/cranelift-wasm/stage-c/programs/{c_plugin_harness,mem_grow_harness}` | `guests/…` |
| `experiments/cranelift-wasm/stage-b/a2/harness/` | `guests/a2-harness/` |
| `experiments/cranelift-wasm/stage-b/a2/{e_math,f_geometry}/` | delete (diverged; `bench/programs/kernels/math_e.rs` + `guests/{e_math,f_geometry}` supersede) |
| `experiments/cranelift-wasm/stage-c/notes/` | `docs/` |
| `experiments/cranelift-wasm/stage-c/results.md`, `results.md`, `stage-b/results/`, `stage-b/README.md`, `stage-b/research-wasmtime.md` | `docs/` (stage A–C history) |
| `experiments/cranelift-wasm/ci/` | `ci/` (merged with the top-level `ci/`) |
| `bench/host/` | `crates/bench-host/` |
| `bench/gap/` (Rust crate only) | `crates/gap-tool/`; scripts + `REPORT.md` stay `bench/gap/` |
| `patches/{rust-1.99.1,llvm-22.1.0-wasip1}/` | `legacy/llvm/patches/…` |
| `config_rewriter/` | `legacy/llvm/config_rewriter/` |
| `docs/update-1.99.1/` | `legacy/llvm/docs/update-1.99.1/` |
| `ci/apply-rust-patches.sh` | `legacy/llvm/ci/apply-rust-patches.sh` |
| `musl*.sh`, `crosstool-ng-build.sh`, `build-{netbsd,solaris}-toolchain.sh`, `*.defconfig`, top-level `toolchain/` | `legacy/llvm/` |
| `tree.txt`, `experiments/cranelift-wasm/programs/`, `experiments/cranelift-wasm/run.sh`, `experiments/cranelift-wasm/scripts/measure.py` | deleted (P0) |

## Toolchain directory and env vars

One ignored root replaces the `stage-b/` / `stage-c/` split inside
`experiments/cranelift-wasm/toolchain/`. Subdirs by role, not by stage:

| New path | Holds today's |
| --- | --- |
| `toolchain/rust/` | `toolchain/stage-b/rust/` (rust source + build tree + native stage1) |
| `toolchain/wasi-sdk/` | `toolchain/stage-b/wasi-sdk*` |
| `toolchain/artifacts/` | `toolchain/stage-c/artifacts/` (`rustc.wasm`) |
| `toolchain/sysroot/` | `toolchain/stage-c/sysroot-pure/` |
| `toolchain/cache/` | `toolchain/stage-c/cache/` |
| `toolchain/work/` | `toolchain/stage-c/work/` + difftest-work |
| `toolchain/checkouts/` | `toolchain/{playground,clif-to-wasm,wild}/` + `upstream/` clones |
| `toolchain/rustup/`, `toolchain/cargo/`, `toolchain/target/` | unchanged roles |

Env var renames, with a fallback to the old names so scripts and workflows can
migrate without a flag-day (introduced in P4, removed in P5):

| Old | New |
| --- | --- |
| `STAGE_C_STAGE1` | `COGWASM_STAGE1` |
| `STAGE_C_SYSROOT`, `STAGE_C_STD_SRC`, `STAGE_C_REAL_SRC` | `COGWASM_SYSROOT`, `COGWASM_STD_SRC`, `COGWASM_REAL_SRC` |
| `STAGE_C_RUSTC_WASM`, `STAGE_C_WILD`, `STAGE_C_HOST` | `COGWASM_RUSTC_WASM`, `COGWASM_WILD`, `COGWASM_HOST` |
| `STAGE_C_CACHE`, `STAGE_C_HOST_TRIPLE`, `STAGE_C_HOST_TOOLCHAIN` | `COGWASM_CACHE`, `COGWASM_HOST_TRIPLE`, `COGWASM_HOST_TOOLCHAIN` |
| `STAGE_C_TIME`, `STAGE_C_SPAWN_DEBUG` | `COGWASM_TIME`, `COGWASM_SPAWN_DEBUG` |

`STAGE_B_*` is retired with the stage-B tree in P2. `CG_CLIF_WASM_STRICT` is
cg_clif's own variable and is unchanged. In P4 every read site becomes
`${COGWASM_X:-${STAGE_C_X:-default}}`; the `STAGE_C_*` fallback is removed in
P5 once the workflows are confirmed green.

## Phased migration

Ordered by risk. Each phase is one PR that leaves CI green.

### P0 — delete dead weight (low-medium risk)

**Scope.** Remove `tree.txt` and the stage-A program crates
`experiments/cranelift-wasm/programs/{a_add,b_alloc,c_plugin,d_hello_std,host}`,
`experiments/cranelift-wasm/run.sh` and `scripts/measure.py` (superseded by
`stage-c/programs`). Update the layout block in
`experiments/cranelift-wasm/README.md` (line 45 lists `scripts/measure.py` and
the stage-A programs).

Generated output is **not** dead: `bench/gap/REPORT.md` links
`results/analysis.json` and `results/hot/`; `bench/README.md:277` and
`bench/gap/README.md:21-41` describe `results/`; `agents/ROADMAP.md` cites
`REPORT.md` as evidence; `stage-c/notes/item3-memory-grow.md:74-75` cites
`stage-c/results/item3.log` and `item3-sizes.txt`; `stage-c/notes/release.md:35`
greps `stage-c/results/build-rustc-wasm.log`. Keep that evidence reachable: keep
the compact `bench/gap/results/{analysis.json,SUMMARY.md,MIR.md,gc.json}` in the
tree, move the bulky `bench/gap/results/hot/**` WAT dumps and the
`stage-c/results/*.log` to release assets (like `bench-history`), and rewrite
every link in `REPORT.md`, `bench/gap/README.md` and the two notes. Keep the
`'!bench/gap/results/**'` filter in `cranelift_wasm.yml:25` while that directory
stays; drop it only if the whole directory moves to a release. `bench/history/`
is handled in the open questions.

**Verify.** `experiments/cranelift-wasm/ci/build-and-test.sh`;
`agents/scripts/review-scan.sh`; grep the edited docs for dangling `results/`
links.

**Rollback.** `git revert`. The edited reports and the moved evidence are one PR.

**Risk.** Low-medium. Deleting the spike code is safe; moving generated evidence
is what can break a report without a code change.

### P1 — root Cargo workspace + single Wasmtime pin (low risk)

**Scope.** Add a root `Cargo.toml` workspace whose members point (by path, still
under `experiments/cranelift-wasm/`) at `stage-c/host`, `stage-c/supervisor`,
`difftest/host`, `bench/host`, `bench/gap`. Set `wasmtime = "=50.0.2"`,
`wasmtime-wasi = "=50.0.2"` and `wasmtime-environ = "=50.0.2"` in
`[workspace.dependencies]` in lockstep (`stage-c/host/Cargo.toml` already pins
`wasmtime-environ` for the sidecar metadata) and have every host crate inherit
all three. Bump the `50.0.1` crates (`c_plugin_harness`, `mem_grow_harness`,
`stage-b/a2/harness`, `programs/host`) to `=50.0.2` and refresh their
`Cargo.lock`. Guest crates (`wasm32-unknown-unknown`, built by `rustc.wasm`) are
**not** members; if any member's dependency graph would pull them in, use a
workspace `exclude`.

**Verify.** `experiments/cranelift-wasm/ci/build-and-test.sh`; `cargo metadata`
shows one resolved Wasmtime. No `.cwasm` rebuild is needed.

**Rollback.** `git revert`. No path moves.

**Risk.** Low. The `50.0.1` crates already lock `50.0.2` (`programs/host`,
`c_plugin_harness`, `mem_grow_harness`, `stage-b/a2/harness`), so no API drift
and no `.cwasm` invalidation are expected; the only change is one resolved
version per crate.

### P2 — cut the stage-B dependency (medium risk)

**Scope.** Move the native stage1 + rust source from `toolchain/stage-b/rust` to
`toolchain/rust/`; move `stage-b/a2/harness` to `guests/a2-harness/`. Rewrite
`build-rustc-wasm.sh` (`WORK=$TOOL/stage-b` → `$TOOL/rust`), the
`STAGE_C_STAGE1` default in `run.sh`, `rustc-wasm-wrapper.sh`,
`build-pure-sysroot.sh`, and the two `cd stage-b/a2/harness` sites in `run.sh`.
Update `release/gen-notices.py` and the three workflows' cache keys. Delete
`stage-b/scripts`, `stage-b/host` (Wasmtime 46 + wasi-threads), `stage-b/run.sh`
and the diverged `stage-b/a2/{e_math,f_geometry}` once no longer referenced.

**Verify.** `experiments/cranelift-wasm/ci/build-and-test.sh` **without**
`--reuse-compiler` (cold, proves stage1 is found); `stage-c/run.sh check`;
`experiments/cranelift-wasm/difftest/run.sh`.

**Rollback.** `git revert`. The stage-B tree is still intact until the PR lands.

**Risk.** Medium-high. Stage B is load-bearing in seven places; miss one and the
build fails loudly (good), but the cache-key edit is easy to forget.

### P3 — extract cogwasm-runtime, dedupe hosts (behaviour-preserving)

**Scope.** New `crates/cogwasm-runtime/` holding `new_store`, `prepare_instance`,
`guest_memory`, `read_cstr`, `add_playground_spawn`, `spawn_impl`,
`startup_once`, `median` (the engine config and precompiled loader already live
in `stage-c/host/src/lib.rs`). `cogwasm-host` and `bench-host` call the shared
crate; delete the two per-host copies. The stage-B host is already gone (P2).
`difftest/host` stays lean (no WASI/`playground.spawn`) and is not forced onto
the runtime here.

**Verify.** Behaviour-preserving: run `experiments/cranelift-wasm/difftest/run.sh`
and `bench/run.sh --quick` **before and after**, diff the result JSON (bit-identical);
`stage-c/run.sh check` still returns `eval(1,2,3,4) == 2.25`.

**Rollback.** `git revert`. The refactor is internal; no external interface changes.

**Risk.** Medium. Must not change engine config, feature set or ordering; a
subtle behavioural drift would be caught only by the difftest/bench diff.

### P4 — the big `git mv` with all path updates (high risk)

**Scope.** Execute the mapping table with `git mv` (preserves history). Update
the workflow path filter in `cranelift_wasm.yml` from
`experiments/cranelift-wasm/**` to the new paths — otherwise CI silently stops
running on pushes. Update `release.yml`, `bench.yml`, `bench/run.sh`,
`bench/run.py`, `bench/gap/*.sh`, `site/index.html`, `CONTRIBUTING.md`,
`README.md`, and `agents/`. Rewrite the toolchain paths and cache keys to the
new `toolchain/` layout and the `COGWASM_*` env names (with `STAGE_C_*`
fallback). Note the cache keys change, so the **first CI run is cold (~45 min)**.

**Verify.** `ci/build-and-test.sh`; `difftest/run.sh`; `bench/run.sh --quick`;
`agents/scripts/review-scan.sh`; `release.yml` dry run; a Pages preview build.

**Rollback.** `git revert` the one PR. Nothing was changed out-of-band.

**Risk.** High: ~46 `experiments/cranelift-wasm` references in `release.yml`,
~39 in `cranelift_wasm.yml`, ~17 in `bench.yml`, plus the rest. Mechanical but
wide; do it as a single atomic PR so no commit has a half-moved tree.

### P5 — patch manifest + generated upstream/ bundles (low risk)

**Scope.** Add `compiler/upstreams.toml` (per upstream: repo, rev, ordered patch
list). Point `ci/setup-toolchain.sh` at it. Generate the `upstream/` PR bundles
from the manifest so `upstream/` stops being hand-maintained. Replace the prose
pins in `experiments/cranelift-wasm/README.md` (now `docs/`) with the manifest.

**Verify.** `ci/build-and-test.sh --reuse-compiler`; a generated bundle is
byte-identical to the committed one (`diff -r`); the applied patch series is
unchanged (same compiler SHA).

**Rollback.** `git revert`.

**Risk.** Low. Must not change which patches apply or the checkout revs.

### P6 — supervisor calls the runtime in-process (feature work)

**Scope.** `crates/cogwasm-supervisor` stops shelling out to `cogwasm-host raw`
(`supervisor/src/real.rs`) and drives the resident compiler through
`cogwasm-runtime`/`cogwasm-host`'s library API, keeping `rustc.wasm` loaded
across jobs. Depends on P3. This is **feature work**, flagged as separate from
the restructure, and changes behaviour (resident compiler vs per-job reload).

**Verify.** Supervisor golden tests in CI; the real-backend matrix from
`docs/supervisor.md`; the Goal 2 rows that exist — first compile after install
in well under 1 s, and a resident compiler service that keeps `rustc.wasm`
loaded (instantiation in about 10 ms is a separate open item). Add a measurable
check that the supervisor no longer reloads `rustc.wasm` per job: compare
per-job wall time against `stage-c-host` resident edit-to-result (about 96 ms
locally per the README).

**Rollback.** `git revert`; P0–P5 do not depend on it.

**Risk.** Medium (new behaviour, not just a move). Land after the restructure is
stable.

### P7 — legacy move (low risk)

**Scope.** Move `patches/{rust-1.99.1,llvm-22.1.0-wasip1}`, `config_rewriter/`,
`docs/update-1.99.1/`, `ci/apply-rust-patches.sh`, the musl/crosstool/netbsd/
solaris scripts, `*.defconfig` and the empty top-level `toolchain/` into
`legacy/llvm/`. Update `build_llvm.yml`, `rustc_minimal.yml`,
`rustc_llvm_with_lld.yml`, `create_release.yml` and the README legacy section.

**Verify.** The legacy workflows are manual-only; confirm their referenced paths
still resolve (or record them as frozen). `agents/scripts/review-scan.sh`.

**Rollback.** `git revert`.

**Risk.** Low, but do not break the legacy workflow YAML if anyone runs it by hand.

## Verification checklist: "no behaviour change"

Run in order after each phase (and all of them before the final merge of P4/P5):

- [ ] LLVM audit passes: `python3 ci/audit-llvm.py <inspect.json> <rustc.wasm>`
  (before P4: `experiments/cranelift-wasm/ci/audit-llvm.py`; the check also runs
  inside `build-and-test.sh`).
- [ ] Difftest green: `difftest/run.sh` (before P4: `experiments/cranelift-wasm/difftest/run.sh`).
- [ ] Bench quick bit-identical: `bench/run.sh --quick` before/after diff shows no numeric drift.
- [ ] `c_plugin` end-to-end: `eval(1,2,3,4) == 2.25` (`compiler/run.sh check`; before P4: `stage-c/run.sh check`).
- [ ] Release dry run succeeds: `release.yml` with `dry_run: true` produces the draft bundle.
- [ ] Pages build: `site/build.py` (or the `deploy_pages.yml` preview) still renders `site/index.html`.

## Open questions (for the owner)

- **Legacy keep vs delete.** Keep `legacy/llvm/` (default) or delete the manual-only
  LLVM path and its three workflows in a follow-up? Nothing here forces the choice.
- **`bench/history/` in repo vs release.** The three committed JSON points are
  redundant with the `bench-history` release. Keep, delete, or add a gitignore
  rule so only the release is the source of truth?
- **Naming.** `crates/` vs `cogwasm/`; `cogwasm-host` vs keeping the `stage-c-host`
  binary name (the supervisor and `bench/host` reference it by name).
- **`bench/gap/` split.** The crate moves to `crates/gap-tool/`; confirm the
  analysis scripts, `REPORT.md` and (gitignored) `results/` stay under `bench/gap/`.

## Status (2026-11-21)

Done on branch `layout-migration`, each phase verified locally with
`ci/build-and-test.sh --reuse-compiler` and `bench/run.sh --quick` against a
baseline (clif and LLVM `-O2` modules identical; only timings differ).

| Phase | Commit | Deviation from the plan |
| --- | --- | --- |
| P0 | `42c74c1` | Only `tree.txt` removed. Stage A went with P2 (stage B still built from it). Generated evidence (`bench/gap/results`, `docs/results/*.log`) is still tracked: moving it to a release asset publishes it, so it waits for an owner decision. |
| P1 | `5311ad0` | As planned. `serde_json` 1.0.154 → 1.0.149 and `smallvec` 1.16.6 → 1.16.5 from unifying the lockfiles. |
| P2 | `ee9f8be` | Stage A removed here. Source setup became `setup-sources.sh`. |
| P3 | `bc9c86a` | Only `Wt` and `SpawnSummary` were identical; the other shared helper names behave differently in each host and stay duplicated until their behaviour is decided. |
| P7 | `d4ec653` | Done before P4. |
| P4 | `571a5e1`, `3e1d0a3` | Split into the move (4a) and the renames (4b). `bench/host` and `bench/gap` stay in `bench/`. Fixed `setup-sources.sh` failing to rewrite `bootstrap.toml` after the toolchain moved. |
| P5 | `f8e37c5` | Upstream PR bundles are checked, not generated (they carry hand-written PR text). All four currently differ from the live patches. |
| P6 | — | Not started: feature work, separate from the restructure. |

Not verified locally: a cold build (setup-toolchain.sh, rustc.wasm from
source), the release workflow and the legacy LLVM workflows. CI caches start
cold on the first run of this branch.
