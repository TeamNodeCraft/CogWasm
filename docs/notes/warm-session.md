# G2.3: a warm rustc across compiles (design note)

Status: draft, 2026-11-22. Nothing here is implemented. The plan item is
G2.3 in [`agents/plans/next-steps.md`](../../agents/plans/next-steps.md); the
measurements it builds on are in [`supervisor.md`](supervisor.md), sections
"Where the CI time goes" and "Fixed versus per-source cost".

## Problem

Goal 2 is edit → result under 100 ms on a CI runner. The resident supervisor
(G2.1) measures **174 ms** there (41 ms on macOS arm64). Swap, call and guest
instantiation cost about 1 ms, so all of it is rustc running inside the guest,
and about 90% of that is the same for every compile:

| part (macOS; CI ≈ 4.2× slower) | time | what it is |
| --- | ---: | --- |
| sysroot crate loading | ≈ 19 ms | `macro_expand_crate` loads `core` (8.2 ms in a metadata-only `no_std` build, 0.1 ms with `no_core`); late resolution loads more for a cdylib |
| session floor | ≈ 8 ms | whole front end of a `#![no_core]` crate |
| link | ≈ 8 ms | Wild subprocess 4.6 ms, rustc-side preparation ≈ 3 ms |
| outside rustc's `total` | ≈ 3 ms | guest start-up, argument and target set-up |
| the source being compiled | ≈ 4 ms | everything that differs between an empty crate and the fixture |

To meet the target on CI, a small edit has to cost about 24 ms on the Mac at
today's ratio. That means removing most of the crate loading and part of the
session floor.

## What the crate-loading cost probably is

On `wasm32`, `rustc_data_structures::memmap::Mmap` is not a mapping:
`Mmap::map` reads the whole file into a `Vec<u8>` (`memmap.rs`, the
`cfg(any(miri, target_arch = "wasm32"))` arm). The metadata locator
(`rustc_metadata/src/locator.rs`, `get_metadata_section`) and the codegen
metadata loader (`rustc_codegen_ssa/src/back/metadata.rs`) both go through it.
In our sysroot:

| file | size |
| --- | ---: |
| `libcore-*.rmeta` | 45.4 MB |
| `liballoc-*.rmeta` | 8.4 MB (only when `alloc` is used) |
| `libcompiler_builtins-*.rmeta` | 2.4 MB |

So every compile copies at least 47 MB from the host through WASI `fd_read`
into a linear memory that was just grown to hold it (and, with a fresh
instance per job, its pages are touched for the first time). A memory copy
of that size alone costs several milliseconds on the Mac and proportionally
more on the runner, which may be most of the 8 ms in expansion. Decoding is
lazy, so the rest of the crate-loading cost should be header and hash checks
and the module-tree and prelude queries that expansion and resolution do
against `core`.

**Hypothesis H1:** most of the crate-loading cost is reading the metadata
bytes, not decoding them. If H1 holds, the cheap fix (option A) gets most of
the win. If it does not, only option C reaches the target.

## Constraints

- **Session ownership.** `CStore` is created per global context
  (`rustc_interface/src/passes.rs`, `create_and_enter_global_ctxt`, about
  line 963) and frozen after loading (`freeze_cstore`). Crate numbers, the
  local crate's `StableCrateId` (derived from the crate name and `-C metadata`)
  and the `DefPathTable` are per session. Nothing in rustc supports
  compiling a second crate in an existing session, and making that work in
  general is a rewrite. Any design has to keep "one session per compile" or
  fix every input that the session identity depends on.
- **The edit loop helps.** Within one `edit-loop` or one supervisor tier,
  the crate name, the options, the target and the sysroot are constant; only
  the source text changes. A design may specialise on that and fall back to a
  cold compile whenever any of these change.
- **Correctness bar.** The output must be byte-identical to a cold compile
  of the same source with the same options, or at least have identical
  difftest results, which is the bar incremental mode uses today. A warm
  compile that is not identical must be visible in CI, not just tolerated.
- **Host side.** Wasmtime has no copy-on-write snapshot of a running
  instance's memory. Restoring a snapshot means copying the memory, and that
  copy has to be cheaper than the work it skips.
- **Patch budget.** Changes to rustc go into `compiler/patches/rust/` as one
  small, wasm-gated patch (like `0003-wasm-file-locks.patch`), so they can
  be upstreamed or dropped.

## Options

### A. A long-lived guest with process-level caches (smallest change)

Keep one rustc guest alive across compiles. A patched `rustc_driver` entry
point, behind an environment variable, loops: read a request (argv plus
source path) from the host, run `run_compiler` as today (a fresh session),
write the status, repeat. Between sessions it keeps process-level state that
is safe to share:

- the metadata bytes, cached by (path, size, mtime) in a process-global map
  and handed out as shared `OwnedSlice`s instead of re-reading the file;
- target spec parsing and sysroot file search results, if they show up in a
  profile.

Expected saving: the file copy and page faults behind H1 (up to ≈ 8–10 ms
on the Mac if H1 holds), plus the ≈ 3 ms of guest start-up. It leaves the
decoding, the session floor and linking.

Risk: low. Each compile is still a fresh session; the cache holds immutable
bytes. Memory grows by the cached files once (≈ 50 MB), not per job.

Host change: the resident backend keeps one store per backend instead of one
per job, and talks to the guest through stdin/stdout or a small
`cogwasm.request` import. The `Teardown` question from G2.1 becomes "drop the
store when the backend goes away".

### B. Stop re-reading metadata at all (no guest loop)

Give the guest the bytes without a copy: a host import that maps the rmeta
into linear memory. Wasmtime has no API to map a host file into an existing
memory, so in practice this means a custom memory creator, or keeping the
bytes in the module's data segment (which bakes the sysroot into
`rustc.wasm` and ties the module to one sysroot). This removes the read cost
without the request loop, but it is more host and engine work than A and
saves less than A plus its caches. Recorded for completeness; not
recommended.

### C. A warm session template per (crate, options), restored per edit

Specialise on the edit loop. The guest runs the compile up to the point just
after the sysroot crates are loaded (session created, `CStore` filled), then
signals the host. The host snapshots the instance's linear memory and
globals as a template. For each edit it restores the template into a fresh
instance and resumes with the new source text.

The obstacle is ordering. Today rustc parses before it loads crates: `core`
is loaded when expansion processes the injected `extern crate core`. So the
patch has to load the sysroot crates before the local crate is parsed (load
`core` and `compiler_builtins` eagerly once the resolver exists), then park at
a resume point that takes the source text. Everything before the resume point
must not depend on the source; asserting that is part of the patch.

Expected saving: crate loading and most of the session floor (≈ 25 ms of 37
on the Mac), so roughly 60–70 ms on the runner. This is the only option that
plausibly meets the target alone.

Risks:

- **Restore cost.** rustc's memory after loading `core` is probably 100 MB or
  more (with no mmap, the 45 MB rmeta is resident). Copying that per edit
  could cost as much as it saves. It needs dirty-page tracking or a
  copy-on-write memory creator, which is real Wasmtime work.
- **Hidden state.** Thread-locals, interner state, global allocators and
  open WASI descriptors all live in the snapshot or in the host context and
  must line up after a restore.
- **Fragility.** The resume point moves with every rustc update; the patch is
  larger and harder to upstream than A.

### D. Make the fixed work cheaper in place

Independent of A and C: profile the 8 ms session floor and the ≈ 3 ms of
rustc-side link preparation with `-Zself-profile` (needs `measureme`'s
`summarize`), and remove hot spots that are wasm-specific (for example a
filesystem walk of the sysroot, lock files, or a hash of the whole rmeta for
the SVH check). Bring the Wild link in-process instead of spawning it, which
saves the subprocess (≈ 4.6 ms on macOS, less on Linux) and is already
possible in principle because Wild is a library. Each of these is small, but
on the runner even 5 ms is 20 ms.

## Experiments before choosing

Each takes under a day and decides a branch:

- **E1 (tests H1).** Time `Mmap::map` for each rmeta inside the guest: a
  temporary `eprintln!` timer in a local rustc.wasm build, or a tiny
  wasm32-wasip1 program that reads `libcore.rmeta` through the same WASI path
  under the same engine config. If the reads are most of the 8 ms, A is
  worth doing first.
- **E2 (sizes C).** Record linear-memory size after crate loading and after a
  full compile (the host can read `Memory::data_size` when `_start` returns,
  and at a temporary hook). Time a `memcpy` of that size on the runner. If
  the restore costs more than ≈ 15 ms on CI, C needs copy-on-write and moves
  out of scope.
- **E3 (for D).** A `-Zself-profile` run of the empty crate, summarised by
  query, to see what the session floor and late resolution spend their time
  on.
- **E4 (CI ratio).** Run the `-Ztime-passes` comparison on the runner once
  (a manual workflow step) to check that the 4.2× ratio holds per pass and
  is not concentrated in I/O. If I/O dominates on the runner, A is worth more
  there than on the Mac.

## E1 result: the metadata read (2026-11-22, macOS arm64)

Probe: [`warm-session-e1-probe.rs`](warm-session-e1-probe.rs), built for
`wasm32-wasip1` with the pinned stable toolchain and run through
`cogwasm-host raw --rustc probe.wasm -- 3 <libcore.rmeta> <libcompiler_builtins.rmeta>`,
so it gets the same engine configuration, WASI context and `/` preopen as
rustc.wasm. It does exactly what rustc's wasm32 `Mmap::map` does
(`File::open` + `read_to_end` into a `Vec`) and keeps the blobs alive. Five
fresh instances, three rounds each:

| | core + compiler_builtins (47.8 MB) | core alone |
| --- | ---: | ---: |
| first read in a fresh instance | 7.1 ms (one 11.8 ms outlier) | 6.5 ms |
| re-read in the same instance (memory already grown) | 3.6–3.9 ms | — |

Reading: **H1 partly holds.**

- The `core` read is about 6.5 ms of the 8.2 ms that loading `core` adds to
  expansion (≈ 80%), and the two reads are about 7 ms of the ≈ 19 ms
  crate-loading cost (≈ 37%). The rest is decoding and resolution against the
  loaded metadata, which a byte cache does not remove.
- About half of the fresh-instance read is not the copy: it is growing the
  linear memory by 46 MiB and touching those pages for the first time.

That second point applies to the whole compile, not just the read. The engine
uses Wasmtime's default on-demand allocator, so each per-job instance starts
with freshly mapped memory. Host RSS grows by about 150 MB between
`rustc --version` (206 MB) and compiling the empty crate (355 MB), so a
compile touches on the order of 150 MB of new memory. At the rate measured
above (≈ 3.5 ms per 46 MiB), that is very roughly 10 ms of first-touch cost
per compile. This is an estimate; E2 should measure the guest memory size
directly.

What this changes:

- **Option A is worth more than the read alone.** A long-lived guest skips
  the read (≈ 7 ms), the guest start-up (≈ 3 ms) and most of the first-touch
  cost, because rustc's allocator reuses memory it already grew. Estimated
  saving: 10–17 ms of 41, which projects to roughly 105–130 ms on the runner.
  With D's in-process Wild link, getting under 100 ms on CI becomes
  plausible without option C.
- **A host-only variant (A0) is worth one experiment first:** Wasmtime's
  pooling allocator, which reuses linear-memory slots between instances
  (optionally keeping pages resident and zeroing them in place). It needs no
  rustc patch. Zeroing 150 MB in place has its own cost, so whether it beats
  fresh mappings is something to measure, not assume. Note that a precompiled
  `.cwasm` is tied to the engine config, so switching allocators changes the
  compatibility hash and forces a re-precompile.
- C's restore cost (E2) also grows: a snapshot to restore is at least the
  46 MiB of metadata plus the session state.

## A0 result: pooling allocator (2026-11-22)

`COGWASM_POOLING=1` (opt-in, `crates/cogwasm-host/src/config.rs`) switches
the engine to Wasmtime's pooling allocator, so each per-job instance reuses a
linear-memory slot. `COGWASM_POOLING_KEEP_RESIDENT_MB` keeps that much of the
slot resident and zeroes it in place instead of decommitting it. The existing
`.cwasm` still loads with pooling on (module load ≈ 17 ms), so the allocation
strategy does not affect the compatibility hash.

`edit-loop`, resident backend, 20 edits, macOS arm64, two interleaved rounds:

| allocator | median (round 1 / round 2) |
| --- | ---: |
| default (on demand) | 39.5 / 40.2 ms |
| pooling, keep resident 0 | 41.9 / 44.7 ms |
| pooling, keep resident 256 MiB | 42.5 / 43.4 ms |
| pooling, keep resident 1 GiB | 40.6 / 41.2 ms |

**No gain on macOS**: pooling is level or slightly slower. Zeroing a reused
slot costs about what first-touching fresh pages does, so the "~10 ms of
first-touch" estimate under E1 is not recovered this way. That estimate was
inferred from host RSS, which also counts host-side memory, so it was
probably too high. Linux decommits with `madvise` and refaults differently,
so `ci/build-and-test.sh` runs one pooling `edit-loop` beside the default one
and reports it as a separate notice (temporary). If Linux shows no gain
either, A0 is dropped and the opt-in removed.

### A0 on Linux (CI run 44969263935, `b29b814`, ubuntu-latest x86_64)

The same job ran `edit-loop` twice back to back: the default allocator, then
pooling with `COGWASM_POOLING_KEEP_RESIDENT_MB=1024`.

| allocator | per-edit edit → result (10 edits) | median | cold build |
| --- | --- | ---: | ---: |
| default (on demand) | 130.4–136.5 ms | **131.3 ms** | 150 ms |
| pooling, keep resident 1 GiB | 93.0–104.1 ms | **96.5 ms** | 162 ms |

**On Linux pooling cuts edit → result by about 35 ms (−27%) and meets the
100 ms target**, where macOS showed nothing. All of the difference is in the
build (swap 0.8 ms and link 4 ms are the same in both), i.e. in rustc's run
inside the guest: on Linux, mapping fresh linear memory for every job and
faulting in its pages is expensive, and reusing a resident, in-place-zeroed
slot avoids it. The default loop shows no warm-up trend over its ten edits,
so running second does not explain the gap.

Caveats:

- One run, one order. Runner hardware varies between runs (the default
  allocator measured 174 ms on run 44912655095 and 131 ms here), so the
  comparison is only valid within a job. It should be repeated with the order
  swapped and with other keep-resident sizes (0, 256 MiB) before the
  setting is fixed.
- The first build is slower with pooling (162 vs 150 ms), the cost of setting
  up the slot.
- With `keep_resident` at 1 GiB, each job zeroes up to 1 GiB in place, so on a
  job that touches more memory the saving could shrink. Plugin-sized sources
  should be measured too.

### A0 confirmed and adopted (CI run 44972190861, `d653fd0`)

`ci/edit-loop-ab.sh` ran six interleaved configurations (pool1024, default,
pool0, pool256, default, pool1024), 10 edits each, on the fixture and on
`bench/programs/medium/json.rs` (562 lines plus the prelude, uses `alloc`).
Medians in ms:

| source | default | pooling, keep 0 | pooling, keep 256 MiB | pooling, keep 1 GiB |
| --- | ---: | ---: | ---: | ---: |
| fixture | 124.0 / 126.3 | 125.9 | **93.4** | 94.8 / 94.1 |
| json | 546.0 / 532.5 | 540.8 | **503.0** | 509.5 / 498.6 |

- **The win is keeping memory resident, not pooling as such.** With nothing
  kept resident, pooling is identical to the default allocator.
- **256 MiB is enough.** It matches 1 GiB on both sources.
- **The saving is a fixed ≈ 31–36 ms per job**, not a percentage: −25% on
  the fixture, −7% on `json`. It is the cost of mapping and first-touching a
  fresh linear memory on Linux, which every per-job instance paid.
- Both orders agree (pool1024 ran first and last; default second and fifth),
  so order does not explain it.

**Decision:** the resident backend now uses `Allocation::RESIDENT_DEFAULT`
(pooling, 256 MiB kept resident; `crates/cogwasm-host/src/config.rs`). The
one-shot CLI and `cogwasm-precompile` keep the on-demand allocator.
`COGWASM_POOLING=0` turns it off and `COGWASM_POOLING_KEEP_RESIDENT_MB`
resizes it. The CI comparison step is removed; `ci/edit-loop-ab.sh` stays for
manual reruns.

What it means for Goal 2: the fixture's edit → result on CI is now about
95 ms, just under the target, but a plugin-sized source is about 500 ms. The
remaining fixed cost (crate loading, session set-up, link) is still the
lever, and on `json` the per-source work is large too. Next: E2/E4, then
option A.

## E2 and E4 on the CI runner, and the WASI read fix (2026-11-22)

`ci/profile-fixed-cost.sh` ran on ubuntu-latest (x86_64) in run 44975284082.

**E4 (rustc passes, median of five, ms):**

| pass | macOS | runner, before | runner, after fix (run 44978461782) |
| --- | ---: | ---: | ---: |
| `macro_expand_crate` | 10.2 | 74.3 | **20.3** |
| `resolve_crate` (of which late resolution) | 10.5 | 71.9 (67.8) | **20.3** (19.2) |
| type check, borrowck, mono, codegen | ≈ 5 | ≈ 5–8 | ≈ 5–9 |
| `link` | 8.0 | 6.6 | 4.6 |
| **total, empty crate** | 39.5 | **166.2** | **54.7** |

The runner was not uniformly 4.2× slower: the compiler's own work ran at
macOS speed, and expansion plus resolution were 88% of the time. **E1 on the
runner** pinned it on I/O: reading libcore + compiler_builtins rmeta took
70.8 ms in a fresh guest and 44.7 ms with memory already grown (≈ 1 GB/s).

**Cause.** `wasmtime-wasi` serves a file `fd_read` either directly into wasm
memory or, unless the context allows blocking the current thread, by
allocating a zeroed host `Vec` the size of the read, running the read on a
tokio blocking thread and copying the result in (`p1.rs`, `fd_read`;
wasmtime issue 7973). Our hosts never set `allow_blocking_current_thread`,
so every 45 MB rmeta read paid a fresh 45 MB allocation, which on Linux costs
about as much as E2's fresh-memory figure. **Fix** (`833d995`, in
`cogwasm-runtime`'s `prepare_instance`): `allow_blocking_current_thread(true)`.
Both hosts are synchronous and rustc.wasm's memory is not shared, so reads go
straight into guest memory. Compiler fuel is unchanged.

**E1 after the fix:** 20.6 ms fresh, 7.0 ms with memory grown (runner).

**E2 (restore cost):** a compile peaks at about 143 MiB of guest memory
whatever the program (bench history: 149.4–151.8 MB). Copying that on the
runner: 7.7–18.5 ms into resident memory (two runs, different machines),
60–140 ms into fresh memory. A snapshot restore (option C) is only viable
into a resident slot, which the pooling allocator provides; at ≈ 8–19 ms it
costs about as much as the ≈ 40 ms of crate loading and session set-up it
would skip, minus a margin, so C is not attractive yet.

**Effect on edit → result (CI runner):**

| measurement | before (run 44975284082) | after (run 44978461782) |
| --- | ---: | ---: |
| resident `edit-loop`, fixture | 134.9 ms | **53.6 ms** (47.4–55.6) |
| `cogwasm-host incremental`, plugin (c) | ≈ 290–330 ms | **119–132 ms** |

What is left on the runner for a small edit: ≈ 20 ms expansion and ≈ 20 ms
resolution (still mostly crate loading: the read is ≈ 7–20 ms of it), ≈ 5 ms
link, ≈ 10 ms of the rest. Option A (a long-lived guest that keeps the
metadata) now targets that ≈ 40 ms, which matters for plugin-sized programs
(plugin (c) is still above 100 ms) more than for the fixture.

## Recommendation

1. E1, A0, E2 and E4 are done (above). E4 found the real CI cost: WASI file
   reads going through tokio's blocking pool. With
   `allow_blocking_current_thread` the runner's edit → result is 53.6 ms,
   under target. Option A and D are now about margin and larger programs, not
   about reaching 100 ms on a small edit.
2. Implement **A** (guest request loop plus metadata cache) and
   **D**'s in-process link, then measure with `edit-loop`. Together they
   plausibly bring the runner to about 110–120 ms.
3. Take on **C** only if A plus D leaves the target clearly missed and E2
   shows a restore can be cheap. It is the only route under 100 ms with
   margin, and also the riskiest.
4. Whatever is chosen, CI's `edit-loop` step and difftest are the acceptance
   checks, and a warm compile is compared byte for byte with a cold compile of
   the same source in a new CI assertion.

## Open questions

- Does `-Cincremental` interact with a long-lived guest (A)? Its on-disk
  state is per session today, so it should be unaffected, but the
  incremental session lock (`0003-wasm-file-locks.patch`) needs checking.
- Can the request loop live in the existing driver patch
  (`0001-wasm-run-compiler-and-codegen-on-calling-thread.patch`), or does it
  need its own?
- For C: what is the earliest point at which the resolver can load `core`
  without the parsed crate, and does `StableCrateId` need anything beyond the
  crate name and `-C metadata`?
