//! The resident backend against the real toolchain: two jobs share one loaded
//! rustc.wasm, a bad job is rejected, and a good job still compiles after it.

use std::fs;
use std::path::{Path, PathBuf};

use cogwasm_supervisor::{
    CompileError, CompileOptions, Compiler, JobSpec, RealCompilerConfig, ResidentCompiler,
    SourceSpec,
};
use tempfile::TempDir;

const BODY_ONE: &str = r#"#![no_std]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn f() -> u64 {
    1
}
"#;

const BODY_TWO: &str = r#"#![no_std]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn f() -> u64 {
    2
}
"#;

const BODY_BAD: &str = r#"#![no_std]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn f() -> u64 {
    1 +
}
"#;

fn env_path(variable: &str, fallback: PathBuf) -> PathBuf {
    std::env::var_os(variable).map_or(fallback, PathBuf::from)
}

/// The real toolchain, or `None` (with a message) when it is not installed.
fn toolchain() -> Option<(PathBuf, PathBuf, PathBuf, Option<PathBuf>)> {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rustc = env_path("COGWASM_RUSTC", repo.join("toolchain/artifacts/rustc.wasm"));
    let sysroot = env_path("COGWASM_SYSROOT", repo.join("toolchain/sysroot"));
    let linker = env_path(
        "COGWASM_LINKER",
        repo.join("toolchain/target/wild/release/wild"),
    );
    let missing: Vec<&Path> = [rustc.as_path(), sysroot.as_path(), linker.as_path()]
        .into_iter()
        .filter(|path| !path.exists())
        .collect();
    if !missing.is_empty() {
        let missing = missing
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        // CI sets this where the toolchain is built, so the test cannot pass
        // there by silently skipping.
        assert!(
            std::env::var_os("COGWASM_REQUIRE_TOOLCHAIN").is_none(),
            "COGWASM_REQUIRE_TOOLCHAIN is set but the toolchain is missing: {missing}"
        );
        eprintln!(
            "skipping resident test: missing {missing} (set COGWASM_RUSTC, COGWASM_SYSROOT \
             and COGWASM_LINKER to override)"
        );
        return None;
    }
    // Optional: the precompiled artifact from `compiler/run.sh precompiled-check`.
    // The loader falls back to the .wasm if it is stale or has no sidecar.
    let cwasm = repo.join("toolchain/work/precompiled/rustc.cwasm");
    let precompiled = cwasm.is_file().then_some(cwasm);
    Some((rustc, sysroot, linker, precompiled))
}

fn source(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, body).expect("writing the test source");
    path
}

fn job(path: &Path, crate_name: &str) -> JobSpec {
    JobSpec::new(SourceSpec::new(path, crate_name), CompileOptions::default())
}

#[test]
fn two_jobs_share_one_loaded_module_and_survive_a_rejection() {
    let Some((rustc, sysroot, linker, precompiled)) = toolchain() else {
        return;
    };
    let dir = TempDir::new().expect("creating the test temp dir");
    let config = RealCompilerConfig {
        // Not used by the resident backend, but the config requires it.
        host: PathBuf::new(),
        rustc,
        sysroot,
        linker,
        cache_dir: None,
        precompiled,
        work_root: dir.path().join("work"),
        compiler_id: "resident-test".to_owned(),
    };
    let compiler = ResidentCompiler::new(config).expect("loading resident rustc.wasm");
    let load_ms = compiler.module_load_ms();
    eprintln!("resident module load: {load_ms:.1} ms");

    let one = source(&dir, "one.rs", BODY_ONE);
    let two = source(&dir, "two.rs", BODY_TWO);
    let job_one = job(&one, "resident_one");
    let job_two = job(&two, "resident_two");

    let first = compiler
        .compile(&job_one, "job-one")
        .expect("the first resident job must compile");
    assert!(!first.bytes.is_empty(), "the artifact must not be empty");
    assert!(
        first.bytes.starts_with(b"\0asm"),
        "the artifact must be a wasm module"
    );
    assert_eq!(compiler.loads(), 1, "the module must be loaded only once");

    let second = compiler
        .compile(&job_two, "job-two")
        .expect("the second resident job must compile");
    assert!(!second.bytes.is_empty(), "the artifact must not be empty");
    assert!(
        second.bytes.starts_with(b"\0asm"),
        "the artifact must be a wasm module"
    );
    assert_ne!(first.bytes, second.bytes, "the sources differ");
    assert_eq!(compiler.loads(), 1, "the module must be loaded only once");

    let bad = source(&dir, "bad.rs", BODY_BAD);
    let bad_job = job(&bad, "resident_bad");
    let error = compiler
        .compile(&bad_job, "job-bad")
        .expect_err("a syntax error must be rejected");
    assert!(
        matches!(error, CompileError::Rejected(_)),
        "unexpected error: {error}"
    );
    assert_eq!(
        compiler.loads(),
        1,
        "a rejected job must not reload the module"
    );

    let after = compiler
        .compile(&job_one, "job-after-error")
        .expect("the backend must still compile after a rejection");
    assert!(
        after.bytes.starts_with(b"\0asm"),
        "the later artifact must still be a wasm module"
    );
    assert_eq!(compiler.loads(), 1, "the module must be loaded only once");

    eprintln!(
        "resident jobs: one {} ms (link {} ms, {} bytes), two {} ms (link {} ms, {} bytes), \
         rejected, after-error {} ms",
        first.compile_ms.unwrap_or_default(),
        first.link_ms.unwrap_or_default(),
        first.bytes.len(),
        second.compile_ms.unwrap_or_default(),
        second.link_ms.unwrap_or_default(),
        second.bytes.len(),
        after.compile_ms.unwrap_or_default()
    );
}
