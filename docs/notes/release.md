# Release track notes: packaging, precompiled compiler, generate-copyright

This note collects the release work: the `generate-copyright` root-cause fix,
the versioned release workflow, the notices generator, the local dry-run
bundle, and the documentation text the lead should merge into the top-level
`README.md` / `results.md`. `docs/notes/precompiled.md` has the `.cwasm`
design and the `main.rs` integration snippet.

## 1. `generate-copyright` packaging failure: root cause and fix

**Root cause.** `src/tools/generate-copyright/src/cargo_metadata.rs` treats the
patched `clif-to-wasm` as an out-of-tree dependency (its manifest is outside
the rust tree). `load_important_files` then assumed every out-of-tree package
is vendored at `VENDOR_DIR/clif-to-wasm-0.3.0` and called `read_dir` on that
path. Bootstrap only vendors crates.io dependencies, so the path does not
exist and `x.py install` failed with `ENOENT` in the final packaging step.

**Fix.** `compiler/patches/rust/0002-generate-copyright-path-deps.patch`
records each package's source directory while reading `cargo metadata` and
uses it when the vendor copy is absent. The vendor directory stays the first
choice, so normal crates.io dependencies are unaffected. The patch includes
three unit tests: an unvendored path dependency is read from its own directory,
a vendored copy still wins, and a package with neither fails loudly. With the
fallback removed, the first test fails with the original `ENOENT`.

**Verification (lead's build window, already done once).** The script now
applies all rust patches, so a fresh tree gets 0002 automatically:

```sh
# From the repository root.
git -C toolchain/rust apply --check \
    "$PWD/compiler/patches/rust/0002-generate-copyright-path-deps.patch"
compiler/scripts/build-rustc-wasm.sh
echo "exit: $?"                  # must be 0
grep -c 'x.py install failed' docs/results/build-rustc-wasm.log || true
```

A successful run ends with `rustc.wasm -> .../toolchain/artifacts/rustc.wasm`;
the script copies the artifact only after `x.py install` exits 0. There is no
message matching anywhere in the script.

Focused crate test (only in a build window; it compiles askama):

```sh
cd toolchain/rust
RUSTC_BOOTSTRAP=1 cargo test \
    --manifest-path src/tools/generate-copyright/Cargo.toml \
    --offline
```

Evidence from the scratch copy (`toolchain/tracks/a/gc-new`, not the live
tree): `cargo test --offline` compiles in 7.4 s and the three tests pass. With
only the `scan_path` fallback reverted to the old
`let scan_path = dep_vendor_path;` in `gc-before`,
`reads_notices_from_an_unvendored_path_dependency` fails with exactly the
original error:

```text
Scraping notices for clif-to-wasm-0.3.0...
panicked: an out-of-tree package must be read from its own source dir:
Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })
test result: FAILED. 0 passed; 1 failed; 2 filtered out
```

The live-tree `x.py install` also completed successfully after 0002 was
applied (21 s, generate-copyright built and passed).

### `build-rustc-wasm.sh` changes

- Header no longer says any failure is tolerated.
- Applies every `compiler/patches/rust/*.patch` in sorted order with the
  reverse-check idempotence pattern (0001, 0002, Track B's 0003, ...).
- Same loop for every `compiler/patches/cg_clif/*.patch` and every
  `compiler/patches/clif-to-wasm/*.patch`; the existing content assertions
  (`llvm.wasm.memory.grow.i32`, `pub enum WasmIntrinsic`, the cg_clif
  `clif-to-wasm` path dependency) are unchanged.
- On any non-zero `x.py install` exit: print the last 200 log lines and exit
  non-zero. The artifact is copied only after a zero exit.

## 2. Versioned release workflow

`.github/workflows/release.yml` is **manual only**
(`workflow_dispatch`). It has no tag-push trigger. Inputs:

| Input | Meaning |
|---|---|
| `tag` (required) | Release tag, must start with `v` + a digit (for example `v0.3.0`). Used in every file name. |
| `dry_run` (boolean, default **true**) | `true` creates a **draft** GitHub release; `false` publishes it. In both cases the assembled bundle is uploaded as the `release-bundle-<tag>` workflow artifact first. |

Jobs:

1. **compiler** (`ubuntu-latest`): reuses `ci/setup-toolchain.sh`,
   `compiler/scripts/build-rustc-wasm.sh`, `build-host.sh` and
   `build-pure-sysroot.sh` (strict), writes `release-metadata.json`
   (rustc.wasm size/SHA-256, sysroot size, rustc version, pins), generates
   `THIRD_PARTY_NOTICES`, and uploads `rustc.wasm`, `sysroot`,
   `release-metadata.json` and the notices.
2. **host** matrix:
   - `aarch64-apple-darwin` on `macos-14`;
   - `x86_64-unknown-linux-gnu` on `ubuntu-latest`.
   Each downloads the one `rustc.wasm`, runs `setup-toolchain.sh`, then
   `release/package.sh`: builds the host and Wild with stable
   1.99.1, runs `cogwasm-precompile precompile` with the shared engine config,
   and packages `rustc-<tag>.wasm`, `rustc-<tag>-<host>.cwasm` (+ `.json`),
   `sysroot-<tag>.tar.gz`, `wild-<tag>-<host>`,
   `cogwasm-host-<tag>-<host>`, `README-<tag>.md`, `THIRD_PARTY_NOTICES` and
   `SHA256SUMS`. Uploaded as `bundle-<host>`.
3. **release** (`ubuntu-latest`, `contents: write`): downloads all host
   bundles, writes `SHA256SUMS-<tag>` over the merged files, adds
   `release-metadata-<tag>.json`, uploads the merged artifact, then uses the
   `gh` CLI with `GITHUB_TOKEN` to create a draft (`dry_run=true`) or published
   release. It does not use a third-party release action or any secret.

Actions are pinned to full SHAs already used in the repository: `checkout`
v7.0.1, `cache/*` v6.1.0 and `upload-artifact` v7.0.2 from
`cranelift_wasm.yml`, and `download-artifact` v7.0.0
(`37930b1c2abaa49bbe596cd826c3c89aef350131`). `free-disk-space@main` is the
same one the existing workflow uses.

**Windows is intentionally omitted.** The reference host preopens the guest
file system as `/` and passes native Windows paths such as `D:\a\...` to
rustc.wasm; WASI preview 1 cannot represent a drive in a preopened path, so the
guest cannot read the source or write objects without a host-side path
translation layer. Wild's Windows support is also untested for this pipeline.
The workflow carries this as a comment where a `windows-latest` job would go.

## 3. THIRD_PARTY_NOTICES generator

`release/gen-notices.py` reads `cargo metadata` for `crates/cogwasm-host`,
`crates/difftest-host` and every `guests/*` (including
`guests/a2_harness`), skips this repository's own crates, and includes
every third-party package with its declared licence expression and the text of
its `LICENSE*`, `COPYRIGHT`, `NOTICE` and `AUTHOR*` files. It prefers `--offline`
and retries with the network only when a graph was never resolved locally. A
package with no licence file falls back to the canonical SPDX text from rust's
`LICENSES/` or the playground's `licenses/spdx/`; if that is not available
either, it exits with the package name. It never writes a placeholder.

Explicit sections cover all six named components: Rust/rustc,
Cranelift/Wasmtime, clif-to-wasm, the rust-playground-inbrowser patch series,
Wild, and wasi-libc (all of `toolchain/playground/licenses/wasi-libc/`, whose
provenance is wasi-sdk 34 / wasi-libc 2e6fb9d8). Wasmtime's licence is found
by globbing `toolchain/cargo/registry/src/*/wasmtime-<lock version>/LICENSE*`,
because the registry directory hash is not stable across hosts; a miss lists
the tried path and fails. The output ends with a provenance block (checkout
revisions, Wasmtime version, wasi-sdk version).

Local run:

```sh
python3 release/gen-notices.py --out release/out/THIRD_PARTY_NOTICES
# wrote .../THIRD_PARTY_NOTICES (2165967 bytes, 195 cargo packages,
# 15 named-component texts, 358 package texts) from 11 manifests
# all six named components are present
```

`release/README.md` is the bundle README: file list, how to drive
`rustc.wasm` through the reference host, the exact Wasmtime 50.0.2
configuration and compatibility-hash requirement, fallback behaviour, integrity
checking and the "modified rustc, not official" statement.

## 4. Local dry-run bundle

Run after the final `rustc.wasm` exists (Track C rebuilds it):

```sh
cd CogWasm
release/package.sh \
    --tag v0.0.0-dryrun \
    --host-triple aarch64-apple-darwin \
    --skip-build \
    --reuse-precompiled toolchain/tracks/a/final/rustc-aarch64-apple-darwin.cwasm \
    --out release/out
```

`--skip-build` reuses the host/precompile binaries built with
`CARGO_TARGET_DIR=toolchain/target/cogwasm-host` and the Wild binary.
`--reuse-precompiled` reuses the artifact just measured instead of
precompiling the same bytes again; it must have been built from the same
`rustc.wasm` (the metadata SHA-256 is what the host checks). Without the flag
the script runs `cogwasm-precompile precompile` itself. The script creates
`release/out/` (gitignored, including `*.cwasm.json`), copies `rustc.wasm` as
`rustc-<tag>.wasm`, tars the sysroot, copies Wild and the host, writes
`README-<tag>.md`, generates/copies `THIRD_PARTY_NOTICES` and writes
`SHA256SUMS`.

Linux (`x86_64-unknown-linux-gnu`) is built by the workflow `host` job, not
locally. The Windows reason is in section 2.

The packaging script was smoke-tested end to end with an 8-byte empty wasm
module (so no 40 s precompile), the real sysroot/Wild/host binaries, the real
notices and a pre-generated `THIRD_PARTY_NOTICES`:

```text
README-v9.9.9-smoke2.md
SHA256SUMS
THIRD_PARTY_NOTICES
rustc-v9.9.9-smoke2-aarch64-apple-darwin.cwasm
rustc-v9.9.9-smoke2-aarch64-apple-darwin.cwasm.json
rustc-v9.9.9-smoke2.wasm
cogwasm-host-v9.9.9-smoke2-aarch64-apple-darwin
sysroot-v9.9.9-smoke2.tar.gz
wild-v9.9.9-smoke2-aarch64-apple-darwin
```

A second run with a different tag removed every stale `rustc-*`, `sysroot-*`,
`wild-*`, `cogwasm-host-*`, `README-*` and `SHA256SUMS*` file before writing
the new bundle, and `SHA256SUMS` listed exactly the eight other files.

Dry-run file list and sizes on the final artifact (`d2404a86`, 144,051,631 B;
`shasum -a 256 -c SHA256SUMS` passes for all eight entries):

| File | Bytes |
|---|---:|
| `rustc-v0.0.0-dryrun.wasm` | 144,051,631 |
| `rustc-v0.0.0-dryrun-aarch64-apple-darwin.cwasm` | 296,372,016 |
| `rustc-v0.0.0-dryrun-aarch64-apple-darwin.cwasm.json` | 3,407 |
| `sysroot-v0.0.0-dryrun.tar.gz` | 17,889,140 |
| `wild-v0.0.0-dryrun-aarch64-apple-darwin` | 13,473,264 |
| `cogwasm-host-v0.0.0-dryrun-aarch64-apple-darwin` | 18,014,480 |
| `README-v0.0.0-dryrun.md` | 5,047 |
| `THIRD_PARTY_NOTICES` | 2,165,967 |
| `SHA256SUMS` | 818 |

## 5. Text for the lead to merge

### Root `README.md` Status: known-limits bullet

Replace the last bullet with (fill in the final `.cwasm` numbers once
measured):

> - The first load of the 137 MB `rustc.wasm` compiles it to native code
>   (38.6 s median on an Apple M4 Pro; 0.44 s from Wasmtime's cache). Release
>   bundles also ship a per-host `.cwasm`: deserializing it takes 19.46 ms
>   (282 MiB, exact Wasmtime 50.0.2 configuration), falls back to the `.wasm`
>   when the version, configuration or checksum does not match, and can
>   optionally re-check both SHA-256 digests.

### `results.md` Stage C section: new bullet

> * **Precompiled `.cwasm`**: `cogwasm-precompile` produces a per-host
>   Wasmtime artifact and a sidecar recording the Wasmtime version, the
>   `Engine::precompile_compatibility_hash()`, the target, the engine
>   configuration and the SHA-256/size of both files. The host checks the
>   sidecar on every load and falls back to `Module::new(rustc.wasm)` on any
>   mismatch; full-digest verification is opt-in. On macOS arm64 the artifact
>   is 296,372,016 bytes and `Module::deserialize_file` takes 19.46 ms (median
>   of 5; 18.74 ms through `load_rustc_module`),
>   versus 38.6 s cold and 0.44 s from Wasmtime's cache. Release packaging is
>   `.github/workflows/release.yml` (manual, draft by default), with
>   per-host bundles, `THIRD_PARTY_NOTICES` and `SHA256SUMS`.

### `docs/compiler.md`

The tolerated-`generate-copyright` paragraph is already replaced with the
hard-fail description, and `0002` is in the patch table.

## 6. Remaining items

- The release workflow has been dry-run locally as a bundle artifact
  (`release/out`, `SHA256SUMS` verified) but has not been run on GitHub
  Actions. Its first real run must confirm the `macos-14` runner downloads or
  builds wasi-sdk and Wild, and that the Linux host job produces a usable
  `.cwasm`; CI's `run.sh precompiled-check` already exercises the same
  precompile/deserialize path on `ubuntu-latest`.
- The Linux `.cwasm` is not built on this Mac; it is the workflow's per-host
  job and CI's precompiled check. Windows remains unsupported for the reasons
  in the workflow comment.
