//! The M1 golden tests: cache, dirty rebuild, tiers, invalidation, failure
//! fallback, supersession and swap ordering, with no toolchain involved.

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use cogwasm_supervisor::{
    ArtifactStore, Begin, CompileOutput, FakeCompiler, FakeRuntime, JobSpec, Runtime, SourceSpec,
    Supervisor, Swapper, Tier,
};
use tempfile::TempDir;

fn source(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, body).expect("write test source");
    path
}

fn job(path: &PathBuf, tier: Tier) -> JobSpec {
    JobSpec::new(SourceSpec::new(path, "generated_stress"), tier)
}

fn store(dir: &TempDir) -> ArtifactStore {
    ArtifactStore::new(dir.path().join("store"))
}

#[test]
fn unchanged_source_hits_the_cache() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());
    let job = job(&src, Tier::Fast);

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
    let job = job(&src, Tier::Fast);

    let first = supervisor.build(&job).unwrap();
    fs::write(&src, "pub fn f() -> u64 { 2 }\n").unwrap();
    let second = supervisor.build(&job).unwrap();

    assert!(!second.was_hit(), "the edit must force a rebuild");
    assert_ne!(first.artifact().key, second.artifact().key);
    assert_eq!(supervisor.compiler().invocations(), 2);
}

#[test]
fn tiers_have_distinct_artifacts() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new());

    let fast = supervisor.build(&job(&src, Tier::Fast)).unwrap();
    let opt = supervisor.build(&job(&src, Tier::Opt)).unwrap();

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
    let job = job(&src, Tier::Fast);

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

    let a = supervisor.build(&job(&first, Tier::Fast)).unwrap();
    let b = supervisor.build(&job(&second, Tier::Fast)).unwrap();

    assert_ne!(a.artifact().key, b.artifact().key);
    assert!(!b.was_hit());
}

#[test]
fn a_failed_tier_keeps_the_previous_artifact() {
    let dir = TempDir::new().unwrap();
    let src = source(&dir, "a.rs", "pub fn f() -> u64 { 1 }\n");
    let supervisor = Supervisor::new(store(&dir), FakeCompiler::new().rejecting(Tier::Opt));
    let fast = job(&src, Tier::Fast);
    let opt = job(&src, Tier::Opt);

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
    let job = job(&src, Tier::Fast);

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
    let fast = supervisor.build(&job(&src, Tier::Fast)).unwrap();
    let opt = supervisor.build(&job(&src, Tier::Opt)).unwrap();

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
