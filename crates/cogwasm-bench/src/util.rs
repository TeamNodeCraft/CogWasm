//! Small process and filesystem helpers shared by the benchmark commands.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Lowercase hex SHA-256 of a byte slice.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
    }
    out
}

/// Total size in bytes of every regular file under `root`.
///
/// A missing directory is zero bytes, so optional toolchain assets do not turn
/// a footprint measurement into an error.
pub fn directory_bytes(root: &Path) -> Result<u64> {
    if !root.exists() {
        return Ok(0);
    }
    let mut total = 0u64;
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .with_context(|| format!("reading {}", directory.display()))?
        {
            let entry =
                entry.with_context(|| format!("reading an entry of {}", directory.display()))?;
            let file_type = entry
                .file_type()
                .with_context(|| format!("reading the type of {}", entry.path().display()))?;
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                total = total.saturating_add(
                    entry
                        .metadata()
                        .with_context(|| format!("reading the size of {}", entry.path().display()))?
                        .len(),
                );
            }
        }
    }
    Ok(total)
}

/// Files under `root` whose extension is one of `extensions`, and their total.
///
/// This is the "intermediate objects" metric: the object/metadata files rustc
/// leaves in an incremental directory, as opposed to the whole directory.
pub fn files_with_extensions(root: &Path, extensions: &[&str]) -> Result<(u64, u64)> {
    if !root.exists() {
        return Ok((0, 0));
    }
    let mut count = 0u64;
    let mut total = 0u64;
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .with_context(|| format!("reading {}", directory.display()))?
        {
            let entry =
                entry.with_context(|| format!("reading an entry of {}", directory.display()))?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .with_context(|| format!("reading the type of {}", path.display()))?;
            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file()
                && path
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|extension| extensions.contains(&extension))
            {
                count += 1;
                total = total.saturating_add(
                    entry
                        .metadata()
                        .with_context(|| format!("reading the size of {}", path.display()))?
                        .len(),
                );
            }
        }
    }
    Ok((count, total))
}

/// Run a command and return its trimmed stdout, mapping a failure to a message
/// that includes both streams.
pub fn command_output(mut command: Command) -> Result<String> {
    let description = format!("{command:?}");
    let output = command
        .output()
        .with_context(|| format!("running {description}"))?;
    if !output.status.success() {
        anyhow::bail!(
            "command failed ({}): {description}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// `git rev-parse HEAD` and the subject at `root`, or `unknown`.
#[must_use]
pub fn git_identity(root: &Path) -> (String, String) {
    let sha = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map_or_else(
            || "unknown".to_owned(),
            |output| {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if value.is_empty() {
                    "unknown".to_owned()
                } else {
                    value
                }
            },
        );
    let subject = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["log", "-1", "--format=%s"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map_or_else(
            || "unknown".to_owned(),
            |output| String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        );
    (sha, subject)
}

/// The current process's peak resident set size in bytes, when the platform
/// reports it.
#[must_use]
pub fn peak_rss_bytes() -> Option<u64> {
    #[cfg(unix)]
    {
        // SAFETY: `getrusage` only writes into the supplied `rusage`; it does
        // not retain the pointer and `RUSAGE_SELF` needs no other state.
        let mut usage = unsafe { std::mem::zeroed::<libc::rusage>() };
        // SAFETY: `usage` is a valid, writable `rusage` for the duration of
        // the call.
        let status = unsafe { libc::getrusage(libc::RUSAGE_SELF, &raw mut usage) };
        if status != 0 {
            return None;
        }
        #[expect(
            clippy::cast_sign_loss,
            reason = "ru_maxrss is a non-negative kernel memory statistic"
        )]
        let value = usage.ru_maxrss as u64;
        // macOS reports bytes, Linux and the BSDs report KiB.
        Some(if cfg!(target_os = "macos") {
            value
        } else {
            value.saturating_mul(1024)
        })
    }
    #[cfg(not(unix))]
    {
        None
    }
}

/// The one-minute load average, when the platform reports it.
#[must_use]
pub fn load_average() -> Option<f64> {
    #[cfg(unix)]
    {
        let mut values = [0.0f64; 3];
        // SAFETY: `values` holds three writable doubles, which is the count
        // `getloadavg` is told to fill.
        let count = unsafe { libc::getloadavg(values.as_mut_ptr(), 3) };
        (count > 0).then_some(values[0])
    }
    #[cfg(not(unix))]
    {
        None
    }
}

/// Number of available CPU cores as reported by the standard library.
#[must_use]
pub fn cores() -> usize {
    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
}

/// A UTC timestamp in the output-directory form `YYYYMMDDTHHMMSSZ`.
#[must_use]
pub fn utc_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    format_utc(seconds)
}

/// Format Unix seconds as `YYYYMMDDTHHMMSSZ` (UTC).
#[must_use]
pub fn format_utc(seconds: u64) -> String {
    let days = i64::try_from(seconds / 86_400).unwrap_or(i64::MAX);
    let remainder = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    let hour = remainder / 3_600;
    let minute = (remainder % 3_600) / 60;
    let second = remainder % 60;
    format!("{year:04}{month:02}{day:02}T{hour:02}{minute:02}{second:02}Z")
}

/// Convert days since 1970-01-01 to a proleptic Gregorian date.
///
/// This is the civil-from-days algorithm from Howard Hinnant's date library.
/// It is here so the tool does not need a date/time dependency just to name an
/// output directory.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = u32::try_from(doy - (153 * mp + 2) / 5 + 1).unwrap_or(1);
    let month = u32::try_from(if mp < 10 { mp + 3 } else { mp - 9 }).unwrap_or(1);
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// The absolute form of `path` with a message naming it.
///
/// `std::path::absolute` keeps `..` components; the benchmark makes paths
/// absolute so the guest sees them, and a work path like
/// `crates/cogwasm-bench/../../toolchain` should be reported in its clean
/// form. The lexical cleanup is safe because the result is already absolute.
pub fn absolute(path: &Path) -> Result<PathBuf> {
    let absolute =
        std::path::absolute(path).with_context(|| format!("making {} absolute", path.display()))?;
    Ok(normalize(&absolute))
}

fn normalize(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !output.pop() {
                    output.push(component);
                }
            }
            other => output.push(other),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_format_matches_known_epochs() {
        assert_eq!(format_utc(0), "19700101T000000Z");
        assert_eq!(format_utc(1_000_000_000), "20010909T014640Z");
        assert_eq!(format_utc(1_700_000_000), "20231114T221320Z");
    }

    #[test]
    fn absolute_removes_dot_and_parent_components() {
        let base = std::path::absolute(".").expect("cwd");
        let path = base.join("crates/cogwasm-bench/../../toolchain/work");
        let cleaned = absolute(&path).expect("absolute");
        assert!(!cleaned.to_string_lossy().contains(".."));
        assert!(cleaned.ends_with("toolchain/work"));
    }

    #[test]
    fn directory_bytes_sums_regular_files() {
        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir_all(dir.path().join("nested")).expect("nested");
        std::fs::write(dir.path().join("a"), b"abc").expect("a");
        std::fs::write(dir.path().join("nested/b"), b"de").expect("b");
        assert_eq!(directory_bytes(dir.path()).expect("size"), 5);
        assert_eq!(
            directory_bytes(&dir.path().join("missing")).expect("missing"),
            0
        );
    }
}
