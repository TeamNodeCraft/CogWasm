// NaN/zero/infinity *property* tests. Rust does not guarantee NaN payload bits,
// so this file compares is_nan/sign/classification and explicit bit patterns,
// never the payload of a computed NaN. `min`/`max` are documented to return the
// non-NaN operand, which this exercises.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use core::num::FpCategory;

fn class(x: f64) -> u8 {
    match x.classify() {
        FpCategory::Nan => 0,
        FpCategory::Infinite => 1,
        FpCategory::Zero => 2,
        FpCategory::Subnormal => 3,
        FpCategory::Normal => 4,
    }
}

fn emit_contract(x: f64) {
    if x.is_nan() {
        emit_u8(0x7e);
        emit_bool(x.is_nan());
        emit_u8(class(x));
    } else {
        emit_u8(0x00);
        emit_u64(x.to_bits());
        emit_u8(class(x));
    }
}

fn body(seed: u64) -> u64 {
    let mut acc = 0u64;
    let nan = f64::NAN;
    let neg_nan = f64::from_bits(0xfff8_0000_0000_0001);
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;

    // Explicit bit patterns round-trip exactly (bitwise guarantee).
    for &bits in &[
        0x7ff8_0000_0000_0000u64,
        0xfff8_0000_0000_0001,
        0x7ff0_0000_0000_0001,
        0x7ff8_dead_beef_cafe,
    ] {
        let x = f64::from_bits(bits);
        emit_u64(x.to_bits());
        emit_bool(x.is_nan());
        emit_bool(x.is_sign_negative());
        emit_bool(x.is_infinite());
    }

    // min/max ignore a NaN operand (guaranteed by Rust).
    emit_contract(nan.min(1.5));
    emit_contract(nan.max(1.5));
    emit_contract(1.5f64.min(nan));
    emit_contract(1.5f64.max(nan));
    emit_contract(neg_nan.min(-2.5));
    emit_contract(neg_nan.max(-2.5));
    emit_contract(nan.min(nan));
    emit_contract(nan.max(nan));
    emit_contract(inf.min(1.0));
    emit_contract(inf.max(1.0));
    emit_contract(neg_inf.min(1.0));
    emit_contract(neg_inf.max(1.0));

    // NaN comparisons are all false except !=.
    emit_bool(nan == nan);
    emit_bool(nan != nan);
    emit_bool(nan < 1.0);
    emit_bool(nan > 1.0);
    emit_bool(nan <= nan);
    emit_bool(nan >= nan);

    // total_cmp gives a total order over NaNs using their bits.
    emit_bool(nan.total_cmp(&nan).is_eq());
    emit_bool(f64::from_bits(0x7ff8_0000_0000_0001).total_cmp(&nan).is_gt());
    emit_bool(neg_nan.total_cmp(&nan).is_lt());
    emit_bool(1.0f64.total_cmp(&nan).is_lt());

    // abs/signum/copysign signs on zeros and infinities.
    let pz = 0.0f64;
    let nz = -0.0f64;
    emit_u64(pz.abs().to_bits());
    emit_u64(nz.abs().to_bits());
    emit_bool(nz.is_sign_negative());
    emit_bool(nz.abs().is_sign_negative());
    emit_contract(pz.signum());
    emit_contract(nz.signum());
    emit_contract(inf.signum());
    emit_contract(neg_inf.signum());
    emit_contract(1.5f64.copysign(-1.0));
    emit_contract(-1.5f64.copysign(1.0));
    emit_contract(nz.copysign(1.0));
    emit_contract(pz.copysign(-1.0));

    // f32 equivalents.
    let nan32 = f32::NAN;
    emit_u32(nan32.to_bits());
    emit_bool(nan32.is_nan());
    emit_bool(nan32 == nan32);
    emit_bool(nan32 != nan32);
    emit_u32(nan32.min(1.0).to_bits());
    emit_u32(nan32.max(1.0).to_bits());
    emit_u32(1.0f32.min(nan32).to_bits());
    emit_u32((-0.0f32).to_bits());
    emit_bool((-0.0f32).is_sign_negative());
    emit_u32((-0.0f32).abs().to_bits());

    // Arithmetic that *produces* NaN: only is_nan is compared.
    emit_contract(inf - inf);
    emit_contract(0.0 / 0.0);
    emit_contract(inf * 0.0);
    emit_contract((inf - inf).min(3.0f64));
    emit_contract((0.0f64 / 0.0f64).max(3.0));

    acc = acc
        .wrapping_add(nan.to_bits())
        .wrapping_add(inf.to_bits())
        .wrapping_add(pz.to_bits());
    emit_u64(acc);
    acc
}
