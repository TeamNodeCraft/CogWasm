# Incremental compilation in `rustc.wasm`

Status: **working, opt-in** (measured on the final artifact below). The goal is the ROADMAP
line "Incremental compilation kept across edits": with the host keeping one
Wasmtime engine and one compiled `rustc.wasm` module resident, each one-line
edit should compile and link faster than a cold compile and must not change
the result.

## What blocked it under wasm

### 1. File locks (fixed by `0003-wasm-file-locks.patch`)

WASI preview 1 has no file-locking API, so on `wasm32-wasip1`
`std::fs::File::try_lock` returns `ErrorKind::Unsupported`. rustc's fallback
for that is `compiler/rustc_data_structures/src/flock/unsupported.rs`, which
rejects every lock. Incremental compilation must create and lock a session
directory before anything else, so every `-C incremental` compile died with:

```text
error: incremental compilation: could not create session directory lock file: file locks not supported on this platform
```

`0003` adds `compiler/rustc_data_structures/src/flock/wasm.rs`, selected with
`cfg_select!` for `target_family = "wasm"`. It keeps the same process-local
registry as the Unix fallback (`flock/unix.rs`): one exclusive holder, or any
number of shared holders, keyed by path, released when the last `Lock` for
that path is dropped. It performs no `fcntl` call because there is nothing to
call; the lock file is still opened and held for the lifetime of the lock,
matching the Unix fallback's lifetime rules.

The tests run on the host (the module is compiled under
`cfg(any(test, target_family = "wasm"))`) and cover:

* an exclusive lock blocks a second exclusive and a shared lock in the same
  process;
* shared locks coexist and block an exclusive lock until the last one is
  dropped;
* dropping the lock releases the path.

**Limitation, on purpose:** a WASI p1 process cannot coordinate locks with
another process. The stage-C host runs one `rustc.wasm` instance at a time and
does not share an incremental directory with another running compiler
process. Running two hosts against one incremental directory would make the
garbage collector's "is this session still in use" decision unsafe; that is a
host contract, not something the fallback can enforce. WASI preview 2 /
`wasi:filesystem` lock types (or a host-provided lock import) would be needed
to lift it.

### 2. Cached CGUs and the synchronous coordinator (fixed by `0004-wasm-sync-cached-cgus.patch`)

`0001` runs codegen synchronously on wasm instead of starting
`rustc_codegen_ssa`'s coordinator thread. It handled freshly compiled CGUs
(`submit_codegened_module_to_llvm`) but not the two cached-CGU submit
functions, which still sent a `Message` to the coordinator channel that no
thread drains.

On the second incremental compile, unchanged CGUs are reused with
`CguReuse::PostLto` and submitted through `submit_post_lto_module_to_llvm` as
`WorkItem::CopyPostLtoArtifacts`. The message was dropped, so the reused
object files never reached the linker. Plugin (c) then failed in Wild with
undefined `drop_glue`, `RawVec::try_allocate_in`, `Iterator::sum` and similar
local generic instantiations. The first incremental compile worked because
every CGU was fresh, and `-Ccodegen-units=1` hid the bug because there is no
cross-CGU reuse.

`0004` makes the synchronous path copy cached artifacts inline with the same
`execute_copy_from_cache_work_item` the coordinator thread uses, and pushes
the result into `CompiledModules`. Pre-LTO reuse only exists for LTO builds,
which cg_clif does not support on wasm, so that path now fails loudly instead
of silently dropping the module.

### Things checked that did **not** need a patch

* **mmap**: `rustc_data_structures::memmap` already falls back to `Vec<u8>`
  on `target_arch = "wasm32"`; dep-graph/query-cache loading never calls
  `memmap2`.
* **fsync, timestamps, rename, `remove_dir_all`, hard links**: WASI p1
  provides `fd_sync`, `fd_filestat_set_times`, `path_rename`,
  `path_remove_directory` and `path_link`; wasi-libc and Wasmtime 50.0.2
  implement them. The session-directory protocol (rename `-working` to
  `-<svh>`, hard-link-or-copy work products, delete the old session) ran
  without a source change. No sleeps or retries were added.
* **compiler/backend caching**: rustc's generic incremental support works with
  cg_clif; no `-Z` flag was needed. (`-Zincremental-*` defaults are unchanged.)

## Host support

`crates/cogwasm-host/src/main.rs`:

| Option / command | Behaviour |
|---|---|
| `--incremental DIR` | off by default; passes `-Cincremental=DIR` to `rustc.wasm`. `DIR` must be absolute because the host preopens host `/` as guest `/`; it can live anywhere on the host filesystem and persists between `rustc.wasm` instances and between host invocations. It is never deleted implicitly. |
| `--fresh-incremental` | requires `--incremental`; deletes only that dedicated directory before compiling (refuses `/`, shallow paths and `$HOME`). Used by the benchmark driver so a run starts clean. |
| `--extern NAME=PATH` | repeatable; needed for e_math's `libm` rlib on the direct rustc path. |
| `incremental` | cold compile with no incremental directory, then N one-line edits with the same resident engine/module and a fresh `rustc.wasm` instance per edit. Prints and writes JSON with cold / first-incremental / median compile, link and edit-to-result times, eval result, cold-vs-incremental wasm SHA-256 and result equality, and incremental-directory file/byte counts. Exits nonzero if the result changes. After the measured edits it also compiles a *semantics-changing* edit (a constant that shifts the result by a known amount) through the same incremental directory; a stale cache would return the old result and fail. |
| `check`, `bench`, `raw` | unchanged defaults; `--incremental` also works on `check`/`raw` for manual iteration. |

`compiler/run.sh incremental` runs the benchmark for both programs and fails
unless the result is correct: plugin (c) must return `eval(1,2,3,4) == 2.25`
on every run; e_math is checked with the same input sequence, transform and
`eval_stats` reference values as `a2-harness check` (mean/min/max, 1e-9
relative tolerance). For e_math, the `libm` dependency rlib is obtained from a
normal cargo build through `rustc-wasm-wrapper.sh`, and the exact
`--extern libm=...` path cargo chose is read from its verbose log, so the
measurement still runs through the resident host.

`difftest/incremental-check.sh` is the regression check for the file-lock /
cached-CGU work: it compiles `corpus/smoke.rs` cold and twice with one
persistent incremental directory, validates all three modules and requires
them to be byte-identical.

## Commands

```sh
# One-time rustc.wasm build (applies rust patches 0001-0004; the script loops
# over all compiler/patches/rust/*.patch and is idempotent).
compiler/scripts/build-rustc-wasm.sh

# Cold vs incremental, median of 5, for plugin (c) and A.2 (e).
compiler/run.sh incremental
# Raw JSON: docs/results/incremental-c.json, incremental-e.json

# Default path must stay unchanged.
compiler/run.sh check

# Differential correctness.
difftest/incremental-check.sh
difftest/run.sh --seeds 24
```

## Measurements

Final artifact `rustc.wasm` 144,051,631 B SHA-256 `d2404a86…6fb91` (all four
rust patches plus the codegen patches), Wasmtime 50.0.2, Apple M4 Pro. Median
of 5 one-line edits, resident engine/module, fresh `rustc.wasm` instance per
edit (`docs/results/incremental-c.json`, `incremental-e.json`):

| program | cold compile | first incremental | steady incremental | cold edit-to-result | incremental edit-to-result | incremental dir |
|---|---:|---:|---:|---:|---:|---:|
| plugin (c) | 96.1 ms | 94.6 ms | **79.9 ms** | 113.7 ms | **97.7 ms** (−14.1%) | 590 KB / 11 files |
| A.2 (e) | 105.9 ms | 106.3 ms | **88.2 ms** | 136.7 ms | **119.8 ms** (−12.4%) | 652 KB / 11 files |

Compile inside `rustc.wasm` falls by 16.9% (c) and 16.7% (e). The first
incremental run after the cold build has no cached CGUs to reuse, so it is
about as slow as the cold compile; the win appears from the second edit on.

The output-size cost is real. Incremental uses a finer CGU/shared-generics
split than a normal release build, so fewer tiny helpers are inlined by the
2026-11-19 clif-to-wasm inliner:

| program | cold wasm | incremental wasm | functions | code bytes |
|---|---:|---:|---:|---:|
| corpus `smoke.rs` | 64,616 B | 75,555 B (+16.9%) | 131 → 190 | 30,687 → 35,452 |
| plugin (c) | 34,583 B | 40,783 B (+18%) | 92 → 120 | 22,990 → 25,520 |
| A.2 (e) | 58,252 B | 65,299 B (+12%) | 150 → 180 | 39,152 → 42,129 |

`-Ccodegen-units=16` with incremental does not change the function count
(120 for plugin (c), same as 256), so the extra functions come from incremental
putting generic instantiations in their defining crate's CGUs, not from the CGU
count. A2 runtime of the incremental module is within noise of the cold one
(6.91 vs 6.83 ms for 100k points).

## Correctness evidence

* `difftest/incremental-check.sh`: cold and two persistent-directory runs of
  corpus `smoke.rs` have identical `run(seed)` values and output buffers for
  seeds 0, 1, 42 and 0x9e3779b97f4a7c15; only the byte size differs.
* `compiler/run.sh incremental`: plugin (c) `eval(1,2,3,4) == 2.25` and e_math
  mean/min/max match the A.2 reference on every run, cold and incremental. A
  final semantics-changing edit moves the result by the expected amount, so a
  stale cache cannot pass.
* `compiler/run.sh check` still passes on the default (non-incremental) path,
  and `difftest/run.sh --seeds 24` passes 72 programs / 288 calls / 0
  mismatches on the final stack.

## Is it worth enabling by default?

Not yet, but it is worth offering. It is correct and gives a consistent
12–14% edit-to-result win (16–17% less compile) for repeated one-line edits of
one crate. It also costs 12–18% more output size and the first edit after a
cold build gains nothing. The default stays off (`--incremental` is opt-in)
until the cold and incremental codegen paths inline the same helpers (for
example by teaching the wasm inliner to see bodies across CGUs, or by using a
coarser external-partition scheme).
