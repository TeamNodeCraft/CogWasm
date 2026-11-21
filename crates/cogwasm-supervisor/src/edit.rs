//! Source edits for the edit → result measurement.
//!
//! The edit loop must never touch the caller's source file. It copies the
//! fixture into its work directory, then applies the same two changes to every
//! edit:
//!
//! - set the `BENCH_EDIT_TWEAK` probe to the requested value, alternating
//!   `0u64` and `1u64`; and
//! - append a `// edit N` marker, so even an edit that happens to leave the
//!   probe's spelling unchanged still changes the source bytes.
//!
//! Both changes happen through the pure helpers below, so the cache-miss
//! guarantee can be tested without a toolchain.

use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow, bail};

/// The declaration the benchmark harness inserts into every prepared source.
const PROBE_DECL: &str = "const BENCH_EDIT_TWEAK: u64 =";

/// Whether `text` contains the benchmark edit probe.
#[must_use]
pub fn has_edit_probe(text: &str) -> bool {
    text.contains(PROBE_DECL)
}

/// The probe's current value, if `text` contains a parseable declaration.
#[must_use]
pub fn probe_value(text: &str) -> Option<u64> {
    let span = probe_span(text).ok()?;
    let declaration = &text[span];
    let value = declaration[PROBE_DECL.len()..declaration.len() - 1].trim();
    value
        .strip_suffix("u64")
        .unwrap_or(value)
        .trim()
        .parse()
        .ok()
}

/// Return `text` with the probe set to `value`.
///
/// # Errors
///
/// Returns an error when `text` has no complete probe declaration.
pub fn set_probe_value(text: &str, value: u64) -> Result<String> {
    let span = probe_span(text)?;
    let mut edited = String::with_capacity(text.len() + 8);
    edited.push_str(&text[..span.start]);
    edited.push_str(&format!("{PROBE_DECL} {value}u64;"));
    edited.push_str(&text[span.end..]);
    Ok(edited)
}

/// Apply edit `index` (1-based): set the probe and add a unique trailing line.
///
/// # Errors
///
/// Returns an error when `text` has no complete probe declaration.
pub fn apply_edit(text: &str, index: usize, probe: u64) -> Result<String> {
    let mut edited = set_probe_value(text, probe)?;
    if !edited.ends_with('\n') {
        edited.push('\n');
    }
    edited.push_str(&format!("// edit {index}\n"));
    Ok(edited)
}

/// Copy `source` into a fresh directory under `work_root`, returning the copy.
///
/// The copy gets a unique path on every run. The source path is part of the
/// artifact cache key, so a previous run's artifact can never turn one of this
/// run's edits into a cache hit.
///
/// # Errors
///
/// Returns an error when `source` has no probe, no file name, or cannot be
/// copied.
pub fn copy_source(source: &Path, work_root: &Path) -> Result<PathBuf> {
    let text =
        std::fs::read_to_string(source).with_context(|| format!("reading {}", source.display()))?;
    if !has_edit_probe(&text) {
        bail!(
            "{} has no `{PROBE_DECL} ...;` benchmark edit probe",
            source.display()
        );
    }
    let name = source
        .file_name()
        .ok_or_else(|| anyhow!("{} has no file name", source.display()))?;
    let dir = work_root.join("edit-loop").join(run_nonce());
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let copy = dir.join(name);
    std::fs::copy(source, &copy)
        .with_context(|| format!("copying {} to {}", source.display(), copy.display()))?;
    Ok(copy)
}

fn probe_span(text: &str) -> Result<Range<usize>> {
    let start = text
        .find(PROBE_DECL)
        .ok_or_else(|| anyhow!("source has no `{PROBE_DECL} ...;` benchmark edit probe"))?;
    let after = start + PROBE_DECL.len();
    let semicolon = text[after..]
        .find(';')
        .ok_or_else(|| anyhow!("the benchmark edit probe has no terminating `;`"))?;
    Ok(start..after + semicolon + 1)
}

fn run_nonce() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let counter = NEXT.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    format!("{}-{nanos}-{counter}", std::process::id())
}
