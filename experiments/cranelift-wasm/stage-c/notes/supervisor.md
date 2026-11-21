# Compile-level supervisor: plan

Status: **M1–M5b and M5 done**. This is the host-side scheduler for Goal 1's
tier 2 and Goal 2's resident compiler service. It keeps the compiler a pure
function of (source, options) and puts the option policy, caching and swapping
in the host, where they can be tested.

## The decision

Build a **thin supervisor**, not a graph compiler.

- The compiler (rustc.wasm + clif-to-wasm + Wild) stays a pure function and
  gains no knowledge of the option space or swapping.
- A host-side supervisor owns the option policy, the artifact cache, the
  background build and the swap.
- The swap is always **instance-level at a quiescent point**. WebAssembly has
  no way to replace a function in a live instance, and the engine call stack
  (return addresses, frames) cannot be migrated; only wasm linear memory is
  ordinary bytes.
- Per-codegen-unit reuse inside a crate is **not** part of this plan yet. The
  generated-stress experiment shows the unit is real (92.7% of object bytes
  survive a module edit) but rustc exposes no boundary that lets a host skip
  codegen for unchanged units; incremental mode reuses its dep graph, not a
  set of objects, and an edit is not cheaper than its own cold incremental
  build. That is a compiler-side research item, not a supervisor feature.

## What the measurements say

From [`module-reuse-experiment.md`](module-reuse-experiment.md) and the
inliner work:

| fact | number |
| --- | --- |
| reusable object bytes after an edit, flat layout | 0.6% |
| reusable object bytes after an edit, 8 modules | 92.7% |
| cost of the module split | +5.7% wasm, +30 ms link |
| incremental vs full compile (`generated_stress`) | 124 vs 383 ms |
| incremental output penalty | +6.6% wasm, link 65 vs 23 ms |
| opt-level effect on compile time | small (328–393 ms across 0–3) |
| precompiled `.cwasm` load | ~19 ms |

Two consequences for the design:

1. **Incremental is the fast tier**, not opt-level.
2. **Object files embed the source path.** Any cache key must include the
   source path (or the build must normalise it), and the same hazard applies
   to Goal 4's reproducible builds.

## Architecture

A new crate beside the host: `experiments/cranelift-wasm/stage-c/supervisor/`
(CLI + library). It drives the existing `stage-c-host` / `bench-host` compile
and eval paths; it does not reimplement them. (Today that logic lives in
`host/src/main.rs`; extracting a library API is a later step, only if
in-process calls become necessary.)

```
JobSpec ──▶ Supervisor ──▶ ArtifactStore ──▶ instance (T0)
                │                   ▲
                ├── background ─────┘  T1 artifact
                └── Swapper: quiesce old, instantiate new, migrate state
```

Components:

- **`JobSpec`** (serde JSON, versioned): source, crate name, target, tier,
  budgets, incremental dir, emit set, cache key inputs.
- **`ArtifactStore`**: content-addressed `{key}/artifact.wasm` + `meta.json`;
  key = hash of source, dependency hashes, compiler sha, flags, tier, target
  and source path.
- **`Tier`**: the tier→flags map (below).
- **`Compiler`** trait: `Real` (spawns the host binary) and `Fake` (instant,
  deterministic, for tests and CI).
- **`Scheduler`**: runs T0 synchronously, T1/T2 in the background; one build
  per key; cancels superseded builds.
- **`Swapper`**: quiesce, generation counter, instantiate, optional
  `snapshot`/`restore`.

## Options and presets

A request is a whitelisted [`CompileOptions`](../stage-c/supervisor/src/job.rs)
spec, not a tier: `opt_level`, `mir_opt_level`, `inline_wasm` (and threshold),
`incremental`, `strip`, `codegen_units`, `debug`. Every field is validated, the
cache key covers the whole spec, and the artifact records it, so any
combination is reproducible and explainable. An app may request combinations
the supervisor has never seen.

`fast`, `opt` and `release` remain as named presets:

| preset | spec | purpose |
| --- | --- | --- |
| `fast` | opt-level 2, incremental, no inliner | edit loop; 1.3–2.2× faster to rebuild |
| `opt` | opt-level 2, inliner | background build; `strings` −41.8% |
| `release` | `opt` + `strip=symbols` | distribution; never a runtime tier |

Why two runtime tiers and a strip option, measured on the stored artifacts:

| program | fast | opt | release | opt/fast |
| --- | ---: | ---: | ---: | ---: |
| `nbody` | 158.4 ms | 70.4 ms | 68.8 ms | **0.445** |
| `strings` | 1.492 ms | 0.763 ms | 0.744 ms | **0.512** |
| `collections` | 0.0615 ms | 0.0543 ms | 0.0514 ms | 0.882 |
| `generated_stress` | 0.0614 ms | 0.0590 ms | 0.0588 ms | 0.962 |

- **`opt` is not optional**: incremental code is 2.0–2.2× slower on the two
  programs that matter most, because it gives up the inliner.
- **`release` is not a third optimisation round**: it is `opt` plus a symbol
  strip — same compile time, same execution, 26–42% smaller. It is a
  packaging flag on the spec, not a runtime tier.

The supervisor stores and compares the full spec; the presets are only
conveniences. The CLI accepts either: `--spec fast` or
`--spec "opt-level=3,mir-opt-level=3,inline=yes,strip=symbols,codegen-units=4"`.

An empirical selector is the next layer, not part of the core: the host
records `(spec, compile_ms, exec_ms, size)` per program and picks the best spec
with enough samples and hysteresis. The compiler and the swap machinery do not
change for it.

## Contract

Request:

```json
{"schema": 1, "request_id": "…", "tier": "fast|opt|small",
 "source": {"path": "…", "crate": "…", "edition": "2021"},
 "budgets": {"compile_ms": 100, "size_bytes": 65536},
 "incremental": {"dir": "…"},
 "emit": ["wasm", "metadata"],
 "cache_key": "…"}
```

Response:

```json
{"schema": 1, "request_id": "…", "tier": "opt", "artifact": "…",
 "size_bytes": 0, "compile_ms": 0, "sha256": "…",
 "diagnostics": [], "cache_hit": false, "superseded": false}
```

Rules:

- A failed background tier never replaces a running one.
- A newer request for the same key supersedes the older one.
- Artifacts are immutable once written; the store is GC-able by age.

## Swap contract

- The swap happens only at a **quiescent boundary**: no call into the old
  instance is in flight.
- A generation counter tags every instance; calls that arrive after the swap
  go to the new generation.
- M3 swaps stateless plugins by cold restart (instantiate, re-run the entry
  with the same inputs).
- M4 adds an explicit state contract: `snapshot() -> bytes` /
  `restore(bytes)` (allocator state included), with an ABI version check.
- A shell/core split (a stable shell calling a swappable core through host
  imports) is only considered if an app must swap without a restart.

## Milestones

| milestone | deliverable | accepted when |
| --- | --- | --- |
| M1 | contract note + `supervisor` crate with `JobSpec`, `ArtifactStore`, `Fake`, golden tests | tests pass without a toolchain: cache hit, edit rebuild, spec switch, invalidation, failure fallback, swap ordering |
| M2 | `Real` backend driving the existing host; spec matrix on `generated_stress` + three suite programs | compile/size/exec published per spec; results identical across specs |
| M3 | cold-restart swap demo | edit→T0 and edit→T1-applied latencies, swap cost, published |
| M4 | state ABI + swap of a stateful plugin | state survives the swap; ABI version mismatch rejects |
| M5 | crate-level cache and multi-crate workspace; remote cache | unchanged crates never rebuild; CI cache hit rate published |
| M5b | composable option specs and an empirical selector | any whitelisted spec is requestable; the selector picks with enough samples and hysteresis |
| M6 | research: per-CGU artifact boundary in rustc | gated on a compiler change; evidence: 92.7% of object bytes are reusable |

## M2 matrix

Built with the reference supervisor against the real pipeline (`stage-c-host
raw`, precompiled `rustc.wasm`), four programs at three tiers. Times are the
`compile_ms` recorded by the supervisor (host start, module load from the
precompiled artifact, compile and link), after a cold run and after a
one-line edit.

| program | tier | cold ms | edit ms | bytes |
| --- | --- | ---: | ---: | ---: |
| `generated_stress` | fast | 500 | **199** | 163,038 |
| `generated_stress` | opt | 435 | 429 | 153,513 |
| `generated_stress` | small | 425 | 428 | 89,753 |
| `strings` | fast | 156 | **114** | 57,459 |
| `strings` | opt | 141 | 142 | 49,503 |
| `strings` | small | 145 | 143 | 41,487 |
| `collections` | fast | 349 | **160** | 251,964 |
| `collections` | opt | 309 | 310 | 217,343 |
| `collections` | small | 298 | 304 | 161,146 |
| `nbody` | fast | 149 | **105** | 44,583 |
| `nbody` | opt | 139 | 140 | 41,734 |
| `nbody` | small | 137 | 144 | 34,988 |

Reading:

- **T0 wins on edits, not on cold builds.** A cold incremental build is
  slightly slower and about 6–16% larger, but after a one-line edit it is
  1.3–2.2× faster than the optimising rebuild because rustc reuses its dep
  graph.
- **T2 is the size tier.** Stripping symbols shrinks `generated_stress` by
  42% and `collections` by 26% over T1 at the same compile time.
- **All three tiers return identical `run(seed)` values and output lengths**
  for every program (checked with `bench-host run` on the stored artifacts).

M1–M5b and M5 are accepted: twenty-eight tests pass in CI (sixteen golden,
seven selector, five workspace), the matrix above is the M2 evidence, the swap
table is the M3 evidence, the state tests are the M4 evidence, the selection
table is the M5b evidence, and the workspace table is the M5 evidence. What
remains is M6, the compiler-side per-CGU boundary, which is research gated on
rustc exposing a reusable unit.

## M3: the swap

`demo` compiles T0, serves it, compiles T1 in the background while calls keep
arriving, then installs T1 and verifies the result. Run on `strings`:

| event | measured |
| --- | ---: |
| T0 cold build | 170 ms |
| edit -> T0 build (incremental) | 122 ms |
| T0 install (compile to native) | 239 ms |
| T1 build, edit -> applied | 515 ms |
| precompile T1 | 267 ms |
| T1 install (precompiled) | **0.3 ms** |
| calls served on T0 during the T1 build | 88 |
| `run(seed)` cold / T0 / T1 | identical |

The finding: **a swap costs a wasm-to-native compile, not an instantiation.**
Installing an artifact whose module is already compiled is sub-millisecond;
installing a fresh one pays 240–270 ms in Wasmtime's compiler. That is the
same trade-off the host's `rustc.wasm` path solved with a precompiled
`.cwasm` (19 ms load against 38 s cold). The production supervisor should
precompile each artifact as it is produced (or store a `.cwasm` next to it),
then the swap is instantiation only.

The reference runtime caches compiled modules in memory by artifact key, which
is enough for the demo; a real service wants the cache on disk and keyed by
the engine's compatibility hash, as `stage-c-precompile` does for `rustc.wasm`.

M3 uses a cold restart: the new instance re-runs `run(seed)` with the same
input, which is correct for a plugin whose state is derived from its inputs.
Stateful plugins need M4's `snapshot`/`restore` contract.

## State across a swap (M4)

A stateful plugin exports four things beside `run`:

| export | contract |
| --- | --- |
| `cogwasm_state_version() -> i32` | ABI version the plugin writes and reads |
| `cogwasm_snapshot(ptr, cap) -> i32` | with `cap = 0`, return `-required`; otherwise write at `ptr` and return the length |
| `cogwasm_restore(ptr, len) -> i32` | zero on success |
| `wasm_alloc(size, align) -> i32` | the buffer the host uses for both calls |

`Swapper::install_preserving_state` snapshots the old instance, instantiates the
new one, checks that its version equals the old one, restores, and only then
makes it current. Any failure — missing contract, changed version, rejected
restore — leaves the old instance serving. A stateless old instance falls back
to a cold restart.

Evidence (hand-written stateful WAT plugin, no toolchain):

- the counter continues across a swap: `run(5)`, `run(7)` -> 12, swap,
  `run(1)` -> 13;
- a version-1 -> version-2 swap is rejected and the old instance keeps serving;
- a stateless plugin falls back to the cold restart.

Not covered yet: in-flight host calls (a swap is still quiescent by
construction), threads inside plugins, host imports, and allocator internals
beyond what the plugin puts in its own snapshot. Those belong to the service,
not the supervisor.

## Workspaces (M5)

A workspace manifest is a JSON list of crates:

```json
{"crates": [
  {"name": "math_lib", "source": "/tmp/ws/math_lib.rs", "deps": []},
  {"name": "app",      "source": "/tmp/ws/app.rs",      "deps": ["math_lib"]}
]}
```

`workspace --manifest ... --spec opt` topologically sorts the crates and
builds each as its own [`JobSpec`]: a crate that others depend on becomes an
`rlib`, the roots become `cdylib` plugins. The job carries `--extern` inputs,
and the cache key covers the **content hash of every extern**, so a changed
rlib invalidates its dependents while an unchanged one stays cached.

Real experiment, two crates, `opt` spec:

| event | math_lib (rlib) | app (cdylib) |
| --- | --- | --- |
| cold | built, 2,494 B, 55 ms | built, 495 B, 77 ms |
| unchanged | hit | hit |
| edit math_lib | built | built (its extern hash changed) |
| edit app | hit | built |
| unchanged | hit | hit |

The linked app runs correctly (`run(21) = 42`), and the app's key changes when
the rlib changes even though the app source did not.

Two constraints found by doing it:

- **rustc finds an `--extern` rlib by name**, so the store names rlib files
  `lib<crate>.rlib`, not `artifact.rlib`; the first real run failed with
  "can't find crate" until that was fixed.
- **cg_clif rlibs contain metadata; the object is resolved at link time.**
  The archive is small (2.5 KB) and Wild links the final cdylib; this works,
  but it means the rlib cache entry is not a reusable object file.

This is a scheduler, not cargo: no build scripts, proc macros, feature
resolution or version constraints. Those belong to the host above it.

## Empirical selection (M5b)

`matrix --exec --samples N --record profiles.json` runs each built artifact and
records `{spec, compile_ms, exec_ms, size_bytes}` per program. `select
--profiles profiles.json --program P --objective exec|size|balanced
[--min-samples N] [--current SPEC] [--margin M]` returns the best known spec.

The policy is conservative: median per spec, a minimum number of measurements
before a spec is eligible, and a fractional margin (default 5%) before
switching away from `--current`, so timing noise cannot make a running service
oscillate. Repeated matrix runs append to the same scoreboard.

One matrix run on the four programs (five execution samples each):

| program | exec objective | size objective | balanced |
| --- | --- | --- | --- |
| `strings` | opt (0.888 ms) | release (41,487 B) | release |
| `nbody` | release (70.06 ms) | release (34,988 B) | release |
| `generated_stress` | opt (0.015 ms) | release (89,753 B) | release |

The opt/release choice on exec is inside noise (`nbody` 70.06 vs 70.29 ms);
compile times are equal, so release is the safe default when execution is a
tie. That is exactly the case the margin exists for.

## Test matrix (M1)

| test | expected |
| --- | --- |
| unchanged source | cache hit, zero compiler invocations |
| one-line edit | exactly one rebuild, T0 artifact current |
| tier change | two artifacts, both load, keys differ |
| compiler sha change | full invalidation |
| background compile fails | T0 keeps serving, error surfaced |
| request superseded | old build cancelled, new artifact used |
| swap | no call reaches the old instance after quiesce |
| cache with different source path | miss (path is part of the key) |

## Risks and open questions

- **Path embedding**: object bytes depend on the source path, so cache keys
  must include it or builds must normalise it. This also matters for Goal 4.
- **Link cost**: every swap relinks; 23–65 ms today, so cheap but not free.
- **Incremental trade-off**: T0 is faster to build but bigger and slightly
  slower to execute; the tier policy must not hide that.
- **Two schedulers**: the supervisor schedules tiers; rustc schedules its own
  query graph. Keep the supervisor at the artifact boundary and never
  second-guess rustc inside a crate.
- **State semantics** (M4): what is app state versus plugin state, and how to
  version it.
- **Cache eviction** policy for a content-addressed store.
- **T2 is not a runtime tier**; it must not be swapped in behind T1.

## Relation to the roadmap

- Goal 1 "Tier 2: background optimising build and swap" — this plan.
- Goal 2 "resident compiler service" — the supervisor is the service host.
- Goal 4 "reproducible builds" — the path-embedding hazard above.
