//! The compile request and the tier policy.
//!
//! A [`JobSpec`] is the host-to-supervisor contract; it is versioned by
//! [`SCHEMA`] and serialisable so it can be logged, cached and replayed.
//! [`Tier`] is the only place that knows which flags a tier means: the compiler
//! never sees a tier name.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Contract version. Bumping it invalidates every cache key, deliberately.
pub const SCHEMA: u32 = 1;

/// Which output the caller asked for.
///
/// `Fast` is the edit loop (rustc's incremental mode), `Opt` is the background
/// optimising build, and `Small` is a distribution artifact that is never
/// swapped into a running tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Fast,
    Opt,
    Small,
}

impl Tier {
    /// The stable name used in keys, logs and the CLI.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fast => "fast",
            Self::Opt => "opt",
            Self::Small => "small",
        }
    }

    /// Compiler flags this tier means. The mapping lives here, in the host,
    /// and never inside the compiler.
    #[must_use]
    pub fn flags(self) -> Vec<String> {
        match self {
            // The measured fast tier: incremental is about 3x faster to build
            // than the full optimising build on the benchmark suite.
            Self::Fast => vec!["-Copt-level=2".to_owned(), "-Cincremental".to_owned()],
            // The measured optimising tier: opt-level 2 plus the loop-aware
            // MIR inliner from step 5b.
            Self::Opt => vec!["-Copt-level=2".to_owned(), "-Zmir-inline-wasm".to_owned()],
            // Distribution only: never swapped into a running tier.
            Self::Small => vec!["-Copt-level=s".to_owned(), "-Cstrip=debuginfo".to_owned()],
        }
    }
}

/// The source unit to compile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpec {
    pub path: PathBuf,
    pub crate_name: String,
    #[serde(default = "default_edition")]
    pub edition: String,
}

impl SourceSpec {
    /// A source spec with the crate's current default edition.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, crate_name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            crate_name: crate_name.into(),
            edition: default_edition(),
        }
    }
}

fn default_edition() -> String {
    "2021".to_owned()
}

/// Optional limits the scheduler may use to reject or downgrade a build.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Budgets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compile_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

/// Extra outputs a build may ask for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Emit {
    Wasm,
    Metadata,
}

/// A single compile request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobSpec {
    pub schema: u32,
    pub source: SourceSpec,
    pub target: String,
    pub tier: Tier,
    #[serde(default)]
    pub budgets: Budgets,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental_dir: Option<PathBuf>,
    #[serde(default)]
    pub emit: Vec<Emit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl JobSpec {
    /// A request for `source` at `tier`, targeting `wasm32-unknown-unknown`.
    #[must_use]
    pub fn new(source: SourceSpec, tier: Tier) -> Self {
        Self {
            schema: SCHEMA,
            source,
            target: "wasm32-unknown-unknown".to_owned(),
            tier,
            budgets: Budgets::default(),
            incremental_dir: None,
            emit: vec![Emit::Wasm],
            request_id: None,
        }
    }

    /// Reject requests the supervisor cannot honour before any work starts.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown schema version, an empty source path,
    /// an empty crate name or an unsupported edition.
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA {
            bail!(
                "job schema {} is not supported (expected {SCHEMA})",
                self.schema
            );
        }
        if self.source.path.as_os_str().is_empty() {
            bail!("job source path is empty");
        }
        if self.source.crate_name.is_empty() {
            bail!("job crate name is empty");
        }
        if !matches!(self.source.edition.as_str(), "2018" | "2021" | "2024") {
            bail!("unsupported edition {:?}", self.source.edition);
        }
        Ok(())
    }

    /// Every flag the compiler receives for this job, tier flags plus the
    /// incremental directory when the job names one.
    #[must_use]
    pub fn compiler_flags(&self) -> Vec<String> {
        let mut flags = self.tier.flags();
        if let Some(dir) = &self.incremental_dir {
            flags.push(format!("-Cincremental={}", dir.display()));
        }
        flags
    }
}
