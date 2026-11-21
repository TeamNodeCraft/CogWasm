// Summation order and precision: naive, Kahan, Neumaier, pairwise, dot product.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn naive(xs: &[f64]) -> f64 {
    let mut s = 0.0;
    for &x in xs {
        s += x;
    }
    s
}

fn kahan(xs: &[f64]) -> f64 {
    let mut s = 0.0;
    let mut c = 0.0;
    for &x in xs {
        let y = x - c;
        let t = s + y;
        c = (t - s) - y;
        s = t;
    }
    s
}

fn neumaier(xs: &[f64]) -> f64 {
    let mut s = 0.0;
    let mut c = 0.0;
    for &x in xs {
        let t = s + x;
        if s.abs() >= x.abs() {
            c += (s - t) + x;
        } else {
            c += (x - t) + s;
        }
        s = t;
    }
    s + c
}

fn pairwise(xs: &[f64]) -> f64 {
    if xs.len() <= 4 {
        return naive(xs);
    }
    let mid = xs.len() / 2;
    pairwise(&xs[..mid]) + pairwise(&xs[mid..])
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    let mut i = 0;
    while i < a.len() && i < b.len() {
        s += a[i] * b[i];
        i += 1;
    }
    s
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut vals: alloc::vec::Vec<f64> = alloc::vec::Vec::with_capacity(64);
    let mut other: alloc::vec::Vec<f64> = alloc::vec::Vec::with_capacity(64);
    for _ in 0..64 {
        let a = ((rng.below(2_000_001) as i64 - 1_000_000) as f64) / 1024.0;
        let b = ((rng.below(2_000_001) as i64 - 1_000_000) as f64) / 777.0;
        vals.push(a);
        other.push(b);
    }
    emit_f64(naive(&vals));
    emit_f64(kahan(&vals));
    emit_f64(neumaier(&vals));
    emit_f64(pairwise(&vals));
    emit_f64(dot(&vals, &other));
    emit_f64(naive(&other) - naive(&vals));
    emit_f64(kahan(&other) * 0.5);

    // f32 accumulation from f64 inputs.
    let mut s32 = 0.0f32;
    let mut c32 = 0.0f32;
    for &x in vals.iter() {
        let x32 = x as f32;
        let y = x32 - c32;
        let t = s32 + y;
        c32 = (t - s32) - y;
        s32 = t;
    }
    emit_f32(s32);
    emit_u32(s32.to_bits());

    // Sum of squares and mean, tiny magnitudes to exercise subnormals.
    let mut sq = 0.0;
    let mut tiny = 0.0;
    for &x in vals.iter() {
        sq += x * x;
        tiny += x * 1e-300;
    }
    emit_f64(sq);
    emit_f64(tiny);
    emit_f64(sq / (vals.len() as f64));

    (naive(&vals).to_bits() ^ kahan(&vals).to_bits().rotate_left(11)) as u64
}
