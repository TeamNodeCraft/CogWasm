//! The M1 golden tests: cache, dirty rebuild, tiers, invalidation, failure
//! fallback, supersession and swap ordering, with no toolchain involved.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use cogwasm_supervisor::{
    ArtifactStore, Begin, CompileOptions, CompileOutput, FakeCompiler, FakeRuntime, JobSpec,
    Preset, Runtime, SourceSpec, Supervisor, Swapper, WasmtimeRuntime, wat_artifact,
};
use tempfile::TempDir;

fn source(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, body).expect("write test source");
    path
}

fn job(path: &PathBuf, preset: Preset) -> JobSpec {
    let mut job = JobSpec::preset(SourceSpec::new(path, "generated_stress"), preset);
    if job.options.incremental {
        job.incremental_dir = Some(
            path.parent()
                .unwrap_or_else(|| Path::new("."))
                .join("incremental"),
        );
    }
    job
}

fn store(dir: &TempDir) -> ArtifactStore {
    ArtifactStore::new(dir.path().join("store"))
}

#[test]
fn unchanged_source_hits_the_cache() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());
    let job = job(&src, Preset::Fast);

    let first = supervisor.build(&job).unwrap();
    assert!(!first.was_hit(), "the first build must compile");
    let second = supervisor.build(&job).unwrap();
    assert!(second.was_hit(), "the second build must hit the cache");
    assert_eq!(first.artifact().bytes, second.artifact().bytes);
    assert_eq!(supervisor.compiler().invocations(), 1);
}

#[test]
fn an_edit_rebuilds() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());
    let job = job(&src, Preset::Fast);

    let first = supervisor.build(&job).unwrap();
    fs::write(&src, "pub fn f() -> u64 { 2 }\n").unwrap();
    let second = supervisor.build(&job).unwrap();

    assert!(!second.was_hit(), "the edit must force a rebuild");
    assert_ne!(first.artifact().key, second.artifact().key);
    assert_eq!(supervisor.compiler().invocations(), 2);
}

#[test]
fn presets_have_distinct_artifacts() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());

    let fast = supervisor.build(&job(&src, Preset::Fast)).unwrap();
    let opt = supervisor.build(&job(&src, Preset::Opt)).unwrap();

    assert_ne!(fast.artifact().key, opt.artifact().key);
    assert!(
        supervisor
            .store()
            .get(&fast.artifact().key)
            .unwrap()
            .is_some()
    );
    assert!(
        supervisor
            .store()
            .get(&opt.artifact().key)
            .unwrap()
            .is_some()
    );
}

#[test]
fn a_compiler_change_invalidates_the_cache() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let job = job(&src, Preset::Fast);

    let old = Supervisor::new(store(&dir), FakeCompiler::with_id("compiler-a"));
    old.build(&job).unwrap();

    let new = Supervisor::new(store(&dir), FakeCompiler::with_id("compiler-b"));
    let outcome = new.build(&job).unwrap();
    assert!(
        !outcome.was_hit(),
        "a new compiler identity must not reuse old artifacts"
    );
}

#[test]
fn the_source_path_is_part_of_the_key() {
    let dir = TempDir::new().unwrap();
    let body = "pub fn f() -> u64 { 1 }\n";
    let first = source(&dir, "a.rs", body);
    let second = source(&dir, "b.rs", body);
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());

    let a = supervisor.build(&job(&first, Preset::Fast)).unwrap();
    let b = supervisor.build(&job(&second, Preset::Fast)).unwrap();

    assert_ne!(a.artifact().key, b.artifact().key);
    assert!(!b.was_hit());
}

#[test]
fn a_failed_tier_keeps_the_previous_artifact() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(
        store(&dir),
        FakeCompiler::new().rejecting(Preset::Opt.options()),
    );
    let fast = job(&src, Preset::Fast);
    let opt = job(&src, Preset::Opt);

    supervisor.build(&fast).unwrap();
    assert!(supervisor.build(&opt).is_err(), "the broken tier must fail");
    assert!(supervisor.cached(&fast).unwrap().is_some());
    assert!(supervisor.cached(&opt).unwrap().is_none());
}

#[test]
fn a_superseded_build_is_dropped() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());
    let job = job(&src, Preset::Fast);

    let Begin::Build(first) = supervisor.begin(&job).unwrap() else {
        panic!("the first request must need a build");
    };
    let Begin::Build(second) = supervisor.begin(&job).unwrap() else {
        panic!("the second request must need a build");
    };
    assert!(first.generation < second.generation);

    assert!(
        supervisor
            .finish(&job, &first, &CompileOutput::new(b"stale".to_vec()))
            .unwrap()
            .is_none(),
        "the older build must not be stored"
    );
    let stored = supervisor
        .finish(&job, &second, &CompileOutput::new(b"fresh".to_vec()))
        .unwrap()
        .expect("the newest build is stored");
    assert_eq!(stored.bytes, b"fresh");
}

#[test]
fn a_swap_sends_calls_to_the_new_generation() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());
    let fast = supervisor.build(&job(&src, Preset::Fast)).unwrap();
    let opt = supervisor.build(&job(&src, Preset::Opt)).unwrap();

    let runtime = Arc::new(FakeRuntime::new());
    let handle: Arc<dyn Runtime> = runtime.clone();
    let mut swapper = Swapper::new(handle);

    assert!(!swapper.is_installed());
    assert!(swapper.call(1).is_err(), "no instance is installed yet");

    let first = swapper.install(fast.artifact()).unwrap();
    assert_eq!(swapper.generation(), first);
    swapper.call(7).unwrap();

    let second = swapper.install(opt.artifact()).unwrap();
    swapper.call(7).unwrap();

    assert!(second > first);
    assert_eq!(
        runtime.calls(first).unwrap(),
        1,
        "the old generation stops receiving calls"
    );
    assert_eq!(runtime.calls(second).unwrap(), 1);
}

#[test]
fn the_wasmtime_runtime_calls_run() {
    let artifact =
        wat_artifact(r#"(module (func (export "run") (param i64) (result i64) local.get 0))"#);
    let runtime = WasmtimeRuntime::new();
    let mut instance = runtime.instantiate(&artifact, 1).unwrap();
    assert_eq!(instance.generation(), 1);
    assert_eq!(instance.call(42).unwrap(), 42);
    assert_eq!(instance.call(u64::MAX).unwrap(), u64::MAX);
}

#[test]
fn specs_parse_presets_and_combinations() {
    let fast = CompileOptions::from_spec("fast").unwrap();
    assert!(fast.incremental);
    assert!(!fast.inline_wasm);

    let opt = CompileOptions::from_spec("opt").unwrap();
    assert!(opt.inline_wasm);
    assert!(!opt.incremental);

    let custom = CompileOptions::from_spec(
        "opt-level=3,inline=yes,inline-threshold=500,strip=symbols,codegen-units=4,mir-opt-level=3",
    )
    .unwrap();
    assert!(custom.inline_wasm);
    assert_eq!(custom.inline_threshold, Some(500));
    assert_eq!(custom.codegen_units, Some(4));
    assert_eq!(custom.mir_opt_level, 3);
    assert_eq!(custom.strip, cogwasm_supervisor::Strip::Symbols);

    assert!(CompileOptions::from_spec("nonsense=1").is_err());
    assert!(CompileOptions::from_spec("debug=yes,strip=symbols").is_err());
    assert!(CompileOptions::from_spec("inline-threshold=10").is_err());
}

#[test]
fn strip_changes_the_key_but_not_the_program() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());

    let opt = supervisor.build(&job(&src, Preset::Opt)).unwrap();
    let release = supervisor.build(&job(&src, Preset::Release)).unwrap();

    assert_ne!(opt.artifact().key, release.artifact().key);
}

#[test]
fn an_arbitrary_combination_reaches_the_flags() {
    let options = CompileOptions::from_spec(
        "opt-level=3,mir-opt-level=1,inline=yes,inline-threshold=250,strip=symbols,codegen-units=4",
    )
    .unwrap();
    let flags = options.flags(None);
    let joined = flags.join(" ");
    assert!(joined.contains("-Copt-level=3"));
    assert!(joined.contains("-Zmir-opt-level=1"));
    assert!(joined.contains("-Zmir-inline-wasm"));
    assert!(joined.contains("-Zmir-inline-wasm-threshold=250"));
    assert!(joined.contains("-Cstrip=symbols"));
    assert!(joined.contains("-Ccodegen-units=4"));
}

#[test]
fn incremental_without_a_directory_is_rejected() {
    let source = SourceSpec::new("/tmp/whatever.rs", "stress");
    let job = JobSpec::preset(source, Preset::Fast);
    assert!(job.validate().is_err(), "incremental needs a directory");
}

fn stateful_wat(version: i32) -> String {
    format!(
        r#"(module
  (memory (export "memory") 1)
  (global $counter (mut i64) (i64.const 0))
  (func (export "wasm_alloc") (param i32 i32) (result i32) i32.const 64)
  (func (export "run") (param $x i64) (result i64)
    global.get $counter
    local.get $x
    i64.add
    global.set $counter
    global.get $counter)
  (func (export "cogwasm_state_version") (result i32) i32.const {version})
  (func (export "cogwasm_snapshot") (param $ptr i32) (param $cap i32) (result i32)
    local.get $cap
    i32.const 8
    i32.lt_u
    if (result i32)
      i32.const -8
    else
      local.get $ptr
      global.get $counter
      i64.store
      i32.const 8
    end)
  (func (export "cogwasm_restore") (param $ptr i32) (param $len i32) (result i32)
    local.get $len
    i32.const 8
    i32.ne
    if (result i32)
      i32.const -1
    else
      local.get $ptr
      i64.load
      global.set $counter
      i32.const 0
    end))"#
    )
}

#[test]
fn state_survives_a_swap() {
    let artifact = wat_artifact(&stateful_wat(1));
    let runtime = Arc::new(WasmtimeRuntime::new());
    let mut swapper = Swapper::new(runtime);

    swapper.install(&artifact).unwrap();
    assert_eq!(swapper.call(5).unwrap(), 5);
    assert_eq!(swapper.call(7).unwrap(), 12);

    let generation = swapper.install_preserving_state(&artifact).unwrap();
    assert!(generation > 1);
    assert_eq!(
        swapper.call(1).unwrap(),
        13,
        "the counter must continue after the swap"
    );
}

#[test]
fn a_state_version_change_rejects_the_swap() {
    let first = wat_artifact(&stateful_wat(1));
    let second = wat_artifact(&stateful_wat(2));
    let runtime = Arc::new(WasmtimeRuntime::new());
    let mut swapper = Swapper::new(runtime);

    swapper.install(&first).unwrap();
    swapper.call(5).unwrap();
    assert!(
        swapper.install_preserving_state(&second).is_err(),
        "a changed state ABI version must not be installed"
    );
    assert_eq!(
        swapper.call(1).unwrap(),
        6,
        "the old instance must keep serving"
    );
}

#[test]
fn a_stateless_plugin_falls_back_to_a_cold_restart() {
    let artifact =
        wat_artifact(r#"(module (func (export "run") (param i64) (result i64) local.get 0))"#);
    let runtime = Arc::new(WasmtimeRuntime::new());
    let mut swapper = Swapper::new(runtime);

    swapper.install(&artifact).unwrap();
    assert_eq!(swapper.call(42).unwrap(), 42);
    swapper.install_preserving_state(&artifact).unwrap();
    assert_eq!(swapper.call(7).unwrap(), 7);
}
