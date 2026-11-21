// Float <-> integer conversions, f64 <-> f32 narrowing/widening, and the
// saturating `as` cast rules (NaN -> 0, out-of-range -> MIN/MAX).
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn emit64(x: f64) {
    if x.is_nan() {
        emit_u8(0x7e);
    } else {
        emit_u8(0x00);
        emit_u64(x.to_bits());
    }
}

fn emit32(x: f32) {
    if x.is_nan() {
        emit_u8(0x7e);
    } else {
        emit_u8(0x00);
        emit_u32(x.to_bits());
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for _ in 0..20 {
        // Values with widely different magnitudes plus exact halves.
        let x = (rng.next_u64() as f64 / 1.7e19) * (rng.below(2) as f64 * 2.0 - 1.0);
        let y = (rng.next_u64() as f64) * 2.0 + 0.5;
        let z = -((rng.next_u64() as f64) * 3.0 + 0.5);
        let f = x + y;

        emit_i8(f as i8);
        emit_u8(f as u8);
        emit_i16(f as i16);
        emit_u16(f as u16);
        emit_i32(f as i32);
        emit_u32(f as u32);
        emit_i64(f as i64);
        emit_u64(f as u64);
        emit_i128(f as i128);
        emit_u128(f as u128);

        emit_i32(z as i32);
        emit_i64(z as i64);
        emit_u64(z as u64);
        emit_i128(z as i128);

        // Integer -> float, including rounding of values above 2^53.
        let u = rng.next_u64();
        let s = rng.next_u64() as i64;
        let u128v = ((rng.next_u64() as u128) << 64) | rng.next_u64() as u128;
        let s128v = u128v as i128;
        emit64(u as f64);
        emit64(s as f64);
        emit64(u128v as f64);
        emit64(s128v as f64);
        emit32(u as f32);
        emit32(s as f32);

        // f64 -> f32 rounding (round-to-nearest-even) and back.
        let a = regular64(&mut rng);
        emit32(a as f32);
        emit64(a as f32 as f64);
        emit64(f64::from(a as f32));
        emit32(a as f32);

        // Explicit edges.
        emit_i32(f64::NAN as i32);
        emit_u32(f64::NAN as u32);
        emit_i64(f64::INFINITY as i64);
        emit_i64(f64::NEG_INFINITY as i64);
        emit_u64(f64::INFINITY as u64);
        emit_i128(f64::INFINITY as i128);
        emit_u128(f64::NEG_INFINITY as u128);
        emit_i32(2147483647.9f64 as i32);
        emit_i32(-2147483648.9f64 as i32);
        emit_u8(300.0f64 as u8);
        emit_i8(-300.0f64 as i8);
        emit_u64(18446744073709551615.0f64 as u64);
        emit_i64(9223372036854775807.0f64 as i64);
        emit_i64(-9223372036854775808.0f64 as i64);

        // f32 edges.
        emit_i32(f32::NAN as i32);
        emit_i32(f32::INFINITY as i32);
        emit_u32(f32::NEG_INFINITY as u32);
        emit_i8(127.9f32 as i8);
        emit_u16(65535.9f32 as u16);

        acc = acc.wrapping_add((f as i64) as u64) ^ (u as f64).to_bits();
    }
    emit_u64(acc);
    acc
}

fn regular64(rng: &mut Rng) -> f64 {
    let sign = (rng.next_u64() & 1) << 63;
    let exp = 1023i64 + (rng.below(61) as i64 - 30);
    let mant = rng.next_u64() & 0x000f_ffff_ffff_ffff;
    f64::from_bits(sign | ((exp as u64) << 52) | mant)
}
