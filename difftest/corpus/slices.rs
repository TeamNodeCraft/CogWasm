// Slice algorithms: sort, binary search, windows/chunks, rotate, split, copy.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::vec::Vec;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut v: Vec<i64> = (0..128).map(|_| rng.next_u64() as i64).collect();
    v.sort_unstable();

    for w in v.windows(3).take(8) {
        emit_i64(w[0] ^ w[1] ^ w[2]);
    }
    for c in v.chunks(16) {
        emit_i64(c.iter().fold(0i64, |a, &b| a.wrapping_add(b)));
    }
    for c in v.chunks_exact(16).take(3) {
        emit_i64(c[c.len() - 1]);
    }

    let key = v[64];
    match v.binary_search(&key) {
        Ok(i) => emit_usize(i),
        Err(i) => emit_usize(i),
    }
    emit_bool(v.binary_search(&-1).is_err());

    v.rotate_left(7);
    v.reverse();
    v.rotate_right(3);
    let (a, b) = v.split_at(32);
    emit_i64(a.iter().copied().max().unwrap_or(0));
    emit_i64(b.iter().copied().min().unwrap_or(0));
    emit_bool(a.iter().all(|&x| x != i64::MIN));

    let mut arr = [0i64; 64];
    arr.copy_from_slice(&v[..64]);
    arr.fill(7);
    emit_i64(arr.iter().sum());
    arr[..16].sort_unstable_by(|x, y| y.cmp(x));
    emit_i64(arr[0]);
    emit_i64(arr[63]);

    v.sort_unstable_by_key(|x| x.wrapping_mul(0x9e37).wrapping_abs() % 97);
    emit_i64(v[0] ^ v[127]);
    v.dedup();
    emit_usize(v.len());

    // split_first / split_last / strip_prefix-like patterns.
    let mut rest = &v[..];
    let mut acc = 0i64;
    while let Some((first, tail)) = rest.split_first() {
        acc = acc.wrapping_add(*first);
        rest = tail;
        if rest.len() < 100 {
            break;
        }
    }
    emit_i64(acc);
    emit_u64(v.len() as u64);
    v[0] as u64 ^ v[v.len() - 1] as u64
}
