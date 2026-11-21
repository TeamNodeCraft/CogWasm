// saturating_* across signed and unsigned widths.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..20u64 {
        let x = rng.next_u64();
        let y = rng.next_u64() | 1;
        emit_u64(x.saturating_add(y));
        emit_u64(x.saturating_sub(y));
        emit_u64(x.saturating_mul(y));
        emit_u64(x.saturating_pow((y & 7) as u32));

        let sx = x as i64;
        let sy = (y as i64) | 1;
        emit_i64(sx.saturating_add(sy));
        emit_i64(sx.saturating_sub(sy));
        emit_i64(sx.saturating_mul(sy));
        emit_i64(sx.saturating_neg());
        emit_i64(sx.saturating_abs());

        let a8 = x as i8;
        let b8 = (y as i8) | 1;
        emit_i8(a8.saturating_add(b8));
        emit_i8(a8.saturating_sub(b8));
        emit_i8(a8.saturating_mul(b8));
        emit_u8((x as u8).saturating_add(y as u8));

        let a16 = x as i16;
        emit_i16(a16.saturating_mul((y as i16) | 1));
        let a32 = x as i32;
        emit_i32(a32.saturating_mul((y as i32) | 1));

        emit_i128((x as i128).saturating_mul((y as i128) | 1));
        emit_u128((x as u128).saturating_mul(y as u128));

        // Deliberate edges.
        emit_i8(i8::MAX.saturating_add(1));
        emit_i8(i8::MIN.saturating_sub(1));
        emit_u8(u8::MAX.saturating_add(1));
        emit_i64(i64::MIN.saturating_neg());
        emit_i64(i64::MAX.saturating_mul(2));
        emit_u64(u64::MAX.saturating_mul(2));

        acc = acc.wrapping_add(x.rotate_left((i & 63) as u32) ^ y);
    }
    emit_u64(acc);
    acc
}
