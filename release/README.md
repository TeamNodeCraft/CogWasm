# CogWasm compiler bundle

This bundle contains a **modified build of the Rust compiler**, not the
official Rust compiler. It is built from `rustc` with the
rust-playground-inbrowser rustc-to-wasm patch series and patches from this
repository, uses `rustc_codegen_cranelift` as its only code generator, and runs
as WebAssembly on Wasmtime. It is not endorsed by the Rust project.

## Files

| File | What it is |
|---|---|
| `rustc-<tag>.wasm` | The compiler, built for `wasm32-wasip1` with Cranelift only. No LLVM inside. It imports WASI preview 1 and `playground.spawn` (used to run the Wild linker). |
| `rustc-<tag>-<host>.cwasm` | `rustc.wasm` precompiled by `Engine::precompile_module` for `<host>`. Native code: it only loads on the exact Wasmtime version, engine configuration and target recorded in its sidecar. |
| `rustc-<tag>-<host>.cwasm.json` | Sidecar metadata: Wasmtime version, precompile compatibility hash (hex), target, configuration summary, and the SHA-256 and size of the `.wasm` and `.cwasm`. |
| `sysroot-pure-<tag>.tar.gz` | `core`, `alloc` and `compiler_builtins` for `wasm32-unknown-unknown`, built by `rustc.wasm` itself. Matches this compiler's metadata exactly. |
| `wild-<tag>-<host>` | The native Wild linker. The host runs it when `rustc.wasm` calls its `playground.spawn` import. |
| `stage-c-host-<tag>-<host>` | The reference Wasmtime host: `compile`, `eval`, `check`, `bench` and `inspect`. |
| `THIRD_PARTY_NOTICES` | Licences and notices of every component and Cargo package in the bundle. |
| `SHA256SUMS` (per-host artifact) / `SHA256SUMS-<tag>` (published release) | SHA-256 of every file above. Verify it before installing. |
| `README-<tag>.md` | This file. |

The `.cwasm` is per host. Do not use the `aarch64-apple-darwin` artifact on a
Linux machine, or vice versa.

## Using `rustc.wasm` with Wasmtime

`rustc.wasm` is not a WASI command that the plain `wasmtime` CLI can run by
itself: it imports `playground.spawn`, and the host must service that import
with Wild. Use the bundled reference host, or provide the same import in your
own embedding.

```sh
# Compile and link one crate. `--precompiled` selects the fast path; omit it
# to compile rustc.wasm from scratch (~40 s cold on an Apple M-series machine).
./stage-c-host-<tag>-aarch64-apple-darwin compile \
    --rustc rustc-<tag>.wasm \
    --sysroot sysroot-pure \
    --linker wild-<tag>-aarch64-apple-darwin \
    --src path/to/plugin/src/lib.rs \
    --work work \
    --out plugin.wasm \
    --precompiled rustc-<tag>-aarch64-apple-darwin.cwasm
```

`check` uses the NodeCraft example ABI and asserts `eval(1,2,3,4) == 2.25`;
`compile` does not evaluate anything.

The host expects a `sysroot-pure` directory; extract
`sysroot-pure-<tag>.tar.gz` first (it contains a `sysroot-pure/` directory).
`stage-c-host --rustc ... --precompiled ...` checks the sidecar, deserializes
the artifact, and falls back to compiling `rustc.wasm` with a warning if
anything does not match. `--verify-precompiled` additionally checks the full
SHA-256 of both files on every load; that is an install-time integrity check,
not the default, because hashing the `.cwasm` and `.wasm` (hundreds of MiB)
costs more than the deserialization it protects.

## Exact Wasmtime requirement

The `.cwasm` is built and tested with **Wasmtime 50.0.2** and this engine
configuration:

```text
wasm_threads         false
wasm_bulk_memory     true
wasm_multi_memory    true
wasm_simd            true
wasm_relaxed_simd    true
max_wasm_stack       8 MiB   (8 << 20)
async_stack_size     16 MiB  (16 << 20)
```

A different Wasmtime version, feature set or target changes
`Engine::precompile_compatibility_hash()`. Compare the host engine's hash with
the `compatibility_hash` field in `rustc-<host>.cwasm.json`; if they differ,
Wasmtime will reject the artifact. The reference host does this comparison for
you and falls back to `rustc.wasm`. If you embed Wasmtime yourself, use the same
`Config` values above and either compare the hash or attempt the deserialize and
treat an error as "use `rustc.wasm`".

## Fallback behaviour

If the `.cwasm` is missing, has the wrong version/configuration/target, a
mismatched sidecar, a truncated file or (with `--verify-precompiled`) a bad
checksum, the host prints a warning naming the reason and compiles
`rustc.wasm` with `Module::new`. Correctness never depends on the artifact
being usable. The fallback is slow (about 40 s cold on an Apple M-series
machine); Wasmtime's module cache makes later loads about 0.44 s.

## Integrity and licences

Run `sha256sum -c SHA256SUMS` (or `shasum -a 256 -c SHA256SUMS` on macOS)
before installing; in a published release the file is `SHA256SUMS-<tag>`. `THIRD_PARTY_NOTICES` contains the licences of every
component in this bundle, including Rust/rustc, Cranelift/Wasmtime,
clif-to-wasm, the rust-playground-inbrowser patch series, Wild and wasi-libc.
The original source code in this repository is MIT OR Apache-2.0; the patch
files are licensed under the project they modify.
