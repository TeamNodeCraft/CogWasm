//! Empirical selection (M5b): record how each spec performed and pick one.
//!
//! The supervisor does not choose specs by itself. A host measures builds and
//! runs, records them here, and asks for the best known spec for a program and
//! an objective. The policy is deliberately conservative: a spec needs a
//! minimum number of samples, and a switch only happens when the new winner is
//! better than the current one by a margin (hysteresis), so noise in execution
//! timings cannot make the choice oscillate.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::job::CompileOptions;

/// What the caller is optimising for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Objective {
    /// Lowest execution time.
    Exec,
    /// Smallest artifact.
    Size,
    /// Both, as the product `exec_ms * size_bytes` (lower is better).
    Balanced,
}

/// One measured build (and run) of one spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub spec: CompileOptions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compile_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec_ms: Option<f64>,
    pub size_bytes: u64,
}

/// Measurements per program.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scoreboard {
    programs: BTreeMap<String, Vec<Measurement>>,
}

/// What a program's measurements say about one spec.
#[derive(Debug, Clone)]
struct Candidate {
    spec: CompileOptions,
    samples: usize,
    exec_ms: Option<f64>,
    compile_ms: Option<u64>,
    size_bytes: u64,
}

impl Scoreboard {
    /// An empty scoreboard.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one measurement for `program`.
    pub fn record(&mut self, program: &str, measurement: Measurement) {
        self.programs
            .entry(program.to_owned())
            .or_default()
            .push(measurement);
    }

    /// Every measurement for `program`.
    #[must_use]
    pub fn samples(&self, program: &str) -> &[Measurement] {
        self.programs.get(program).map_or(&[], Vec::as_slice)
    }

    /// The best spec for `program` under `objective`, if any spec has at least
    /// `min_samples` measurements.
    #[must_use]
    pub fn best(
        &self,
        program: &str,
        objective: Objective,
        min_samples: usize,
    ) -> Option<CompileOptions> {
        self.candidates(program, min_samples)
            .into_iter()
            .min_by(|left, right| {
                score(left, objective)
                    .partial_cmp(&score(right, objective))
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| {
                        left.compile_ms
                            .unwrap_or(u64::MAX)
                            .cmp(&right.compile_ms.unwrap_or(u64::MAX))
                    })
                    .then_with(|| left.size_bytes.cmp(&right.size_bytes))
                    .then_with(|| right.samples.cmp(&left.samples))
            })
            .map(|candidate| candidate.spec)
    }

    /// Choose a spec, keeping `current` unless `best` beats it by `margin`.
    ///
    /// `margin` is a fraction: `0.05` means the new spec must be at least 5%
    /// better. When `current` has too few samples, the best spec wins.
    #[must_use]
    pub fn choose(
        &self,
        program: &str,
        objective: Objective,
        min_samples: usize,
        current: Option<&CompileOptions>,
        margin: f64,
    ) -> Option<CompileOptions> {
        let best = self.candidates(program, min_samples);
        let current_candidate = current.and_then(|spec| {
            best.iter()
                .find(|candidate| candidate.spec == *spec)
                .cloned()
        });
        let best_candidate = best.into_iter().min_by(|left, right| {
            score(left, objective)
                .partial_cmp(&score(right, objective))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| right.samples.cmp(&left.samples))
        })?;
        match current_candidate {
            Some(current) if current.spec != best_candidate.spec => {
                let current_score = score(&current, objective);
                let best_score = score(&best_candidate, objective);
                if best_score < current_score * (1.0 - margin) {
                    Some(best_candidate.spec)
                } else {
                    Some(current.spec)
                }
            }
            Some(current) => Some(current.spec),
            None => Some(best_candidate.spec),
        }
    }

    /// Read a scoreboard from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or parsed.
    pub fn load(path: &Path) -> Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    /// Write the scoreboard to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be written.
    pub fn save(&self, path: &Path) -> Result<()> {
        let text = serde_json::to_string_pretty(self).context("serialising the scoreboard")?;
        std::fs::write(path, text).with_context(|| format!("writing {}", path.display()))
    }

    fn candidates(&self, program: &str, min_samples: usize) -> Vec<Candidate> {
        let mut groups: Vec<Vec<&Measurement>> = Vec::new();
        for measurement in self.samples(program) {
            match groups
                .iter_mut()
                .find(|group| group[0].spec == measurement.spec)
            {
                Some(group) => group.push(measurement),
                None => groups.push(vec![measurement]),
            }
        }
        groups
            .into_iter()
            .filter(|group| group.len() >= min_samples)
            .map(|group| Candidate {
                spec: group[0].spec.clone(),
                samples: group.len(),
                exec_ms: median(group.iter().filter_map(|m| m.exec_ms)),
                compile_ms: median_u64(group.iter().filter_map(|m| m.compile_ms)),
                size_bytes: group[0].size_bytes,
            })
            .collect()
    }
}

fn score(candidate: &Candidate, objective: Objective) -> f64 {
    match objective {
        Objective::Exec => candidate.exec_ms.unwrap_or(f64::MAX),
        Objective::Size => u64_to_f64(candidate.size_bytes),
        Objective::Balanced => {
            candidate.exec_ms.unwrap_or(f64::MAX) * u64_to_f64(candidate.size_bytes)
        }
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "artifact sizes and compile times are far below 2^52"
)]
fn u64_to_f64(value: u64) -> f64 {
    value as f64
}

fn median_u64(values: impl Iterator<Item = u64>) -> Option<u64> {
    let mut values: Vec<u64> = values.collect();
    if values.is_empty() {
        return None;
    }
    values.sort_unstable();
    let middle = values.len() / 2;
    if values.len().is_multiple_of(2) {
        Some(u64::midpoint(values[middle - 1], values[middle]))
    } else {
        Some(values[middle])
    }
}

fn median(values: impl Iterator<Item = f64>) -> Option<f64> {
    let mut values: Vec<f64> = values.collect();
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    if values.len().is_multiple_of(2) {
        Some(f64::midpoint(values[middle - 1], values[middle]))
    } else {
        Some(values[middle])
    }
}
