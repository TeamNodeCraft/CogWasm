// `as` casts between integer widths and the char/bool/usize edges.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for _ in 0..20 {
        let x = rng.next_u64();
        let y = rng.next_u64();

        emit_u8(x as u8);
        emit_i8(x as i8);
        emit_u16(x as u16);
        emit_i16(x as i16);
        emit_u32(x as u32);
        emit_i32(x as i32);
        emit_u64(y as u64);
        emit_i64(y as i64);
        emit_u128(x as u128);
        emit_i128((y as i128) << 3);

        emit_u32((x as i32) as u32);
        emit_i64((x as u32) as i64);
        emit_u64((x as i8) as u64);
        emit_u64((x as i16) as u64);
        emit_i128((x as i64) as i128);

        // Narrowing picks the low bits; widening sign/zero-extends.
        let packed = (x & 0xff) | ((y & 0xff) << 8) | (((x >> 8) & 0xff) << 16);
        emit_u32(packed as u32);
        emit_u64(((packed as u32) as i16) as u64);
        emit_i32(((packed as u32) as i16) as i32);
        emit_u8(((x as i32).wrapping_neg()) as u8);

        emit_bool((x != 0) as bool);
        emit_u8(true as u8);
        emit_u8(false as u8);

        let c = char::from_u32((x as u32) % 0x10ffff).unwrap_or('\u{1f600}');
        emit_u32(c as u32);
        emit_bool(c.is_ascii());
        emit_u32(('A' as u32) + ((x & 31) as u32));

        emit_usize(x as usize);
        emit_u64((x as usize) as u64);
        emit_u64((y as u32 as usize) as u64);

        acc = acc
            .wrapping_add(packed as u64)
            .wrapping_add(c as u64)
            .wrapping_add((x as i32) as u32 as u64);
    }
    emit_u64(acc);
    acc
}
