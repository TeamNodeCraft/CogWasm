// Closures: captures, Fn/FnMut/FnOnce, returning closures, higher-order calls.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::boxed::Box;

fn apply<F: Fn(u64) -> u64>(f: F, x: u64) -> u64 {
    f(x)
}

fn twice<F: FnMut(u64) -> u64>(mut f: F, x: u64) -> u64 {
    let y = f(x);
    f(y)
}

fn make_adder(n: u64) -> impl Fn(u64) -> u64 {
    move |x| x.wrapping_add(n)
}

fn make_stateful(mut acc: u64) -> impl FnMut(u64) -> u64 {
    move |x| {
        acc = acc.wrapping_mul(31).wrapping_add(x);
        acc
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;

    let base = seed ^ 0x1234;
    let add_base = |x: u64| x.wrapping_add(base);
    for i in 0..16u64 {
        acc = acc.wrapping_add(add_base(rng.next_u64() ^ i));
    }
    emit_u64(acc);

    let mut counter = 0u64;
    let mut bump = |x: u64| {
        counter += x;
        counter
    };
    for _ in 0..8 {
        acc ^= bump(rng.next_u64() % 1000);
    }
    emit_u64(counter);

    let consumed = {
        let owned = alloc::vec![1u64, 2, 3, 4];
        let f = move || owned.iter().sum::<u64>();
        f()
    };
    emit_u64(consumed);

    emit_u64(apply(|x| x.wrapping_mul(3).wrapping_add(1), seed));
    emit_u64(twice(|x| x.rotate_left(7), seed));
    let add7 = make_adder(7);
    emit_u64(add7(seed));
    let add7b = make_adder(7);
    emit_u64(add7b(100));

    let mut stateful = make_stateful(seed & 0xff);
    for i in 0..12u64 {
        emit_u64(stateful(i));
    }

    let boxed: Box<dyn Fn(u64) -> u64> = Box::new(move |x| x ^ base);
    emit_u64(boxed(seed));
    emit_u64(boxed(0));

    let mut v: alloc::vec::Vec<u64> = (0..32).map(|i| i ^ seed).collect();
    v.sort_unstable_by_key(|&x| (x.wrapping_mul(0x9e37) ^ base) % 101);
    emit_u64(v[0] ^ v[31]);

    // Closure that captures a mutable slice and mutates through it.
    let mut data = [0u64; 16];
    let mut fill = |i: usize, x: u64| data[i] = x;
    for i in 0..16 {
        fill(i, rng.next_u64() ^ i as u64);
    }
    emit_u64(data.iter().fold(0u64, |a, &b| a.wrapping_add(b)));

    acc ^ counter ^ consumed ^ v[16]
}
