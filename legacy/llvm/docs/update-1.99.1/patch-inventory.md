# rustc.wasm — phase A patch inventory (target: Rust 1.99.1)

Local analysis only. No pushes, no repo creation, no CI. Work area:
`/Users/alexandermitchell/Documents/github/rust-wasm-compiler`.

## TL;DR

- The fork branch `oligamiq/rust@compile_rustc_for_wasm17` (`f7a223c1c21d00cbd63b75f8836b8e64192f24b5`)
  contains **6 patch commits** on top of upstream commit `39cb3386ddc`.
- The merge base is `39cb3386ddc6c71657418be28dbb3987eea4aa4b` (2024-11-27), whose
  `src/version` is **1.85.0**, not 1.83. The `v3.0.0` release was actually built from
  `cf327c2068549194a29160499c2ecafa9061e46e` (1.83.0) on the older
  `compile_rustc_for_wasm16_cargo` branch: `rustc_llvm_with_lld.yml` and
  `rustc_minimal.yml` checked out the wasm17 branch and then hard-checked out
  `cf327c2`, silently overriding it.
- Rebase branch: **`compile_rustc_for_wasm-1.99.1`**, based on tag `1.99.1`
  (peeled commit `58cf8f9c35b355199727627d07ae34a4e2846d67`), **7 commits**
  (the five rebased patches plus the new `cc` 1.2.65 fix and the wasi-sdk 34
  libc++ fix). Patch 3 (`Update comment.txt`) was dropped as
  documentation-only.
- **No commit was blocked**, but one important version decision was needed:
  Rust 1.99.1 pins LLVM `rustc/23.1-2026-07-22`, while YoWASP's wasm LLVM fork
  stops at `llvmorg-22.1.0+wasm` (`97196c8e…`). Rust 1.99.1 accepts LLVM >= 21
  (`src/bootstrap/src/core/build_steps/llvm.rs:651-663`) and has LLVM-22/23
  guards in `rustc_llvm/llvm-wrapper`, so the branch pins the **22.1.0+wasm**
  fork. A 23.x wasm port does not exist yet. This is the main build risk.
- Net rebased diff vs `1.99.1`: **18 files, +484/−27**.

## Sources and SHAs

| What | Ref | SHA |
|---|---|---|
| Patch source branch | `oligamiq/rust` `compile_rustc_for_wasm17` | `f7a223c1c21d00cbd63b75f8836b8e64192f24b5` |
| Merge base / upstream parent | upstream master | `39cb3386ddc6c71657418be28dbb3987eea4aa4b` |
| Upstream target tag | `rust-lang/rust` `1.99.1` | `58cf8f9c35b355199727627d07ae34a4e2846d67` |
| Old built commit (release v3.0.0) | `compile_rustc_for_wasm16_cargo` | `cf327c2068549194a29160499c2ecafa9061e46e` |
| LLVM fork branch used | `YoWASP/llvm-project` `llvmorg-22.1.0+wasm` | `97196c8eeb1d495fa43bb8af2fb26af5ef5b89fb` |
| wasi-sdk (latest release) | `wasi-sdk-34` | assets `wasi-sdk-34.0-*` |

The clone in `rust/` is a partial clone (`--filter=blob:none`); the oligamiq
branch was fetched with `--depth=400`.

## Patch series (original)

Base `39cb338` → tip `f7a223c`. All six commits are authored by bjorn3/oligami
and are ordered oldest → newest:

| # | Commit | Subject | Files | Size |
|---|---|---|---|---|
| 1 | `2af9f557993` | `[WIP] Support compiling rustc to WebAssembly` | 22 | +404/−36 |
| 2 | `08ec940e111` | `Compile rustc+LLVM to wasm` | 10 | +269/−10 |
| 3 | `132c6388ad7` | `Update comment.txt` | 1 | +1/−1 |
| 4 | `bab49c215b1` | `Link against lld for linking compiled object files` | 5 | +43/−2 |
| 5 | `315dd3f99d9` | `Make linking for wasm32-wasip1 work by default` | 5 | +11/−8 |
| 6 | `f7a223c1c21` | `Disable rpath for lld when requested` | 1 | +3/−0 |

### Grouped by purpose

**1. In-process LLD / no process spawn**
- `compiler/rustc_codegen_ssa/src/back/link.rs`: call `RustRunLld(argc, argv)`
  instead of spawning the linker (commit 4).
- `compiler/rustc_llvm/llvm-wrapper/LLD.cpp`: new `LLD_HAS_DRIVER(elf)`,
  `LLD_HAS_DRIVER(wasm)`, `RustRunLld` wrapper around `lld::lldMain` (commit 4).
- `compiler/rustc_llvm/build.rs`: compile `LLD.cpp`; link `LLVMOption`,
  `lldCommon`, `lldELF`, `lldWasm` (commit 4).
- `src/bootstrap/.../llvm.rs`: enable the `lld` CMake project for the wasm-host
  LLVM (commit 4).
- `compiler/rustc_codegen_ssa/src/back/linker.rs`: do not build `PATH` on wasm
  (commit 1).

**2. sysroot / `current_dll_path` / dlopen**
- `compiler/rustc_session/src/filesearch.rs`: WASI `current_dll_path` /
  `sysroot_candidates` stub (commit 1; now upstreamed, see below).
- `compiler/rustc_metadata/src/creader.rs`: cfg-gate `libloading`/dlopen code and
  provide a `load_symbol_from_dylib` stub for non-unix/windows (commit 1; code
  moved to `rustc_metadata/src/host_dylib.rs` in 1.99.1).

**3. Threads / stack**
- `compiler/rustc_interface/src/util.rs`: on wasm run
  `run_in_thread_with_globals` directly on the main thread; raise the default
  stack to 32 MiB (commit 1).
- `compiler/rustc_codegen_cranelift/src/driver/aot.rs` and `src/lib.rs`:
  remove the concurrency limiter and `thread::spawn`, disable the hot-plug
  `#[no_mangle]` entrypoint (commit 1; obsolete upstream, see below).
- `wasm32_wasip1_threads.rs`: shared memory, 1 GiB max memory,
  `-lwasi-emulated-mman` (commits 1–2).
- `src/bootstrap/.../llvm.rs`: `LLVM_ENABLE_THREADS=ON/OFF`, `-pthread`,
  `-D_WASI_EMULATED_MMAN`, 4 GiB linker max memory, 1 MiB C++ stack (commit 2).

**4. tmpdir / process identity**
- `link.rs`: create the link temp dir in the output directory on wasm (commit 1;
  superseded upstream, see below).
- `rustc_driver_impl/src/lib.rs`: use pid `1` for ICE file names on wasm
  (commit 1).

**5. jobserver**
- No direct jobserver patch. `jobserver` only gets linked when
  `cgcx.parallel` is `Some` (`rustc_codegen_ssa/src/back/write.rs:1027`,
  `:1270`); the wasm compiler is non-parallel, so the helper thread is not
  created. The cg_clif patch added a `jobserver` dependency only to replace the
  removed concurrency limiter.

**6. Panic**
- The recipe builds `panic=abort` and `panic=unwind` variants separately. The
  compiler patches only adjust the ICE file name/pid; the spike's "diagnostics
  then `unreachable` trap" comes from `panic=abort`, not from a patch.

**7. Bootstrap / `x.py` / config**
- Commit 1: new `config.toml` (cranelift host), workspace/Cargo changes,
  `cranelift` features, `CodegenBackend` skip lists, `config.toml` default.
- Commit 2: new `config.llvm.toml`, `llvm.rs` WASI CMake block,
  `rustc_llvm/build.rs` WASI flags, `libc++.a`, `.gitmodules` +
  `src/llvm-project` submodule, `linker.rs` `-O1` removal.
- Commit 5: `src/bootstrap/src/core/builder/cargo.rs`
  `-Clink-self-contained=no` for wasi; drop `-march=native` from the config.
- Commit 6: `CMAKE_SKIP_RPATH=ON` when building LLD without rpath.

**8. CI-only / docs**
- `comment.txt` (commits 1–3), `.gitignore` entries. No workflow files are in
  the compiler patch series; CI lives in `rust_wasm`.

## Toolchain config (`oligamiq/toolchain-for-building-rustc`)

- The `gh-actions` branch referenced by the task does **not** exist in that repo
  now (`git ls-remote --heads` shows only `main`, `v0.0.0`, `v0.1.0`). The
  `gh-actions` refs in the workflows belong to `rust_wasm`, not the toolchain.
- The repo's `toolchain-for-building-rustc/config.toml` template sets:
  - `profile = "compiler"`, `change-id = 9999999`
  - `[rust] codegen-backends = ["llvm"]`, `deny-warnings=false`,
    `llvm-bitcode-linker=false`, no debug info
  - `[llvm] static-libstdcpp=true`, `ninja=false`, `download-ci-llvm=false`,
    `link-shared=false`, `targets="WebAssembly;X86"`
  - `[build] docs/extended=false`, `tools=[]`,
    `host=["wasm32-wasip1-threads"]`,
    `target=["x86_64-unknown-linux-gnu","wasm32-wasip1"]`,
    `cargo-native-static=true`
  - `[install] prefix="dist"`, `sysconfdir="etc"`
  - per-target `wasi-root`/`linker` = wasi-sdk 24 paths and
    `codegen-backends=["llvm"]`; `x86_64-unknown-linux-gnu cc="gcc"`.
- The workflow runs `toolchain-for-building-rustc ... -f config.llvm.toml`
  which **generates/overwrites** `config.llvm.toml`, then seds the wasi path to
  an absolute path. The rebase adapted the branch's own `config.llvm.toml` to
  1.99.1, but in the current recipe that file is still overwritten by the
  generator. Phase B should either (a) let the generated config remain
  authoritative and bump its wasi-sdk sed to 34, or (b) stop overwriting the
  branch config. The chosen recipe change keeps (a) and bumps the sed.

## Upstream changes that made hunks unnecessary (proof)

| Dropped hunk | Why | Where in 1.99.1 |
|---|---|---|
| `filesearch.rs` `current_dll_path`/`sysroot_candidates` stubs | upstream added a WASI `dll_path` that returns an error | `compiler/rustc_session/src/filesearch.rs:260` (`#[cfg(target_os = "wasi")]`); absent at `39cb338` |
| `link.rs` wasm `tempdir_in(current_dir())` | upstream creates the link temp dir in the output directory, which is already a preopened dir | `compiler/rustc_codegen_ssa/src/back/link.rs:352-358` (`TempDirBuilder::tempdir_in(output.parent())`); base used global `tempdir()` |
| cg_clif `ConcurrencyLimiter` / `thread::spawn` / `#[no_mangle]` changes | cg_clif's AOT driver was rewritten; the limiter is gone and `par_map` is used, which is serial in a non-parallel compiler | `compiler/rustc_codegen_cranelift/src/driver/aot.rs` at 1.99.1; no `concurrency_limiter` module |
| cg_clif as an in-tree static backend (`Cargo.toml`, workspace exclude, `cranelift` features, bootstrap skip lists) | 1.99.1 has a first-class out-of-tree cg_clif bootstrap step; the LLVM recipe does not enable cranelift | `src/bootstrap/src/core/build_steps/compile.rs:1739` `CraneliftCodegenBackend`, `:2408` `CodegenBackendKind::Cranelift` |
| `wasm32_wasip1_threads.rs` 1 GiB max-memory hunk | upstream target spec already sets `--max-memory=1073741824` | `compiler/rustc_target/src/spec/targets/wasm32_wasip1_threads.rs:21-34`; absent at base |
| `link_self_contained = False` → `True` toggles | patch 1 set False, patch 5 set True; net no-op and upstream is already True | `wasm32_wasip1.rs:29`, `wasm32_wasip1_threads.rs:40` |
| `comment.txt` commits 1–3 | documentation only, stale (1.83/bjorn3 instructions) | n/a |
| `linker.rs` `//self.link_arg("-O1")` | undocumented global change; the wasm link goes through in-process LLD, not `GccLinker` | n/a |
| `interface/util.rs` cranelift `get_codegen_backend` arm | cranelift not used by the LLVM recipe; would require in-tree cg_clif dependencies | `compiler/rustc_interface/src/util.rs:360` at 1.99.1 |

## Rebase branch and per-patch outcome

Branch: `compile_rustc_for_wasm-1.99.1` (local only, based on `1.99.1`).

| # | Original | New commit | Outcome | Adaptation |
|---|---|---|---|---|
| 1 | `2af9f557993` | `3d08ef18932` | **adapted** | Kept WASI main-thread execution, 32 MiB wasm stack, ICE pid, `PATH` gate; retargeted dlopen gating from `creader.rs` to the new `host_dylib.rs`; dropped cranelift, tmpdir, filesearch, target-spec and config hunks as above. 5 files. |
| 2 | `08ec940e111` | `9187905acb8` | **adapted** | Kept the WASI LLVM CMake block, `rustc_llvm/build.rs` flags, `libc++.a`, and submodule change; pinned LLVM to YoWASP `llvmorg-22.1.0+wasm`; rewrote `config.llvm.toml` for 1.99.1/bootstrap keys + wasi-sdk 34 relative paths; scoped `LLVM_ENABLE_PROJECTS=lld` later. 7 files. |
| 3 | `132c6388ad7` | — | **dropped** | Documentation only (`comment.txt`), content obsolete. |
| 4 | `bab49c215b1` | `66b874f9926` | **adapted** | Kept `RustRunLld`/`LLD.cpp`/lld libs; gated `LLD.cpp` and lld link libs behind `target.contains("wasi")` so native `x.py check` does not need lld headers; removed stale `retry_on_segfault` context. 4 files. |
| 5 | `315dd3f99d9` | `c8871cb8a52` | **adapted** | Only `-Clink-self-contained=no` survives; target-spec/config changes were already upstream or done in patch 2. 1 file. |
| 6 | `f7a223c1c21` | `f5e1d56ab21` | **adapted** | Kept `CMAKE_SKIP_RPATH=ON` in the `else` of the rpath condition while preserving 1.99.1's separate linux/darwin rpaths. 1 file. |

Two commits are new in this recipe and appended after the rebased series:

| Patch | New commit | Subject | What it does |
|---|---|---|---|
| 0006 | `7bbd7e60d99` | `Update cc to 1.2.65 for wasm32-wasip1-threads clang target` | Bumps `rustc_llvm`'s exact `cc` pin and the lockfile so `wasm32-wasip1-threads` is no longer mapped to the deprecated `wasm32-wasi` clang target. |
| 0007 | `7cc730ae66c` | `Find libc++ for wasi hosts in the sysroot's noeh dir` | wasi-sdk 34 moved `libc++.a` into `lib/<triple>/{noeh,eh}/`. Resolves the multilib from the configured `wasi-root` (preferring `noeh` for LLVM's `-fno-exceptions` build), falls back to the flat layout and then clang, and makes `compiler_file` panic on a non-absolute or missing result. |

No patch is blocked. The LLVM 23 → 22.1 version skew is a documented risk, not a
merge blocker, because 1.99.1 still supports LLVM 22.

## Verification performed

- `./x.py check --config config.llvm.toml --dry-run` with the adapted config:
  **success**. Bootstrap parsed the config, planned a `wasm32-wasip1-threads`
  stage-2 host, and included `x86_64-unknown-linux-gnu` + `wasm32-wasip1`
  targets. `WASI_SDK_PATH` was unset locally, so the LLVM CMake block reported
  its warning path only; no wasm LLVM was built.
- Host `./x.py check` with a temporary `bootstrap.toml` (local aarch64 host,
  CI LLVM, `download-ci-llvm=true`): **success** in 5m26s. It compiled/checked
  `rustc_llvm`, `rustc_codegen_ssa`, `rustc_metadata`, `rustc_interface`,
  `rustc_driver_impl`, `rustc-main`, rustdoc, and the tools. This type-checks
  the `cfg!(target_family = "wasm")` branches, but **not** the
  `#[cfg(target_os = "wasi")]` in-process-LLD block or `LLD.cpp`.
- Not verified (out of cheap scope): wasm-host rustc build, LLVM 22.1.0+wasm
  compilation against rustc_llvm 1.99.1, wasi-sdk 34 layout/ABI, `panic=unwind`
  on wasip1-threads, actual rustc.wasm execution.

## Risks / open items for phase B

1. **LLVM version skew.** Rust 1.99.1 defaults to LLVM 23.1; the wasm fork is
   22.1.0. `rustc_llvm` has `#if LLVM_VERSION_GE(23, 0)` guards and bootstrap
   accepts >= 21, so this should compile, but it is untested. If it fails, port
   YoWASP's wasm patches to LLVM 23.1 or use a newer fork.
2. **wasi-sdk 34 layout.** The workflows assume
   `wasi-sdk-34.0-x86_64-linux.tar.gz` extracts to `wasi-sdk-34.0-*` and that
   `share/wasi-sysroot` + `bin/clang` exist. Asset names were checked; the
   internal target-prefixed compiler names should be verified in phase B.
   Patch 0007 resolves the `noeh`/`eh` libc++ layout, since 34 no longer ships
   a flat `libc++.a`.
3. **Recipe config overwrite.** `toolchain-for-building-rustc` still writes
   `config.llvm.toml`; the branch's adapted config is only used if that step is
   changed. The recipe currently keeps the generated config and updates its
   wasi-sdk sed to 34.
4. **Recipe sed.** The generated config still contains the toolchain
   template's `wasi-sdk-24.0-*` linker path, so the recipe now uses a
   version-agnostic `wasi-sdk-[0-9][^/"]*` sed to rewrite it to the extracted
   `rust/wasi-sdk`. This is required for SDK 34 to be found.
5. **Main-thread stack.** Running rustc on the wasm main thread means
   `DEFAULT_STACK_SIZE` no longer controls the stack; the wasm linker's
   `-z stack-size` does. The current target spec does not set it for rustc.wasm.
   Phase B should check `--print sysroot`/compile behavior and add
   `-C link-arg=-Wl,-z,stack-size=...` if needed.
6. **`panic=unwind` job.** 1.99.1 may have different wasm EH requirements; this
   is unchanged by the rebase but should be tested.

## Disk

- Peak during the host check: `rust/` ~13 GB (12 GB of `build/` artifacts).
- Removed `rust/build/` and `rust/.cargo-home/` after verification.
- Final: `rust/` ~705 MB (mostly the partial clone), `rust_wasm/` 1.1 MB,
  `toolchain-src/` 1.7 MB, `notes/` 288 KB, ~28 GB free on the volume.
