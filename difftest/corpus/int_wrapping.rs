// Integer wrapping arithmetic across widths, plus overflow flags.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use core::num::Wrapping;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..24u64 {
        let x = rng.next_u64();
        let y = rng.next_u64() | 1;
        let a = x.wrapping_add(y);
        let s = x.wrapping_sub(y);
        let m = x.wrapping_mul(y);
        let n = x.wrapping_neg();
        emit_u64(a);
        emit_u64(s);
        emit_u64(m);
        emit_u64(n);
        emit_bool(x.overflowing_add(y).1);
        emit_bool(x.overflowing_sub(y).1);
        emit_bool(x.overflowing_mul(y).1);

        let x32 = x as u32;
        let y32 = y as u32;
        emit_u32(x32.wrapping_add(y32));
        emit_u32(x32.wrapping_sub(y32));
        emit_u32(x32.wrapping_mul(y32));
        emit_i32((x32 as i32).wrapping_mul(y32 as i32));
        emit_i32((x32 as i32).wrapping_neg());

        let x8 = x as u8;
        let y8 = y as u8;
        emit_u8(x8.wrapping_add(y8));
        emit_u8(x8.wrapping_mul(y8));
        emit_i8((x8 as i8).wrapping_sub(y8 as i8));

        let x16 = x as u16;
        let y16 = y as u16;
        emit_u16(x16.wrapping_mul(y16));
        emit_i16((x16 as i16).wrapping_add(y16 as i16));

        let wx = Wrapping(x);
        let wy = Wrapping(y);
        emit_u64((wx + wy).0);
        emit_u64((wx - wy).0);
        emit_u64((wx * wy).0);
        emit_u64((-wx).0);

        acc = acc
            .wrapping_add(a ^ s)
            .wrapping_add(m.rotate_left((i & 63) as u32))
            .wrapping_add(n ^ i);
    }
    emit_u64(acc);
    acc
}
