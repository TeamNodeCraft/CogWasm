// Rounding semantics implemented with arithmetic + int casts, because
// floor/ceil/trunc/round are std-only (they need libm) under no_std.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn trunc64(x: f64) -> f64 {
    if !x.is_finite() || x == 0.0 {
        return x;
    }
    if x >= 0.0 {
        (x as u64) as f64
    } else {
        -(((-x) as u64) as f64)
    }
}

fn floor64(x: f64) -> f64 {
    let t = trunc64(x);
    if t > x {
        t - 1.0
    } else {
        t
    }
}

fn ceil64(x: f64) -> f64 {
    let t = trunc64(x);
    if t < x {
        t + 1.0
    } else {
        t
    }
}

fn round_half_away(x: f64) -> f64 {
    let t = trunc64(x);
    let f = x - t;
    if f >= 0.5 {
        t + 1.0
    } else if f <= -0.5 {
        t - 1.0
    } else {
        t
    }
}

fn round_ties_even(x: f64) -> f64 {
    let t = trunc64(x);
    let f = x - t;
    let af = if f < 0.0 { -f } else { f };
    if af < 0.5 {
        t
    } else if af > 0.5 {
        t + if x > 0.0 { 1.0 } else { -1.0 }
    } else {
        // Exactly halfway: choose the even neighbour.
        let ti = t as i64;
        if ti % 2 == 0 {
            t
        } else {
            t + if x > 0.0 { 1.0 } else { -1.0 }
        }
    }
}

fn emit64(x: f64) {
    if x.is_nan() {
        emit_u8(0x7e);
    } else {
        emit_u8(0x00);
        emit_bool(x.is_sign_negative());
        emit_u64(x.to_bits());
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..48u64 {
        let whole = (rng.below(2_000_001) as i64 - 1_000_000) as f64;
        let frac = (rng.below(1001) as f64) / 1000.0;
        let mut x = whole + frac;
        if i & 1 == 1 {
            x = -x;
        }
        // Force exact halves and quarters regularly.
        if i % 8 == 0 {
            x = (rng.below(21) as f64 - 10.0) / 2.0;
        }
        if i % 8 == 4 {
            x = (rng.below(21) as f64 - 10.0) / 4.0;
        }

        emit64(trunc64(x));
        emit64(floor64(x));
        emit64(ceil64(x));
        emit64(round_half_away(x));
        emit64(round_ties_even(x));
        emit64(-trunc64(-x));
        emit_i64(x as i64);
        emit_i64(floor64(x) as i64);
        emit_i64(ceil64(x) as i64);

        // A few exact knowns.
        emit64(floor64(-0.5));
        emit64(ceil64(-0.5));
        emit64(round_ties_even(0.5));
        emit64(round_ties_even(1.5));
        emit64(round_ties_even(2.5));
        emit64(round_ties_even(-1.5));
        emit64(round_ties_even(-2.5));
        emit64(round_half_away(2.5));
        emit64(round_half_away(-2.5));

        acc = acc
            .wrapping_add(trunc64(x).to_bits())
            .wrapping_add(round_ties_even(x).to_bits().rotate_left(13));
    }
    emit_u64(acc);
    acc
}
