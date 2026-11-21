// Iterator adapters and consumers over ranges, Vec, slices and chars.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::vec::Vec;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let v: Vec<u64> = (0..128).map(|_| rng.next_u64() % 10_000).collect();

    emit_u64(v.iter().copied().map(|x| x.wrapping_mul(3)).sum());
    emit_u64(v.iter().copied().filter(|&x| x & 1 == 0).count() as u64);
    emit_u64(v.iter().copied().fold(0u64, |a, b| a.wrapping_add(b.rotate_left(1))));
    emit_u64(v.iter().copied().max().unwrap_or(0));
    emit_u64(v.iter().copied().min().unwrap_or(0));
    emit_u64(v.iter().copied().position(|x| x > 5000).unwrap_or(usize::MAX) as u64);
    emit_bool(v.iter().any(|&x| x == 0));
    emit_bool(v.iter().all(|&x| x < 10_000));

    let zipped: u64 = v
        .iter()
        .copied()
        .zip((0..128u64).rev())
        .map(|(a, b)| a ^ b)
        .sum();
    emit_u64(zipped);

    let chained: Vec<u64> = (0..16u64)
        .chain(100..108)
        .chain(v.iter().copied().take(4))
        .collect();
    emit_usize(chained.len());
    emit_u64(chained.iter().sum());

    let scanned: Vec<u64> = (1..10u64)
        .scan(1u64, |state, x| {
            *state = state.wrapping_mul(x);
            Some(*state)
        })
        .collect();
    emit_usize(scanned.len());
    emit_u64(scanned[9 - 1]);

    let flattened: Vec<u64> = (0..4u64)
        .flat_map(|i| (0..i).map(move |j| i * 10 + j))
        .collect();
    emit_usize(flattened.len());
    emit_u64(flattened.iter().sum());

    let evens: Vec<u64> = (0..64u64).filter(|x| x % 2 == 0).collect();
    let (lo, hi): (Vec<u64>, Vec<u64>) = (0..32u64).partition(|x| *x < 16);
    emit_usize(evens.len());
    emit_usize(lo.len());
    emit_usize(hi.len());
    emit_u64(lo.iter().sum::<u64>() ^ hi.iter().sum::<u64>());

    let mut it = (0..10u64).peekable();
    emit_u64(*it.peek().unwrap_or(&0));
    emit_u64(it.next().unwrap_or(0));
    emit_usize(it.clone().count());
    it.next();
    emit_u64(it.next().unwrap_or(0));

    let chars: Vec<char> = "difftest-cranelift".chars().collect();
    let upper: u64 = chars
        .iter()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase() as u64)
        .fold(0u64, |a, c| a.wrapping_mul(131).wrapping_add(c));
    emit_u64(upper);

    let mut acc = 0u64;
    for (i, x) in v.iter().enumerate().skip(10).take(20) {
        acc = acc.wrapping_add(x.rotate_left((i & 63) as u32));
    }
    emit_u64(acc);

    let step: u64 = (0..100u64).step_by(7).sum();
    emit_u64(step);
    let revsum: u64 = (0..100u64).rev().take(10).sum();
    emit_u64(revsum);
    let incl: u64 = (1..=10u64).product();
    emit_u64(incl);
    let contains = (0..100u64).contains(&42);
    emit_bool(contains);

    acc ^ step ^ revsum ^ incl
}
