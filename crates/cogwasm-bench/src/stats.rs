//! Statistics shared by every measurement and by `compare`.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

/// One measured quantity: the spread of its runs and whether it is expected to
/// be bit-for-bit reproducible.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub min: f64,
    pub median: f64,
    pub max: f64,
    pub runs: usize,
    /// True for counters and sizes (exact, comparable directly), false for
    /// wall-clock timings (compare only outside the run-to-run spread).
    pub deterministic: bool,
}

impl Metric {
    /// Build a metric from samples; an empty slice is an error because a
    /// measurement with no samples is not a measurement.
    ///
    /// # Errors
    ///
    /// Returns an error when `values` is empty or contains a NaN.
    pub fn new(values: &[f64], deterministic: bool) -> Result<Self> {
        if values.is_empty() {
            bail!("cannot summarise zero samples");
        }
        if values.iter().any(|value| value.is_nan()) {
            bail!("a measurement sample is NaN");
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(f64::total_cmp);
        let middle = sorted.len() / 2;
        let median = if sorted.len().is_multiple_of(2) {
            sorted[middle - 1] + (sorted[middle] - sorted[middle - 1]) / 2.0
        } else {
            sorted[middle]
        };
        Ok(Self {
            min: sorted[0],
            median,
            max: sorted[sorted.len() - 1],
            runs: sorted.len(),
            deterministic,
        })
    }

    /// A metric with one sample.
    ///
    /// # Errors
    ///
    /// Returns an error when `value` is NaN.
    pub fn one(value: f64, deterministic: bool) -> Result<Self> {
        Self::new(&[value], deterministic)
    }

    /// The nearest-rank percentile, using the same rule as the supervisor's
    /// edit-loop summary.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty slice or a percentile outside `0.0..=1.0`.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the sample count is far below 2^53"
    )]
    pub fn percentile(values: &[f64], percentile: f64) -> Result<f64> {
        if values.is_empty() {
            bail!("cannot take a percentile of zero samples");
        }
        if !(0.0..=1.0).contains(&percentile) {
            bail!("percentile must be within 0.0..=1.0, got {percentile}");
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(f64::total_cmp);
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "the rank is a positive count of samples, far below u64::MAX"
        )]
        let rank = ((sorted.len() as f64) * percentile).ceil() as usize;
        Ok(sorted[rank.max(1) - 1])
    }

    /// Whether `self` lies outside `other`'s spread.
    #[must_use]
    pub fn outside_spread_of(&self, other: &Self) -> bool {
        self.median > other.max || self.median < other.min
    }
}

/// The geometric mean of positive values, or `None` when there are none.
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    reason = "the sample count is far below 2^53"
)]
pub fn geomean(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    let mut count = 0usize;
    let mut log_sum = 0.0f64;
    for value in values {
        if value > 0.0 && value.is_finite() {
            count += 1;
            log_sum += value.ln();
        }
    }
    (count > 0).then(|| (log_sum / count as f64).exp())
}

/// Median of a slice, as a plain number, for values that are not measurements
/// in their own right (suite medians, ratios).
///
/// # Errors
///
/// Returns an error for an empty slice or a NaN sample.
pub fn median(values: &[f64]) -> Result<f64> {
    Metric::new(values, false).map(|metric| metric.median)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_reports_min_median_max_and_runs() {
        let metric = Metric::new(&[3.0, 1.0, 2.0], false).expect("metric");
        assert_eq!(metric.min, 1.0);
        assert_eq!(metric.median, 2.0);
        assert_eq!(metric.max, 3.0);
        assert_eq!(metric.runs, 3);
        assert!(!metric.deterministic);
    }

    #[test]
    fn even_sample_median_averages_the_middle_pair() {
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]).expect("median"), 2.5);
    }

    #[test]
    fn percentile_is_nearest_rank() {
        let values = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        assert_eq!(Metric::percentile(&values, 0.9).expect("p90"), 90.0);
        assert_eq!(Metric::percentile(&values, 0.5).expect("p50"), 50.0);
        assert_eq!(Metric::percentile(&[7.0], 0.9).expect("p90"), 7.0);
    }

    #[test]
    fn geomean_multiplies_in_log_space() {
        let value = geomean([1.0, 4.0, 16.0]).expect("geomean");
        assert!((value - 4.0).abs() < 1e-12);
        assert!(geomean(std::iter::empty()).is_none());
    }

    #[test]
    fn spread_test_uses_the_baseline_min_and_max() {
        let baseline = Metric::new(&[10.0, 12.0, 14.0], false).expect("baseline");
        let inside = Metric::new(&[11.0, 13.0, 15.0], false).expect("inside");
        let outside = Metric::new(&[14.5, 15.0, 16.0], false).expect("outside");
        assert!(!inside.outside_spread_of(&baseline));
        assert!(outside.outside_spread_of(&baseline));
    }

    #[test]
    fn rejects_empty_and_nan() {
        assert!(Metric::new(&[], false).is_err());
        assert!(Metric::new(&[f64::NAN], false).is_err());
    }
}
