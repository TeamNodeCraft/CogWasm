// Division/remainder sign semantics and the MIN / -1 edges (wrapping forms).
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for _ in 0..24 {
        let x = rng.next_u64();
        let y = rng.next_u64() | 1;

        emit_u64(x / y);
        emit_u64(x % y);
        emit_u64(x.wrapping_div(y));
        emit_u64(x.wrapping_rem(y));
        emit_bool(x.checked_div(y).is_some());

        let sx = x as i64;
        let sy = (y as i64) | 1;
        emit_i64(sx / sy);
        emit_i64(sx % sy);
        emit_i64(sx.wrapping_div(sy));
        emit_i64(sx.wrapping_rem(sy));
        emit_i64(sx.div_euclid(sy));
        emit_i64(sx.rem_euclid(sy));

        // Negative variants exercise rounding toward zero vs floor.
        let nx = -sx;
        emit_i64(nx / sy);
        emit_i64(nx % sy);
        emit_i64(nx.div_euclid(sy));
        emit_i64(nx.rem_euclid(sy));

        let a32 = x as i32;
        let b32 = (y as i32) | 1;
        emit_i32(a32.wrapping_div(b32));
        emit_i32(a32.wrapping_rem(b32));
        emit_i32(a32.div_euclid(b32));
        emit_i32(a32.rem_euclid(b32));

        emit_u8((x as u8) / ((y as u8) | 1));
        emit_i8((x as i8).wrapping_div((y as i8) | 1));
        emit_u16((x as u16) % ((y as u16) | 1));

        // const edges
        emit_i32(i32::MIN.wrapping_div(-1));
        emit_i32(i32::MIN.wrapping_rem(-1));
        emit_i64(i64::MIN.wrapping_div(-1));
        emit_i64(i64::MIN.wrapping_rem(-1));
        emit_i32(i32::MIN.div_euclid(1));
        emit_i32(i32::MAX.rem_euclid(-3));

        acc ^= (sx.wrapping_div(sy) as u64).rotate_left((x & 63) as u32);
    }
    emit_u64(acc);
    acc
}
