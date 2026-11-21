// VecDeque, BinaryHeap and BTreeSet: ring buffers and heap ordering.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::collections::{BTreeSet, BinaryHeap, VecDeque};

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut dq: VecDeque<u64> = VecDeque::new();
    for i in 0..64u64 {
        if i & 1 == 0 {
            dq.push_back(rng.next_u64());
        } else {
            dq.push_front(rng.next_u64());
        }
    }
    emit_usize(dq.len());
    emit_u64(dq.front().copied().unwrap_or(0) ^ dq.back().copied().unwrap_or(0));
    dq.rotate_left(9);
    dq.rotate_right(3);
    for _ in 0..8 {
        emit_u64(dq.pop_front().unwrap_or(0));
    }
    for _ in 0..8 {
        emit_u64(dq.pop_back().unwrap_or(0));
    }
    let (a, b) = dq.as_slices();
    emit_usize(a.len());
    emit_usize(b.len());
    emit_u64(dq.iter().fold(0u64, |x, &y| x.wrapping_add(y)));
    dq.make_contiguous();
    emit_usize(dq.as_slices().1.len());

    let mut heap: BinaryHeap<u64> = BinaryHeap::new();
    for _ in 0..48 {
        heap.push(rng.next_u64() % 10_000);
    }
    emit_usize(heap.len());
    emit_u64(*heap.peek().unwrap_or(&0));
    let mut popped = 0u64;
    while let Some(x) = heap.pop() {
        popped = popped.wrapping_mul(31).wrapping_add(x);
    }
    emit_u64(popped);
    emit_usize(heap.len());

    let mut minheap: BinaryHeap<core::cmp::Reverse<u32>> = BinaryHeap::new();
    for i in 0..32u32 {
        minheap.push(core::cmp::Reverse((i * 7) % 32));
    }
    let sorted = minheap.into_sorted_vec();
    for r in sorted.iter().take(8) {
        emit_u32(r.0);
    }

    let mut set: BTreeSet<u32> = BTreeSet::new();
    for _ in 0..100 {
        set.insert(rng.next_u32() % 64);
    }
    emit_usize(set.len());
    emit_u32(*set.first().unwrap_or(&0));
    emit_u32(*set.last().unwrap_or(&0));
    let mut h = 0u64;
    for &x in &set {
        h = h.wrapping_mul(131).wrapping_add(x as u64);
    }
    emit_u64(h);
    emit_usize(set.range(10..30).count());
    set.retain(|&x| x & 1 == 0);
    emit_usize(set.len());
    emit_bool(set.remove(&0));

    h ^ popped ^ dq.len() as u64
}
