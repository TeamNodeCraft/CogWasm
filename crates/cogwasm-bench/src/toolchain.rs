//! Toolchain paths, overrides and the toolchain-footprint measurement.

use crate::size::{gzip_len, strip_custom_sections};
use crate::util;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Paths the benchmark reads. Every field has a repository-relative default and
/// every field can be overridden from the command line, matching the
/// supervisor's option style.
#[derive(Debug, Clone)]
pub struct Toolchain {
    pub root: PathBuf,
    pub install_dir: PathBuf,
    pub rustc_wasm: PathBuf,
    pub precompiled: Option<PathBuf>,
    pub sysroot: PathBuf,
    pub wild: PathBuf,
    pub native_rustc: PathBuf,
    pub cache_dir: PathBuf,
    pub work_root: PathBuf,
    pub repl_assets: PathBuf,
    pub deps_root: PathBuf,
    pub cargo: PathBuf,
    pub rustup_home: PathBuf,
    pub cargo_home: PathBuf,
}

/// Command-line overrides for [`Toolchain`]. `None` means "use the default".
#[derive(Debug, Clone, Default)]
pub struct ToolchainOverrides {
    pub root: Option<PathBuf>,
    pub toolchain: Option<PathBuf>,
    pub rustc_wasm: Option<PathBuf>,
    pub precompiled: Option<PathBuf>,
    pub sysroot: Option<PathBuf>,
    pub wild: Option<PathBuf>,
    pub native_rustc: Option<PathBuf>,
    pub cache_dir: Option<PathBuf>,
    pub work_root: Option<PathBuf>,
    pub deps_root: Option<PathBuf>,
}

impl Toolchain {
    /// Resolve paths from the overrides and the repository defaults.
    ///
    /// # Errors
    ///
    /// Returns an error only if the repository root cannot be made absolute.
    pub fn resolve(overrides: &ToolchainOverrides) -> Result<Self> {
        let root = match &overrides.root {
            Some(root) => util::absolute(root)?,
            None => util::absolute(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .as_path(),
            )?,
        };
        let toolchain = overrides
            .toolchain
            .clone()
            .unwrap_or_else(|| root.join("toolchain"));
        let toolchain = util::absolute(&toolchain)?;
        let rustc_wasm = overrides
            .rustc_wasm
            .clone()
            .unwrap_or_else(|| toolchain.join("artifacts/rustc.wasm"));
        let precompiled = match &overrides.precompiled {
            Some(path) => Some(path.clone()),
            None => default_precompiled(&toolchain),
        };
        let sysroot = overrides
            .sysroot
            .clone()
            .unwrap_or_else(|| toolchain.join("sysroot"));
        let wild = overrides
            .wild
            .clone()
            .unwrap_or_else(|| toolchain.join("target/wild/release/wild"));
        let native_rustc = overrides
            .native_rustc
            .clone()
            .unwrap_or_else(|| default_native_rustc(&toolchain));
        let cache_dir = overrides
            .cache_dir
            .clone()
            .unwrap_or_else(|| toolchain.join("cache"));
        let work_root = overrides
            .work_root
            .clone()
            .unwrap_or_else(|| toolchain.join("work"));
        let deps_root = overrides
            .deps_root
            .clone()
            .unwrap_or_else(|| root.join("bench/work/deps"));
        let cargo = default_cargo(&toolchain);
        Ok(Self {
            root,
            install_dir: toolchain.clone(),
            rustc_wasm,
            precompiled,
            sysroot,
            wild,
            native_rustc,
            cache_dir,
            work_root,
            repl_assets: toolchain.join("work/repl-assets"),
            deps_root,
            cargo,
            rustup_home: toolchain.join("rustup"),
            cargo_home: toolchain.join("cargo"),
        })
    }

    /// Fail before a long run if a required compiler artifact is missing.
    ///
    /// # Errors
    ///
    /// Names the missing path and what it is used for.
    pub fn validate(&self) -> Result<()> {
        for (path, purpose) in [
            (&self.rustc_wasm, "the rustc.wasm compiler"),
            (&self.sysroot, "the wasm32-unknown-unknown sysroot"),
            (&self.wild, "the Wild linker"),
        ] {
            if !path.exists() {
                bail!(
                    "missing {purpose} at {}; build it with ci/build-and-test.sh or pass an override",
                    path.display()
                );
            }
        }
        if !self.native_rustc.exists() {
            bail!(
                "missing the native LLVM rustc at {}; pass --native-rustc or install the nightly toolchain",
                self.native_rustc.display()
            );
        }
        Ok(())
    }

    /// The native rustc's `--version` line.
    ///
    /// # Errors
    ///
    /// Returns an error when the compiler cannot be run.
    pub fn native_rustc_version(&self) -> Result<String> {
        let mut command = std::process::Command::new(&self.native_rustc);
        command.arg("--version");
        util::command_output(command)
    }

    /// The native rustc's host triple.
    ///
    /// # Errors
    ///
    /// Returns an error when the compiler cannot be run or prints no host.
    pub fn host_triple(&self) -> Result<String> {
        let mut command = std::process::Command::new(&self.native_rustc);
        command.arg("-vV");
        let verbose = util::command_output(command)?;
        verbose
            .lines()
            .find_map(|line| line.strip_prefix("host: "))
            .map(|host| host.trim().to_owned())
            .with_context(|| format!("{} -vV printed no host line", self.native_rustc.display()))
    }

    /// The active rustup toolchain name, if the isolated rustup has one.
    #[must_use]
    pub fn rustup_toolchain(&self) -> Option<String> {
        let root = self.rustup_home.join("toolchains");
        let entries = std::fs::read_dir(root).ok()?;
        entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .find(|name| name.starts_with("nightly-"))
    }

    /// The stable toolchain name used to build the vendored dependencies.
    #[must_use]
    pub fn stable_toolchain(&self) -> Option<String> {
        let root = self.rustup_home.join("toolchains");
        let entries = std::fs::read_dir(root).ok()?;
        entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .find(|name| name.starts_with("stable-"))
    }
}

fn default_precompiled(toolchain: &Path) -> Option<PathBuf> {
    [
        toolchain.join("work/precompiled/rustc.cwasm"),
        toolchain.join("work/bench/rustc.cwasm"),
    ]
    .into_iter()
    .find(|path| path.is_file())
}

fn default_native_rustc(toolchain: &Path) -> PathBuf {
    let root = toolchain.join("rustup/toolchains");
    let mut candidates: Vec<PathBuf> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    candidates.sort();
    for wanted in ["nightly-", "stable-", "1."] {
        for candidate in candidates.iter().rev() {
            let name = candidate
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");
            if name.starts_with(wanted) {
                let rustc = candidate.join("bin/rustc");
                if rustc.is_file() {
                    return rustc;
                }
            }
        }
    }
    PathBuf::from("rustc")
}

fn default_cargo(toolchain: &Path) -> PathBuf {
    let cargo = toolchain.join("cargo/bin/cargo");
    if cargo.is_file() {
        cargo
    } else {
        PathBuf::from("cargo")
    }
}

/// One file or directory in the toolchain footprint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetSize {
    pub bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<u64>,
}

impl AssetSize {
    fn file(path: &Path) -> Result<Self> {
        Ok(Self {
            bytes: std::fs::metadata(path)
                .with_context(|| format!("reading the size of {}", path.display()))?
                .len(),
            files: None,
        })
    }

    fn directory(path: &Path) -> Result<Self> {
        Ok(Self {
            bytes: util::directory_bytes(path)?,
            files: Some(count_files(path)?),
        })
    }
}

fn count_files(root: &Path) -> Result<u64> {
    if !root.exists() {
        return Ok(0);
    }
    let mut count = 0u64;
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
                count += 1;
            }
        }
    }
    Ok(count)
}

/// The build-size section's toolchain footprint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolchainFootprint {
    pub rustc_wasm_bytes: u64,
    pub rustc_wasm_sha256: String,
    pub rustc_wasm_stripped_bytes: u64,
    pub rustc_wasm_gzip_bytes: u64,
    pub rustc_wasm_stripped_gzip_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwasm_bytes: Option<u64>,
    pub sysroot: AssetSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repl_assets: Option<AssetSize>,
}

/// Measure the toolchain footprint. This does not compile anything.
///
/// # Errors
///
/// Returns an error when a required file cannot be read or is not a wasm
/// module.
pub fn measure_footprint(toolchain: &Toolchain) -> Result<ToolchainFootprint> {
    let rustc = std::fs::read(&toolchain.rustc_wasm)
        .with_context(|| format!("reading {}", toolchain.rustc_wasm.display()))?;
    let stripped = strip_custom_sections(&rustc).with_context(|| {
        format!(
            "stripping custom sections from {}",
            toolchain.rustc_wasm.display()
        )
    })?;
    let cwasm_bytes = toolchain
        .precompiled
        .as_deref()
        .filter(|path| path.is_file())
        .map(AssetSize::file)
        .transpose()?
        .map(|size| size.bytes);
    let repl_assets = toolchain
        .repl_assets
        .is_dir()
        .then(|| AssetSize::directory(&toolchain.repl_assets))
        .transpose()?;
    Ok(ToolchainFootprint {
        rustc_wasm_bytes: u64::try_from(rustc.len()).unwrap_or(u64::MAX),
        rustc_wasm_sha256: util::sha256_hex(&rustc),
        rustc_wasm_stripped_bytes: u64::try_from(stripped.len()).unwrap_or(u64::MAX),
        rustc_wasm_gzip_bytes: gzip_len(&rustc)?,
        rustc_wasm_stripped_gzip_bytes: gzip_len(&stripped)?,
        cwasm_bytes,
        sysroot: AssetSize::directory(&toolchain.sysroot)?,
        repl_assets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_directory_counts_files_and_bytes() {
        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir_all(dir.path().join("nested")).expect("nested");
        std::fs::write(dir.path().join("a"), b"1234").expect("a");
        std::fs::write(dir.path().join("nested/b"), b"12").expect("b");
        let asset = AssetSize::directory(dir.path()).expect("directory");
        assert_eq!(asset.bytes, 6);
        assert_eq!(asset.files, Some(2));
    }
}
