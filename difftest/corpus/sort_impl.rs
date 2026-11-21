// Hand-written sorting algorithms and comparisons against core's sort.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::vec::Vec;

fn insertion(a: &mut [u64]) {
    let mut i = 1;
    while i < a.len() {
        let key = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > key {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = key;
        i += 1;
    }
}

fn partition(a: &mut [u64]) -> usize {
    let pivot = a[a.len() - 1];
    let mut i = 0;
    let mut j = 0;
    while j + 1 < a.len() {
        if a[j] <= pivot {
            a.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    a.swap(i, a.len() - 1);
    i
}

fn quicksort(a: &mut [u64]) {
    if a.len() <= 8 {
        insertion(a);
        return;
    }
    let p = partition(a);
    let (left, right) = a.split_at_mut(p);
    quicksort(left);
    if right.len() > 1 {
        quicksort(&mut right[1..]);
    }
}

fn merge_sort(a: &[u64]) -> Vec<u64> {
    if a.len() <= 1 {
        return a.to_vec();
    }
    let mid = a.len() / 2;
    let l = merge_sort(&a[..mid]);
    let r = merge_sort(&a[mid..]);
    let mut out = Vec::with_capacity(a.len());
    let (mut i, mut j) = (0, 0);
    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            out.push(l[i]);
            i += 1;
        } else {
            out.push(r[j]);
            j += 1;
        }
    }
    out.extend_from_slice(&l[i..]);
    out.extend_from_slice(&r[j..]);
    out
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut v: Vec<u64> = (0..256).map(|_| rng.next_u64() % 100_000).collect();
    insertion(&mut v);
    for i in (0..v.len()).step_by(32) {
        emit_u64(v[i]);
    }

    let mut w: Vec<u64> = (0..256).map(|_| rng.next_u64() % 100_000).collect();
    quicksort(&mut w);
    emit_bool(v == w);

    let src: Vec<u64> = (0..128).map(|_| rng.next_u64() % 50_000).collect();
    let m = merge_sort(&src);
    let mut expected = src.clone();
    expected.sort_unstable();
    emit_bool(m == expected);
    emit_u64(m[0] ^ m[m.len() - 1]);

    let mut by_key: Vec<(u64, u64)> = (0..64)
        .map(|_| (rng.next_u64(), rng.next_u64() % 1000))
        .collect();
    by_key.sort_unstable_by_key(|&(_, k)| k);
    by_key.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    emit_u64(by_key[0].1);
    emit_u64(by_key[63].1);

    // Already-sorted and reverse-sorted inputs.
    let mut inc: Vec<u32> = (0..1000).collect();
    let mut dec: Vec<u32> = (0..1000).rev().collect();
    inc.sort_unstable();
    dec.sort_unstable();
    emit_bool(inc == dec);
    emit_u32(inc[999]);

    v[0] ^ w[255] ^ m[64] ^ by_key[32].0
}
