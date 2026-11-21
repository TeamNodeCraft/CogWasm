// Bitwise operations, rotates, byte swaps and bit counts.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for _ in 0..20 {
        let x = rng.next_u64();
        let y = rng.next_u64() | 1;
        let n = (rng.next_u64() & 63) as u32;

        emit_u64(x & y);
        emit_u64(x | y);
        emit_u64(x ^ y);
        emit_u64(!x);
        emit_u64(x.rotate_left(n));
        emit_u64(x.rotate_right(n));
        emit_u32(x.count_ones());
        emit_u32(x.leading_zeros());
        emit_u32(x.trailing_zeros());
        emit_u64(x.swap_bytes());
        emit_u64(x.reverse_bits());
        emit_bool(x.is_power_of_two());
        emit_u64(x.checked_next_power_of_two().unwrap_or(0));

        let x32 = x as u32;
        emit_u32(x32.rotate_left((n & 31) + 0));
        emit_u32(x32.count_ones());
        emit_u8((x as u8).reverse_bits());
        emit_u16((x as u16).swap_bytes());
        emit_u32(x32.reverse_bits());

        let x128 = ((x as u128) << 64) | y as u128;
        emit_u128(x128.rotate_left(n));
        emit_u32(x128.count_ones());
        emit_u32(x128.leading_zeros());
        emit_u32(x128.trailing_zeros());

        emit_u64(u64::from_le(x));
        emit_u64(u64::from_be(x));
        emit_u32(u32::from_le_bytes([x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8]));
        emit_u32(u32::from_be_bytes([x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8]));
        emit_u64(u64::from_le_bytes((x ^ y).to_le_bytes()));
        emit_u64(u64::from_be_bytes((x ^ y).to_be_bytes()));
        emit_u64(u64::from_ne_bytes(x.to_ne_bytes()));

        acc ^= x.rotate_left(n) ^ y.swap_bytes().rotate_right(n);
    }
    emit_u64(acc);
    acc
}
