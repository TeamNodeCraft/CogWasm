//! Wall-clock calibration from deterministic counters.
//!
//! A counters-mode run on a busy machine can still answer "roughly how long
//! would this have taken?" if an idle calibration run fitted wall-clock
//! milliseconds against fuel and native instructions retired. This module
//! stores that fit in `result.json`, and `run --mode counters --calibration`
//! prints the estimate with its root-mean-square error band.
//!
//! Estimates are always labelled as estimates: a busy machine can change the
//! constant factor (cache pressure, frequency), which no fit can recover.

use crate::result::ProgramResult;
use crate::stats::MetricClass;
use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One calibration observation: a compile's wall time and deterministic work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FitPoint {
    /// `cold`, `warm` or `edit`.
    pub kind: String,
    /// The program class, e.g. `small` or `medium`.
    pub program_class: String,
    pub wall_ms: f64,
    pub fuel: f64,
    pub instructions: f64,
}

/// A linear fit `wall_ms = intercept + fuel_coeff * fuel + instruction_coeff *
/// instructions`, with its error band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fit {
    pub kind: String,
    /// The program class, or `all` for the kind-wide fallback.
    pub program_class: String,
    pub samples: usize,
    pub intercept_ms: f64,
    pub fuel_ms: f64,
    pub instructions_ms: f64,
    /// Coefficient of determination on the fitted points.
    pub r2: f64,
    /// Root-mean-square error in milliseconds; this is the error band.
    pub rmse_ms: f64,
}

/// Every fit from one calibration run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Calibration {
    pub schema: u32,
    pub generated_at: String,
    /// Whether every calibration window was idle-qualified. A busy-machine
    /// fit is stored with its evidence but must not be trusted as definitive.
    #[serde(default)]
    pub idle_qualified: bool,
    pub fits: Vec<Fit>,
}

/// An estimated wall time and its error band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Estimate {
    pub wall_ms: f64,
    /// One standard-error band from the fit (`rmse_ms`).
    pub error_ms: f64,
    /// The class whose fit was used (`all` is the kind-wide fallback).
    pub fit_program_class: String,
    pub fit_samples: usize,
}

/// One program/config/kind estimate stored in a counters-mode result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallEstimate {
    pub program: String,
    pub config: String,
    pub kind: String,
    #[serde(flatten)]
    pub estimate: Estimate,
}

/// Fit every compile kind, per program class and across classes.
#[must_use]
pub fn fit(points: &[FitPoint], idle_qualified: bool) -> Calibration {
    let mut kinds: Vec<String> = points.iter().map(|point| point.kind.clone()).collect();
    kinds.sort();
    kinds.dedup();
    let mut fits = Vec::new();
    for kind in &kinds {
        let kind_points: Vec<&FitPoint> =
            points.iter().filter(|point| &point.kind == kind).collect();
        if let Some(fit) = fit_group(kind, "all", &kind_points) {
            fits.push(fit);
        }
        let mut classes: Vec<String> = kind_points
            .iter()
            .map(|point| point.program_class.clone())
            .collect();
        classes.sort();
        classes.dedup();
        for class in classes {
            let class_points: Vec<&FitPoint> = kind_points
                .iter()
                .copied()
                .filter(|point| point.program_class == class)
                .collect();
            if class_points.len() >= 3
                && let Some(fit) = fit_group(kind, &class, &class_points)
            {
                fits.push(fit);
            }
        }
    }
    Calibration {
        schema: 1,
        generated_at: util::utc_rfc3339(),
        idle_qualified,
        fits,
    }
}

/// Fit one group. Fewer than three points cannot identify three coefficients,
/// so the caller falls back to the kind-wide fit.
fn fit_group(kind: &str, class: &str, points: &[&FitPoint]) -> Option<Fit> {
    if points.len() < 3 {
        return None;
    }
    // Normal equations for X = [1, fuel, instructions].
    let mut matrix = [[0.0_f64; 4]; 3];
    for point in points {
        let row = [1.0, point.fuel, point.instructions];
        for (index, left) in row.iter().enumerate() {
            for (column, right) in row.iter().enumerate() {
                matrix[index][column] += left * right;
            }
            matrix[index][3] += left * point.wall_ms;
        }
    }
    let coefficients = solve3(matrix)?;
    let [intercept, fuel_coeff, instruction_coeff] = coefficients;
    let predicted: Vec<f64> = points
        .iter()
        .map(|point| intercept + fuel_coeff * point.fuel + instruction_coeff * point.instructions)
        .collect();
    let mean = points.iter().map(|point| point.wall_ms).sum::<f64>() / len_as_f64(points.len());
    let mut residual_sum = 0.0;
    let mut total_sum = 0.0;
    for (point, predicted) in points.iter().zip(&predicted) {
        let residual = point.wall_ms - predicted;
        residual_sum += residual * residual;
        let deviation = point.wall_ms - mean;
        total_sum += deviation * deviation;
    }
    let count = len_as_f64(points.len());
    Some(Fit {
        kind: kind.to_owned(),
        program_class: class.to_owned(),
        samples: points.len(),
        intercept_ms: intercept,
        fuel_ms: fuel_coeff,
        instructions_ms: instruction_coeff,
        r2: if total_sum > 0.0 {
            1.0 - residual_sum / total_sum
        } else {
            1.0
        },
        rmse_ms: (residual_sum / count).sqrt(),
    })
}

#[expect(
    clippy::cast_precision_loss,
    reason = "calibration sample counts are tiny"
)]
fn len_as_f64(value: usize) -> f64 {
    value as f64
}

/// Gaussian elimination with partial pivoting for a 3x4 augmented matrix.
fn solve3(mut matrix: [[f64; 4]; 3]) -> Option<[f64; 3]> {
    for column in 0..3 {
        let pivot = (column..3).max_by(|left, right| {
            matrix[*left][column]
                .abs()
                .total_cmp(&matrix[*right][column].abs())
        })?;
        if matrix[pivot][column].abs() < 1e-12 {
            return None;
        }
        matrix.swap(column, pivot);
        let divisor = matrix[column][column];
        for entry in &mut matrix[column][column..4] {
            *entry /= divisor;
        }
        let pivot_row = matrix[column];
        for (row, matrix_row) in matrix.iter_mut().enumerate() {
            if row == column {
                continue;
            }
            let factor = matrix_row[column];
            for (entry, pivot) in matrix_row[column..4].iter_mut().zip(&pivot_row[column..4]) {
                *entry -= factor * pivot;
            }
        }
    }
    Some([matrix[0][3], matrix[1][3], matrix[2][3]])
}

impl Calibration {
    /// Estimate one compile kind for one program class, preferring the class
    /// fit and falling back to the kind-wide fit.
    #[must_use]
    pub fn estimate(
        &self,
        kind: &str,
        program_class: &str,
        fuel: f64,
        instructions: f64,
    ) -> Option<Estimate> {
        let exact = self
            .fits
            .iter()
            .find(|fit| fit.kind == kind && fit.program_class == program_class);
        let fallback = self
            .fits
            .iter()
            .find(|fit| fit.kind == kind && fit.program_class == "all");
        let fit = exact.or(fallback)?;
        Some(Estimate {
            wall_ms: fit.intercept_ms + fit.fuel_ms * fuel + fit.instructions_ms * instructions,
            error_ms: fit.rmse_ms,
            fit_program_class: fit.program_class.clone(),
            fit_samples: fit.samples,
        })
    }
}

/// Collect the calibration points from a completed run.
#[must_use]
pub fn points(programs: &BTreeMap<String, ProgramResult>) -> Vec<FitPoint> {
    let mut points = Vec::new();
    for program in programs.values() {
        for result in program.configs.values() {
            let build = &result.build;
            let host = counter_value(&build.native.host_instructions);
            if let (Some(fuel), Some(instructions)) =
                (build.fuel.as_ref().map(|fuel| fuel.fuel.median), host)
            {
                points.push(FitPoint {
                    kind: "cold".to_owned(),
                    program_class: program.class.clone(),
                    wall_ms: build.cold_compile_ms.median,
                    fuel,
                    instructions,
                });
            }
            if let (Some(fuel), Some(instructions)) =
                (build.warm_fuel.as_ref().map(|fuel| fuel.fuel.median), host)
            {
                points.push(FitPoint {
                    kind: "warm".to_owned(),
                    program_class: program.class.clone(),
                    wall_ms: build.warm_compile_ms.median,
                    fuel,
                    instructions,
                });
            }
            if let (Some(fuel), Some(instructions)) =
                (build.edit.fuel.as_ref().map(|fuel| fuel.fuel.median), host)
            {
                points.push(FitPoint {
                    kind: "edit".to_owned(),
                    program_class: program.class.clone(),
                    wall_ms: build.edit.p50_ms,
                    fuel,
                    instructions,
                });
            }
        }
    }
    points
}

/// Estimates for a counters-mode run, one per program/config/kind.
#[must_use]
pub fn estimates(
    programs: &BTreeMap<String, ProgramResult>,
    calibration: &Calibration,
) -> Vec<WallEstimate> {
    let mut output = Vec::new();
    for (name, program) in programs {
        for (config, result) in &program.configs {
            let build = &result.build;
            let host = counter_value(&build.native.host_instructions);
            for (kind, fuel) in [
                ("cold", build.fuel.as_ref().map(|fuel| fuel.fuel.median)),
                (
                    "warm",
                    build.warm_fuel.as_ref().map(|fuel| fuel.fuel.median),
                ),
                (
                    "edit",
                    build.edit.fuel.as_ref().map(|fuel| fuel.fuel.median),
                ),
            ] {
                let (Some(fuel), Some(instructions)) = (fuel, host) else {
                    continue;
                };
                if let Some(estimate) =
                    calibration.estimate(kind, &program.class, fuel, instructions)
                {
                    output.push(WallEstimate {
                        program: name.clone(),
                        config: config.clone(),
                        kind: kind.to_owned(),
                        estimate,
                    });
                }
            }
        }
    }
    output
}

fn counter_value(metric: &crate::stats::Metric) -> Option<f64> {
    (metric.class == MetricClass::Counter && metric.is_available()).then_some(metric.median)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(kind: &str, class: &str, fuel: f64, instructions: f64, wall: f64) -> FitPoint {
        FitPoint {
            kind: kind.to_owned(),
            program_class: class.to_owned(),
            wall_ms: wall,
            fuel,
            instructions,
        }
    }

    #[test]
    fn a_linear_fit_recovers_the_coefficients() {
        let points: Vec<FitPoint> = (0..6)
            .map(|index| {
                let fuel = 1000.0 + f64::from(index) * 100.0;
                // Deliberately not collinear with fuel: the normal equations
                // must identify both coefficients.
                let instructions =
                    10_000.0 + f64::from(index) * 500.0 + f64::from(index * index) * 37.0;
                let wall = 2.0 + 0.001 * fuel + 0.000_02 * instructions;
                point("warm", "small", fuel, instructions, wall)
            })
            .collect();
        let calibration = fit(&points, true);
        let estimate = calibration
            .estimate("warm", "small", 1300.0, 12_537.0)
            .expect("estimate");
        assert!((estimate.wall_ms - (2.0 + 1.3 + 0.250_74)).abs() < 1e-9);
        assert!(estimate.error_ms < 1e-9);
        assert_eq!(estimate.fit_program_class, "small");
        let wide = calibration
            .estimate("warm", "unknown", 1300.0, 12_537.0)
            .expect("fallback");
        assert_eq!(wide.fit_program_class, "all");
    }

    #[test]
    fn too_few_points_do_not_produce_a_class_fit() {
        let points = vec![
            point("edit", "small", 10.0, 100.0, 5.0),
            point("edit", "small", 20.0, 200.0, 6.0),
        ];
        let calibration = fit(&points, true);
        assert!(calibration.fits.is_empty());
        assert!(calibration.estimate("edit", "small", 10.0, 100.0).is_none());
    }

    #[test]
    fn a_singular_matrix_is_rejected() {
        let matrix = [
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 2.0],
            [1.0, 1.0, 1.0, 3.0],
        ];
        assert!(solve3(matrix).is_none());
    }

    #[test]
    fn estimates_are_only_for_unavailable_wall_values() {
        // The full-run wiring is covered by the integration test; this checks
        // that `counter_value` refuses unavailable counters.
        let mut metric = crate::stats::Metric::one(5.0, MetricClass::Counter).expect("metric");
        assert_eq!(counter_value(&metric), Some(5.0));
        metric.unavailable = Some("no perf".to_owned());
        assert_eq!(counter_value(&metric), None);
    }
}
