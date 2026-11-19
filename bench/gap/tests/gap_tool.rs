//! End-to-end checks for the `gap-tool` command line.

use std::path::Path;
use std::process::Command;

/// A minimal valid module: one `() -> ()` function exported as `f`.
const MINIMAL_WASM: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic and version
    0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // type: one `() -> ()`
    0x03, 0x02, 0x01, 0x00, // function: one function, type 0
    0x07, 0x05, 0x01, 0x01, 0x66, 0x00, 0x00, // export: "f"
    0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b, // code: empty body
];

/// `bench/gap/work/` is git-ignored, so a fresh checkout has no
/// `work/instrumented/`. The command must create its output directories
/// instead of failing with `ENOENT` as it did in CI after building sha256 and
/// sort but before measuring them.
#[test]
fn instrument_creates_missing_output_directories() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("missing-output-directories");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("create the test directory");
    let wasm = root.join("input.wasm");
    std::fs::write(&wasm, MINIMAL_WASM).expect("write the input module");
    let out = root.join("work").join("instrumented").join("module.wasm");
    let meta = root.join("meta").join("module.meta.json");
    let status = Command::new(env!("CARGO_BIN_EXE_gap-tool"))
        .arg("instrument")
        .arg("--wasm")
        .arg(&wasm)
        .arg("--out")
        .arg(&out)
        .arg("--meta")
        .arg(&meta)
        .status()
        .expect("run gap-tool instrument");
    assert!(status.success(), "gap-tool instrument failed: {status}");
    assert!(out.is_file(), "{} was not written", out.display());
    assert!(meta.is_file(), "{} was not written", meta.display());
}
