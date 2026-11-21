# Tier 2 supervisor: plan

Status: **M1–M3 done**. This is the host-side scheduler for Goal 1's
tier 2 and Goal 2's resident compiler service. It keeps the compiler a pure
function of (source, options) and puts tiering, caching and swapping in the
host, where they can be tested.

## The decision

Build a **thin supervisor**, not a graph compiler.

- The compiler (rustc.wasm + clif-to-wasm + Wild) stays a pure function and
  gains no knowledge of tiers or swapping.
- A host-side supervisor owns the tier policy, the artifact cache, the
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

## Tiers

| tier | flags | purpose |
| --- | --- | --- |
| T0 fast | opt-level 2 + `-Cincremental=<dir>` | edit loop; 1.3–2.2× faster than T1 after a one-line edit |
| T1 optimised | opt-level 2 + `-Zmir-inline-wasm` | background build; `strings` −41.8% |
| T2 small | opt-level 2 + `-Zmir-inline-wasm` + `-Cstrip=symbols` | distribution only; never a runtime tier |

`-Copt-level=s` alone was *larger* than opt-level 2 in the M2 matrix, so T2
uses the optimising flags plus a symbol strip instead.

Exact flags are set from the job spec; only the supervisor knows the tier
names.

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
| M1 | contract note + `supervisor` crate with `JobSpec`, `ArtifactStore`, `Fake`, golden tests | tests pass without a toolchain: cache hit, edit rebuild, tier switch, invalidation, failure fallback, swap ordering |
| M2 | `Real` backend driving the existing host; tier matrix on `generated_stress` + three suite programs | compile/size/exec published per tier; results identical across tiers |
| M3 | cold-restart swap demo | edit→T0 and edit→T1-applied latencies, swap cost, published |
| M4 | state ABI + swap of a stateful plugin | state survives the swap; ABI version mismatch rejects |
| M5 | crate-level cache and multi-crate workspace; remote cache | unchanged crates never rebuild; CI cache hit rate published |
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

M1, M2 and M3 are accepted: the nine golden tests pass in CI, the matrix above
is the M2 evidence, and the swap table above is the M3 evidence. M4 (the
state ABI for stateful plugins) is next.

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
