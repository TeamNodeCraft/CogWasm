// Q32.32 fixed-point arithmetic with i64/i128 intermediates and an integer
// Newton sqrt. Exercises 64x64->128 multiply and 128/64 division paths.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

const ONE: i64 = 1 << 32;

fn fx_mul(a: i64, b: i64) -> i64 {
    ((a as i128 * b as i128) >> 32) as i64
}

fn fx_div(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 0;
    }
    (((a as i128) << 32) as i64) / b
}

fn fx_from_int(x: i64) -> i64 {
    x << 32
}

fn fx_to_int(x: i64) -> i64 {
    x >> 32
}

fn fx_frac(x: i64) -> u32 {
    (x as u64 & 0xffff_ffff) as u32
}

fn fx_sqrt(x: i64) -> i64 {
    if x <= 0 {
        return 0;
    }
    // Integer sqrt of x<<32 gives sqrt(x)*2^32.
    let v: u128 = (x as u128) << 32;
    let mut g: u128 = v;
    let mut next = (g + 1) / 2;
    while next < g {
        g = next;
        next = (g + v / g) / 2;
    }
    g as i64
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0i64;
    for _ in 0..24 {
        let ai = (rng.next_u64() % 2001) as i64 - 1000;
        let bi = (rng.next_u64() % 2001) as i64 - 1000;
        let a = fx_from_int(ai) + (rng.next_u64() % 1000) as i64 - 500;
        let b = fx_from_int(bi) + (rng.next_u64() % 1000) as i64 - 500;
        emit_i64(a);
        emit_i64(b);
        emit_i64(fx_mul(a, b));
        emit_i64(fx_div(a, b));
        emit_i64(fx_to_int(a));
        emit_u32(fx_frac(a));
        emit_i64(fx_sqrt(a.abs()));
        emit_i64(fx_from_int(ai).wrapping_mul(fx_from_int(bi)));
        acc = acc.wrapping_add(fx_mul(a, b)).wrapping_add(fx_div(a, b));
    }
    // Constants.
    for (a, b) in [
        (ONE, ONE),
        (ONE / 2, ONE / 3),
        (-ONE, ONE / 7),
        (ONE * 3, ONE / 4),
        (1 << 40, 1 << 20),
    ] {
        emit_i64(fx_mul(a, b));
        emit_i64(fx_div(a, b));
    }
    emit_i64(fx_sqrt(ONE * 2));
    emit_i64(fx_sqrt(ONE * 9 / 4));
    emit_i64(fx_sqrt(ONE / 4));
    emit_i64(fx_sqrt(0));
    emit_i64(acc);
    acc as u64
}
