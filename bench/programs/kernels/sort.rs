#![no_std]

include!("../prelude.rs");

use alloc::vec::Vec;

const N: usize = 20_000;

fn sift_down(data: &mut [u64], mut root: usize, end: usize) {
    loop {
        let mut child = root * 2 + 1;
        if child >= end {
            return;
        }
        if child + 1 < end && data[child] < data[child + 1] {
            child += 1;
        }
        if data[root] >= data[child] {
            return;
        }
        data.swap(root, child);
        root = child;
    }
}

/// In-place binary heapsort; no slice::sort is used for the primary result.
fn heapsort(data: &mut [u64]) {
    let n = data.len();
    let mut i = n / 2;
    while i > 0 {
        i -= 1;
        sift_down(data, i, n);
    }
    let mut end = n;
    while end > 1 {
        end -= 1;
        data.swap(0, end);
        sift_down(data, 0, end);
    }
}

fn checksum(data: &[u64]) -> u64 {
    let mut hash = 0x243f_6a88_85a3_08d3u64;
    let mut i = 0;
    while i < data.len() {
        hash = hash.rotate_left(5)
            ^ data[i]
                .wrapping_mul(0x9e37_79b9_7f4a_7c15)
                .wrapping_add(i as u64);
        i += 1;
    }
    hash
}

fn sum(data: &[u64]) -> u64 {
    let mut total = 0u64;
    let mut i = 0;
    while i < data.len() {
        total = total.wrapping_add(data[i]);
        i += 1;
    }
    total
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut data: Vec<u64> = Vec::with_capacity(N);
    let mut i = 0;
    while i < N {
        let word = rng.next_u64();
        // Mix in the index so equal RNG words cannot produce a run of
        // already-ordered input.
        data.push(
            word ^ (i as u64)
                .rotate_left(17)
                .wrapping_mul(0xbf58_476d_1ce4_e5b9),
        );
        i += 1;
    }

    let mut reference = data.clone();
    let before = checksum(&data);
    heapsort(&mut data);
    reference.sort_unstable();
    let equal = data == reference;

    let heap_sum = sum(&data);
    let ref_sum = sum(&reference);
    let heap_hash = checksum(&data);
    let ref_hash = checksum(&reference);

    emit_u64(N as u64);
    emit_u64(before);
    emit_u64(heap_sum);
    emit_u64(ref_sum);
    emit_u64(heap_hash);
    emit_u64(ref_hash);
    emit_bool(equal);

    let mut head = 0;
    while head < 8 {
        emit_u64(data[head]);
        emit_u64(reference[head]);
        head += 1;
    }
    let mut tail = N - 8;
    while tail < N {
        emit_u64(data[tail]);
        emit_u64(reference[tail]);
        tail += 1;
    }

    heap_hash ^ ref_hash.rotate_left(7) ^ heap_sum.rotate_left(23) ^ ref_sum.rotate_left(41)
}
