//! Statistics shared by the hosts.

use anyhow::{Result, bail};

/// The median of `values`, or an error when the slice is empty.
///
/// Even-length slices average the two middle values as
/// `a + (b - a) / 2`, not `(a + b) / 2`. For two large-but-finite timings the
/// second form can overflow to infinity even though the median is finite.
pub fn median(values: &[f64]) -> Result<f64> {
    if values.is_empty() {
        bail!("median of an empty timing slice");
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        Ok(sorted[middle - 1] + (sorted[middle] - sorted[middle - 1]) / 2.0)
    } else {
        Ok(sorted[middle])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_of_an_odd_slice_is_the_middle_value() -> Result<()> {
        assert_eq!(median(&[3.0, 1.0, 2.0])?, 2.0);
        Ok(())
    }

    #[test]
    fn median_of_an_even_slice_averages_the_two_middle_values() -> Result<()> {
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0])?, 2.5);
        Ok(())
    }

    #[test]
    fn median_of_an_empty_slice_is_an_error() {
        assert!(median(&[]).is_err());
    }

    #[test]
    fn median_does_not_overflow_where_the_sum_would() -> Result<()> {
        // `f64::MAX + f64::MAX` is infinity, so `(a + b) / 2` returns infinity
        // for this finite pair. `a + (b - a) / 2` stays at `f64::MAX`.
        let median = median(&[f64::MAX, f64::MAX])?;
        assert!(median.is_finite(), "median overflowed to {median}");
        assert_eq!(median, f64::MAX);
        Ok(())
    }
}
