//! Load rustc.wasm either from its `.wasm` source or from a precompiled
//! Wasmtime `.cwasm` artifact.
//!
//! A `.cwasm` contains native machine code. Wasmtime's `deserialize` family is
//! therefore `unsafe`: it trusts the bytes. This module only calls it after
//! checking the artifact against a sidecar metadata file that records the
//! Wasmtime version, the engine's precompile compatibility hash, the host
//! target, and the size and SHA-256 of both the `.wasm` and the `.cwasm`.
//! Sizes are always checked; the digests are checked only when the caller
//! opts in (they are an install-time integrity check, not a per-load one).
//! Any mismatch, or any deserialization error, falls back to compiling the
//! `.wasm`.

use crate::config::ConfigSummary;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::hash::{Hash as _, Hasher};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;
use wasmtime::{Engine, Module};

/// Version of the sidecar metadata layout (not of Wasmtime).
pub const METADATA_FORMAT_VERSION: u32 = 1;

/// Where a loaded module came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleSource {
    /// Compiled from the `.wasm` file in this process.
    Wasm,
    /// Deserialized from a precompiled `.cwasm` file.
    Precompiled,
}

impl ModuleSource {
    /// Stable name for logs and JSON.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wasm => "wasm",
            Self::Precompiled => "precompiled",
        }
    }
}

/// A loaded rustc.wasm module together with how it was obtained.
pub struct LoadedModule {
    /// The module, ready to instantiate.
    pub module: Module,
    /// Whether the module came from the `.wasm` or the `.cwasm`.
    pub source: ModuleSource,
    /// Wall time spent loading (read, verify and compile/deserialize).
    pub load_ms: f64,
    /// Why the precompiled artifact was not used, when it was supplied.
    pub fallback_reason: Option<String>,
}

/// Host information that a `.cwasm` is tied to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetInfo {
    /// `std::env::consts::ARCH`.
    pub arch: String,
    /// `std::env::consts::OS`.
    pub os: String,
    /// `std::env::consts::FAMILY`.
    pub family: String,
    /// Target environment (`gnu`, `musl`, `msvc`, ...), from `cfg!(target_env)`.
    pub env: String,
    /// Conventional vendor component of the Rust target triple.
    pub vendor: String,
    /// Best-effort Rust target triple derived from the constants above.
    pub triple: String,
}

impl TargetInfo {
    /// The host this process was built for.
    #[must_use]
    pub fn current() -> Self {
        let arch = std::env::consts::ARCH.to_owned();
        let os = std::env::consts::OS.to_owned();
        let family = std::env::consts::FAMILY.to_owned();
        let env = if cfg!(target_env = "gnu") {
            "gnu"
        } else if cfg!(target_env = "musl") {
            "musl"
        } else if cfg!(target_env = "msvc") {
            "msvc"
        } else if cfg!(target_env = "sgx") {
            "sgx"
        } else {
            ""
        }
        .to_owned();
        let vendor = match os.as_str() {
            "macos" => "apple",
            "windows" => "pc",
            _ => "unknown",
        };
        let triple = match (os.as_str(), env.as_str()) {
            ("macos", _) => format!("{arch}-apple-darwin"),
            ("ios", _) => format!("{arch}-apple-ios"),
            ("linux", "musl") => format!("{arch}-unknown-linux-musl"),
            ("linux", _) => format!("{arch}-unknown-linux-gnu"),
            ("windows", _) => format!("{arch}-pc-windows-msvc"),
            ("freebsd", _) => format!("{arch}-unknown-freebsd"),
            _ => format!("{arch}-{vendor}-{os}"),
        };
        Self {
            arch,
            os,
            family,
            env,
            vendor: vendor.to_owned(),
            triple,
        }
    }
}

/// SHA-256 and size of one file in the bundle.
///
/// The digest is an install-time/download-integrity check: it is only computed
/// on every load when `--verify-precompiled` is requested. The size is cheap
/// and is always checked, so a truncated file still falls back.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileInfo {
    /// Lowercase hex SHA-256.
    pub sha256: String,
    /// Size in bytes.
    pub bytes: u64,
}

impl FileInfo {
    /// Hash a file without loading it all into memory.
    pub fn from_path(path: &Path) -> Result<Self> {
        Ok(Self {
            sha256: sha256_file(path)?,
            bytes: file_size(path)?,
        })
    }
}

/// Size of a file, with a message naming it.
pub fn file_size(path: &Path) -> Result<u64> {
    Ok(std::fs::metadata(path)
        .with_context(|| format!("reading size of {}", path.display()))?
        .len())
}

/// Sidecar metadata written next to a `.cwasm`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactMetadata {
    /// Layout version; checked before any other field.
    pub format_version: u32,
    /// Program that wrote the file, for diagnosis.
    pub generator: String,
    /// Wasmtime version the artifact was compiled with.
    pub wasmtime_version: String,
    /// `Engine::precompile_compatibility_hash()` as lowercase hex.
    pub compatibility_hash: String,
    /// Host the artifact is for.
    pub target: TargetInfo,
    /// Configuration summary the artifact was compiled with.
    pub config: ConfigSummary,
    /// The `.wasm` this artifact was compiled from.
    pub wasm: FileInfo,
    /// The `.cwasm` itself.
    pub cwasm: FileInfo,
}

impl ArtifactMetadata {
    /// Build the metadata record for an existing `.wasm`/`.cwasm` pair.
    pub fn new(
        engine: &Engine,
        config: &ConfigSummary,
        wasm_path: &Path,
        cwasm_path: &Path,
    ) -> Result<Self> {
        Ok(Self {
            format_version: METADATA_FORMAT_VERSION,
            generator: format!("stage-c-precompile {}", env!("CARGO_PKG_VERSION")),
            wasmtime_version: wasmtime_version(),
            compatibility_hash: compatibility_hash_hex(engine),
            target: TargetInfo::current(),
            config: config.clone(),
            wasm: FileInfo::from_path(wasm_path)?,
            cwasm: FileInfo::from_path(cwasm_path)?,
        })
    }

    /// Read and parse a sidecar metadata file.
    pub fn read(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("reading precompiled metadata {}", path.display()))?;
        serde_json::from_slice(&bytes)
            .with_context(|| format!("parsing precompiled metadata {}", path.display()))
    }

    /// Serialize to pretty JSON with a trailing newline.
    pub fn to_json(&self) -> Result<String> {
        let mut text = serde_json::to_string_pretty(self).context("serializing metadata")?;
        text.push('\n');
        Ok(text)
    }

    /// Check this record against the current engine and the two files.
    ///
    /// The version, compatibility hash, target and both file sizes are always
    /// checked. Full SHA-256 digests are only checked when `verify_digests` is
    /// set: on the hot path a 282 MiB `.cwasm` plus a 137 MiB `.wasm` digest
    /// would cost more than the deserialization it protects. The digests are
    /// an install-time/download-integrity check (the release bundle ships
    /// `SHA256SUMS`), while Wasmtime itself rejects an artifact built for a
    /// different version, configuration or target.
    ///
    /// Every failure is a hard requirement to fall back to compiling the
    /// `.wasm`.
    pub fn verify(
        &self,
        wasm_path: &Path,
        cwasm_path: &Path,
        engine: &Engine,
        verify_digests: bool,
    ) -> Result<()> {
        if self.format_version != METADATA_FORMAT_VERSION {
            bail!(
                "metadata format version {} is not supported (expected {METADATA_FORMAT_VERSION})",
                self.format_version
            );
        }
        let current_version = wasmtime_version();
        if self.wasmtime_version != current_version {
            bail!(
                "precompiled with Wasmtime {}, host has Wasmtime {current_version}",
                self.wasmtime_version
            );
        }
        let current_hash = compatibility_hash_hex(engine);
        if self.compatibility_hash != current_hash {
            bail!(
                "precompile compatibility hash {} does not match this engine ({current_hash})",
                self.compatibility_hash
            );
        }
        let current_target = TargetInfo::current();
        if self.target != current_target {
            bail!(
                "precompiled for {} ({}), host is {} ({})",
                self.target.triple,
                self.target.os,
                current_target.triple,
                current_target.os
            );
        }

        let cwasm_bytes = file_size(cwasm_path)?;
        if cwasm_bytes != self.cwasm.bytes {
            bail!(
                "{} is {cwasm_bytes} bytes, metadata records {}",
                cwasm_path.display(),
                self.cwasm.bytes
            );
        }
        let wasm_bytes = file_size(wasm_path)?;
        if wasm_bytes != self.wasm.bytes {
            bail!(
                "{} is {wasm_bytes} bytes, metadata records {}",
                wasm_path.display(),
                self.wasm.bytes
            );
        }
        if !verify_digests {
            return Ok(());
        }

        let cwasm_sha256 = sha256_file(cwasm_path)?;
        if cwasm_sha256 != self.cwasm.sha256 {
            bail!(
                "{} SHA-256 {cwasm_sha256} does not match metadata {}",
                cwasm_path.display(),
                self.cwasm.sha256
            );
        }
        let wasm_sha256 = sha256_file(wasm_path)?;
        if wasm_sha256 != self.wasm.sha256 {
            bail!(
                "{} SHA-256 {wasm_sha256} does not match the .wasm the artifact was built from ({})",
                wasm_path.display(),
                self.wasm.sha256
            );
        }
        Ok(())
    }
}

/// The conventional sidecar path: `<cwasm>.json`.
#[must_use]
pub fn metadata_sidecar(cwasm: &Path) -> PathBuf {
    let mut name = cwasm.as_os_str().to_owned();
    name.push(".json");
    PathBuf::from(name)
}

/// The Wasmtime version this binary links.
#[must_use]
pub fn wasmtime_version() -> String {
    // `wasmtime` does not re-export its version, while the internal environ
    // crate exposes the same `CARGO_PKG_VERSION` as a public constant. The
    // dependency is pinned to the same version as `wasmtime`.
    wasmtime_environ::VERSION.to_owned()
}

/// A deterministic hex digest of `Engine::precompile_compatibility_hash()`.
///
/// The hash is only meaningful between programs built with the same Wasmtime
/// and Rust releases, but it is stable within a release, which is exactly what
/// a `.cwasm` bundle needs.
#[must_use]
pub fn compatibility_hash_hex(engine: &Engine) -> String {
    let mut hasher = ByteHasher::default();
    engine.precompile_compatibility_hash().hash(&mut hasher);
    let mut out = String::with_capacity(hasher.bytes.len() * 2);
    for byte in &hasher.bytes {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
    }
    out
}

/// A `Hasher` that keeps the exact byte stream written by `Hash::hash`.
#[derive(Default)]
struct ByteHasher {
    bytes: Vec<u8>,
}

impl Hasher for ByteHasher {
    fn finish(&self) -> u64 {
        // Not used: the hex digest is over `bytes`, not over a 64-bit result.
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
}

/// Load rustc.wasm, preferring a precompiled artifact when one is supplied.
///
/// With `precompiled == None` this is exactly the old behaviour: the `.wasm`
/// is read and compiled with `Module::new`. With a `.cwasm`, the sidecar
/// metadata is verified and the artifact is deserialized; any error falls back
/// to `Module::new` with a warning naming the reason, so a bad or stale
/// artifact can never make the compiler unusable.
pub fn load_rustc_module(
    engine: &Engine,
    wasm_path: &Path,
    precompiled: Option<&Path>,
    verify_digests: bool,
) -> Result<LoadedModule> {
    let started = Instant::now();
    let mut fallback_reason = None;
    if let Some(cwasm_path) = precompiled {
        match load_precompiled(engine, wasm_path, cwasm_path, verify_digests) {
            Ok(module) => {
                return Ok(LoadedModule {
                    module,
                    source: ModuleSource::Precompiled,
                    load_ms: started.elapsed().as_secs_f64() * 1e3,
                    fallback_reason: None,
                });
            }
            Err(error) => {
                let reason = format!("{error:#}");
                eprintln!(
                    "[stage-c host] warning: precompiled {} not usable: {reason}; \
                     falling back to compiling {}",
                    cwasm_path.display(),
                    wasm_path.display()
                );
                fallback_reason = Some(reason);
            }
        }
    }

    let bytes =
        std::fs::read(wasm_path).with_context(|| format!("reading {}", wasm_path.display()))?;
    let module = Module::new(engine, &bytes)
        .map_err(|error| anyhow!("compiling {}: {error}", wasm_path.display()))?;
    Ok(LoadedModule {
        module,
        source: ModuleSource::Wasm,
        load_ms: started.elapsed().as_secs_f64() * 1e3,
        fallback_reason,
    })
}

fn load_precompiled(
    engine: &Engine,
    wasm_path: &Path,
    cwasm_path: &Path,
    verify_digests: bool,
) -> Result<Module> {
    let sidecar = metadata_sidecar(cwasm_path);
    let metadata = ArtifactMetadata::read(&sidecar)?;
    metadata.verify(wasm_path, cwasm_path, engine, verify_digests)?;

    // SAFETY: `deserialize_file` trusts the bytes to be a Wasmtime-serialized
    // module: arbitrary data could make Wasmtime execute arbitrary native
    // code. This file is a trusted, read-only installed build artifact that
    // shares this host binary's trust boundary: it was produced by this
    // repository's release pipeline for this exact Wasmtime version,
    // configuration and target, and the sidecar was checked above (format,
    // Wasmtime version, precompile compatibility hash, host target and sizes;
    // optionally the full SHA-256). Wasmtime rejects an artifact written by a
    // different version/config/target itself. The file must stay unchanged
    // for the lifetime of the returned `Module` because Wasmtime mmaps it; the
    // bundle is installed read-only and the host never rewrites it. The
    // SHA-256 is a download/install-integrity check (the bundle ships
    // SHA256SUMS), not a per-load sandbox: a party able to replace both the
    // artifact and its metadata already controls the installed files.
    let module = unsafe { Module::deserialize_file(engine, cwasm_path) }
        .map_err(|error| anyhow!("deserializing {}: {error}", cwasm_path.display()))?;
    Ok(module)
}

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

/// Lowercase hex SHA-256 of a file, read in chunks.
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("opening {} to hash it", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1 << 20];
    loop {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("reading {} to hash it", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |duration| duration.as_nanos());
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "stage-c-precompiled-{name}-{}-{nanos}-{unique}",
                std::process::id()
            ));
            std::fs::create_dir_all(&path).expect("creating the test temp dir");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).expect("removing the test temp dir");
        }
    }

    fn fixture(dir: &TempDir) -> (PathBuf, PathBuf, Engine) {
        let wasm = dir.path().join("rustc.wasm");
        let cwasm = dir.path().join("rustc.cwasm");
        std::fs::write(&wasm, b"\0asm fake rustc module").expect("writing fake wasm");
        std::fs::write(&cwasm, b"fake precompiled module").expect("writing fake cwasm");
        let engine = crate::config::engine(None).expect("creating a Wasmtime engine");
        (wasm, cwasm, engine)
    }

    #[test]
    fn sha256_matches_known_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn digest_check_rejects_a_same_size_corrupt_cwasm() {
        let dir = TempDir::new("corrupt");
        let (wasm, cwasm, engine) = fixture(&dir);
        let metadata = ArtifactMetadata::new(
            &engine,
            &crate::config::ConfigSummary::shipped(),
            &wasm,
            &cwasm,
        )
        .expect("building metadata");
        metadata
            .verify(&wasm, &cwasm, &engine, true)
            .expect("the freshly written artifact verifies");

        // Same length as the original, so the default size check still passes:
        // this isolates the optional digest check.
        std::fs::write(&cwasm, b"fake precompiled modulE").expect("corrupting the fake cwasm");
        metadata
            .verify(&wasm, &cwasm, &engine, false)
            .expect("the hot path does not hash every artifact");
        let error = metadata
            .verify(&wasm, &cwasm, &engine, true)
            .expect_err("a corrupt cwasm must be rejected when digests are checked");
        assert!(
            format!("{error:#}").contains("does not match metadata"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn default_verification_rejects_a_truncated_cwasm() {
        let dir = TempDir::new("truncated");
        let (wasm, cwasm, engine) = fixture(&dir);
        let metadata = ArtifactMetadata::new(
            &engine,
            &crate::config::ConfigSummary::shipped(),
            &wasm,
            &cwasm,
        )
        .expect("building metadata");
        std::fs::write(&cwasm, b"short").expect("truncating the fake cwasm");
        let error = metadata
            .verify(&wasm, &cwasm, &engine, false)
            .expect_err("a truncated cwasm must be rejected without hashing it");
        assert!(
            format!("{error:#}").contains("bytes, metadata records"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn metadata_rejects_a_stale_wasm() {
        let dir = TempDir::new("stale-wasm");
        let (wasm, cwasm, engine) = fixture(&dir);
        let metadata = ArtifactMetadata::new(
            &engine,
            &crate::config::ConfigSummary::shipped(),
            &wasm,
            &cwasm,
        )
        .expect("building metadata");
        std::fs::write(&wasm, b"short").expect("replacing the wasm");
        let error = metadata
            .verify(&wasm, &cwasm, &engine, false)
            .expect_err("a different wasm must be rejected");
        assert!(
            format!("{error:#}").contains("bytes, metadata records"),
            "unexpected error: {error:#}"
        );

        // Same size, different bytes: only the digest check can notice.
        std::fs::write(&wasm, b"\0asm fake rustc modulE").expect("writing a same-size wasm");
        let error = metadata
            .verify(&wasm, &cwasm, &engine, true)
            .expect_err("a same-size but different wasm must be rejected by the digest");
        assert!(
            format!("{error:#}").contains("does not match the .wasm"),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn metadata_rejects_a_different_wasmtime_or_engine() {
        let dir = TempDir::new("environment");
        let (wasm, cwasm, engine) = fixture(&dir);
        let metadata = ArtifactMetadata::new(
            &engine,
            &crate::config::ConfigSummary::shipped(),
            &wasm,
            &cwasm,
        )
        .expect("building metadata");

        let mut wrong_version = metadata.clone();
        wrong_version.wasmtime_version = "0.0.0".to_owned();
        assert!(wrong_version.verify(&wasm, &cwasm, &engine, false).is_err());

        let mut wrong_hash = metadata.clone();
        wrong_hash.compatibility_hash = "00".to_owned();
        assert!(wrong_hash.verify(&wasm, &cwasm, &engine, false).is_err());

        let mut wrong_target = metadata;
        wrong_target.target.triple = "wasm32-wasip1".to_owned();
        assert!(wrong_target.verify(&wasm, &cwasm, &engine, false).is_err());
    }

    #[test]
    fn a_different_engine_config_has_a_different_hash() {
        let default = crate::config::engine(None).expect("default engine");
        let summary = crate::config::ConfigSummary {
            wasm_simd: false,
            wasm_relaxed_simd: false,
            ..crate::config::ConfigSummary::shipped()
        };
        let config = crate::config::engine_config_with(&summary, None).expect("no-simd config");
        let no_simd = Engine::new(&config).expect("no-simd engine");
        assert_ne!(
            compatibility_hash_hex(&default),
            compatibility_hash_hex(&no_simd),
            "disabling SIMD must change the precompile compatibility hash"
        );
    }

    #[test]
    fn a_bad_cwasm_falls_back_to_compiling_the_wasm() {
        let dir = TempDir::new("fallback");
        let wasm = dir.path().join("rustc.wasm");
        let cwasm = dir.path().join("rustc.cwasm");
        std::fs::write(&wasm, b"\0asm\x01\0\0\0").expect("writing an empty wasm module");
        std::fs::write(&cwasm, b"this is not a Wasmtime artifact").expect("writing a bad cwasm");
        let engine = crate::config::engine(None).expect("creating an engine");
        let metadata = ArtifactMetadata::new(
            &engine,
            &crate::config::ConfigSummary::shipped(),
            &wasm,
            &cwasm,
        )
        .expect("building metadata");
        std::fs::write(
            metadata_sidecar(&cwasm),
            metadata.to_json().expect("metadata json"),
        )
        .expect("writing the sidecar");

        let loaded = load_rustc_module(&engine, &wasm, Some(&cwasm), true)
            .expect("a bad artifact must fall back, not fail");
        assert_eq!(loaded.source, ModuleSource::Wasm);
        assert!(
            loaded.fallback_reason.is_some(),
            "the fallback must explain why the artifact was not used"
        );
    }

    #[test]
    fn a_missing_sidecar_falls_back_to_compiling_the_wasm() {
        let dir = TempDir::new("missing-sidecar");
        let wasm = dir.path().join("rustc.wasm");
        let cwasm = dir.path().join("rustc.cwasm");
        std::fs::write(&wasm, b"\0asm\x01\0\0\0").expect("writing an empty wasm module");
        std::fs::write(&cwasm, b"unused").expect("writing a cwasm without metadata");
        let engine = crate::config::engine(None).expect("creating an engine");

        let loaded = load_rustc_module(&engine, &wasm, Some(&cwasm), false)
            .expect("a missing sidecar must fall back, not fail");
        assert_eq!(loaded.source, ModuleSource::Wasm);
        assert!(loaded.fallback_reason.is_some());
    }

    #[test]
    fn cache_directory_does_not_change_the_compatibility_hash() {
        let dir = TempDir::new("cache-hash");
        let cache_dir = dir.path().join("cache");
        std::fs::create_dir_all(&cache_dir).expect("creating the cache dir");
        let plain = crate::config::engine(None).expect("engine without a cache");
        let cached = crate::config::engine(Some(&cache_dir)).expect("engine with a cache");
        assert_eq!(
            compatibility_hash_hex(&plain),
            compatibility_hash_hex(&cached),
            "a host with --cache-dir must accept a .cwasm precompiled without one"
        );
    }

    #[test]
    fn sidecar_path_appends_json() {
        assert_eq!(
            metadata_sidecar(Path::new("/bundle/rustc-aarch64-apple-darwin.cwasm")),
            PathBuf::from("/bundle/rustc-aarch64-apple-darwin.cwasm.json")
        );
    }
}
