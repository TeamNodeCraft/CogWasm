//! A.2 (e) maths node, in the benchmark ABI: 100k f64 points transformed with
//! `y = sin(x)*0.5 + cos(x*0.25) + sqrt(|x|)` and reduced to mean/min/max.
//! Uses the `libm` crate (vendored under bench/vendor) exactly like
//! `stage-b/a2/e_math`, so both compilers build the same arithmetic.
#![no_std]

extern crate libm;

include!("../prelude.rs");

#[inline]
fn transform(x: f64) -> f64 {
    libm::sin(x) * 0.5 + libm::cos(x * 0.25) + libm::sqrt(if x < 0.0 { -x } else { x })
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let n = 100_000usize;
    let mut sum = 0.0f64;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for i in 0..n {
        let x = (i as f64 - n as f64 / 2.0) * 0.001 + (rng.next_u32() & 0xff) as f64 * 1e-9;
        let y = transform(x);
        sum += y;
        min = min.min(y);
        max = max.max(y);
    }
    let mean = sum / n as f64;
    emit_f64(mean);
    emit_f64(min);
    emit_f64(max);
    mean.to_bits() ^ min.to_bits().rotate_left(17) ^ max.to_bits().rotate_left(33)
}
