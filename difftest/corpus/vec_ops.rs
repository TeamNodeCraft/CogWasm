// Vec operations: growth, insert/remove/retain/drain/splice/split_off.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::vec::Vec;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut v: Vec<u64> = Vec::new();
    for i in 0..200u64 {
        v.push(rng.next_u64() ^ i);
    }
    emit_usize(v.len());
    emit_u64(v[0] ^ v[199]);

    v.insert(50, 0xdead_beef);
    v.insert(0, 0xcafe_f00d);
    emit_u64(v.remove(51));
    emit_u64(v.swap_remove(0));
    emit_usize(v.len());

    v.retain(|&x| x & 1 == 0);
    emit_usize(v.len());
    v.resize(64, 0x5555_5555_5555_5555);
    emit_usize(v.len());
    emit_u64(v[63]);
    v.truncate(40);
    emit_usize(v.len());

    let tail = v.split_off(20);
    emit_u64(tail[0] ^ tail[tail.len() - 1]);
    emit_usize(v.len());

    let drained: Vec<u64> = v.drain(5..15).collect();
    emit_u64(drained.iter().fold(0u64, |a, &b| a.wrapping_add(b)));
    emit_usize(v.len());

    v.splice(3..6, [1u64, 2, 3, 4, 5].iter().copied());
    emit_usize(v.len());
    emit_u64(v[3] ^ v[7]);

    v.sort_unstable();
    v.dedup();
    emit_usize(v.len());
    v.dedup_by(|a, b| *a / 1000 == *b / 1000);
    emit_usize(v.len());

    let mut w: Vec<u32> = (0..64).map(|i| (i * 2654435761u32)).collect();
    w.sort_by(|a, b| b.cmp(a));
    emit_u32(w[0]);
    emit_u32(w[63]);
    w.rotate_left(5);
    emit_u32(w[0] ^ w[31]);
    w.append(&mut tail.into_iter().map(|x| x as u32).collect());

    // Nested Vec / String.
    let mut rows: Vec<Vec<u8>> = Vec::new();
    for r in 0..8u8 {
        let mut row = Vec::new();
        for c in 0..8u8 {
            row.push(r.wrapping_mul(c).wrapping_add(seed as u8));
        }
        rows.push(row);
    }
    let total: u32 = rows.iter().flat_map(|r| r.iter()).map(|&x| x as u32).sum();
    emit_u32(total);
    emit_u8(rows[7][7]);

    total as u64 ^ v.len() as u64 ^ w.len() as u64
}
