// Smoke test: arithmetic, allocation, output buffer.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    let mut v: alloc::vec::Vec<u64> = alloc::vec::Vec::new();
    for i in 0..64u64 {
        let x = rng.next_u64() ^ i.wrapping_mul(seed);
        v.push(x);
        acc = acc.wrapping_add(x.rotate_left((i & 63) as u32));
    }
    v.sort_unstable();
    for &x in &v {
        emit_u64(x);
    }
    let s: u64 = v.iter().fold(0u64, |a, &b| a.wrapping_add(b));
    emit_u64(acc);
    acc ^ s
}
