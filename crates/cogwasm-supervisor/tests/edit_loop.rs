//! Pure tests for the edit-loop source helpers: no toolchain, no compiler.
//!
//! The edit loop relies on two invariants that are cheaper to pin down here
//! than through a real build: every edit changes the source bytes, and the
//! caller's file is never written.

use std::fs;
use std::path::PathBuf;

use cogwasm_supervisor::{apply_edit, copy_source, has_edit_probe, probe_value};
use tempfile::TempDir;

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/edit_loop.rs")
}

#[test]
fn edits_change_bytes_alternate_probe_and_leave_the_original_untouched() {
    let dir = TempDir::new().expect("creating the test temp dir");
    let original = fixture();
    let before = fs::read(&original).expect("reading the fixture");

    let copy = copy_source(&original, dir.path()).expect("copying the fixture");
    assert_eq!(
        fs::read(&copy).expect("reading the copy"),
        before,
        "copy_source must copy the source bytes unchanged"
    );

    let mut text = fs::read_to_string(&copy).expect("reading the copy as text");
    assert!(has_edit_probe(&text), "the fixture must contain the probe");
    assert_eq!(probe_value(&text), Some(0), "the fixture starts at 0u64");

    let mut seen = Vec::new();
    for index in 1..=6 {
        // Edit 1 sets 0, edit 2 sets 1, and so on.
        let probe = u64::from(index % 2 == 0);
        let next = apply_edit(&text, index, probe).expect("applying an edit");
        assert_ne!(
            next.as_bytes(),
            text.as_bytes(),
            "edit {index} must change the bytes"
        );
        assert_eq!(
            probe_value(&next),
            Some(probe),
            "edit {index} must set the probe to {probe}"
        );
        assert!(
            next.ends_with(&format!("// edit {index}\n")),
            "edit {index} must leave its marker"
        );
        seen.push(probe);
        text = next;
    }
    assert_eq!(seen, [0, 1, 0, 1, 0, 1], "the probe must alternate");

    assert_eq!(
        fs::read(&original).expect("re-reading the fixture"),
        before,
        "the caller's source must not be modified"
    );
}
