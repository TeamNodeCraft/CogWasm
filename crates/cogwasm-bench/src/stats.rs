//! Statistics shared by every measurement and by `compare`.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

/// How a metric may be compared between two runs.
///
/// - [`MetricClass::Exact`]: fuel, executed wasm instructions and byte counts.
///   They are bit-for-bit reproducible, so any difference is a change.
/// - [`MetricClass::Counter`]: native CPU counters (instructions retired,
///   cycles). They are deterministic per process, but the operating system and
///   the compiler emit a few instructions more or less between builds, so the
///   comparison applies a small relative tolerance.
/// - [`MetricClass::Wall`]: wall-clock milliseconds. They are only comparable
///   when both runs were idle-qualified; otherwise a busy machine, not a code
///   change, can move them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricClass {
    Exact,
    Counter,
    Wall,
}

impl MetricClass {
    /// The class a legacy `deterministic: bool` schema-1 metric maps to.
    #[must_use]
    pub const fn from_deterministic(deterministic: bool) -> Self {
        if deterministic {
            Self::Exact
        } else {
            Self::Wall
        }
    }

    /// A short name for reports and errors.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Counter => "counter",
            Self::Wall => "wall",
        }
    }
}

/// One measured quantity: the spread of its runs, its comparison class and,
/// for wall-clock metrics, whether the run that produced it was idle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "RawMetric")]
pub struct Metric {
    pub min: f64,
    pub median: f64,
    pub max: f64,
    pub runs: usize,
    /// How this metric may be compared between runs.
    pub class: MetricClass,
    /// For [`MetricClass::Wall`], whether every timed window that produced it
    /// was idle-qualified. `None` for exact/counter metrics and for schema-1
    /// results, which never recorded idle evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_qualified: Option<bool>,
    /// Why the metric has no value (a counter the platform cannot read, or a
    /// wall-clock loop skipped in counters mode). Present only when `runs` is
    /// zero; a missing counter is never reported as zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<String>,
}

/// The schema-1 shape, plus the schema-2 class field. Keeping this separate is
/// what lets `compare` read a schema-1 result: `deterministic` is mapped to
/// [`MetricClass::Exact`] or [`MetricClass::Wall`].
#[derive(Deserialize)]
struct RawMetric {
    min: f64,
    median: f64,
    max: f64,
    runs: usize,
    #[serde(default)]
    class: Option<MetricClass>,
    #[serde(default)]
    deterministic: Option<bool>,
    #[serde(default)]
    idle_qualified: Option<bool>,
    #[serde(default)]
    unavailable: Option<String>,
}

impl From<RawMetric> for Metric {
    fn from(raw: RawMetric) -> Self {
        let class = raw
            .class
            .unwrap_or_else(|| MetricClass::from_deterministic(raw.deterministic.unwrap_or(false)));
        Self {
            min: raw.min,
            median: raw.median,
            max: raw.max,
            runs: raw.runs,
            class,
            idle_qualified: raw.idle_qualified,
            unavailable: raw.unavailable,
        }
    }
}

impl Default for Metric {
    /// A metric with no value. Used when reading an older result that predates
    /// a field, so the field is explicitly unavailable rather than zero.
    fn default() -> Self {
        Self::unavailable(MetricClass::Wall, "not present in this result")
    }
}

impl Metric {
    /// Build a metric from samples; an empty slice is an error because a
    /// measurement with no samples is not a measurement.
    ///
    /// # Errors
    ///
    /// Returns an error when `values` is empty or contains a NaN.
    pub fn new(values: &[f64], class: MetricClass) -> Result<Self> {
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
            class,
            idle_qualified: None,
            unavailable: None,
        })
    }

    /// A metric with one sample.
    ///
    /// # Errors
    ///
    /// Returns an error when `value` is NaN.
    pub fn one(value: f64, class: MetricClass) -> Result<Self> {
        Self::new(&[value], class)
    }

    /// A metric with no value, with the reason it could not be measured.
    ///
    /// The zero bounds and `runs: 0` make it impossible to mistake this for a
    /// real reading; reports render `unavailable` instead of a number.
    #[must_use]
    pub fn unavailable(class: MetricClass, reason: impl Into<String>) -> Self {
        Self {
            min: 0.0,
            median: 0.0,
            max: 0.0,
            runs: 0,
            class,
            idle_qualified: None,
            unavailable: Some(reason.into()),
        }
    }

    /// Whether this metric carries a real value.
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.runs > 0 && self.unavailable.is_none()
    }

    /// Record whether the timed window(s) that produced this wall metric were
    /// idle-qualified; exact and counter metrics ignore the flag.
    #[must_use]
    pub fn with_idle_qualified(mut self, qualified: bool) -> Self {
        if self.class == MetricClass::Wall {
            self.idle_qualified = Some(qualified);
        }
        self
    }

    /// Whether two runs may compare this metric. Wall metrics require both
    /// sides to be idle-qualified; exact and counter metrics always may.
    #[must_use]
    pub fn comparable_with(&self, other: &Self) -> bool {
        self.is_available()
            && other.is_available()
            && self.class == other.class
            && (self.class != MetricClass::Wall
                || (self.idle_qualified == Some(true) && other.idle_qualified == Some(true)))
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
    Metric::new(values, MetricClass::Wall).map(|metric| metric.median)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_reports_min_median_max_and_runs() {
        let metric = Metric::new(&[3.0, 1.0, 2.0], MetricClass::Wall).expect("metric");
        assert_eq!(metric.min, 1.0);
        assert_eq!(metric.median, 2.0);
        assert_eq!(metric.max, 3.0);
        assert_eq!(metric.runs, 3);
        assert_eq!(metric.class, MetricClass::Wall);
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
        let baseline = Metric::new(&[10.0, 12.0, 14.0], MetricClass::Wall).expect("baseline");
        let inside = Metric::new(&[11.0, 13.0, 15.0], MetricClass::Wall).expect("inside");
        let outside = Metric::new(&[14.5, 15.0, 16.0], MetricClass::Wall).expect("outside");
        assert!(!inside.outside_spread_of(&baseline));
        assert!(outside.outside_spread_of(&baseline));
    }

    #[test]
    fn wall_metrics_need_both_runs_idle_while_exact_ones_do_not() {
        let wall = Metric::one(1.0, MetricClass::Wall)
            .expect("wall")
            .with_idle_qualified(false);
        assert!(!wall.comparable_with(&wall));
        let qualified = wall.clone().with_idle_qualified(true);
        assert!(qualified.comparable_with(&qualified));
        let exact = Metric::one(1.0, MetricClass::Exact).expect("exact");
        assert!(exact.comparable_with(&exact));
        let counter = Metric::one(1.0, MetricClass::Counter).expect("counter");
        assert!(counter.comparable_with(&counter));
    }

    #[test]
    fn unavailable_metrics_are_not_comparable() {
        let metric = Metric::unavailable(MetricClass::Counter, "no perf_event_open");
        assert!(!metric.is_available());
        assert!(!metric.comparable_with(&metric));
        assert_eq!(metric.runs, 0);
    }

    #[test]
    fn schema_one_metrics_map_deterministic_to_exact_or_wall() {
        let exact: Metric = serde_json::from_str(
            r#"{"min":1.0,"median":1.0,"max":1.0,"runs":1,"deterministic":true}"#,
        )
        .expect("schema 1 exact");
        assert_eq!(exact.class, MetricClass::Exact);
        let wall: Metric = serde_json::from_str(
            r#"{"min":2.0,"median":2.0,"max":2.0,"runs":1,"deterministic":false}"#,
        )
        .expect("schema 1 wall");
        assert_eq!(wall.class, MetricClass::Wall);
        assert_eq!(wall.idle_qualified, None);
    }

    #[test]
    fn rejects_empty_and_nan() {
        assert!(Metric::new(&[], MetricClass::Wall).is_err());
        assert!(Metric::new(&[f64::NAN], MetricClass::Wall).is_err());
    }
}
