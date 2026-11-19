#![no_std]

include!("../prelude.rs");

// Generic collections workload: Vec, BTreeMap, BTreeSet, BinaryHeap,
// VecDeque, sorting, iterator adapters, and incremental string building.
// Every input is derived from the seed through Rng.

use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;

fn mix(hash: &mut u64, value: u64) {
    *hash = (*hash ^ value)
        .wrapping_mul(0x100_0000_01b3)
        .rotate_left(9);
}

fn min_max<T: Ord + Copy>(items: &[T]) -> Option<(T, T)> {
    let mut iter = items.iter();
    let first = *iter.next()?;
    let mut lo = first;
    let mut hi = first;
    for &value in iter {
        if value < lo {
            lo = value;
        }
        if value > hi {
            hi = value;
        }
    }
    Some((lo, hi))
}

fn count_at_least<T: Ord>(items: &[T], bound: &T) -> usize {
    let mut total = 0;
    for item in items {
        if item >= bound {
            total += 1;
        }
    }
    total
}

fn median_u64(sorted: &[u64]) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        sorted[mid]
    } else {
        sorted[mid - 1].wrapping_add(sorted[mid]) / 2
    }
}

fn top_k(values: &[u64], k: usize) -> Vec<u64> {
    let mut heap: BinaryHeap<u64> = BinaryHeap::new();
    for &value in values {
        heap.push(value);
    }
    let mut out = Vec::new();
    while out.len() < k {
        match heap.pop() {
            Some(value) => out.push(value),
            None => break,
        }
    }
    out
}

fn sliding_max(values: &[u64], window: usize) -> Vec<u64> {
    let mut out = Vec::new();
    if window == 0 || values.is_empty() {
        return out;
    }
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut index = 0;
    while index < values.len() {
        while let Some(&back) = deque.back() {
            if values[back] <= values[index] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(index);
        if let Some(&front) = deque.front() {
            if front + window <= index {
                deque.pop_front();
            }
        }
        if index + 1 >= window {
            if let Some(&front) = deque.front() {
                out.push(values[front]);
            }
        }
        index += 1;
    }
    out
}

fn word_frequencies(text: &str) -> BTreeMap<String, u64> {
    let mut frequencies: BTreeMap<String, u64> = BTreeMap::new();
    let mut word = String::new();
    for &byte in text.as_bytes() {
        if byte.is_ascii_alphanumeric() {
            word.push(byte as char);
        } else if !word.is_empty() {
            *frequencies.entry(word.clone()).or_insert(0) += 1;
            word.clear();
        }
    }
    if !word.is_empty() {
        *frequencies.entry(word).or_insert(0) += 1;
    }
    frequencies
}

fn push_u64(out: &mut String, mut value: u64) {
    if value == 0 {
        out.push('0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut at = buf.len();
    while value > 0 {
        at -= 1;
        buf[at] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    if let Ok(text) = core::str::from_utf8(&buf[at..]) {
        out.push_str(text);
    }
}

fn build_report(
    seed: u64,
    count: usize,
    lo: u64,
    hi: u64,
    digest: u64,
    text: &str,
) -> String {
    let mut out = String::new();
    out.push_str("seed=");
    push_u64(&mut out, seed);
    out.push_str(" count=");
    push_u64(&mut out, count as u64);
    out.push_str(" min=");
    push_u64(&mut out, lo);
    out.push_str(" max=");
    push_u64(&mut out, hi);
    out.push_str(" digest=");
    push_u64(&mut out, digest);
    out.push_str(" text=");
    out.push_str(text);
    out
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let count = 96 + (seed % 65) as usize;
    let mut values: Vec<u64> = Vec::with_capacity(count);
    let mut index = 0;
    while index < count {
        let raw = rng.next_u64() % 10_000;
        values.push(raw ^ (index as u64).wrapping_mul(17));
        index += 1;
    }

    let mut sorted = values.clone();
    sorted.sort();
    let mut unstable = values.clone();
    unstable.sort_unstable();
    emit_bool(sorted == unstable);

    let mut deduped = sorted.clone();
    deduped.dedup();
    let (lo, hi) = min_max(&sorted).unwrap_or((0, 0));
    let median = median_u64(&sorted);
    emit_u64(lo);
    emit_u64(hi);
    emit_u64(median);
    emit_u64(count_at_least(&sorted, &median) as u64);
    emit_u64(deduped.len() as u64);

    let mut descending = values.clone();
    descending.sort_unstable_by(|a, b| b.cmp(a));
    emit_bool(descending.first() == sorted.last());

    let target = sorted[count / 2];
    let position = sorted.binary_search(&target).unwrap_or(0);
    emit_u64(position as u64);
    emit_bool(sorted.binary_search(&target).is_ok());

    let filtered = values
        .iter()
        .enumerate()
        .filter(|(index, _)| *index % 3 != 0)
        .map(|(index, &value)| value.wrapping_mul(index as u64 + 1))
        .fold(0u64, |acc, value| acc.wrapping_add(value));
    let mut iter_hash = 0u64;
    mix(&mut iter_hash, filtered);

    let mut zip_hash = 0u64;
    let left = sorted.iter().take(16).copied();
    let right = sorted.iter().rev().take(16).copied();
    for (a, b) in left.zip(right) {
        mix(&mut zip_hash, a ^ b.rotate_left(17));
    }

    let mut chain_hash = 0u64;
    for value in values.iter().take(8).chain(deduped.iter().take(8)) {
        mix(&mut chain_hash, *value);
    }

    let top = top_k(&values, 8);
    for &value in &top {
        emit_u64(value);
    }

    let window = 4 + (seed % 5) as usize;
    let maxima = sliding_max(&values, window);
    let window_sum = maxima
        .iter()
        .fold(0u64, |acc, &value| acc.wrapping_add(value));
    emit_u64(maxima.len() as u64);
    emit_u64(window_sum);

    let mut deque: VecDeque<u64> = VecDeque::new();
    let mut filled = 0;
    while filled < 24 && filled < sorted.len() {
        deque.push_back(sorted[filled]);
        filled += 1;
    }
    let rotate_by = (seed % 7) as usize;
    let mut rotated = 0;
    while rotated < rotate_by {
        if let Some(value) = deque.pop_front() {
            deque.push_back(value);
        }
        rotated += 1;
    }
    let mut deque_hash = 0u64;
    for &value in &deque {
        mix(&mut deque_hash, value);
    }

    let mut set_a: BTreeSet<u64> = BTreeSet::new();
    for &value in sorted.iter().take(48) {
        set_a.insert(value % 101);
    }
    let mut set_b: BTreeSet<u64> = BTreeSet::new();
    for &value in sorted.iter().skip(16).take(48) {
        set_b.insert(value % 67);
    }
    let intersection: Vec<u64> = set_a.intersection(&set_b).copied().collect();
    let union_count = set_a.union(&set_b).count();
    let mut set_hash = 0u64;
    for &value in &intersection {
        mix(&mut set_hash, value);
    }
    mix(&mut set_hash, union_count as u64);

    const WORDS: [&str; 8] = [
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
    ];
    let mut text = String::new();
    let word_count = 24 + (seed % 17) as usize;
    let mut word = 0;
    while word < word_count {
        if word > 0 {
            text.push(' ');
        }
        let pick = (rng.next_u32() as usize) % WORDS.len();
        text.push_str(WORDS[pick]);
        if word % 6 == 0 {
            text.push(' ');
            push_u64(&mut text, rng.below(1000));
        }
        word += 1;
    }
    let frequencies = word_frequencies(&text);
    let mut freq_hash = 0u64;
    for (entry, repetitions) in &frequencies {
        mix(&mut freq_hash, entry.len() as u64);
        for &byte in entry.as_bytes() {
            mix(&mut freq_hash, byte as u64);
        }
        mix(&mut freq_hash, *repetitions);
    }
    emit_u64(frequencies.len() as u64);

    let report = build_report(seed, count, lo, hi, freq_hash ^ set_hash, &text);
    emit_str(&report);

    let mut summary = splitmix64(seed);
    mix(&mut summary, iter_hash);
    mix(&mut summary, zip_hash);
    mix(&mut summary, chain_hash);
    mix(&mut summary, deque_hash);
    mix(&mut summary, freq_hash);
    mix(&mut summary, set_hash);
    mix(&mut summary, window_sum);
    mix(&mut summary, median);
    emit_u64(summary);
    summary
}
