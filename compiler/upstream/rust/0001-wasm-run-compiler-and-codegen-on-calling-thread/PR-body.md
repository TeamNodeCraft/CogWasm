# wasm: run the compiler and codegen coordinator on the calling thread

> **Draft research note, not submission text.** A human owns any future
> upstream contribution and must write the issue/PR in their own words under
> the project's current AI policy. See `../../README.md` and
> `CogWasm/CONTRIBUTING.md`.

## Problem

A rustc built for a WebAssembly host without threads cannot run. Two separate
places force a spawned thread:

* `rustc_interface::run_in_thread_with_globals` always builds a `"rustc"`
  thread with `std::thread::Builder::new().stack_size(...)` and runs the
  compiler closure on it.
* `rustc_codegen_ssa::back::write::start_executing_work` always spawns a
  `"coordinator"` thread, even when there is a single codegen unit and even
  when `-Zthreads=1` is used.

`wasm32-wasip1` has no `wasi:thread-spawn`. Wasmtime removed
`wasmtime-wasi-threads` in 47.0.0, so a rustc.wasm built with the normal
threading model can no longer run on current engines. An embedder has to
either pin an old engine or remove the two spawns.

## Fix

When rustc itself is compiled for `wasm32`:

* Run the compiler closure directly on the calling thread in
  `run_in_thread_with_globals`. The guest stack is fixed at link time with
  `-Clink-arg=-Wl,-z,stack-size=...`; `thread_stack_size`/`RUST_MIN_STACK` do
  not apply and are deliberately ignored.
* Run codegen synchronously in `rustc_codegen_ssa`. `start_executing_work`
  returns a `CoordinatorFuture`, which is either the existing coordinator
  thread or a new `SyncCodegen` state. `SyncCodegen` optimizes and codegens
  each submitted CGU immediately and finalizes the allocator module in `join`.
  The channel-based waits and the blocking pre-join emitter check are skipped
  in this mode; the post-join checks still report the same diagnostics.

The native path is untouched: on non-wasm hosts the code still spawns the
compiler and coordinator threads exactly as before.

## Tests

Against current `rust-lang/rust` `main` (`e3b9a8345a8a58fda3ac9446c532625d5a59acd8`):

```text
./x.py check compiler/rustc_interface compiler/rustc_codegen_ssa
    Build completed successfully in 0:00:40

./x.py fmt --check
    fmt: checked 7458 modified files
    Build completed successfully in 0:00:26
```

End-to-end evidence from the development tree (not rerun for this
submission): a rustc built for `wasm32-wasip1` (non-threads) with these two
changes, run under Wasmtime 50.0.2 with plain WASI preview 1, imports no
`wasi:thread-spawn`, and compiles a `no_std` plugin that evaluates
`eval(1,2,3,4) == 2.25`. The compiler and the codegen coordinator both ran on
the calling thread.

There is no in-tree rustc test for this change. Building a wasm-host rustc is
not part of normal compiler CI, so a `tests/ui` or `run-make` test would need
a new wasm32-wasip1-host build job. A maintainer may reasonably ask for one;
see "Notes" below.

## How found

While building rustc for `wasm32-wasip1` to run a compiler inside a WebAssembly
runtime without `wasi-threads`. The two forced spawns were the only blockers;
the parallel codegen path, the jobserver helper and codegen worker threads are
not reached with `-Zthreads=1` and no `parallel-compiler` feature.

## Upstream status

No equivalent change exists on current `main`:

* `compiler/rustc_interface/src/util.rs` and
  `compiler/rustc_codegen_ssa/src/back/write.rs` at
  `e3b9a8345a8a58fda3ac9446c532625d5a59acd8` still spawn both threads
  unconditionally. Neither file contains a `target_arch = "wasm32"` gate.
* `git log -G'target_arch = "wasm32"'` over both files for the history
  available in the scratch clone (since 2025-06) has no match.
* GitHub issue/PR searches found no equivalent. Related, but not equivalent:
  * https://github.com/rust-lang/rust/issues/126857 (wasm32-wasip1-threads
    documentation)
  * https://github.com/rust-lang/rust/issues/153475 (`std::thread::spawn`
    disabled on WASI)
  * https://github.com/rust-lang/rust/issues/146721 (thread spawn broken on
    wasm32-wasip1-threads)
  None of them changes rustc's own compiler/codegen threading.

Recommended disposition: keep. This is not a duplicate.

## Conflicts

Rebased from `b6eeb93c992027a03e1fd120b6652546396b142c` (2026-10-18) to
`e3b9a8345a8a58fda3ac9446c532625d5a59acd8` (2026-11-18):

* `rustc_interface/src/util.rs` still matches the original context; the hunk
  applies with an offset.
* `start_executing_work` gained an unrelated
  `let incr_comp_session = ...` line at the top of the function. That line is
  inside the 3-line trailing context of the return-type hunk, so the refreshed
  patch does not apply to the older `b6eeb93` tree with default context. It
  applies with `git apply -C1`, and the semantic change is unchanged.
  `rustfmt` reflowed a few expressions.
* No API was renamed in the touched code.

Against the older pinned tree, all code hunks still apply; the only difference
is the upstream-added context line above.

## Licence

MIT OR Apache-2.0, as a contribution to the Rust project.

## Notes

**Is this likely acceptable upstream as-is?** It is a small, target-gated
change and the native path is untouched, but wasm hosts are niche and rustc
has no supported wasm host target. Expect the change to need a design
discussion rather than a quick merge.

**Questions maintainers will probably ask:**

* *`cfg(target_arch = "wasm32")` vs `target_os`/`target_family`.* The current
  gate also matches `wasm32-wasip1-threads`, where the normal spawned-thread
  path works. A narrower gate would be
  `cfg(all(target_family = "wasm", not(target_feature = "atomics")))`, which
  leaves threaded wasm hosts alone and also covers wasm64. The patch keeps the
  original `target_arch = "wasm32"` gate to stay minimal; this is the first
  thing to settle in review.
* *Thread and stack semantics.* Running on the caller's thread means the
  caller must provide the stack; `RUST_MIN_STACK` and
  `run_in_thread_with_globals`'s `thread_stack_size` are silently ignored.
  The guest stack is set at link time instead. Panic propagation changes from
  `JoinHandle::join` to `resume_unwind` on the calling thread.
* *Effect on `wasm32-wasip1-threads`.* With the current gate, a threaded wasm
  host also loses its compiler/coordinator threads. This is correct but
  unnecessary; see the narrower gate above.
* *Tests.* There is no cheap in-tree test. The natural test is a
  `run-make`/CI job that builds rustc for `wasm32-wasip1` and inspects the
  imports for the absence of `wasi:thread-spawn`, which is much larger than
  this patch.
* *MSRV/bootstrap.* No new language or library features are used; `RefCell`,
  `cfg!`, and the existing channel types are already in the compiler.

**Best venue.** Start a Zulip thread in `#t-compiler` (compiler scheduling and
`rustc_interface`/`rustc_codegen_ssa`) and cross-post to `#wg-cranelift`, then
open a PR only after the gate and the test expectation are agreed. The change
is not suitable for a drive-by PR.
