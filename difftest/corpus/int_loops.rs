// Straight-line + looped integer arithmetic with data-dependent control flow.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn step(x: u64, y: u64) -> u64 {
    let mut v = x ^ y;
    let mut i = 0u32;
    while i < (y & 15) as u32 {
        v = v
            .wrapping_mul(0x9e37_79b9_7f4a_7c15)
            .wrapping_add((i as u64) << 7)
            .rotate_left(i & 63);
        i += 1;
    }
    v
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    let mut a = rng.next_u64();
    let mut b = rng.next_u64() | 1;
    for i in 0..128u64 {
        let t = step(a, b);
        a = b.wrapping_add(t);
        b = a ^ t.rotate_right((i & 63) as u32);
        match (a ^ b) % 5 {
            0 => acc = acc.wrapping_add(a),
            1 => acc = acc.wrapping_sub(b),
            2 => acc ^= a.rotate_left(17),
            3 => acc = acc.wrapping_mul(3).wrapping_add(b >> 3),
            _ => {
                acc = acc.wrapping_add(step(b, a));
            }
        }
        if i % 8 == 0 {
            emit_u64(a);
            emit_u64(b);
            emit_u64(acc);
        }
    }
    // Nested loop with continue/break and a labelled outer break.
    'outer: for i in 0..16u64 {
        for j in 0..16u64 {
            let x = i.wrapping_mul(j).wrapping_add(acc);
            if x & 7 == 0 {
                continue;
            }
            acc ^= x.rotate_left(((i + j) & 63) as u32);
            if acc & 0xfff == 0 {
                break 'outer;
            }
        }
    }
    emit_u64(acc);
    acc
}
