//! The JSON run specification read by `run --spec FILE`.
//!
//! Every field is optional; a command-line flag given alongside the spec wins.
//! Relative paths are resolved against the spec file's directory, so a spec
//! committed next to its outputs works from any working directory.
//!
//! ```json
//! {
//!   "suite": "quick",
//!   "programs": ["a_add", "nbody"],
//!   "configs": ["default", "capture"],
//!   "runs": 3,
//!   "fuel": true,
//!   "out": "../toolchain/work/bench/nightly",
//!   "chart": { "out": "chart", "config": "default", "history": ["history"] }
//! }
//! ```

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// A run specification.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSpec {
    pub suite: Option<String>,
    #[serde(default)]
    pub programs: Vec<String>,
    #[serde(default)]
    pub configs: Vec<String>,
    pub runs: Option<usize>,
    pub fuel: Option<bool>,
    pub out: Option<PathBuf>,
    pub chart: Option<ChartSpec>,
}

/// Where and how to write the chart document after the run.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChartSpec {
    /// Directory for `data.json`.
    pub out: PathBuf,
    /// The config to plot (defaults to `default`).
    pub config: Option<String>,
    /// Earlier `result.json` files or directories of them, plotted before
    /// this run.
    #[serde(default)]
    pub history: Vec<PathBuf>,
}

impl RunSpec {
    /// Parse a spec from JSON text, resolving relative paths against `base`.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed JSON or unknown fields.
    pub fn parse(text: &str, base: &Path) -> Result<Self> {
        let mut spec: Self = serde_json::from_str(text).context("parsing the run spec")?;
        let resolve = |path: &mut PathBuf| {
            if path.is_relative() {
                *path = base.join(&*path);
            }
        };
        if let Some(out) = spec.out.as_mut() {
            resolve(out);
        }
        if let Some(chart) = spec.chart.as_mut() {
            resolve(&mut chart.out);
            chart.history.iter_mut().for_each(resolve);
        }
        Ok(spec)
    }

    /// Read and parse a spec file.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or parsed.
    pub fn load(path: &Path) -> Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let base = path.parent().unwrap_or_else(|| Path::new("."));
        Self::parse(&text, base).with_context(|| format!("in {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_fields_are_optional_and_paths_resolve_against_the_spec() {
        let spec = RunSpec::parse(
            r#"{"suite":"plugin","configs":["capture"],"runs":2,"out":"o",
                "chart":{"out":"c","history":["h","/abs"]}}"#,
            Path::new("/specs"),
        )
        .expect("spec parses");
        assert_eq!(spec.suite.as_deref(), Some("plugin"));
        assert_eq!(spec.configs, ["capture"]);
        assert_eq!(spec.runs, Some(2));
        assert_eq!(spec.out, Some(PathBuf::from("/specs/o")));
        let chart = spec.chart.expect("chart");
        assert_eq!(chart.out, PathBuf::from("/specs/c"));
        assert_eq!(
            chart.history,
            [PathBuf::from("/specs/h"), PathBuf::from("/abs")]
        );
        assert_eq!(
            RunSpec::parse("{}", Path::new("/")).expect("empty"),
            RunSpec::default()
        );
    }

    #[test]
    fn unknown_fields_are_rejected() {
        assert!(RunSpec::parse(r#"{"suit":"quick"}"#, Path::new("/")).is_err());
    }
}
