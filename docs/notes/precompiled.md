# Precompiled `rustc.wasm` (`.cwasm`)

The CogWasm compiler can ship a Wasmtime-precompiled `rustc.wasm` so a host does not pay
the ~40 s cold compile of the 137 MiB module. This note covers the new library
split, the exact `main.rs` integration, and how the artifact is verified and
packaged. The lead integrated this snippet into `main.rs` after Track B; this file
remains the reference for the option semantics. One difference from the
snippet below: the integrated `self_args` passes `--precompiled` through to
`cold-once`/`cached-once`, so `cogwasm-host bench --precompiled X` reports the
precompiled path in its cold/cached fields. `cogwasm-precompile bench` is the
tool that reports the `.wasm` cold/cached and `.cwasm` deserialize numbers
separately.

## What was added

| File | Purpose |
|---|---|
| `crates/cogwasm-host/src/config.rs` | The single Wasmtime engine configuration (moved from `main.rs`, behaviour unchanged) plus a `ConfigSummary` that is recorded in metadata. |
| `crates/cogwasm-host/src/precompiled.rs` | `load_rustc_module`, sidecar metadata, SHA-256 helpers, compatibility hash, fallback. |
| `crates/cogwasm-host/src/lib.rs` | Library target shared by the host and precompile binaries. |
| `crates/cogwasm-host/src/bin/precompile.rs` | `cogwasm-precompile precompile` and `cogwasm-precompile bench`. |
| `release/package.sh` | Build host + Wild, precompile, assemble one host's release bundle. |

`cogwasm-host` itself is unchanged in this track; the snippet below is what
the lead applies.

## CLI

```sh
# One-time precompile for this host. Writes rustc.cwasm and rustc.cwasm.json.
cogwasm-precompile precompile \
    --wasm rustc.wasm --out rustc-aarch64-apple-darwin.cwasm \
    --metadata rustc-aarch64-apple-darwin.cwasm.json

# Hot-path vs opt-in verified load, medians of 5. --skip-cold reuses the
# existing cold baseline instead of recompiling rustc.wasm five times.
cogwasm-precompile bench \
    --wasm rustc.wasm --cwasm rustc-aarch64-apple-darwin.cwasm \
    --runs 5 --skip-cold --verify \
    --json bench.json
```

`precompile --wasm-simd false` is a deliberately different engine
configuration, used only to test the compatibility fallback; it also disables
relaxed SIMD because Wasmtime rejects SIMD off with relaxed SIMD on.

## Metadata and verification

`rustc-<host>.cwasm.json` records:

- `format_version` (currently 1);
- `generator`: the binary and version that wrote the file
  (`cogwasm-precompile <version>`), for diagnosis only;
- the Wasmtime version (`50.0.2`);
- `compatibility_hash`: `Engine::precompile_compatibility_hash()` as lowercase
  hex (long: it is the exact hash byte stream);
- `target`: arch/os/family/env/vendor and a derived Rust triple;
- `config`: the `ConfigSummary` fields;
- `wasm` and `cwasm`: SHA-256 and byte size.

The compatibility hash is Wasmtime's hash of the engine configuration,
Wasmtime version and target; it does **not** include the host crate name, the
precompile binary name or any `stage-c` string. Renaming the package or the
binary therefore does not change the hash. The `generator` string does change,
but verification never compares it, so existing `.cwasm` files still load.
A `.cwasm` is invalidated only by a Wasmtime version/configuration/target
change, and then the host falls back to compiling the `.wasm`.

On every load the host checks the format version, Wasmtime version,
compatibility hash, target and both file sizes, then
`Module::deserialize_file`. Sizes catch truncation without hashing 400+ MiB.
The full SHA-256 of both files is **opt-in**
(`--verify-precompiled` on the host, `--verify` on `precompile bench`); it is an
install/download-integrity check, not a per-load one. The release bundle ships
`SHA256SUMS`, so an installer verifies once. A mismatch of any kind prints a
warning naming the reason and falls back to `Module::new(rustc.wasm)`, which is
the old behaviour.

The `unsafe { Module::deserialize_file }` SAFETY comment says the cwasm is a
trusted, read-only installed build artifact sharing the host binary's trust
boundary; Wasmtime itself rejects a different version/config/target, and the
sidecar checks make a mix-up fall back rather than crash. It must stay
unmodified for the module's lifetime because Wasmtime mmaps it.

## `main.rs` integration snippet

Apply these edits after Track B's `main.rs` changes. They do not alter
behaviour when `--precompiled` is absent.

### 1. Imports

```diff
-use wasmtime::{Cache, CacheConfig, Caller, Config, Engine, Extern, Linker, Memory, Module, Store};
+use cogwasm_host::config::engine;
+use cogwasm_host::precompiled::load_rustc_module;
+use wasmtime::{Caller, Engine, Extern, Linker, Memory, Module, Store};
```

### 2. `Opts`, usage and parsing

```diff
 struct Opts {
     ...
     cache_dir: Option<PathBuf>,
+    /// Optional Wasmtime-precompiled rustc.wasm.
+    precompiled: Option<PathBuf>,
+    /// Hash the .wasm and .cwasm on every load (opt-in, slow).
+    verify_precompiled: bool,
     ...
 }
```

```diff
        --cache-dir DIR    enable Wasmtime's compiled-module cache
+       --precompiled PATH  load this .cwasm instead of compiling rustc.wasm
+       --verify-precompiled  also check the full SHA-256 of both files
```

```diff
             cache_dir: None,
+            precompiled: None,
+            verify_precompiled: false,
             runs: 5,
```

```diff
             "--cache-dir" => o.cache_dir = Some(val("--cache-dir")?.into()),
+            "--precompiled" => o.precompiled = Some(val("--precompiled")?.into()),
+            "--verify-precompiled" => o.verify_precompiled = true,
```

### 3. Remove the now-shared engine functions

Delete the whole `fn engine_config(cache_dir: Option<&Path>) -> Result<Config>`
and `fn engine(cache_dir: Option<&Path>) -> Result<Engine>` definitions (the
`Engine / store` section around them). `engine` now comes from
`cogwasm_host::config`; its behaviour is identical (threads off, bulk memory,
multi-memory, SIMD, 8 MiB wasm stack, 16 MiB async stack, same cache settings).

### 4. `run_rustc`

```diff
 fn run_rustc(o: &Opts, expect: f64, measure_plugin: bool) -> Result<(RunResult, f64)> {
-    let bytes =
-        std::fs::read(&o.rustc).with_context(|| format!("reading {}", o.rustc.display()))?;
     let engine = engine(o.cache_dir.as_deref())?;
-    let t = Instant::now();
-    let module = Module::new(&engine, &bytes).wt("compiling rustc.wasm")?;
-    let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
-    let mut r = run_rustc_with(o, &engine, &module, measure_plugin)?;
+    let loaded = load_rustc_module(
+        &engine, &o.rustc, o.precompiled.as_deref(), o.verify_precompiled,
+    )?;
+    let module_load_ms = loaded.load_ms;
+    let mut r = run_rustc_with(o, &engine, &loaded.module, measure_plugin)?;
     r.module_load_ms = module_load_ms;
     let _ = expect;
-    std::mem::forget(module);
+    std::mem::forget(loaded.module);
     std::mem::forget(engine);
     Ok((r, module_load_ms))
 }
```

### 5. `startup_once` (used by `cold-once` / `cached-once`)

```diff
 fn startup_once(o: &Opts, cached: bool) -> Result<()> {
-    let bytes =
-        std::fs::read(&o.rustc).with_context(|| format!("reading {}", o.rustc.display()))?;
     let e = engine(if cached { o.cache_dir.as_deref() } else { None })?;
-    let t = Instant::now();
-    let m = Module::new(&e, &bytes).wt("startup Module::new")?;
-    let module_load_ms = t.elapsed().as_secs_f64() * 1e3;
+    let loaded = load_rustc_module(
+        &e, &o.rustc, o.precompiled.as_deref(), o.verify_precompiled,
+    )?;
+    let module_load_ms = loaded.load_ms;
+    let source = loaded.source.as_str();
+    let fallback_reason = loaded.fallback_reason.clone();
+    let wasm_bytes = std::fs::metadata(&o.rustc)
+        .with_context(|| format!("reading {}", o.rustc.display()))?
+        .len();
     let t = Instant::now();
-    let (store, _instance) = prepare_instance(&e, &m, o, rustc_args(o))?;
+    let (store, _instance) = prepare_instance(&e, &loaded.module, o, rustc_args(o))?;
     let instantiate_ms = t.elapsed().as_secs_f64() * 1e3;
     std::mem::forget(store);
     println!(
         "{}",
         serde_json::json!({
             "mode": if cached { "cached" } else { "cold" },
+            "source": source,
+            "fallback_reason": fallback_reason,
             "module_load_ms": module_load_ms,
             "instantiate_ms": instantiate_ms,
-            "bytes": bytes.len(),
+            "bytes": wasm_bytes,
         })
     );
```

### 6. `self_args` / `subprocess_startup` and `bench`

Add a `with_precompiled` argument so the cold baseline can still measure the
`.wasm` path even when a `.cwasm` is supplied, and measure the precompiled
startup separately:

```diff
-fn self_args(o: &Opts, cmd: &str, cached: bool) -> Vec<String> {
+fn self_args(o: &Opts, cmd: &str, cached: bool, with_precompiled: bool) -> Vec<String> {
     ...
+    if with_precompiled {
+        if let Some(precompiled) = &o.precompiled {
+            v.push("--precompiled".into());
+            v.push(precompiled.display().to_string());
+            if o.verify_precompiled {
+                v.push("--verify-precompiled".into());
+            }
+        }
+    }
     v
 }
```

```diff
-fn subprocess_startup(o: &Opts, cmd: &str, cached: bool) -> Result<(f64, f64)> {
+fn subprocess_startup(
+    o: &Opts,
+    cmd: &str,
+    cached: bool,
+    with_precompiled: bool,
+) -> Result<(f64, f64)> {
     let exe = std::env::current_exe()?;
     let out = Command::new(exe)
-        .args(self_args(o, cmd, cached))
+        .args(self_args(o, cmd, cached, with_precompiled))
```

In `bench`, keep the existing `cold-once` / `cached-once` calls on the
`.wasm` path (`false`), then add:

```rust
    let mut precompiled_loads = Vec::new();
    if o.precompiled.is_some() {
        for _ in 0..o.runs {
            let (load, _) = subprocess_startup(o, "cold-once", false, true)?;
            precompiled_loads.push(load);
        }
        med.insert("module_load_precompiled_ms".into(), median(&precompiled_loads));
    }
```

### 7. The warm module in `bench`

```diff
     let warm_engine = engine(o.cache_dir.as_deref())?;
-    let t = Instant::now();
-    let warm_module = Module::new(&warm_engine, &rustc_bytes).wt("loading rustc.wasm")?;
-    let warm_module_load_ms = t.elapsed().as_secs_f64() * 1e3;
+    let warm_loaded = load_rustc_module(
+        &warm_engine, &o.rustc, o.precompiled.as_deref(), o.verify_precompiled,
+    )?;
+    let warm_module_load_ms = warm_loaded.load_ms;
+    let warm_module = warm_loaded.module;
```

`rustc_bytes` is still read above for `rustc_sha256`; that is unchanged.

### 8. `incremental` and `raw`

Both load `rustc.wasm` directly; use the same helper:

```diff
-    let rustc_bytes =
-        std::fs::read(&o.rustc).with_context(|| format!("reading {}", o.rustc.display()))?;
-    let module = Module::new(&engine, &rustc_bytes).wt("loading rustc.wasm")?;
+    let loaded = load_rustc_module(
+        &engine, &o.rustc, o.precompiled.as_deref(), o.verify_precompiled,
+    )?;
+    let module = loaded.module;
```

```diff
-    let bytes = std::fs::read(&o.rustc)
-        .with_context(|| format!("reading {}", o.rustc.display()))?;
     let engine = engine(o.cache_dir.as_deref())?;
-    let module = Module::new(&engine, &bytes).wt("compiling rustc.wasm")?;
+    let loaded = load_rustc_module(
+        &engine, &o.rustc, o.precompiled.as_deref(), o.verify_precompiled,
+    )?;
+    let module = loaded.module;
```

`rustc_args`/`prepare_instance` are unchanged; `eval_plugin` still compiles the
small plugin `.wasm` directly and is untouched.

## Fallback test evidence

Commands (run on macOS arm64 with the regenerated artifact):

```sh
# (a) corrupt one byte: only the opt-in digest path can see it; the default
# path relies on Wasmtime's own file validation.
cp rustc.cwasm bad.cwasm && cp rustc.cwasm.json bad.cwasm.json
printf '\x00' | dd of=bad.cwasm bs=1 seek=4096 conv=notrunc
cogwasm-host check --precompiled bad.cwasm --verify-precompiled ...   # warning + fallback
# (b) compile with --wasm-simd false: compatibility hash differs; falls back.
# (c) both still compile and evaluate plugin (c): eval(1,2,3,4) == 2.25.
```

Results: **pending the final `rustc.wasm`** (Track C rebuilds it). The unit
tests already cover the metadata and fallback paths (11 tests, all passing):

- the moved engine configuration has the same compatibility hash as the exact
  historical `main.rs` configuration, so removing the local functions cannot
  change runtime behaviour or invalidate existing Wasmtime caches;
- same-size corrupt cwasm: default check passes (no digest), `--verify` rejects
  with a SHA-256 mismatch;
- truncated cwasm: default check rejects on size;
- stale/same-size different `.wasm`: digest check rejects;
- wrong Wasmtime version, compatibility hash or target: rejected;
- a no-SIMD engine has a different compatibility hash;
- a cache-directory engine has the **same** compatibility hash, so a host run
  with `--cache-dir` accepts a `.cwasm` precompiled without one;
- a non-cwasm artifact and a missing sidecar both fall back to
  `Module::new` (the fallback compiles a tiny empty wasm module, so the test is
  fast and does not need rustc.wasm).

## Measurements

Before/after on this machine (Apple M4 Pro, macOS arm64, Wasmtime 50.0.2,
median of 5). Before is the previous stage-C build; after is the final
combined-build artifact 144,051,631 B SHA-256 `d2404a86…6fb91`:

| Path | Before | After |
|---|---:|---:|
| cold `Module::new(144,051,631 B .wasm)`, no cache | 37,561 ms | same path, unchanged |
| Wasmtime cache load | 442.5 ms | **461.5 ms** |
| `Module::deserialize_file(.cwasm)` | — | **19.46 ms** |
| `load_rustc_module` (sidecar + sizes + deserialize) | — | **18.74 ms** |
| `load_rustc_module --verify` (both SHA-256) | — | **759.0 ms** |
| precompile the .cwasm | — | **38,247 ms** |

Raw samples on the final artifact: deserialize
`[23.48, 16.75, 20.48, 19.46, 17.50]`, default load
`[22.26, 17.62, 18.43, 19.23, 18.74]`, verified
`[773.0, 750.4, 764.9, 759.0, 749.2]`.

`rustc.wasm` 144,051,631 B → `.cwasm` **296,372,016 B** (282 MiB). Wasmtime
native code is larger than the wasm. The opt-in digest check is ~40× the hot
path; it is for install/download integrity, not per-load use.
Wasmtime
native code is larger than the wasm. The final numbers are regenerated after
Track C's build and recorded here.
