# Upstream submission staging (drafts only)

Four existing stage-C patches, refreshed against the upstream default branches
and prepared for review. Nothing here has been pushed, opened as a PR/issue, or
committed to the tracked repository. All work is in scratch clones under
`toolchain/upstream/` (gitignored).

> **Draft status.** These are research notes for a human maintainer, not
> submission text. Per [`../../../CONTRIBUTING.md`](../../../CONTRIBUTING.md),
> nobody opens a pull request, issue or discussion upstream without a decision
> to do so, a person writes the upstream text themselves, and any AI assistance
> is disclosed the way that project currently requires. The `PR-body.md` files
> are background notes, not text to paste. Re-read each project's current AI
> and contribution policy before any future submission.

Upstream default branches were verified on 2026-11-19 with `git ls-remote`:
`rust-lang/rust` and `rust-lang/rustc_codegen_cranelift` use `main`, and
`ryancbuckley/clif-to-wasm` uses `main`.

## Index

| Project | Upstream commit / date | Patch | Status | Conflicts | Tests | Licence | Venue / next step |
|---|---|---|---|---|---|---|---|
| rust-lang/rust | `e3b9a8345a8a58fda3ac9446c532625d5a59acd8` / 2026-11-18 | [`rust/0001-wasm-run-compiler-and-codegen-on-calling-thread/0001-wasm-run-compiler-and-codegen-on-calling-thread.patch`](rust/0001-wasm-run-compiler-and-codegen-on-calling-thread/0001-wasm-run-compiler-and-codegen-on-calling-thread.patch) | **New.** No equivalent found on `main` or in history; keep. | Rebased `b6eeb93` → `e3b9a83`; upstream added `let incr_comp_session` in the context of the return-type hunk (default 3-line context fails on the old tree, `git apply -C1` passes). `rustfmt` reflow only. | `./x.py check compiler/rustc_interface compiler/rustc_codegen_ssa` **PASS** (40.7 s); `./x.py fmt --check` **PASS** (7,458 files); no in-tree wasm-host test (documented). | MIT OR Apache-2.0 | Zulip `#t-compiler` + `#wg-cranelift` first; wasm hosts are niche and the `target_arch="wasm32"` gate also hits `wasm32-wasip1-threads`, so settle the gate and test plan before a PR. |
| rust-lang/rustc_codegen_cranelift | `a4af0a9fd1ce26e03364c07ab0c482d084fa97fc` / 2026-11-18 | [`cg_clif/0001-wasm-lower-memory-grow-and-size/0001-wasm-lower-memory-grow-and-size.patch`](cg_clif/0001-wasm-lower-memory-grow-and-size/0001-wasm-lower-memory-grow-and-size.patch) | **New.** No equivalent found; keep. | Rebased `4f78fd3` → `a4af0a9`; unrelated TLS test added in `build_system/tests.rs`/`config.txt` shifts the new test (default context fails on old tree, `git apply -C1` passes). Code hunk unchanged. | cg_clif wasm32-wasip1 suite **PASS**: `[AOT] wasm_memory_intrinsics` (plus f128) after building the wasm sysroot; direct compile for both wasm targets **PASS**; negative check warns-and-traps without patch. | MIT OR Apache-2.0 | cg_clif PR, paired with clif-to-wasm; depends on the wasm backend and `clif-to-wasm::WasmIntrinsic`, so it cannot compile on plain upstream yet. Coordinate in `#wg-cranelift`. |
| rust-lang/rustc_codegen_cranelift | `a4af0a9fd1ce26e03364c07ab0c482d084fa97fc` / 2026-11-18 | [`cg_clif/0002-wasm-f128-sret-return/0002-wasm-f128-sret-return.patch`](cg_clif/0002-wasm-f128-sret-return/0002-wasm-f128-sret-return.patch) | **New.** No equivalent found; keep. | Rebased `4f78fd3` → `a4af0a9`; same TLS/context shift; `src/abi/mod.rs` unchanged, code hunk applies as-is. | cg_clif wasm32-wasip1 suite **PASS**: `[AOT] f128_wasm_abi`; direct compile for wasm32-wasip1 and wasm32-unknown-unknown **PASS**; negative check fails with the `__multf3`/sret signature error without the patch. | MIT OR Apache-2.0 | cg_clif PR, stacked with the wasm backend; test needs a wasm target (`TARGET_TUPLE`). `#wg-cranelift`. |
| ryancbuckley/clif-to-wasm | `0e4e0458f5a190a7fcdbb2298d9ad780678c44bc` / 2026-11-14 | [`clif-to-wasm/0001-lower-wasm-memory-grow-and-size/0001-lower-wasm-memory-grow-and-size.patch`](clif-to-wasm/0001-lower-wasm-memory-grow-and-size/0001-lower-wasm-memory-grow-and-size.patch) | **New.** No equivalent found; keep. | None; upstream `main` is exactly the pinned revision. | `cargo test --all-features` **PASS** (95.5 s; `tests/memory.rs` 3/3); `cargo fmt --check` **PASS**; clean apply against `0e4e0458`. | Apache-2.0 WITH LLVM-exception | clif-to-wasm PR first, then bump cg_clif; no CONTRIBUTING/CI in the repo, use `cargo test --all-features` + `cargo fmt --check`. |

The two cg_clif patches both insert a test entry after `aot.float-minmax-pass`
in `build_system/tests.rs`/`config.txt`. They are independent PRs, but when
stacked the second needs a trivial rebase of those two lines. The code changes
do not conflict. The f128 test also needs a wasm backend (the test is gated on
a `wasm32` target), so both cg_clif patches are stacked on the wasm-backend
series, which is not part of this submission.


## Equivalent-change search summary

No upstream equivalent was found for any of the four. None were dropped.

| Patch | What was searched | Result |
|---|---|---|
| rust calling-thread | current `main` sources; `git log -G'target_arch = "wasm32"'` on `rustc_interface/src/util.rs` + `rustc_codegen_ssa/src/back/write.rs` (history since 2025-06); GitHub issue/PR search | No match. Related but not equivalent: [#126857](https://github.com/rust-lang/rust/issues/126857), [#153475](https://github.com/rust-lang/rust/issues/153475), [#146721](https://github.com/rust-lang/rust/issues/146721). |
| cg_clif f128 sret | current `src/abi/mod.rs`; `git log -S'indirect_f128'`; GitHub issue/PR search | No wasm f128/sret change. The existing `indirect_f128` path came from commit `6a6f2419` ("fix f128 libcall abi", 2026-07-10) for s390x/x86_64 Windows only; earlier ABI work is [PR #1546](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1546), also not wasm. Related: [#1717](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1717). |
| cg_clif memory intrinsics | current `src/intrinsics/llvm.rs`; `git log -S'llvm.wasm.memory'`; GitHub issue/PR search | No match. Plain upstream also has no wasm backend or `clif-to-wasm` dependency. |
| clif-to-wasm memory | current `src/lower.rs`; `git log --all -S'MemoryGrow'` / `-S'memory_grow'`; GitHub issue/PR search | No match. |

## Re-pointing against the pinned stage-C tree

Without modifying `toolchain/stage-b/rust`, `toolchain/cg_clif`, or
`toolchain/clif-to-wasm`, each refreshed patch was checked against a clean
`git archive` snapshot of the pinned base:

| Patch | Pinned base | `git apply --check` |
|---|---|---|
| rust | `1a2bcfc19` (stage-B base + prior-art series, without the stage-C commit) | fails with default context on the upstream-added `incr_comp_session` line; **passes with `-C1`** |
| cg_clif f128 | `4f78fd3` + stage-A wasm backend patch | fails at `config.txt` because upstream added a TLS entry; **passes with `-C1`** |
| cg_clif memory | `4f78fd3` + stage-A wasm backend patch | same `config.txt` context issue; **passes with `-C1`** |
| clif-to-wasm | `0e4e0458` clean | **passes with default context** |

## Licence mapping

As required by `CogWasm/README.md` ("Patch files"):

* `rust/**` and `cg_clif/**` patches: **MIT OR Apache-2.0** (the Rust
  project's licence). The commit messages say so explicitly.
* `clif-to-wasm/**` patch: **Apache-2.0 WITH LLVM-exception** (the crate's
  licence). The commit message says so explicitly.

Each patch is a contribution to its target project under that project's
licence, not under the parent repository's MIT OR Apache-2.0 option.

## What was not done

* Nothing was pushed to an upstream project, and no PR, issue or discussion was
  opened.
* The preparation changed no existing tracked `CogWasm` file; this change
  only adds the `upstream/` drafts. The parent session commits them in the
  repository's normal history, not in an upstream repository.
* No `git add`/`commit` was run by the preparation step.
* The cg_clif patches were tested on a scratch stage-A wasm-backend tree with
  nightly-2026-10-19, not on plain upstream `main`: plain upstream has no wasm
  backend, and its current `nightly-2026-11-17` toolchain was not installed.
  The upstream-rebased commits do apply cleanly to `a4af0a9`.
* The rust patch was `x.py check`ed and formatted, but the end-to-end
  rustc.wasm/Wasmtime run was not repeated in this submission; it is recorded
  in `../stage-c/README.md` and `../stage-c/results.md` for the same source
  changes on `b6eeb93`.
