//! The compile request and the option space.
//!
//! A [`JobSpec`] is the host-to-supervisor contract; it is versioned by
//! [`SCHEMA`] and serialisable so it can be logged, cached and replayed.
//!
//! [`CompileOptions`] is the request itself: a whitelisted set of knobs the
//! supervisor can vary. [`Preset`] names the three combinations that matter in
//! practice (`fast`, `opt`, `release`), but they are aliases for a spec, not
//! the only values. The compiler never sees an option name; it sees flags.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

/// Contract version. Bumping it invalidates every cache key, deliberately.
pub const SCHEMA: u32 = 2;

/// `-Copt-level`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptLevel {
    O0,
    O1,
    O2,
    O3,
    S,
    Z,
}

impl OptLevel {
    /// The value rustc expects after `-Copt-level=`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::O0 => "0",
            Self::O1 => "1",
            Self::O2 => "2",
            Self::O3 => "3",
            Self::S => "s",
            Self::Z => "z",
        }
    }
}

impl FromStr for OptLevel {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "0" => Ok(Self::O0),
            "1" => Ok(Self::O1),
            "2" => Ok(Self::O2),
            "3" => Ok(Self::O3),
            "s" => Ok(Self::S),
            "z" => Ok(Self::Z),
            other => Err(format!("unknown opt-level {other:?} (0, 1, 2, 3, s or z)")),
        }
    }
}

/// `-Cstrip`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Strip {
    None,
    Debuginfo,
    Symbols,
}

impl Strip {
    /// The option name used in a spec.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Debuginfo => "debuginfo",
            Self::Symbols => "symbols",
        }
    }

    /// The value rustc expects after `-Cstrip=`, or `None` for no flag.
    #[must_use]
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Debuginfo => Some("debuginfo"),
            Self::Symbols => Some("symbols"),
        }
    }
}

impl FromStr for Strip {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "debuginfo" => Ok(Self::Debuginfo),
            "symbols" => Ok(Self::Symbols),
            other => Err(format!(
                "unknown strip {other:?} (none, debuginfo or symbols)"
            )),
        }
    }
}

/// The knobs a request may vary. Every field is checked before it reaches the
/// compiler, so an app can choose combinations but never arbitrary flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileOptions {
    pub opt_level: OptLevel,
    pub mir_opt_level: u8,
    /// The loop-aware MIR inliner from step 5b.
    #[serde(default)]
    pub inline_wasm: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_threshold: Option<usize>,
    /// rustc's incremental mode; the job must name a directory.
    #[serde(default)]
    pub incremental: bool,
    pub strip: Strip,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codegen_units: Option<u32>,
    /// Emit debug info (cannot be combined with a strip).
    #[serde(default)]
    pub debug: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            opt_level: OptLevel::O2,
            mir_opt_level: 2,
            inline_wasm: false,
            inline_threshold: None,
            incremental: false,
            strip: Strip::None,
            codegen_units: None,
            debug: false,
        }
    }
}

impl CompileOptions {
    /// Reject combinations that cannot mean anything.
    ///
    /// # Errors
    ///
    /// Returns an error for a MIR level above 3, debug info combined with a
    /// strip, or an inliner threshold without the inliner.
    pub fn validate(&self) -> Result<()> {
        if self.mir_opt_level > 3 {
            bail!("mir-opt-level must be 0..=3, got {}", self.mir_opt_level);
        }
        if self.debug && self.strip != Strip::None {
            bail!(
                "debug info cannot be combined with strip={}",
                self.strip.name()
            );
        }
        if self.inline_threshold.is_some() && !self.inline_wasm {
            bail!("inline-threshold requires inline-wasm");
        }
        Ok(())
    }

    /// The rustc flags this spec means.
    #[must_use]
    pub fn flags(&self, incremental_dir: Option<&PathBuf>) -> Vec<String> {
        let mut flags = vec![
            format!("-Copt-level={}", self.opt_level.as_str()),
            format!("-Zmir-opt-level={}", self.mir_opt_level),
        ];
        if self.inline_wasm {
            flags.push("-Zmir-inline-wasm".to_owned());
            if let Some(threshold) = self.inline_threshold {
                flags.push(format!("-Zmir-inline-wasm-threshold={threshold}"));
            }
        }
        if self.incremental
            && let Some(dir) = incremental_dir
        {
            flags.push(format!("-Cincremental={}", dir.display()));
        }
        if let Some(strip) = self.strip.as_str() {
            flags.push(format!("-Cstrip={strip}"));
        }
        if let Some(units) = self.codegen_units {
            flags.push(format!("-Ccodegen-units={units}"));
        }
        if self.debug {
            flags.push("-Cdebuginfo=2".to_owned());
        }
        flags
    }

    /// The canonical spec string: the inverse of [`CompileOptions::from_spec`].
    #[must_use]
    pub fn to_spec(&self) -> String {
        let mut parts = vec![
            format!("opt-level={}", self.opt_level.as_str()),
            format!("mir-opt-level={}", self.mir_opt_level),
            format!("inline={}", yes_no(self.inline_wasm)),
        ];
        if let Some(threshold) = self.inline_threshold {
            parts.push(format!("inline-threshold={threshold}"));
        }
        parts.push(format!("incremental={}", yes_no(self.incremental)));
        parts.push(format!("strip={}", self.strip.name()));
        if let Some(units) = self.codegen_units {
            parts.push(format!("codegen-units={units}"));
        }
        parts.push(format!("debug={}", yes_no(self.debug)));
        parts.join(",")
    }

    /// Parse `"fast"`, `"opt"`, `"release"` or a comma-separated option list
    /// such as `"opt-level=2,inline=yes,strip=symbols"`.
    ///
    /// # Errors
    ///
    /// Returns a message naming the first unknown key or value.
    pub fn from_spec(text: &str) -> Result<Self, String> {
        if let Ok(preset) = Preset::from_str(text) {
            return Ok(preset.options());
        }
        let mut options = Self::default();
        for part in text.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| format!("expected key=value, got {part:?}"))?;
            match key {
                "opt-level" => options.opt_level = OptLevel::from_str(value)?,
                "mir-opt-level" => {
                    options.mir_opt_level = value
                        .parse()
                        .map_err(|_| format!("mir-opt-level must be 0..=3, got {value:?}"))?;
                }
                "inline" => options.inline_wasm = parse_bool(value)?,
                "inline-threshold" => {
                    options.inline_threshold = Some(value.parse().map_err(|_| {
                        format!("inline-threshold must be a number, got {value:?}")
                    })?);
                }
                "incremental" => options.incremental = parse_bool(value)?,
                "strip" => options.strip = Strip::from_str(value)?,
                "codegen-units" => {
                    options.codegen_units =
                        Some(value.parse().map_err(|_| {
                            format!("codegen-units must be a number, got {value:?}")
                        })?);
                }
                "debug" => options.debug = parse_bool(value)?,
                other => return Err(format!("unknown option {other:?}")),
            }
        }
        options.validate().map_err(|error| error.to_string())?;
        Ok(options)
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value {
        "yes" | "true" | "1" => Ok(true),
        "no" | "false" | "0" => Ok(false),
        other => Err(format!("expected yes/no, got {other:?}")),
    }
}

/// Named points in the option space. They are conveniences: the supervisor
/// still stores and compares the full [`CompileOptions`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Preset {
    /// Edit loop: incremental, no inliner.
    Fast,
    /// Background optimising build.
    Opt,
    /// Distribution: optimised and symbol-stripped.
    Release,
}

impl Preset {
    /// The spec this preset expands to.
    #[must_use]
    pub fn options(self) -> CompileOptions {
        match self {
            Self::Fast => CompileOptions {
                incremental: true,
                ..CompileOptions::default()
            },
            Self::Opt => CompileOptions {
                inline_wasm: true,
                ..CompileOptions::default()
            },
            Self::Release => CompileOptions {
                inline_wasm: true,
                strip: Strip::Symbols,
                ..CompileOptions::default()
            },
        }
    }

    /// The preset name as written on the command line.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fast => "fast",
            Self::Opt => "opt",
            Self::Release => "release",
        }
    }
}

impl FromStr for Preset {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "fast" => Ok(Self::Fast),
            "opt" => Ok(Self::Opt),
            "release" => Ok(Self::Release),
            other => Err(format!("unknown preset {other:?}")),
        }
    }
}

/// What the crate produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CrateKind {
    /// A linked plugin, the thing the supervisor swaps in.
    #[default]
    Cdylib,
    /// A library consumed as `--extern` by another crate.
    Rlib,
}

impl CrateKind {
    /// The value rustc expects after `--crate-type`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cdylib => "cdylib",
            Self::Rlib => "rlib",
        }
    }
}

/// One `--extern name=path` input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternSpec {
    pub name: String,
    pub path: PathBuf,
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
    pub kind: CrateKind,
    pub options: CompileOptions,
    /// `--extern` libraries this crate links against.
    #[serde(default)]
    pub externs: Vec<ExternSpec>,
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
    /// A request for `source` with `options`, targeting
    /// `wasm32-unknown-unknown`.
    #[must_use]
    pub fn new(source: SourceSpec, options: CompileOptions) -> Self {
        Self {
            schema: SCHEMA,
            source,
            target: "wasm32-unknown-unknown".to_owned(),
            kind: CrateKind::Cdylib,
            options,
            externs: Vec::new(),
            budgets: Budgets::default(),
            incremental_dir: None,
            emit: vec![Emit::Wasm],
            request_id: None,
        }
    }

    /// A request for `source` at a named preset.
    #[must_use]
    pub fn preset(source: SourceSpec, preset: Preset) -> Self {
        Self::new(source, preset.options())
    }

    /// Reject requests the supervisor cannot honour before any work starts.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown schema version, an empty source path,
    /// an empty crate name, an unsupported edition, invalid options, or an
    /// incremental request without a directory.
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
        self.options.validate()?;
        if self.options.incremental && self.incremental_dir.is_none() {
            bail!("incremental option needs an incremental directory");
        }
        let mut names = std::collections::BTreeSet::new();
        for extern_spec in &self.externs {
            if extern_spec.name.is_empty() {
                bail!("an --extern name is empty");
            }
            if !names.insert(extern_spec.name.as_str()) {
                bail!("duplicate --extern name {:?}", extern_spec.name);
            }
        }
        Ok(())
    }

    /// Every flag the compiler receives for this job.
    #[must_use]
    pub fn compiler_flags(&self) -> Vec<String> {
        self.options.flags(self.incremental_dir.as_ref())
    }
}
