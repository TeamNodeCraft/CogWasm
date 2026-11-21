# Threads assessment for a single-threaded rustc.wasm host

Status: report only. The rebased branch keeps `wasm32-wasip1-threads`.

## What the current (threads) host does

- Target: `wasm32-wasip1-threads`, `singlethread = false`, shared memory,
  atomics, 1 GiB max memory, `-lwasi-emulated-mman`
  (`compiler/rustc_target/src/spec/targets/wasm32_wasip1_threads.rs:21-66`).
- The compiler executes on the wasm **main thread**:
  `run_in_thread_with_globals` returns directly under
  `if cfg!(target_family = "wasm")`
  (`compiler/rustc_interface/src/util.rs:160-183`). The normal path would spawn
  a named thread with `thread_stack_size`.
- The thread-pool path is only entered when
  `sync::check_dyn_thread_safe()` returns `Some`
  (`compiler/rustc_interface/src/util.rs:211-243`). A non-parallel wasm build
  returns `None`, so `rustc_thread_pool` is not started.
- The ctrl-C handler is already disabled on wasm
  (`compiler/rustc_driver_impl/src/lib.rs:1650`,
  `#[cfg(all(not(miri), not(target_family = "wasm")))]`).
- Codegen parallelism/jobserver is gated on `cgcx.parallel`:
  `rustc_codegen_ssa/src/back/write.rs:1027` and `:1270` only create the
  `jobserver` helper thread when `parallel` is `Some`. On wasm it is `None`.
- LLVM is built with `LLVM_ENABLE_THREADS=ON` for the threads target
  (`src/bootstrap/src/core/build_steps/llvm.rs:740-742`), so LLVM may use its
  own thread pool. The C++ toolchain is given `-pthread` and 1 MiB of C stack
  at build/link time (`llvm.rs:584`, `:593`).
- Linking is in-process: `RustRunLld` is called from
  `compiler/rustc_codegen_ssa/src/back/link.rs:1285-1305`; the wrapper is
  `compiler/rustc_llvm/llvm-wrapper/LLD.cpp`. No linker subprocess is spawned.
- `DEFAULT_STACK_SIZE` (32 MiB on wasm,
  `compiler/rustc_interface/src/util.rs:126-131`) only feeds
  `thread_stack_size` for a spawned thread. On the direct main-thread path it is
  **not used**. The wasm main-thread stack is fixed at link time; the target
  spec does not currently pass `-z stack-size` for rustc.wasm. `RUST_MIN_STACK`
  is likewise only read by `init_stack_size`.
- The old cg_clif AOT driver's `ConcurrencyLimiter`/`thread::spawn` code is gone
  in 1.99.1; cg_clif is not used by the LLVM recipe anyway.

## What a single-threaded `wasm32-wasip1` host needs

1. **Target**: bootstrap host `wasm32-wasip1` (not `-threads`). The base wasm
   options already set `singlethread: true`
   (`compiler/rustc_target/src/spec/base/wasm.rs:72`), and the target list
   already builds `wasm32-wasip1` std. The recipe would only need to change
   `[build] host` and the source/branch name.
2. **LLVM**: build with `LLVM_ENABLE_THREADS=OFF`. The WASI CMake block already
   branches on `target.contains("threads")`
   (`src/bootstrap/src/core/build_steps/llvm.rs:740-742`), and the old patch's
   own comment warns that `-DLLVM_ENABLE_THREADS=OFF` is "kind of broken". This
   is the largest unknown.
3. **rustc thread spawns**: the `cfg!(target_family = "wasm")` gate in
   `util.rs` covers both wasip1 and wasip1-threads, so the main-thread path is
   already selected. `rustc_thread_pool` still compiles; it must not be called
   (it is behind `check_dyn_thread_safe`). The `ctrlc` handler is already out.
   Codegen/jobserver helpers are behind `cgcx.parallel`.
4. **Main-thread stack**: add a link-time stack size for the rustc binary, e.g.
   a target pre-link arg `-Wl,-z,stack-size=...` (or a bootstrap
   `-Clink-arg`), and make sure wasmtime's `max_wasm_stack` /
   `async_stack_size` is at least that. Today the target spec sets no stack size
   for rustc.wasm, so the 32 MiB `DEFAULT_STACK_SIZE` is misleading on the
   main-thread path.
5. **wasi-sdk**: use the non-pthread sysroot/compiler mode (no `-pthread`, no
   shared-memory flags). wasi-sdk 34 provides both.
6. **Host runtime**: the latest wasmtime (>=48) without
   `wasmtime-wasi-threads`/`wasi-common`, using the current `wasmtime-wasi`
   preview1 implementation. The module must import only
   `wasi_snapshot_preview1` and must not require shared memory/atomics.
7. **Panic/diagnostics**: `panic=abort` behavior and the "diagnostics then
   `unreachable`" trap stay the same; the host already treats that as a compile
   failure.

## Recommendation

**Keep `wasm32-wasip1-threads` for the first 1.99.1 release.** The thread pin is
a host-runtime constraint, not a compiler-patch constraint; switching targets
adds a second large variable (LLVM threads off, stack sizing, wasi-sdk ABI,
wasmtime API) on top of the LLVM 22 + wasi-sdk 34 upgrade. Get 1.99.1 building
and running on the existing wasmtime 46 path first, then do a separate
single-threaded spike. If that spike succeeds, the compiler-side changes are
small: the `cfg!(target_family = "wasm")` main-thread path already exists, and
the only new work is the target/LLVM-threads switch and the link-time stack
size.

## Concrete follow-up checklist (single-thread spike)

- [ ] Add a `wasm32-wasip1` host build config and build LLVM with
      `LLVM_ENABLE_THREADS=OFF`.
- [ ] Audit every `thread::spawn` reachable from `rustc_driver_impl::main` on a
      non-parallel wasm build; keep them behind `cfg(not(target_family =
      "wasm"))` or the `parallel` option.
- [ ] Add `-Wl,-z,stack-size=<N>` for rustc.wasm and test a deeply recursive
      crate at the chosen N.
- [ ] Build a non-threads sysroot (`wasm32-wasip1`) and run `rustc --version`,
      `--print sysroot`, and a real plugin compile.
- [ ] Run it on the latest wasmtime without `wasmtime-wasi-threads`; confirm the
      module has no shared-memory/atomics imports.
