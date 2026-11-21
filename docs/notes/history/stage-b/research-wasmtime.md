# Running `rustc.wasm` (wasm32-wasip1-threads) under Wasmtime 50.0.2

Research note for Stage B. No tracked git files were modified; this file is new.
Third-party checkouts used for this note live under the gitignored
`toolchain/stage-b/research/` area.

**Verdict up front:** `playground.spawn` is easy to implement in a Wasmtime host.
**`wasi-threads` is not.** Wasmtime 47.0.0 removed the `wasi-threads` host
implementation, and 50.0.2 has no `wasi:thread-spawn`; `Config::wasm_threads`
only enables shared memory/atomics. `rustc` always runs compilation on a spawned
`rustc` thread (even with `-Zthreads=1`), so an unmodified `rustc.wasm` cannot
run under Wasmtime 50.0.2. See §2 for the escapes.

Pins used below:

| Project | Pin | Date |
| --- | --- | --- |
| `ryancbuckley/rust-playground-inbrowser` | `f5298630dda0f2dc2b44f6ae1b54add7a77d8316` | checked out in `toolchain/playground` |
| `ryancbuckley/browser-kernel` | `a2cdcb3f69bb735ec8bb0269cd1299d85460c29e` | 2026-11-03 (HEAD at clone time) |
| `wasmtime` / `wasmtime-wasi` / `wasmtime-cli` | `50.0.2` | 2026-11-16 |
| `wasmtime-wasi-threads` | `46.0.3` (last published) | 2026-08-20 |
| Rust source for rustc patches | `rust-lang/rust@b6eeb93c992027a03e1fd120b6652546396b142c` (`nightly-2026-10-19`) | 2026-10-18 |

---

## 1. `playground.spawn` ABI and kernel semantics

### 1.1 Import signature

The rust patch declares ([`patches/rust/0003-...patch:928-953`](https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/patches/rust/0003-Run-the-linker-through-the-host-s-playground.spawn-o.patch#L928-L953)):

```rust
#[link(wasm_import_module = "playground")]
unsafe extern "C" {
    fn spawn(argv: *const *const c_char, argc: usize) -> i32;
}
```

In wasm terms this is `(i32, i32) -> i32`:

* `argv` — guest pointer to an array of `argc` 32-bit guest pointers. Each pointed-to
  string is NUL-terminated UTF-8.
* `argc` — number of entries, including `argv[0]`.
* Return `0` = child exited 0; `-1` = host cannot run it / no such program;
  any other value = the child's exit code. The Rust side treats all non-zero,
  non-`-1` codes as a fatal link failure, not as a normal `Output`
  ([`0015-...patch:20-38`](https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/patches/rust/0015-WASI-linker-report-a-failed-link-as-such-not-as-a-mi.patch#L20-L38)).
  The `Command`'s env, current dir, and stdin are ignored; only program + args
  are sent. Linker stdout/stderr are not captured into `Output`; they go to the
  compiler's own streams.

### 1.2 Kernel implementation

The worker wires the import to the kernel syscall at
[`browser-kernel/thread-worker.js:90-95`](https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/thread-worker.js#L90-L95):

```js
const imports = {
  env: { memory },
  wasi_snapshot_preview1: wasi,
  wasi: { "thread-spawn": wasi.thread_spawn },
  playground: { spawn: wasi.playground_spawn },
};
```

and the kernel side is
[`browser-kernel/kernel.js:1143-1175`](https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/kernel.js#L1143-L1175):

```js
sys_playground_spawn(p, a) {
  const v = p.view();
  const argv = [];
  for (let i = 0; i < u32(a[1]); i++) {
    let s = v.getUint32(u32(a[0]) + 4 * i, true), e = s;
    while (v.getUint8(e)) e++;
    argv.push(p.string(s, e - s));
  }
  const name = argv[0]?.split("/").pop() ?? "";
  const program = this.programs[name];
  if (!program) {
    p.onStderr(encoder.encode(`kernel: no program named ${name}\n`));
    return -1;
  }
  const child = this.spawn(program, {
    args: argv,
    env: p.envVars,
    stdout: p.onStdout,
    stderr: p.onStderr,
  });
  p.children.add(child);
  return child.exited.then((code) => {
    p.children.delete(child);
    return code;
  });
}
```

Key points:

* **Program lookup is by basename of `argv[0]`.** `/anything/wasm-ld` matches a
  registered program named `wasm-ld`. The playground registers
  `{ "wasm-ld": wild }` ([`web/kernel/kernel-worker.js:224-236`](https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/web/kernel/kernel-worker.js#L224-L236)).
  `wild.wasm` is loaded as a separate module and spawned as a separate process
  (separate linear memory and fd table), not as a function call in rustc's
  instance.
* **Same filesystem, not the same fd table.** `this.spawn` uses the same
  `Kernel`/`MemFS`; the child's default preopen is `"/"` of that same MemFS
  ([`kernel.js:271-288`](https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/kernel.js#L271-L288)).
  The child gets fresh fds 0/1/2 and fd 3 = preopen `/`. It does **not** inherit
  parent fds other than stdio; it does not share rustc's linear memory.
* **Env:** the child gets the parent's complete `envVars` map (`p.envVars`),
  not a delta. `Command`-scoped env changes are lost because the Rust patch does
  not read `cmd.get_envs()`.
* **Stdio:** child fd 1/2 call the parent's `onStdout`/`onStderr` JS callbacks,
  so bytes appear on the same stream. Child stdin is the default empty
  `Uint8Array` (EOF); `stdin` is not forwarded.
* **cwd:** there is no cwd. WASI p1 resolves paths through preopen fds; the
  browser child sees `/` = the shared MemFS root.
* **Lifetime:** parent exit kills all children ([`kernel.js:653`](https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/kernel.js#L653)).
* **Blocking:** the kernel returns a promise which the worker's synchronous
  syscall channel waits on; the guest `spawn` call blocks until the linker exits.

For a Wasmtime host, this maps naturally to a host function that reads argv from
guest memory and runs either native Wild (`std::process::Command`) or a second
Wasmtime `Store` executing `wild.wasm`. If using preopens that differ from host
paths, rewrite guest `/work`, `/sysroot`, `/tmp` prefixes to the host paths
before invoking native Wild; the browser kernel avoids this because its guest
and MemFS paths are identical. The existing Stage B host's choice
(`preopened_dir("/", "/")` + native Wild) makes paths identical but gives the
guest the whole host filesystem read/write.

---

## 2. Threads: Wasmtime 50.0.2 does **not** support `wasi-threads`

### 2.1 What 50.0.2 does provide

`Config::wasm_threads` exists, but its own docs say it is only the core threads
proposal — shared memories and atomics — and **not** thread spawning
([`wasmtime-50.0.2/src/config.rs:1009-1032`](https://docs.rs/crate/wasmtime/50.0.2/source/src/config.rs#L1009-L1032)):

```rust
config.wasm_threads(true); // shared memory + atomic instructions only
```

The `threads` Cargo feature is on by default
(`wasmtime-50.0.2/Cargo.toml:98-122,209`), so this is already enabled unless
you disable default features. `SharedMemory::new` is available with the
`threads` feature (`src/runtime/memory.rs:859-890`), which is enough to
*instantiate* a shared-memory module, but not to service `wasi:thread-spawn`.

### 2.2 `wasi-threads` was removed in 47.0.0

* Wasmtime 47 release notes: “Support for wasi-threads and Wasmtime's
  `wasi-common` crate have been removed.”
  ([`release-47.0.0/RELEASES.md:91`](https://github.com/bytecodealliance/wasmtime/blob/release-47.0.0/RELEASES.md#L91)).
* Removal PR: [`bytecodealliance/wasmtime#13558`](https://github.com/bytecodealliance/wasmtime/pull/13558),
  merged 2026-06-05, commit
  [`b4b23fe5`](https://github.com/bytecodealliance/wasmtime/commit/b4b23fe583173131a047141947a785b4c1c646ae).
  The PR says 46.0.0 is the last release with support and 36.0.0 is an LTS that
  keeps it for another year.
* The 50.0.2 CLI explicitly bails:

  ```text
  $ wasmtime run -Sthreads=y ...
  Error: support for wasi-threads has been removed from Wasmtime
  ```

  Source: [`wasmtime-cli-50.0.2/src/commands/run.rs:1265-1267`](https://docs.rs/crate/wasmtime-cli/50.0.2/source/src/commands/run.rs#L1265-L1267);
  `-Sthreads` is also rejected at lines 1140-1151. `-Spreview2=n` is likewise
  no longer supported; core modules default to WASI p1.
* `wasmtime-wasi-threads` has no 47/48/49/50 release; the last is **46.0.3**
  ([crates.io versions](https://crates.io/crates/wasmtime-wasi-threads/versions)).
  Its API is in `wasmtime-wasi-threads-46.0.3/src/lib.rs`:
  `WasiThreadsCtx::new(module, Arc<Linker<T>>, use_async)`,
  `add_to_linker(linker, store, module, get_cx)` registering
  `("wasi", "thread-spawn")` as `(i32) -> i32`, and a new OS thread + `Store`
  calling the module's exported `wasi_thread_start(tid, start_arg)`.
  It requires `T: Clone + Send + 'static` and calls `std::process::exit` on a
  thread trap, so it is CLI-oriented, not a clean embedder API.

### 2.3 `-Zthreads=1` does not avoid the import or the call

`rustc`'s single-threaded path still spawns a thread. In
`compiler/rustc_interface/src/util.rs:175-210` (upstream Rust, unchanged by the
playground patches):

```rust
// The "thread pool" is a single spawned thread in the non-parallel compiler.
let builder = thread::Builder::new().name("rustc".to_string()).stack_size(thread_stack_size);
thread::scope(|s| {
    let r = builder.spawn_scoped(s, move || { /* run compilation */ }).unwrap().join();
    ...
});
```

The parallel path uses `rustc_thread_pool::ThreadPoolBuilder`, also spawning
threads. `-Zthreads` is an alias for `--jobs-frontend`
(`compiler/rustc_session/src/config.rs:1768-1780`); it only sets the frontend
job count and does not move compilation onto the main thread.
Therefore rustc.wasm imports **and calls** `wasi.thread-spawn` on every
invocation. The playground's own comment says each compile “starts several
threads” (`web/kernel/kernel-worker.js:233-234`).

Empirical import check with the local nightly for wasm32-wasip1-threads:

| Test program | `wasi.thread-spawn` import |
| --- | --- |
| `fn main() { println!("hi"); }` | absent (7 WASI imports total) |
| `fn main() { std::thread::spawn(||{}).join().unwrap(); }` | present |

So wasm-ld only emits the import when reachable. A module that merely imports it
but never calls it can be instantiated under Wasmtime 50 by defining a stub or
calling `Linker::define_unknown_imports_as_traps(&module)`
([`src/runtime/linker.rs:277-295`](https://docs.rs/crate/wasmtime/50.0.2/source/src/runtime/linker.rs#L277-L295));
it will trap only if called. That is **not** sufficient for unmodified rustc,
because the first `spawn_scoped` calls it.

### 2.4 Viable escapes

1. **Pin Wasmtime 46.0.3 (or 36.0.21 LTS) + `wasmtime-wasi-threads` 46.0.3**
   and use the old `-Sthreads=y`-equivalent API. This is the only supported
   off-the-shelf route. Trade-off: 46's implementation exits the whole host
   process on a thread trap, and requires a cloneable host state.
2. **Patch rustc to run the non-parallel compiler on the current thread on
   WASI**, and remove/disable all `thread::Builder`/rayon spawns. Then no
   `wasi.thread-spawn` call remains; `define_unknown_imports_as_traps` handles
   any residual import. This is a source patch to rustc, not a host setting.
3. **Custom host implementation.** A new `wasi:thread-spawn` host function must
   create a new `Store` sharing the `SharedMemory`, call
   `wasi_thread_start(tid, arg)` on a new OS thread, and share the WASI fd table
   across threads. In 50.0.2, `WasiP1Ctx` is not `Clone` and has no shared fd
   table API (`wasmtime-wasi-50.0.2/src/p1.rs:141-146`), so this is a
   substantial reimplementation, not a wrapper. Not recommended for this
   experiment.

**Blocker summary:** under Wasmtime 50.0.2, `rustc.wasm` will fail at
instantiation with an unknown `wasi::thread-spawn` import unless stubbed, and
will trap at the first compilation thread if stubbed. There is no config flag
that restores the removed implementation.

---

## 3. WASI preview1 host setup

### 3.1 Crates and API

Use `wasmtime = "=50.0.2"` and `wasmtime-wasi = "=50.0.2"` with
`default-features = false, features = ["p1"]` (plus `wasmtime`'s `cranelift`,
`runtime`, `std`, `threads`, and `anyhow` features if you use `anyhow`).
`p1` pulls in `p2` internally (`wasmtime-wasi-50.0.2/Cargo.toml:46-60`), so the
component-model machinery is linked, but the runtime API is the p1 one:

* `wasmtime_wasi::p1::WasiP1Ctx` — store-local state.
* `wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |t| &mut t.wasi)` —
  registers `wasi_snapshot_preview1`; the sync variant blocks via
  `wasmtime_wasi::runtime::in_tokio`.
* `WasiCtxBuilder::build_p1()` — finalize to `WasiP1Ctx`
  ([`src/ctx.rs:479-486`](https://docs.rs/crate/wasmtime-wasi/50.0.2/source/src/ctx.rs#L479-L486)).
* `WasiCtxBuilder::preopened_dir(host, guest, FsPerms)`
  ([`src/ctx.rs:296-312`](https://docs.rs/crate/wasmtime-wasi/50.0.2/source/src/ctx.rs#L296-L312)).
  `FsPerms::ReadOnly` / `FsPerms::ReadWrite`.

### 3.2 Snippet (compile-checked against 50.0.2)

The following was `cargo check`ed against `wasmtime 50.0.2` /
`wasmtime-wasi 50.0.2` with the local nightly. It is the shape of a host:

```rust
use wasmtime::*;
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::{FsPerms, WasiCtxBuilder};

struct Host {
    wasi: WasiP1Ctx,
    limits: StoreLimits,
}

let mut cfg = Config::new();
cfg.wasm_threads(true);                 // shared memory + atomics, NOT thread-spawn
cfg.max_wasm_stack(16 << 20);
cfg.async_stack_size(32 << 20);         // must be >= max_wasm_stack
cfg.memory_reservation(4 << 30);        // virtual reservation, default on 64-bit
cfg.memory_guard_size(32 << 20);        // default on 64-bit
let engine = Engine::new(&cfg)?;

let module = Module::from_file(&engine, "rustc.wasm")?;
let mut linker: Linker<Host> = Linker::new(&engine);
p1::add_to_linker_sync(&mut linker, |h| &mut h.wasi)?;
linker.func_wrap("playground", "spawn", |mut caller: Caller<'_, Host>, argv: u32, argc: u32| -> i32 {
    // read argv/argc from caller's exported "memory", run Wild, return exit code
    0
})?;
linker.define_unknown_imports_as_traps(&module)?; // stubs thread-spawn, if present

let wasi = WasiCtxBuilder::new()
    .args(&["rustc", "/work/src/lib.rs", "--target=wasm32-unknown-unknown"])
    .env("RUST_MIN_STACK", "16777216")
    .env("RUST_BACKTRACE", "0")
    .env("HOME", "/tmp")
    .env("TMPDIR", "/tmp")
    .preopened_dir(host_sysroot, "/sysroot", FsPerms::ReadOnly)?
    .preopened_dir(host_work, "/work", FsPerms::ReadWrite)?
    .preopened_dir(host_tmp, "/tmp", FsPerms::ReadWrite)?
    .build_p1();

let limits = StoreLimitsBuilder::new().memory_size(384 << 20).build();
let mut store = Store::new(&engine, Host { wasi, limits });
store.limiter(|h| &mut h.limits);

let instance = linker.instantiate(&mut store, &module)?;
let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
start.call(&mut store, ())?;
```

Notes:

* `add_to_linker_sync` + `start.call` is the synchronous path. If you use
  `add_to_linker_async`, you must drive `_start` with `call_async` on a Tokio
  runtime; do not mix sync `call` with async WASI imports.
* `initial_cwd` (`src/ctx.rs:248-259`) is a p2 `wasi:cli/environment` concept.
  WASI p1 has no `getcwd`; absolute guest paths are resolved through preopens.
  Do not expect `Command::current_dir` to work through this ABI.
* Preopen the sysroot read-only. `rustc.wasm` needs to write only `/work` and
  `/tmp` for this task. Mapping guest `/sysroot` directly to the host sysroot and
  `/work` to the host work dir avoids rewriting linker arguments only if native
  Wild also sees those host paths; otherwise rewrite prefixes in `spawn`.

### 3.3 WASI p1 syscall coverage

Wasmtime's p1 implementation covers the p1 calls rustc needs: `args_*`,
`environ_*`, `fd_prestat_get`, `fd_prestat_dir_name`, `path_open`,
`fd_read`/`fd_write`/`fd_seek`/`fd_readdir`, `fd_filestat_*`, `path_*`,
`random_get`, `clock_time_get`, `poll_oneoff`, `sched_yield`, `proc_exit`.
`path_link`/`path_symlink`/`path_readlink` are implemented
(`wasmtime-wasi-50.0.2/src/p1.rs:2134-2300`). `proc_raise` returns
`ENOTSUP` (`p1.rs:2610-2617`), but rustc does not use it. The only imports
Wasmtime 50 lacks for this workload are **`wasi.thread-spawn`** and
**`playground.spawn`**. `sock_*` are present but irrelevant (and can be left
denied).

---

## 4. Stack, memory, guard pages

### 4.1 The 16 MiB `RUST_MIN_STACK`

`RUST_MIN_STACK=16777216` is read by **rustc itself**, not by Wasmtime. In
`compiler/rustc_interface/src/util.rs:145-172`, `init_stack_size` parses
`RUST_MIN_STACK` and uses it as the `thread::Builder::stack_size` for the
spawned `rustc` thread. The default is `DEFAULT_STACK_SIZE = 17 * 1024 * 1024`
(17 MiB) in `compiler/rustc_session/src/config.rs:57`. The browser passes
16 MiB; for a Wasmtime host, pass it as a WASI env var:

```rust
.env("RUST_MIN_STACK", "16777216")   // or 17 MiB / 32 MiB
```

This controls the wasi-libc pthread stack allocated from the guest heap, not
Wasmtime's native stack.

### 4.2 Wasmtime native stack

`Config::max_wasm_stack` defaults to **512 KiB**
(`wasmtime-50.0.2/src/config.rs:295`, docs at `:857`). It is the native stack
available to wasm frames and host calls, not the guest linear-memory stack.
Set it larger for rustc (16-32 MiB). With Wasmtime's default `async` feature,
`max_wasm_stack` must be `<= async_stack_size`, whose default is 2 MiB
(`config.rs:2701-2705`); set `async_stack_size(32 << 20)` as in the snippet, or
`Engine::new` fails. The guest's main-thread linear-memory stack is fixed at
link time by wasm-ld (`-z stack-size`); Wasmtime cannot change it at runtime.
Spawned thread stacks come from the guest heap and `RUST_MIN_STACK`.

### 4.3 Memory limits

* 32-bit wasm linear memory is limited to 4 GiB = 65536 pages.
  `MemoryType::new(min, Some(max))` panics if the limits exceed what a 32-bit
  address can represent; use `MemoryType::shared(initial, max)` for shared
  memories (`wasmtime-50.0.2/src/runtime/types.rs:3472-3530`).
* `Config::memory_reservation` defaults to **4 GiB on 64-bit hosts**
  (`config.rs:1960-1970`) and reserves virtual address space, not physical
  memory. This is what lets Wasmtime elide bounds checks for wasm32. Do not
  shrink it unless you accept bounds checks.
* `Config::memory_guard_size` defaults to **32 MiB on 64-bit**
  (`config.rs:2051-2054`). `memory_reservation_for_growth` defaults to 2 GiB
  (`config.rs:2115-2118`). These are virtual reservations; a rustc Store plus a
  Wild Store each reserve their own 4 GiB of address space.
* To enforce the browser's 384 MiB ceiling, use
  `StoreLimitsBuilder::new().memory_size(384 << 20).build()` and
  `store.limiter(|h| &mut h.limits)`
  ([`src/runtime/limits.rs:238`](https://docs.rs/crate/wasmtime/50.0.2/source/src/runtime/limits.rs#L238),
  [`src/runtime/store.rs:902-970`](https://docs.rs/crate/wasmtime/50.0.2/source/src/runtime/store.rs#L902-L970)).
  `StoreLimits` limits growth, not the initial allocation (`store.rs:915`), so
  if you need a hard initial cap, define the imported shared memory yourself
  with `SharedMemory::new(&engine, MemoryType::shared(initial, Some(6144)))`
  (6144 pages = 384 MiB) and `linker.define("env", "memory", mem)`.
* The browser's `maxPages` is a `WebAssembly.Memory` maximum. The Wasmtime
  analogue is either the module's declared max or a host-defined shared memory.
  Wasmtime does not provide a `Config` knob to lower a module-defined memory's
  max.
* Guard pages in Wasmtime protect the *linear memory allocation*, not guest
  thread stacks. Wasm has no guard pages inside its own stack; a thread that
  overflows writes into the heap below it. The playground patch adds
  `__stack_limit` checks for cg_clif code (`patches/rust/0021-...`), but LLVM
  code (std, prebuilt crates) does not check, hence the README's warning.

---

## 5. Existing evidence: Node and older Wasmtime

### 5.1 Node kernel command (exact)

The playground README's “Running under Node” section
([`README.md:133-148`](https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/README.md#L133-L148)):

```sh
SC=/sysroot/lib/rustlib/wasm32-wasip1-threads/lib/self-contained
node web/browser-kernel/run.js --dir vendor/rust/dist=/sysroot --dir $PWD/work=/work \
  --program wasm-ld=vendor/wild/target/wasm32-wasip1-threads/release/wild.wasm \
  --save /work/main.wasm=work/main.wasm \
  vendor/rust/build/wasm32-wasip1-threads/stage2/bin/rustc.wasm /work/main.rs \
  --target=wasm32-wasip1-threads --sysroot=/sysroot -o /work/main.wasm \
  -Clinker=wasm-ld -Clinker-flavor=wasm-ld -Clink-arg=$SC/crt1-command.o -L $SC
```

`run.js` accepts `--dir host=guest`, `--program name=file.wasm`,
`--env name=value`, `--save guest=host`; its default env is only
`RUST_BACKTRACE=0`, stdin is empty, stdout/stderr go to the Node process
([`browser-kernel/run.js:1-10,46-49`](https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/run.js#L1-L10)).
The Node kernel is the same `browser-kernel` package, using `worker_threads`
instead of Web Workers. It implements wasi-threads and `playground.spawn` in
plain JS. It proves the ABI and the compiler can run, but it is not directly
embeddable as a Wasmtime-in-Rust host; it is a separate runtime.

### 5.2 Older Wasmtime command (proof it used to work)

The rust patch's own comment (removed later) documents running `rustc.wasm`
under Wasmtime with wasi-threads
([`patches/rust/0001-...patch:119-129`](https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/patches/rust/0001-WIP-Support-compiling-rustc-to-WebAssembly.patch#L119-L129)):

```sh
wasmtime run -Sthreads=y -Spreview2=n --dir tmp::/ --dir dist \
  --env RUST_MIN_STACK=16777216 dist/bin/rustc.wasm - --sysroot dist \
  --target x86_64-unknown-linux-gnu -Csave-temps
```

That is the exact prior art for the intended host. In Wasmtime 50.0.2 both
`-Sthreads` and `-Spreview2=n` are rejected; p1 is the default core-module WASI.
The playground repo itself has no Wasmtime-in-Rust host; `tools/wild-test-env.sh`
mentions `tools/wasmtime` only for Wild's own wasm tests.

### 5.3 Adapting to a Wasmtime-in-Rust host

* Keep the same argv protocol; implement `playground.spawn` with
  `Caller::get_export("memory")`, `Memory::read` for the pointer array and each
  NUL-terminated string (don't take a raw slice; use `Memory::read` to handle
  out-of-bounds safely).
* Run native Wild with `std::process::Command` and `.args(&argv[1..])`, inheriting
  stdio. Return `status.code().unwrap_or(-1)`. This is what the current Stage B
  host does. It is simpler than a second wasm instance and avoids the wild.wasm
  thread issue.
* If you want browser parity (wild.wasm), instantiate `wild.wasm` in a second
  `Store` with the same preopened host dirs and `_start(argv)`, then return its
  exit code. The child needs its own `WasiP1Ctx`; do not share the rustc Store.
* Path mapping: with native Wild, guest `/work`/`/sysroot` must equal host
  paths (preopen `/` at `/`), or rewrite those prefixes in argv before spawning.
* Env: the ABI passes no env delta. Set `RUST_MIN_STACK`, `HOME`, `TMPDIR`,
  `RUST_BACKTRACE=0` on the rustc `WasiCtxBuilder`. Native Wild inherits the
  host env; wild.wasm gets a fixed minimal env.
* The hard part remains §2: this design still needs a wasi-threads host for
  rustc's `rustc` compilation thread.

---

## 6. Proc macros for a no_std plugin

The playground `rustc.wasm` includes wasmi-based wasm proc-macro support. The
flag is opt-in:

* `-Zwasm-proc-macros` is an unstable rustc option, default **false**
  (`compiler/rustc_session/src/options.rs:3026`). It enables compiling
  `--crate-type=proc-macro` to wasm and loading wasm proc macros
  (`src/doc/unstable-book/src/compiler-flags/wasm-proc-macros.md`).
* For a **no_std plugin with no proc-macro dependencies**, it is **not needed**.
  Do not pass it. rustc only tries the wasm proc-macro path when the flag is set
  (`compiler/rustc_metadata/src/creader.rs:730-745`); normal crates compile
  without it.
* If a dependency is a wasm proc-macro crate (as in the playground's `--extern`
  crates), pass `-Zwasm-proc-macros`; the playground adds it whenever it passes
  extern crates (`web/kernel/crates.js:81-87`), and its `crates/build.py`
  wrapper adds it to every target-crate compile.
* If it must be forced off despite a build default, use
  `-Zwasm-proc-macros=no`. There is no runtime environment variable; the
  `rust.wasm-proc-macros` setting in `bootstrap.toml` is build-time only
  (`rust/bootstrap.example.toml:960`). The current Stage B `bootstrap.toml`
  only has a comment for the wasm32-wasip1 target; it does not set the build
  default to true.
* `-Zthreads=1` is unrelated to proc macros; it limits parallel compiler jobs.

---

## 7. Recommendations for Stage B

1. **Do not build the Stage B host on Wasmtime 50.0.2 if it must run
   `rustc.wasm` unmodified.** The missing `wasi:thread-spawn` is a hard blocker.
2. Short term: pin `wasmtime = wasmtime-wasi = wasmtime-wasi-threads = 46.0.3`
   (or 36.0.21 LTS) for the rustc host, and use `WasiThreadsCtx::new` +
   `add_to_linker` with a cloneable host state. Keep the 50.0.2 host for
   evaluating the linked plugin, which does not need wasi-threads.
3. If staying on 50.0.2, patch rustc's `run_in_thread_with_globals` (and the
   parallel path) to execute on the current thread on WASI, then use
   `define_unknown_imports_as_traps` for the residual `wasi.thread-spawn`
   import. This is the cleanest way to keep a single modern Wasmtime.
4. Implement `playground.spawn` as a blocking host call that runs native Wild
   on the same host paths; this is independent of the thread issue and can be
   reused with either Wasmtime version.

---

## Sources

* Playground repo (pinned): <https://github.com/ryancbuckley/rust-playground-inbrowser/tree/f5298630dda0f2dc2b44f6ae1b54add7a77d8316>
  * `web/kernel/kernel-worker.js`: <https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/web/kernel/kernel-worker.js>
  * `patches/rust/0003-...patch`: <https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/patches/rust/0003-Run-the-linker-through-the-host-s-playground.spawn-o.patch>
  * `patches/rust/0015-...patch`: <https://github.com/ryancbuckley/rust-playground-inbrowser/blob/f5298630dda0f2dc2b44f6ae1b54add7a77d8316/patches/rust/0015-WASI-linker-report-a-failed-link-as-such-not-as-a-mi.patch>
* Browser kernel (pinned): <https://github.com/ryancbuckley/browser-kernel/tree/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e>
  * `kernel.js`: <https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/kernel.js>
  * `thread-worker.js`: <https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/thread-worker.js>
  * `run.js`: <https://github.com/ryancbuckley/browser-kernel/blob/a2cdcb3f69bb735ec8bb0269cd1299d85460c29e/run.js>
* Wasmtime 50.0.2 crate: <https://crates.io/crates/wasmtime/50.0.2> / <https://docs.rs/wasmtime/50.0.2/wasmtime/>
  * `Config::wasm_threads`: <https://docs.rs/wasmtime/50.0.2/wasmtime/struct.Config.html#method.wasm_threads>
  * `Config::max_wasm_stack`: <https://docs.rs/wasmtime/50.0.2/wasmtime/struct.Config.html#method.max_wasm_stack>
  * `Config::memory_reservation`: <https://docs.rs/wasmtime/50.0.2/wasmtime/struct.Config.html#method.memory_reservation>
  * `StoreLimitsBuilder::memory_size`: <https://docs.rs/wasmtime/50.0.2/wasmtime/struct.StoreLimitsBuilder.html#method.memory_size>
  * `Linker::define_unknown_imports_as_traps`: <https://docs.rs/wasmtime/50.0.2/wasmtime/struct.Linker.html#method.define_unknown_imports_as_traps>
* Wasmtime WASI 50.0.2: <https://docs.rs/wasmtime-wasi/50.0.2/wasmtime_wasi/>
  * `p1::WasiP1Ctx`: <https://docs.rs/wasmtime-wasi/50.0.2/wasmtime_wasi/p1/struct.WasiP1Ctx.html>
  * `p1::add_to_linker_sync`: <https://docs.rs/wasmtime-wasi/50.0.2/wasmtime_wasi/p1/fn.add_to_linker_sync.html>
  * `WasiCtxBuilder::preopened_dir`: <https://docs.rs/wasmtime-wasi/50.0.2/wasmtime_wasi/struct.WasiCtxBuilder.html#method.preopened_dir>
* `wasmtime-wasi-threads` 46.0.3 (last release): <https://docs.rs/wasmtime-wasi-threads/46.0.3/wasmtime_wasi_threads/>
* Removal of wasi-threads: <https://github.com/bytecodealliance/wasmtime/pull/13558>, <https://github.com/bytecodealliance/rfcs/pull/47>, release notes <https://github.com/bytecodealliance/wasmtime/blob/release-47.0.0/RELEASES.md#L91>.
* Rust source (`nightly-2026-10-19`): <https://github.com/rust-lang/rust/tree/b6eeb93c992027a03e1fd120b6652546396b142c>
  * `compiler/rustc_interface/src/util.rs`: <https://github.com/rust-lang/rust/blob/b6eeb93c992027a03e1fd120b6652546396b142c/compiler/rustc_interface/src/util.rs#L175-L210>
  * `compiler/rustc_session/src/config.rs` (`DEFAULT_STACK_SIZE`): <https://github.com/rust-lang/rust/blob/b6eeb93c992027a03e1fd120b6652546396b142c/compiler/rustc_session/src/config.rs#L57>
  * `compiler/rustc_session/src/options.rs` (`wasm_proc_macros`): <https://github.com/rust-lang/rust/blob/b6eeb93c992027a03e1fd120b6652546396b142c/compiler/rustc_session/src/options.rs#L3026>
