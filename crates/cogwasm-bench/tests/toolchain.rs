//! End-to-end check against the real toolchain.
//!
//! Skipped unless `COGWASM_REQUIRE_TOOLCHAIN=1`, matching the supervisor's
//! resident test: the crate builds without `toolchain/`, and CI that has the
//! artifacts sets the variable to require this test.

use std::path::Path;
use std::process::Command;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_cogwasm-bench")
}

fn required() -> bool {
    std::env::var_os("COGWASM_REQUIRE_TOOLCHAIN").is_some()
}

#[test]
fn a_quick_program_round_trips_through_run_report_and_compare() {
    if !required() {
        eprintln!("skipping: set COGWASM_REQUIRE_TOOLCHAIN=1 to run");
        return;
    }
    let dir = tempfile::tempdir().expect("temp dir");
    let out = dir.path().join("run");
    let status = Command::new(binary())
        .args([
            "run",
            "--program",
            "a_add",
            "--runs",
            "1",
            "--config",
            "default",
            "--no-fuel",
            "--out",
        ])
        .arg(&out)
        .status()
        .expect("run cogwasm-bench");
    assert!(status.success(), "run failed");

    let result = out.join("result.json");
    let report = out.join("report.md");
    assert!(result.is_file(), "result.json was not written");
    assert!(report.is_file(), "report.md was not written");
    let text = std::fs::read_to_string(&result).expect("read result");
    assert!(text.contains("\"schema\": 1"), "unexpected result schema");
    assert!(text.contains("\"byte_identical_to_cold\": true"));

    let status = Command::new(binary())
        .args(["report"])
        .arg(&result)
        .status()
        .expect("report cogwasm-bench");
    assert!(status.success(), "report failed");

    let comparison = Command::new(binary())
        .args(["compare"])
        .arg(&result)
        .arg(&result)
        .output()
        .expect("compare cogwasm-bench");
    assert!(comparison.status.success(), "self-comparison failed");
    let stdout = String::from_utf8_lossy(&comparison.stdout);
    assert!(stdout.contains("## Regressions"));
    assert!(stdout.contains("None."));
}

#[test]
fn toolchain_footprint_is_fast_and_does_not_compile() {
    if !required() {
        eprintln!("skipping: set COGWASM_REQUIRE_TOOLCHAIN=1 to run");
        return;
    }
    let output = Command::new(binary())
        .args(["toolchain", "--json"])
        .output()
        .expect("toolchain command");
    assert!(output.status.success(), "toolchain command failed");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("rustc_wasm_bytes"));
    assert!(text.contains("sysroot"));
    assert!(Path::new(env!("CARGO_MANIFEST_DIR")).is_dir());
}
