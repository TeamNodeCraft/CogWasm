# Cranelift-wasm: a pure-Rust wasm backend for rustc (cg_clif + clif-to-wasm)

This is the cranelift-wasm experiment for the NodeCraft plugin compiler: compile
Rust to a wasm core module **without LLVM**, using
`rustc_codegen_cranelift` (cg_clif) for Rust → CLIF and
[`clif-to-wasm`](https://github.com/ryancbuckley/clif-to-wasm) for CLIF →
relocatable wasm objects. Objects are linked with either
[Wild](https://github.com/wild-linker/wild) or `rust-lld`, validated with
`wasmparser`, and run in Wasmtime.

The live pipeline is **stage C**, built by [`compiler/`](../compiler/) and run
by [`crates/cogwasm-host/`](../crates/cogwasm-host/). The stage-A and stage-B
trees have been retired; their notes and raw results are kept under
[`docs/notes/history/`](notes/history/), and the combined measurement history is
[`stage-a-results.md`](notes/history/stage-a-results.md).

**Verdict: (a), (b) and (c) build, link, validate and run.** Program (c) is a
NodeCraft-style `no_std` + `alloc` plugin with a custom `wasm_import_module`,
an f64 parameter list, allocation, and a result returned through linear memory.
(d) was the `std`/wasip1 comparison built by LLVM. Stage C now builds the plugin
sysroot with rustc.wasm itself and runs the compiler on plain WASI p1; details
are in [`docs/compiler.md`](compiler.md) and
[`docs/results.md`](results.md).

## Pins

Everything is checked out under `toolchain/` (gitignored) and built with an
isolated `RUSTUP_HOME`/`CARGO_HOME` so no other toolchain is touched.

| Component | Repository | Revision | Version / note |
|---|---|---|---|
| rustc | static.rust-lang.org | `nightly-2026-10-19` | rustc 1.101.0-nightly (`b6eeb93c9 2026-10-18`) |
| cg_clif | https://github.com/rust-lang/rustc_codegen_cranelift | `4f78fd3e5f2ce8a688865b4a6197e6d33211c09c` | upstream moved here from `bjorn3/…` |
| clif-to-wasm | https://github.com/ryancbuckley/clif-to-wasm | `0e4e0458f5a190a7fcdbb2298d9ad780678c44bc` | v0.3.0, built as a path dependency |
| wild | https://github.com/wild-linker/wild | `fd79578abb968275788428a6c639d967d974128f` | 0.11.0; built with stable 1.99.1 |
| wasmtime (host) | crates.io | `=50.0.2` | pinned once in the root `Cargo.toml` / `Cargo.lock` |
| wasmparser (host) | crates.io | `0.263` | validation |

The retained stage-A cg_clif patch is
[`notes/history/stage-a/0001-cg_clif-wasm-backend.patch`](notes/history/stage-a/0001-cg_clif-wasm-backend.patch);
the live patch series is under
[`compiler/patches/{rust,cg_clif,clif-to-wasm,wild}/`](../compiler/patches/).
The stage-A patch is derived from the wasm backport in
[`ryancbuckley/rust-playground-inbrowser`](https://github.com/ryancbuckley/rust-playground-inbrowser)
at `f5298630dda0f2dc2b44f6ae1b54add7a77d8316` (`patches/cg_clif/0001…0003`),
adapted to the pinned revisions, with the `clif-to-wasm` path dependency and
`ObjectOptions` API updated for v0.3.0. It includes `Cargo.lock` so cranelift
0.137.2 is pinned. The playground's optional stack-limit patch (0004) is not
included; it needs a `__stack_limit` global that our programs do not define.

## Layout

```
compiler/                      # build rustc.wasm + the pure sysroot
├── bootstrap.toml
├── run.sh                     # plugins/check/a2check/bench/a2/incremental
├── scripts/                   # setup-sources + rustc.wasm/host/sysroot builds
├── patches/                   # rust, cg_clif, clif-to-wasm and wild patches
└── upstream/                  # upstream PR drafts
crates/                        # host workspace
├── cogwasm-runtime/
├── cogwasm-host/              # the Wasmtime host (today's stage-c-host)
├── cogwasm-supervisor/
└── difftest-host/
guests/                        # wasm32-unknown-unknown programs + harnesses
difftest/                      # LLVM vs cranelift-wasm differential tests
ci/                            # setup-toolchain.sh, build-and-test.sh, audit
release/                       # packaging + THIRD_PARTY_NOTICES generator
docs/                          # compiler/pipeline/results docs + notes/history
toolchain/                     # downloads/build outputs (gitignored)
```

The ignored `toolchain/` directory still uses the old `stage-b/` and `stage-c/`
names for its contents; renaming that layout is a separate migration step. The
tracked code does not depend on those path names beyond the toolchain lookups.

## Reproduce

```sh
# Fetch the pinned rust source, prior-art patch series and wasi-sdk (idempotent).
compiler/scripts/setup-sources.sh

# Build rustc.wasm, the Wasmtime host and the pure sysroot.
compiler/scripts/build-rustc-wasm.sh
compiler/scripts/build-host.sh
CG_CLIF_WASM_STRICT=1 compiler/scripts/build-pure-sysroot.sh

# Build the plugins, then run the correctness checks.
compiler/run.sh plugins
compiler/run.sh check      # eval(1,2,3,4) == 2.25
compiler/run.sh a2check    # A.2 (e)/(f) correctness
```

`ci/setup-toolchain.sh` does the same setup plus the isolated rustup and the
pinned clif-to-wasm/Wild checkouts. The scripts pick wasi-sdk and rustup by
OS/arch rather than assuming macOS arm64. See [`docs/compiler.md`](compiler.md)
for the full build and measurement commands.

## CI

[`.github/workflows/cranelift_wasm.yml`](../.github/workflows/cranelift_wasm.yml)
runs on every push to `main` that touches the product directories
(`compiler/`, `crates/`, `guests/`, `difftest/`, `ci/`, `release/`, `bench/`,
the root Cargo manifests) or the workflow itself (and on `workflow_dispatch`). It is deliberately thin: one job on `ubuntu-latest`
runs [`ci/build-and-test.sh`](../ci/build-and-test.sh), which

1. installs the pinned isolated rustup (`nightly-2026-10-19` plus `1.99.1` for
   Wild) and fetches the pinned rust/clif-to-wasm/Wild/wasi-sdk inputs;
2. builds stage-C `rustc.wasm` (`wasm32-wasip1`, cg_clif only);
3. builds the Wasmtime 50.0.2 host and the pure `core`/`alloc`/
   `compiler_builtins` sysroot with `rustc.wasm` itself (`CG_CLIF_WASM_STRICT=1`);
4. runs plugin (c) end to end and asserts `eval(1,2,3,4) == 2.25`;
5. builds plugins (a), (b), (c), (e), (f) and runs the A.2 (e)/(f)
   correctness assertions (values vs a host-side reference, not timings);
6. audits `rustc.wasm` for LLVM imports, symbol names and marker strings.

The same command runs locally on macOS arm64:

```sh
ci/build-and-test.sh
```

The scripts pick wasi-sdk and rustup by OS/arch, derive the stage1 host triple
instead of assuming `aarch64-apple-darwin`, and use `nproc` when available (CI)
or `sysctl` (macOS). GitHub caches the rustup toolchain, Cargo registry/git and
the pinned checkouts, keyed on the pins. It also caches the compiler outputs -
stage1, `rustc.wasm`, the pure sysroot and the Wild binary, about 800 MB -
keyed on the patch series, the scripts that pin the rust commit, the nightly and
wasi-sdk, and the manifest the sysroot is built with, so a run whose inputs did
not change skips setup and the 26-minute `rustc.wasm` build. The 12 GB rustc
build tree is still never cached; the host is rebuilt every run, and every
correctness step and benchmark program is recompiled from source against the
cached compiler. No compiled benchmark modules are cached, so a reused compiler
cannot make a benchmark look unchanged. To exercise that path locally, pass
`--reuse-compiler`:

```sh
ci/build-and-test.sh --reuse-compiler
```

The full log is published to the
[`ci-logs` release](https://github.com/TeamNodeCraft/CogWasm/releases/tag/ci-logs),
and `rustc.wasm` plus `sysroot-pure` are workflow artifacts kept for 7 days.

### Green CI run (Linux x86_64)

[Run 44340745478](https://github.com/AlexMitchellMus/rust_wasm/actions/runs/44340745478)
on `ubuntu-latest` (4 build jobs) passed in **45m21s** (build/test step 41m37s;
setup 4m35s, rustc.wasm 26m25s, host 3m27s, pure sysroot 2m48s, A.2 check
4m03s). The runner had **120 GiB free** after the disk-space step and
**102 GiB** free at the end; `toolchain/` was **18 GB**. The saved caches total
**1.52 GB** (rustup 803 MB, Cargo registry/git 531 MB, wasi-sdk 180 MB, pinned
checkouts 4.5 MB), so the 10 GB limit is not close. The workflow artifact
(`rustc.wasm` + `sysroot-pure`) is **45,552,961 B**.

Compared with the macOS arm64 run above, Linux produced a slightly different
compiler and plugin (same source, different host toolchain/linker):

| Item | macOS arm64 (local) | Linux x86_64 CI |
|---|---:|---:|
| `rustc.wasm` | 143,849,756 B | **143,844,645 B** (SHA-256 `dcc0bcd4…`) |
| `sysroot-pure` | 56,684 KiB | **56,648 KiB** |
| plugin (c) | 34,929 B | **34,657 B** |
| plugin (c) compile inside rustc.wasm | 137.2 ms | **309.6 ms** |
| Wild link | 26.6 ms | **6.6 ms** |
| edit-to-result | 149.1 ms | **342.2 ms** |
| A.2 (e) mean / min / max | 4.70874063666322229 / 1.0 / 8.42724827497044870 | **identical** |
| A.2 (f) value (200k host calls) | 41.0753225026058786 | **identical** |

The A.2 values are bit-identical; only sizes and timings differ. The Linux
runner is slower per compile but the pipeline is otherwise the same.

CI iterations (each fix a separate commit):

| Run | Result | Failure → fix |
|---|---|---|
| 1 | fail 8m34s | `download-ci-llvm` 404: GitHub Actions bootstrap used `HEAD^1` (a `git am` patch commit) as an upstream SHA. Pass `--ci=false` so the local freshness logic resolves the pinned `b6eeb93` (0b6337c). |
| 2 | fail 34m33s | The expected `generate-copyright` failure was rejected because Linux's longer backtrace and `run::GenerateCopyright` casing pushed the lowercase name out of a 50-line tail. Match case-insensitively over 200 lines (3d149a7). |
| 3 | fail 45m55s | `git -C compiler/rustc_codegen_cranelift apply` silently skipped repo-root-relative patch paths ("Skipped patch", exit 0), so a fresh tree had no memory.grow/size or f128 patches and plugin (c) trapped in the bump allocator. Apply from the repo root with `--directory` and assert the patched content exists (7fdd81c). |
| 4 | **pass 45m21s** | — |

## What the patch does

cg_clif already lowers Rust to CLIF. The patch makes wasm targets use a
`cranelift_module::Module` that emits wasm objects instead of native objects:

1. **`src/wasm.rs`** — detects `Arch::Wasm32/Wasm64`, builds a Pulley32/64
   `TargetIsa` (`clif-to-wasm::object_module::frontend_isa`) because Cranelift has
   no native wasm ISA, collects target features, and constructs
   `WasmObjectModule` with `demote_multi_results = true` (LLVM's Basic C ABI for
   wasm) and an `import_names` callback fed from rustc's
   `wasm_import_module_map` so `#[link(wasm_import_module = "...")]` works.
2. **`src/driver/aot.rs`** — `AotObject` wraps either the native
   `UnwindModule<ObjectModule>` or `WasmObjectModule`; wasm codegen units are
   written as relocatable `.o` files. clif-to-wasm warnings about functions
   replaced by traps become compiler warnings; `CG_CLIF_WASM_STRICT=1` turns
   them into errors.
3. **`src/main_shim.rs`** — wasm entry points take no argc/argv
   (`main_needs_argc_argv`).
4. **`src/lib.rs` / `src/base.rs`** — report wasm target features to
   `cfg(target_feature)` and follow the newer `compiled_code()` API.

The native path is unchanged. The backend still needs the normal rustc frontend;
only code generation and object emission are pure Rust.

## ABI used by programs (a)–(c)

* `no_std`, `panic = "abort"`, `#[panic_handler]` → `unreachable`.
* `core` + `alloc` (+ `compiler_builtins`) are built from `rust-src` by
  `-Zbuild-std=core,alloc,compiler_builtins` with cg_clif.
* A fixed 1 MiB heap in `.bss` and a bump `GlobalAlloc`; no `memory.size` /
  `memory.grow` (clif-to-wasm does not support those intrinsics).
* Exported `wasm_alloc(size, align) -> ptr` lets the host place inputs.
* (c) imports `nodecraft::host_bias(f64) -> f64` and exports
  `eval(params_ptr, len, out_ptr) -> i32`.
* No WASI, no filesystem, no clock, no randomness.

(a) and (b) also export `add`/`bench`/`sum_slice` for the harness. `b_alloc`
exercises `Vec`, `String`, `format!`, closures, trait objects, and `libm`
`sqrt`/`sin`.

Program (d) is explicitly the exception: `std` on `wasm32-wasip1`, built by LLVM
against rustup's prebuilt std, linked and run under WASI preview 1. It exists only
as a comparison and is not the plugin target.

## Stage B and C

Stage B proved that rustc.wasm is feasible with the playground's patch series;
stage C is the kept pipeline. See the history at the end of
[`stage-a-results.md`](notes/history/stage-a-results.md), the stage-C
notes in [`docs/results.md`](results.md), and
[`docs/compiler.md`](compiler.md).
