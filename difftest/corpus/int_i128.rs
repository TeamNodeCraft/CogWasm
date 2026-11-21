// i128 / u128 arithmetic, shifts, multiplication and conversions.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..20u64 {
        let a: u128 = ((rng.next_u64() as u128) << 64) | rng.next_u64() as u128;
        let b: u128 = (((rng.next_u64() as u128) << 64) | rng.next_u64() as u128) | 1;
        emit_u128(a.wrapping_add(b));
        emit_u128(a.wrapping_sub(b));
        emit_u128(a.wrapping_mul(b));
        emit_u128(a.wrapping_neg());
        emit_u128(a >> (b & 127) as u32);
        emit_u128(a << (b & 127) as u32);
        emit_u128(a.rotate_left((b & 127) as u32));
        emit_u32(a.count_ones());
        emit_u32(a.leading_zeros());
        emit_u32(a.trailing_zeros());
        emit_bool(a < b);
        emit_bool(a == a);
        emit_u128(a & b);
        emit_u128(a | b);
        emit_u128(a ^ b);
        emit_u128(!a);

        let sa = a as i128;
        let sb = (b as i128) | 1;
        emit_i128(sa.wrapping_add(sb));
        emit_i128(sa.wrapping_sub(sb));
        emit_i128(sa.wrapping_mul(sb));
        emit_i128(sa.wrapping_neg());
        emit_i128(sa.wrapping_shl((b & 127) as u32));
        emit_i128(sa.wrapping_shr((b & 127) as u32));
        emit_bool(sa < sb);
        emit_bool(sa.checked_add(sb).is_some());
        emit_bool(sa.checked_mul(sb).is_some());

        // Widening multiply by hand: (hi,lo) halves must agree with the full
        // product after recombination.
        let ah = (a >> 64) as u64;
        let al = a as u64;
        emit_u64(ah.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        emit_u64(al.wrapping_mul(0xd6e8_feb8_6659_fd93));
        emit_u64((a.wrapping_mul(b) >> 64) as u64);

        emit_u64(a as u64);
        emit_i64(a as i64);
        emit_u64(sa as u64);
        emit_i64((sa >> 64) as i64);

        acc = acc.wrapping_add((a as u64).rotate_left((i & 63) as u32)) ^ (b as u64);
    }
    emit_u64(acc);
    acc
}
