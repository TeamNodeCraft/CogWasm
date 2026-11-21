// Explicit bit patterns: classification, signs, total_cmp, payload-preserving
// from_bits/to_bits (bitwise-transparent conversions are guaranteed).
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use core::num::FpCategory;

fn class_code(x: f64) -> u8 {
    match x.classify() {
        FpCategory::Nan => 0,
        FpCategory::Infinite => 1,
        FpCategory::Zero => 2,
        FpCategory::Subnormal => 3,
        FpCategory::Normal => 4,
    }
}

fn ord_code(o: core::cmp::Ordering) -> u8 {
    match o {
        core::cmp::Ordering::Less => 0,
        core::cmp::Ordering::Equal => 1,
        core::cmp::Ordering::Greater => 2,
    }
}

const BITS64: [u64; 18] = [
    0x0000_0000_0000_0000, // +0
    0x8000_0000_0000_0000, // -0
    0x0000_0000_0000_0001, // smallest subnormal
    0x8000_0000_0000_0001,
    0x000f_ffff_ffff_ffff, // largest subnormal
    0x0010_0000_0000_0000, // smallest normal
    0x3ff0_0000_0000_0000, // 1.0
    0xbff0_0000_0000_0000, // -1.0
    0x7ff0_0000_0000_0000, // +inf
    0xfff0_0000_0000_0000, // -inf
    0x7ff8_0000_0000_0000, // canonical quiet NaN
    0xfff8_0000_0000_0000, // negative quiet NaN
    0x7ff8_0000_0000_0001, // NaN with payload 1
    0x7ff0_0000_0000_0001, // signalling NaN
    0x7fef_ffff_ffff_ffff, // f64::MAX
    0xffef_ffff_ffff_ffff,
    0x3ff8_0000_0000_0000, // 1.5
    0x4009_21fb_5444_2d18, // pi
];

const BITS32: [u32; 10] = [
    0x0000_0000,
    0x8000_0000,
    0x0000_0001,
    0x007f_ffff,
    0x0080_0000,
    0x3f80_0000,
    0x7f80_0000,
    0xff80_0000,
    0x7fc0_0000,
    0x7fc0_0001,
];

fn body(seed: u64) -> u64 {
    let mut acc = 0u64;
    for &bits in &BITS64 {
        let x = f64::from_bits(bits);
        // from_bits/to_bits round-trip, including NaN payloads, is bitwise.
        emit_u64(x.to_bits());
        emit_u8(class_code(x));
        emit_bool(x.is_nan());
        emit_bool(x.is_infinite());
        emit_bool(x.is_finite());
        emit_bool(x.is_sign_negative());
        emit_bool(x == x);
        emit_bool(x < 0.0);
        emit_bool(x == 0.0);
        let y = f64::from_bits(bits.rotate_left(17));
        emit_u8(ord_code(x.total_cmp(&y)));
        emit_u8(ord_code(y.total_cmp(&x)));
        acc = acc.wrapping_add(x.to_bits().rotate_left((bits & 63) as u32));
    }
    for &bits in &BITS32 {
        let x = f32::from_bits(bits);
        emit_u32(x.to_bits());
        emit_u8(class_code(x as f64));
        emit_bool(x.is_nan());
        emit_bool(x.is_infinite());
        emit_bool(x.is_sign_negative());
        let y = f32::from_bits(bits ^ 0x8000_0001);
        emit_u8(ord_code(x.total_cmp(&y)));
        acc ^= x.to_bits() as u64;
    }

    // Arithmetic on explicit finite values: signs of zeros are observable.
    let mut rng = Rng::new(seed);
    for _ in 0..16 {
        let a = (rng.next_u64() & 0x800f_ffff_ffff_ffff) as f64;
        let b = -(a * 0.0);
        emit_f64(a + b);
        emit_f64(a * -1.0);
        emit_f64(a.abs());
        emit_f64(a.signum());
        emit_f64(a.copysign(-1.0));
        acc = acc.wrapping_add(a.to_bits() ^ b.to_bits());
    }
    emit_u64(acc);
    acc
}
