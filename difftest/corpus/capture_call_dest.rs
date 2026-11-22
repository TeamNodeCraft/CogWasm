// A call inside the loop writes its result into the whole aggregate. The
// call's destination is a write to the base, so the hoist must not reuse the
// preheader field read. The callee only takes a scalar, so the old "borrowed
// whole local" rule does not protect this case.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[inline(never)]
fn step(value: u64) -> Point {
    Point { x: value.wrapping_add(1), y: value.rotate_left(3) }
}

#[inline(never)]
fn peek(point: &Point) -> u64 {
    point.x
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed, y: seed.rotate_left(7) };
    // Taking the address keeps `p` in a stack slot and gives the hoist a
    // preheader read; `peek` does not retain it.
    let mut acc = peek(&p);
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 6 {
            let v = p.x;
            p = step(v);
            acc = acc.wrapping_add(v);
            i += 1;
        }
        round += 1;
    }
    emit_u64(acc);
    emit_u64(p.x);
    acc ^ p.y
}
