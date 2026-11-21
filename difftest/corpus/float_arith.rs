// f32/f64 arithmetic over finite values, subnormals, infinities and NaNs.
// NaN results are folded to a canonical marker: Rust/the wasm spec do not
// guarantee NaN payload bits, so only is_nan is compared (see FINDINGS.md).
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn regular64(rng: &mut Rng) -> f64 {
    let sign = (rng.next_u64() & 1) << 63;
    let exp = 1023i64 + (rng.below(61) as i64 - 30);
    let mant = rng.next_u64() & 0x000f_ffff_ffff_ffff;
    f64::from_bits(sign | ((exp as u64) << 52) | mant)
}

fn regular32(rng: &mut Rng) -> f32 {
    let sign = ((rng.next_u64() & 1) as u32) << 31;
    let exp = 127i32 + (rng.below(41) as i32 - 20);
    let mant = (rng.next_u64() as u32) & 0x007f_ffff;
    f32::from_bits(sign | ((exp as u32) << 23) | mant)
}

fn emit64(x: f64) {
    if x.is_nan() {
        emit_u8(0x7e);
        emit_bool(true);
    } else {
        emit_u8(0x00);
        emit_bool(x.is_sign_negative() && x == 0.0);
        emit_u64(x.to_bits());
    }
}

fn emit32(x: f32) {
    if x.is_nan() {
        emit_u8(0x7e);
        emit_bool(true);
    } else {
        emit_u8(0x00);
        emit_bool(x.is_sign_negative() && x == 0.0);
        emit_u32(x.to_bits());
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..32u64 {
        let x = regular64(&mut rng);
        let y = regular64(&mut rng);
        emit64(x + y);
        emit64(x - y);
        emit64(x * y);
        emit64(x / y);
        emit64(x % y);
        emit64(-x);
        emit64(x * 2.0);
        emit64(x / 2.0);
        emit_bool(x < y);
        emit_bool(x <= y);
        emit_bool(x == y);
        emit64(x.min(y));
        emit64(x.max(y));

        let a = regular32(&mut rng);
        let b = regular32(&mut rng);
        emit32(a + b);
        emit32(a - b);
        emit32(a * b);
        emit32(a / b);
        emit32(a % b);
        emit32(-a);
        emit_bool(a < b);
        emit32(a.min(b));
        emit32(a.max(b));

        // Cross-width arithmetic.
        emit64(a as f64 + x);
        emit32(x as f32 + a);

        if i % 4 == 0 {
            // Explicit non-finite inputs: inf arithmetic is fully specified.
            let inf = f64::INFINITY;
            let neg = f64::NEG_INFINITY;
            emit64(inf + 1.0);
            emit64(inf - inf); // NaN -> marker
            emit64(inf * 0.0); // NaN -> marker
            emit64(1.0 / 0.0);
            emit64(-1.0 / 0.0);
            emit64(0.0 / 0.0); // NaN -> marker
            emit64(neg.min(1.0));
            emit64(inf.max(1.0));
            emit_bool(inf > f64::MAX);
            emit_bool(neg < f64::MIN);
            emit64(x % inf);
        }
        acc = acc
            .wrapping_add((x + y).to_bits().rotate_left((i & 63) as u32))
            .wrapping_add((a * b).to_bits() as u64);
    }
    emit_u64(acc);
    acc
}
