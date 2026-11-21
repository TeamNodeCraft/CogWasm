// Shifts with runtime amounts (masked by the language when overflow checks
// are off) and the explicit wrapping/checked forms.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..24u64 {
        let x = rng.next_u64();
        let y = rng.next_u64();
        // Spread of amounts: 0..64 mostly, plus >= width to exercise masking.
        let n8 = (rng.next_u64() & 15) as u32;
        let n16 = (rng.next_u64() & 31) as u32;
        let n32 = (rng.next_u64() & 63) as u32;
        let n64 = (rng.next_u64() & 127) as u32;

        emit_u8((x as u8) << (n8 & 7));
        emit_u8((x as u8) >> (n8 & 7));
        emit_i8((x as i8) >> (n8 & 7));
        emit_u16((x as u16) << (n16 & 15));
        emit_i16((x as i16) >> (n16 & 15));
        emit_u32((x as u32) << (n32 & 31));
        emit_i32((x as i32) >> (n32 & 31));
        emit_u64(x << (n64 & 63));
        emit_i64((y as i64) >> (n64 & 63));

        // Amounts the optimizer cannot treat as in-range.
        let big = (n64 + (i as u32)) & 127;
        emit_u64(x << big);
        emit_u64(x >> big);
        emit_u32((x as u32) << (big & 63));
        emit_u32((x as u32) >> (big & 63));
        emit_i32((x as i32) >> (big & 63));
        emit_u128((((x as u128) << 64) | y as u128) << (big & 127));

        emit_u64(x.wrapping_shl(n64));
        emit_u64(x.wrapping_shr(n64));
        emit_u64(x.checked_shl(n64).unwrap_or(0));
        emit_u64(x.checked_shl(n64 & 63).unwrap_or(0));
        emit_u128((x as u128).wrapping_shl(n64));
        emit_u128((x as u128).wrapping_shr(n64));
        emit_i128((x as i128).wrapping_shl(n64));
        emit_i128((y as i128).wrapping_shr(n64));

        // Logically vs arithmetically shifted negatives.
        let neg = (x as i64) | i64::MIN;
        emit_i64(neg >> (n64 & 63));
        emit_u64((neg as u64) >> (n64 & 63));

        acc = acc
            .wrapping_add(x.wrapping_shl(n64))
            .wrapping_add(x.wrapping_shr(n64));
    }
    emit_u64(acc);
    acc
}
