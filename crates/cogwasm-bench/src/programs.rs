//! Benchmark program manifest, selection, source preparation and the vendored
//! LLVM/CogWasm dependency rlibs.

use crate::generator::{LARGE_MODULES, LARGE_SEED, LARGE_STAGES, generate_large, generate_stress};
use crate::toolchain::Toolchain;
use crate::util;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The benchmark program list, matching `bench/programs.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct ProgramManifest {
    pub schema: u32,
    pub seeds: Vec<u64>,
    pub programs: Vec<ProgramDef>,
}

/// One entry in `bench/programs.json`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProgramDef {
    pub name: String,
    pub class: String,
    pub source: String,
    #[serde(default)]
    pub quick: bool,
    #[serde(default)]
    pub incremental: Option<bool>,
    #[serde(default)]
    pub externs: Vec<String>,
    #[serde(default)]
    pub generated: bool,
}

/// A program whose source has been prepared under the work tree.
#[derive(Debug, Clone)]
pub struct Program {
    pub def: ProgramDef,
    pub source: PathBuf,
    pub lines: usize,
    pub bytes: u64,
}

/// The suites the command line accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suite {
    Quick,
    Full,
    Large,
    Plugin,
}

impl Suite {
    /// Parse a suite name.
    ///
    /// # Errors
    ///
    /// Returns an error naming the accepted values.
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "quick" => Ok(Self::Quick),
            "full" => Ok(Self::Full),
            "large" => Ok(Self::Large),
            "plugin" => Ok(Self::Plugin),
            other => bail!("unknown suite {other:?} (quick, full, large or plugin)"),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Quick => "quick",
            Self::Full => "full",
            Self::Large => "large",
            Self::Plugin => "plugin",
        }
    }
}

/// Read the committed program manifest.
///
/// # Errors
///
/// Returns an error when the file cannot be read or parsed.
pub fn load_manifest(toolchain: &Toolchain) -> Result<ProgramManifest> {
    let path = toolchain.root.join("bench/programs.json");
    let text =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let manifest: ProgramManifest =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    if manifest.schema != 1 {
        bail!("unsupported program manifest schema {}", manifest.schema);
    }
    Ok(manifest)
}

fn large_program_def() -> ProgramDef {
    ProgramDef {
        name: "large".to_owned(),
        class: "large".to_owned(),
        source: String::new(),
        quick: false,
        incremental: Some(true),
        externs: Vec::new(),
        generated: true,
    }
}

/// Select and prepare programs for a suite (or an explicit `--program` list).
///
/// # Errors
///
/// Returns an error for an unknown program name, no selected programs, or a
/// source that cannot be prepared.
pub fn prepare_programs(
    toolchain: &Toolchain,
    suite: Suite,
    names: &[String],
) -> Result<Vec<Program>> {
    let manifest = load_manifest(toolchain)?;
    let selected: Vec<ProgramDef> = if names.is_empty() {
        match suite {
            Suite::Quick => manifest
                .programs
                .iter()
                .filter(|program| program.quick)
                .cloned()
                .collect(),
            Suite::Full => manifest.programs.clone(),
            Suite::Plugin => manifest
                .programs
                .iter()
                .filter(|program| program.class == "medium")
                .cloned()
                .collect(),
            Suite::Large => vec![large_program_def()],
        }
    } else {
        let mut wanted: BTreeSet<&str> = names.iter().map(String::as_str).collect();
        let include_large = wanted.remove("large") || wanted.remove("large/large");
        let mut selected: Vec<ProgramDef> = manifest
            .programs
            .iter()
            .filter(|program| {
                wanted.contains(program.name.as_str())
                    || wanted.contains(format!("{}/{}", program.class, program.name).as_str())
            })
            .cloned()
            .collect();
        let found: BTreeSet<String> = selected
            .iter()
            .flat_map(|program| {
                [
                    program.name.clone(),
                    format!("{}/{}", program.class, program.name),
                ]
            })
            .collect();
        let missing: Vec<&str> = wanted
            .iter()
            .copied()
            .filter(|name| !found.contains(*name))
            .collect();
        if !missing.is_empty() {
            bail!("unknown benchmark program(s): {}", missing.join(", "));
        }
        if include_large {
            selected.push(large_program_def());
        }
        selected
    };
    if selected.is_empty() {
        bail!("no benchmark programs selected");
    }
    let generated_root = toolchain.work_root.join("bench/generated");
    let source_root = toolchain.work_root.join("bench/sources");
    std::fs::create_dir_all(&generated_root)?;
    std::fs::create_dir_all(&source_root)?;
    let prelude = util::absolute(&toolchain.root.join("bench/programs/prelude.rs"))?;
    let mut programs = Vec::with_capacity(selected.len());
    for def in selected {
        let program = if def.name == "large" || suite == Suite::Large && names.is_empty() {
            let text = generate_large(
                &prelude.display().to_string(),
                LARGE_MODULES,
                LARGE_STAGES,
                LARGE_SEED,
            );
            let path = generated_root.join("large.rs");
            std::fs::write(&path, &text).with_context(|| format!("writing {}", path.display()))?;
            Program {
                lines: text.lines().count(),
                bytes: u64::try_from(text.len()).unwrap_or(u64::MAX),
                source: path,
                def,
            }
        } else if def.generated && def.name == "generated_stress" {
            let mut text = generate_stress(&prelude.display().to_string());
            append_probe(&mut text);
            let path = generated_root.join("generated_stress.rs");
            std::fs::write(&path, &text).with_context(|| format!("writing {}", path.display()))?;
            Program {
                lines: text.lines().count(),
                bytes: u64::try_from(text.len()).unwrap_or(u64::MAX),
                source: path,
                def,
            }
        } else {
            prepare_source(toolchain, &def, &source_root, &prelude)?
        };
        programs.push(program);
    }
    Ok(programs)
}

/// Add the benchmark edit probe to a prepared source when it does not have
/// one. The two spellings used by the edit loop are both zero.
fn append_probe(text: &mut String) {
    if text.contains("BENCH_EDIT_TWEAK") {
        return;
    }
    text.push_str(
        "\n// Added by the benchmark harness: a one-line edit whose two spellings are both\n\
         // zero, so an incremental recompile never changes the result.\n\
         const BENCH_EDIT_TWEAK: u64 = 0u64;\n\
         #[unsafe(no_mangle)]\n\
         pub extern \"C\" fn bench_edit_probe() -> u64 { BENCH_EDIT_TWEAK }\n",
    );
}

fn prepare_source(
    toolchain: &Toolchain,
    def: &ProgramDef,
    source_root: &Path,
    prelude: &Path,
) -> Result<Program> {
    let original = toolchain.root.join(&def.source);
    let text = std::fs::read_to_string(&original)
        .with_context(|| format!("reading benchmark source {}", original.display()))?;
    let mut prepared = String::with_capacity(text.len() + 256);
    let prelude_include = format!("include!(\"{}\");", prelude.display());
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line.contains("include!(\"") && line.contains("prelude.rs") {
            prepared.push_str(&prelude_include);
            replaced = true;
        } else {
            prepared.push_str(line);
        }
        prepared.push('\n');
    }
    append_probe(&mut prepared);
    let path = source_root.join(format!("{}.rs", def.name));
    std::fs::write(&path, &prepared)
        .with_context(|| format!("writing prepared source {}", path.display()))?;
    Ok(Program {
        lines: prepared.lines().count(),
        bytes: u64::try_from(prepared.len()).unwrap_or(u64::MAX),
        source: path,
        def: def.clone(),
    })
}

/// Where the vendored dependency rlibs for each pipeline live.
#[derive(Debug, Clone)]
pub struct DependencySet {
    pub clif: PathBuf,
    pub llvm: PathBuf,
}

/// Locate (and, if necessary, build) the vendored dependencies.
///
/// # Errors
///
/// Returns an error when a dependency rlib is missing and cannot be built.
pub fn ensure_dependencies(toolchain: &Toolchain, programs: &[Program]) -> Result<DependencySet> {
    let mut required = BTreeSet::new();
    for program in programs {
        for krate in &program.def.externs {
            required.insert(krate.as_str());
        }
    }
    let deps = DependencySet {
        clif: toolchain.deps_root.join("clif"),
        llvm: toolchain.deps_root.join("llvm"),
    };
    for (pipeline, root) in [("clif", &deps.clif), ("llvm", &deps.llvm)] {
        let missing: Vec<&str> = required
            .iter()
            .copied()
            .filter(|krate| find_rlib(root, krate).is_none())
            .collect();
        if missing.is_empty() {
            continue;
        }
        build_dependencies(toolchain, root, pipeline, &missing)?;
    }
    Ok(deps)
}

fn build_dependencies(
    toolchain: &Toolchain,
    target: &Path,
    pipeline: &str,
    missing: &[&str],
) -> Result<()> {
    let manifest = toolchain.root.join("bench/vendor/Cargo.toml");
    if !manifest.is_file() {
        bail!(
            "missing vendored dependency manifest {}; pass --deps-root with prebuilt rlibs",
            manifest.display()
        );
    }
    eprintln!(
        "[cogwasm-bench] building vendored dependencies for {pipeline} ({})",
        missing.join(", ")
    );
    std::fs::create_dir_all(target).with_context(|| format!("creating {}", target.display()))?;
    let mut command = Command::new(&toolchain.cargo);
    command
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("-p")
        .arg("libm")
        .arg("-p")
        .arg("miniz_oxide")
        .env("CARGO_TARGET_DIR", target)
        .env("RUSTUP_HOME", &toolchain.rustup_home)
        .env("CARGO_HOME", &toolchain.cargo_home);
    if pipeline == "clif" {
        let wrapper = toolchain
            .root
            .join("compiler/scripts/rustc-wasm-wrapper.sh");
        let host = toolchain
            .install_dir
            .join("target/cogwasm-host/release/cogwasm-host");
        command
            .env("RUSTC", &wrapper)
            .env("COGWASM_HOST_TRIPLE", toolchain.host_triple()?)
            .env("COGWASM_SYSROOT", &toolchain.sysroot)
            .env("COGWASM_RUSTC_WASM", &toolchain.rustc_wasm)
            .env("COGWASM_HOST", host)
            .env("COGWASM_WILD", &toolchain.wild)
            .env(
                "COGWASM_STAGE1",
                toolchain
                    .install_dir
                    .join("rust/build/host/stage1/bin/rustc"),
            )
            .env("COGWASM_CACHE", &toolchain.cache_dir);
        if let Some(stable) = toolchain.stable_toolchain() {
            command.env("RUSTUP_TOOLCHAIN", stable);
        }
    } else {
        command.env_remove("RUSTC");
        if let Some(nightly) = toolchain.rustup_toolchain() {
            command.env("RUSTUP_TOOLCHAIN", nightly);
        }
    }
    let output = command
        .output()
        .with_context(|| format!("running {}", toolchain.cargo.display()))?;
    if !output.status.success() {
        bail!(
            "building vendored {pipeline} dependencies failed ({}):\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

/// The `--extern name=path` arguments for one program on one pipeline.
///
/// # Errors
///
/// Returns an error when an rlib cannot be found.
pub fn externs_for(
    program: &ProgramDef,
    deps: &DependencySet,
    pipeline: &str,
) -> Result<Vec<String>> {
    let root = if pipeline == "clif" {
        &deps.clif
    } else {
        &deps.llvm
    };
    let mut externs = Vec::new();
    for krate in &program.externs {
        let (rlib, rmeta) = find_artifact(root, krate).with_context(|| {
            format!(
                "finding {krate} rlib under {} for {}",
                root.display(),
                program.name
            )
        })?;
        externs.push(format!("{krate}={}", rlib.display()));
        if let Some(rmeta) = rmeta {
            externs.push(format!("{krate}={}", rmeta.display()));
        }
    }
    Ok(externs)
}

/// `-L dependency=` directories covering every rlib under a dependency root.
#[must_use]
pub fn search_paths(root: &Path) -> Vec<String> {
    let mut directories = BTreeSet::new();
    collect_rlib_directories(root, &mut directories);
    directories
        .into_iter()
        .map(|directory| directory.display().to_string())
        .collect()
}

fn collect_rlib_directories(root: &Path, directories: &mut BTreeSet<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rlib_directories(&path, directories);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "rlib")
            && let Some(parent) = path.parent()
        {
            directories.insert(parent.to_path_buf());
        }
    }
}

/// Find the newest `lib<crate>.rlib` or `lib<crate>-<hash>.rlib` under `root`.
#[must_use]
pub fn find_rlib(root: &Path, krate: &str) -> Option<PathBuf> {
    find_library(root, krate, "rlib")
}

fn find_artifact(root: &Path, krate: &str) -> Option<(PathBuf, Option<PathBuf>)> {
    let rlib = find_library(root, krate, "rlib")?;
    let rmeta = find_library(root, krate, "rmeta");
    Some((rlib, rmeta))
}

fn find_library(root: &Path, krate: &str, extension: &str) -> Option<PathBuf> {
    let exact = format!("lib{krate}.{extension}");
    let prefix = format!("lib{krate}-");
    let suffix = format!(".{extension}");
    let mut matches = Vec::new();
    collect_libraries(root, &exact, &prefix, &suffix, &mut matches);
    matches.sort_by(|left, right| {
        let left_exact = left.file_name().is_some_and(|name| name == exact.as_str());
        let right_exact = right.file_name().is_some_and(|name| name == exact.as_str());
        right_exact
            .cmp(&left_exact)
            .then_with(|| modified(right).cmp(&modified(left)))
            .then_with(|| right.cmp(left))
    });
    matches.into_iter().next()
}

fn collect_libraries(
    root: &Path,
    exact: &str,
    prefix: &str,
    suffix: &str,
    matches: &mut Vec<PathBuf>,
) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_libraries(&path, exact, prefix, suffix, matches);
        } else if let Some(name) = path.file_name().and_then(std::ffi::OsStr::to_str)
            && (name == exact || name.starts_with(prefix) && name.ends_with(suffix))
        {
            matches.push(path);
        }
    }
}

fn modified(path: &Path) -> std::time::SystemTime {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_exact_and_hashed_rlibs_and_prefers_exact() {
        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir_all(dir.path().join("deps")).expect("deps");
        std::fs::write(dir.path().join("deps/liblibm.rlib"), b"x").expect("exact");
        std::fs::write(dir.path().join("deps/liblibm-abc.rlib"), b"y").expect("hashed");
        assert_eq!(
            find_rlib(dir.path(), "libm"),
            Some(dir.path().join("deps/liblibm.rlib"))
        );
        assert_eq!(find_rlib(dir.path(), "missing"), None);
    }

    #[test]
    fn search_paths_cover_rlib_directories() {
        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir_all(dir.path().join("a/b")).expect("a/b");
        std::fs::write(dir.path().join("a/b/libx.rlib"), b"x").expect("rlib");
        let paths = search_paths(dir.path());
        assert!(paths.iter().any(|path| path.ends_with("a/b")));
    }
}
