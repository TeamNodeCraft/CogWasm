// Force many memory.grow pages with a bump allocator: thousands of small
// allocations plus a few large ones, then checksum every byte.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::string::String;
use alloc::vec::Vec;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;

    let mut chunks: Vec<Vec<u8>> = Vec::new();
    for i in 0..512u64 {
        let mut c: Vec<u8> = Vec::with_capacity(4096);
        let fill = (rng.next_u64() ^ i) as u8;
        for j in 0..4096 {
            c.push(fill.wrapping_add((j as u8).wrapping_mul(3)));
        }
        acc = acc.wrapping_add(c[0] as u64).wrapping_add(c[4095] as u64);
        chunks.push(c);
    }
    emit_usize(chunks.len());
    emit_usize(chunks.iter().map(|c| c.len()).sum());

    let mut total = 0u64;
    for (i, c) in chunks.iter().enumerate() {
        let mut h = i as u64;
        for &b in c.iter() {
            h = h.wrapping_mul(31).wrapping_add(b as u64);
        }
        total ^= h.rotate_left((i & 63) as u32);
    }
    emit_u64(total);

    // A single large allocation (1 MiB) and a long String.
    let mut big: Vec<u64> = Vec::with_capacity(131_072);
    for i in 0..131_072u64 {
        big.push(i.wrapping_mul(seed));
    }
    emit_u64(big[0] ^ big[131_071]);
    emit_u64(big.iter().fold(0u64, |a, &b| a.wrapping_add(b.rotate_left(7))));

    let mut s = String::new();
    for i in 0..4000u64 {
        let _ = core::fmt::Write::write_fmt(&mut s, format_args!("{i:04},"));
    }
    emit_usize(s.len());
    emit_u8(s.as_bytes()[0] ^ s.as_bytes()[s.len() - 1]);
    let mut h = 0u64;
    for &b in s.as_bytes() {
        h = h.wrapping_mul(131).wrapping_add(b as u64);
    }
    emit_u64(h);

    // Keep everything alive until here so the bump allocator really grows.
    acc ^ total ^ h ^ chunks.len() as u64
}
