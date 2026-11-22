//! Content-addressed artifact store.
//!
//! A key covers every input that can change the bytes: contract schema,
//! compiler identity, target, the option spec, the source path and the source
//! content. The path is deliberately part of the key: rustc object files embed
//! the source path, so two checkouts of identical sources do not produce
//! identical artifacts.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use crate::compiler::CompileOutput;
use crate::job::{CompileOptions, CrateKind, Emit, JobSpec};

/// Metadata stored beside every artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactMeta {
    pub schema: u32,
    pub key: String,
    pub options: CompileOptions,
    #[serde(default)]
    pub kind: CrateKind,
    #[serde(default)]
    pub crate_name: String,
    pub compiler: String,
    pub source_sha256: String,
    pub source_path: String,
    pub bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compile_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_ms: Option<u64>,
}

/// A compiled artifact and the metadata that describes how it was made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub key: String,
    pub bytes: Vec<u8>,
    pub meta: ArtifactMeta,
}

/// A directory of artifacts keyed by their inputs.
#[derive(Debug, Clone)]
pub struct ArtifactStore {
    root: PathBuf,
}

impl ArtifactStore {
    /// Create a store rooted at `root`; the directory is created on first use.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The store's root directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The key a job would be stored under for the given compiler identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the source file cannot be read.
    pub fn key_for(job: &JobSpec, compiler: &str) -> Result<String> {
        let source = source_sha256(&job.source.path)?;
        let externs = job
            .externs
            .iter()
            .map(|item| {
                let digest = source_sha256(&item.path)?;
                Ok((item.name.clone(), digest))
            })
            .collect::<Result<Vec<_>>>()?;
        let material = KeyMaterial::new(job, compiler, &source, externs);
        let encoded = serde_json::to_vec(&material).context("serialising the cache key")?;
        Ok(hex(&Sha256::digest(encoded)))
    }

    /// The name of `job`'s incremental directory: [`Self::key_for`] without
    /// the source and extern contents, so successive edits of one source with
    /// one set of options share rustc's incremental state instead of each
    /// starting from an empty directory. rustc itself detects what changed.
    ///
    /// # Errors
    ///
    /// Returns an error when the key cannot be serialised.
    pub fn incremental_key_for(job: &JobSpec, compiler: &str) -> Result<String> {
        let externs = job
            .externs
            .iter()
            .map(|item| (item.name.clone(), String::new()))
            .collect();
        let material = KeyMaterial::new(job, compiler, "", externs);
        let mut encoded = b"incremental\0".to_vec();
        encoded.extend(serde_json::to_vec(&material).context("serialising the incremental key")?);
        Ok(hex(&Sha256::digest(encoded)))
    }

    /// Build the artifact record for `bytes` produced for `job`.
    ///
    /// # Errors
    ///
    /// Returns an error when the source cannot be read or the key cannot be
    /// computed.
    pub fn artifact_for(
        &self,
        job: &JobSpec,
        compiler: &str,
        output: &CompileOutput,
    ) -> Result<Artifact> {
        let key = Self::key_for(job, compiler)?;
        let source_sha256 = source_sha256(&job.source.path)?;
        let length = u64::try_from(output.bytes.len()).context("artifact length")?;
        let meta = ArtifactMeta {
            schema: job.schema,
            key: key.clone(),
            options: job.options.clone(),
            kind: job.kind,
            crate_name: job.source.crate_name.clone(),
            compiler: compiler.to_owned(),
            source_sha256,
            source_path: job.source.path.display().to_string(),
            bytes: length,
            compile_ms: output.compile_ms,
            link_ms: output.link_ms,
        };
        Ok(Artifact {
            key,
            bytes: output.bytes.clone(),
            meta,
        })
    }

    /// The artifact stored under `key`, if any.
    ///
    /// # Errors
    ///
    /// Returns an error when the entry exists but cannot be read or parsed.
    pub fn get(&self, key: &str) -> Result<Option<Artifact>> {
        let dir = self.root.join(key);
        if !dir.is_dir() {
            return Ok(None);
        }
        let meta_bytes = fs::read(dir.join("meta.json"))
            .with_context(|| format!("reading metadata for {key}"))?;
        let meta: ArtifactMeta =
            serde_json::from_slice(&meta_bytes).context("parsing artifact metadata")?;
        let bytes = fs::read(dir.join(meta.file_name()))
            .with_context(|| format!("reading artifact {key}"))?;
        let length = u64::try_from(bytes.len()).context("artifact length")?;
        if meta.bytes != length {
            bail!(
                "artifact {key} is {length} bytes but its metadata says {}",
                meta.bytes
            );
        }
        Ok(Some(Artifact {
            key: meta.key.clone(),
            bytes,
            meta,
        }))
    }

    /// Store an artifact under its key. Writing an existing key is a no-op.
    ///
    /// # Errors
    ///
    /// Returns an error when the directory or files cannot be written.
    pub fn put(&self, artifact: &Artifact) -> Result<()> {
        let dir = self.root.join(&artifact.key);
        if dir.join(artifact.meta.file_name()).is_file() {
            return Ok(());
        }
        fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
        fs::write(dir.join(artifact.meta.file_name()), &artifact.bytes)
            .with_context(|| format!("writing artifact {}", artifact.key))?;
        let meta = serde_json::to_vec_pretty(&artifact.meta).context("serialising metadata")?;
        fs::write(dir.join("meta.json"), meta)
            .with_context(|| format!("writing metadata for {}", artifact.key))?;
        Ok(())
    }
}

/// SHA-256 of a source file, as lowercase hex.
///
/// # Errors
///
/// Returns an error when the file cannot be read.
pub fn source_sha256(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(hex(&Sha256::digest(&bytes)))
}

/// Everything that can change the artifact bytes, in a stable order.
#[derive(Debug, Serialize)]
struct KeyMaterial<'a> {
    schema: u32,
    compiler: &'a str,
    target: &'a str,
    kind: CrateKind,
    crate_name: &'a str,
    options: &'a CompileOptions,
    externs: Vec<(String, String)>,
    source_path: String,
    source_sha256: &'a str,
    emit: &'a [Emit],
}

impl<'a> KeyMaterial<'a> {
    fn new(
        job: &'a JobSpec,
        compiler: &'a str,
        source_sha256: &'a str,
        externs: Vec<(String, String)>,
    ) -> Self {
        Self {
            schema: job.schema,
            compiler,
            target: &job.target,
            kind: job.kind,
            crate_name: &job.source.crate_name,
            options: &job.options,
            externs,
            source_path: job.source.path.display().to_string(),
            source_sha256,
            emit: &job.emit,
        }
    }
}

impl ArtifactMeta {
    /// The file name of the artifact inside its key directory. rustc finds an
    /// `--extern` rlib by name, so rlibs must be `lib<crate>.rlib`.
    #[must_use]
    pub fn file_name(&self) -> String {
        match self.kind {
            CrateKind::Cdylib => "artifact.wasm".to_owned(),
            CrateKind::Rlib => format!("lib{}.rlib", self.crate_name),
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut out, byte| {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
        out
    })
}
