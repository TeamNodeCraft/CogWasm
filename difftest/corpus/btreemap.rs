// BTreeMap / BTreeSet: ordered iteration, entry API, ranges, removals.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::collections::{BTreeMap, BTreeSet};

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut map: BTreeMap<u64, u64> = BTreeMap::new();
    for i in 0..128u64 {
        let k = rng.next_u64() % 1000;
        *map.entry(k).or_insert(0) += i;
    }
    emit_usize(map.len());
    for (&k, &v) in map.iter().take(16) {
        emit_u64(k);
        emit_u64(v);
    }
    emit_usize(map.range(100..500).count());
    emit_usize(map.range(..=250).count());
    emit_u64(map.get(&500).copied().unwrap_or(0));
    emit_bool(map.contains_key(&999));

    for k in [0u64, 1, 2, 3, 500, 999] {
        match map.remove(&k) {
            Some(v) => {
                emit_bool(true);
                emit_u64(v);
            }
            None => emit_bool(false),
        }
    }
    emit_usize(map.len());
    let first = map.first_key_value().map(|(k, _)| *k).unwrap_or(0);
    let last = map.last_key_value().map(|(k, _)| *k).unwrap_or(0);
    emit_u64(first);
    emit_u64(last);

    let set: BTreeSet<u64> = map.keys().copied().collect();
    emit_usize(set.len());
    let evens: BTreeSet<u64> = (0..1000u64).filter(|x| x % 2 == 0).collect();
    emit_usize(set.intersection(&evens).count());
    emit_usize(set.union(&evens).count());
    emit_usize(set.difference(&evens).count());
    emit_bool(set.is_subset(&evens));
    emit_u64(*set.iter().next().unwrap_or(&0));

    // Nested map with String keys.
    let mut names: BTreeMap<alloc::string::String, u64> = BTreeMap::new();
    for i in 0..16u64 {
        let key = alloc::format!("key-{:02}", (i * 7) % 16);
        names.insert(key, i);
    }
    emit_usize(names.len());
    let mut h = 0u64;
    for (k, v) in &names {
        for b in k.bytes() {
            h = h.wrapping_mul(131).wrapping_add(b as u64);
        }
        h ^= v;
    }
    emit_u64(h);

    // pop_first / pop_last drain the ordered map.
    let mut small: BTreeMap<u32, u32> = (0..8u32).map(|i| (i * 3, i)).collect();
    while let Some((k, v)) = small.pop_first() {
        emit_u32(k);
        emit_u32(v);
    }
    emit_usize(small.len());

    h ^ map.len() as u64 ^ set.len() as u64
}
