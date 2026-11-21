// Control flow: labelled loops, loop break values, match state machines,
// while-let, early returns and nested conditionals.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn collatz(mut n: u64) -> (u64, u64) {
    let mut steps = 0;
    let mut peak = n;
    while n != 1 {
        if n & 1 == 0 {
            n /= 2;
        } else {
            n = n.wrapping_mul(3).wrapping_add(1);
        }
        if n > peak {
            peak = n;
        }
        steps += 1;
        if steps > 10_000 {
            break;
        }
    }
    (steps, peak)
}

fn first_prime_factor(mut n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut d = 2;
    loop {
        if d * d > n {
            return n;
        }
        if n % d == 0 {
            return d;
        }
        d += if d == 2 { 1 } else { 2 };
    }
}

enum State {
    Start,
    Seen(u64),
    Done,
}

fn run_machine(input: &[u8]) -> u64 {
    let mut state = State::Start;
    let mut acc = 0u64;
    for &b in input {
        state = match state {
            State::Start => {
                if b == 0 {
                    State::Start
                } else {
                    State::Seen(b as u64)
                }
            }
            State::Seen(x) => {
                acc = acc.wrapping_mul(31).wrapping_add(x);
                if b == 0xff {
                    State::Done
                } else {
                    State::Seen((x ^ b as u64).wrapping_add(1))
                }
            }
            State::Done => break,
        };
    }
    match state {
        State::Start => acc + 1,
        State::Seen(x) => acc ^ x,
        State::Done => acc.wrapping_neg(),
    }
}

fn body(seed: u64) -> u64 {
    let mut acc = 0u64;

    // Labelled break out of nested loops.
    'outer: for i in 0..64u64 {
        for j in 0..64u64 {
            if (i * j) ^ seed == 12345 {
                acc = acc.wrapping_add(i | (j << 8));
                break 'outer;
            }
            if i + j > 100 {
                continue;
            }
            acc ^= (i.wrapping_mul(j)).rotate_left(3);
        }
    }
    emit_u64(acc);

    // loop with a value-producing break.
    let mut k = seed | 1;
    let val = loop {
        k = k.wrapping_mul(6364136223846793005).wrapping_add(1);
        if k & 0xff == 7 {
            break k;
        }
        if k == 0 {
            break 0;
        }
    };
    emit_u64(val);

    // while let over an iterator with early termination.
    let mut it = (0..100u64).filter(|x| x % 7 == 0);
    let mut sum = 0u64;
    while let Some(x) = it.next() {
        if x > 50 {
            break;
        }
        sum += x;
    }
    emit_u64(sum);

    for n in [1u64, 6, 27, 97, 871, 77031] {
        let (s, p) = collatz(n);
        emit_u64(s);
        emit_u64(p);
    }
    for n in [0u64, 1, 2, 3, 97, 2 * 3 * 5 * 7 * 11 * 13, u32::MAX as u64] {
        emit_u64(first_prime_factor(n));
    }

    let input: alloc::vec::Vec<u8> = (0..=255u8).collect();
    emit_u64(run_machine(&input));
    emit_u64(run_machine(&[0, 1, 2, 0xff, 3, 4]));

    // Nested if/else chain with short-circuit boolean operators.
    let mut fizz = 0u64;
    for i in 1..100u64 {
        if i % 15 == 0 {
            fizz = fizz.wrapping_add(1000);
        } else if i % 3 == 0 {
            fizz = fizz.wrapping_add(100);
        } else if i % 5 == 0 {
            fizz = fizz.wrapping_add(10);
        } else if i & 1 == 0 && (i > 50 || i < 10) {
            fizz = fizz.wrapping_add(1);
        }
    }
    emit_u64(fizz);

    acc ^ val ^ sum ^ fizz
}
