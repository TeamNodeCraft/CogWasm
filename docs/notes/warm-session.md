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

## Recommendation

1. Run E1, E2 and E4 first; they are cheap and decide between A and C.
2. If H1 holds: implement **A** (guest request loop plus metadata cache) and
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
