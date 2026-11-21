// i128/u128 division, remainder and the MIN / -1 edge.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u128;
    for _ in 0..16 {
        // Keep magnitudes moderate so div_euclid/rem_euclid cannot overflow.
        let a: u128 = ((rng.next_u64() as u128) << 32) | (rng.next_u64() as u128 & 0xffff_ffff);
        let b: u128 = (rng.next_u64() as u128 | 1) & 0xffff_ffff_ffff;
        emit_u128(a / b);
        emit_u128(a % b);
        emit_u128(a.div_euclid(b));
        emit_u128(a.rem_euclid(b));
        emit_bool(a.checked_div(b).is_some());

        let sa = a as i128;
        let sb = (b as i128) | 1;
        emit_i128(sa / sb);
        emit_i128(sa % sb);
        emit_i128(sa.div_euclid(sb));
        emit_i128(sa.rem_euclid(sb));
        emit_i128((-sa) / sb);
        emit_i128((-sa) % sb);
        emit_bool(sa.checked_div(sb).is_some());

        // const edges
        emit_i128(i128::MIN.wrapping_div(-1));
        emit_i128(i128::MIN.wrapping_rem(-1));
        emit_i128(i128::MAX / -1);
        emit_i128(i128::MAX % -1);
        emit_u128(u128::MAX / 3);
        emit_u128(u128::MAX % 3);

        acc ^= (a / b) << 1;
    }
    emit_u128(acc);
    acc as u64
}
