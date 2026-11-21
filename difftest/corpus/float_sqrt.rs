// Software sqrt (Newton) and sin (range reduction + Taylor): dense f64
// multiply/divide/add/sub sequences whose bit results are fully specified.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn sqrt_newton(x: f64) -> f64 {
    if x <= 0.0 {
        return x;
    }
    let mut g = if x > 1.0 { x } else { 1.0 };
    let mut i = 0;
    while i < 64 {
        g = 0.5 * (g + x / g);
        i += 1;
    }
    g
}

fn sin_taylor(mut x: f64) -> f64 {
    const TWO_PI: f64 = 6.283_185_307_179_586_476_925_286_766_559;
    const PI: f64 = 3.141_592_653_589_793_238_462_643_383_279;
    while x > PI {
        x -= TWO_PI;
    }
    while x < -PI {
        x += TWO_PI;
    }
    let x2 = x * x;
    let mut term = x;
    let mut sum = x;
    let mut n: i64 = 1;
    while n < 14 {
        let d = ((2 * n) as f64) * ((2 * n + 1) as f64);
        term = term * (-x2) / d;
        sum += term;
        n += 1;
    }
    sum
}

fn cos_taylor(mut x: f64) -> f64 {
    const TWO_PI: f64 = 6.283_185_307_179_586_476_925_286_766_559;
    const PI: f64 = 3.141_592_653_589_793_238_462_643_383_279;
    while x > PI {
        x -= TWO_PI;
    }
    while x < -PI {
        x += TWO_PI;
    }
    let x2 = x * x;
    let mut term = 1.0;
    let mut sum = 1.0;
    let mut n: i64 = 1;
    while n < 14 {
        let d = ((2 * n - 1) as f64) * ((2 * n) as f64);
        term = term * (-x2) / d;
        sum += term;
        n += 1;
    }
    sum
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for _ in 0..16 {
        let x = (rng.below(10_000_000) as f64) / 1000.0 + 0.000_001;
        let s = sqrt_newton(x);
        emit_f64(s);
        emit_f64(s * s);
        emit_f64(s / x);
        emit_bool(s >= 0.0);

        let y = ((rng.below(20_000) as f64) / 1000.0) - 10.0;
        emit_f64(sin_taylor(y));
        emit_f64(cos_taylor(y));
        emit_f64(sin_taylor(y) * sin_taylor(y) + cos_taylor(y) * cos_taylor(y));
        emit_f32(sin_taylor(y) as f32);
        emit_f32(cos_taylor(y) as f32);

        acc = acc
            .wrapping_add(s.to_bits().rotate_left((x as u64 & 63) as u32))
            .wrapping_add(y.to_bits());
    }
    emit_u64(acc);
    acc
}
