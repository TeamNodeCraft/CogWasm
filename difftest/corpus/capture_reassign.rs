// A loop that reassigns the whole aggregate between field reads. The hoist
// must treat an assignment to the whole local as a write, not only an
// assignment to one of its fields; otherwise the preheader read is stale.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[inline(never)]
fn peek(point: &Point) -> u64 {
    point.x
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed, y: seed.rotate_left(5) };
    // Keep `p` in a stack slot so the hoist sees field reads.
    let mut acc = peek(&p);
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 8 {
            let v = p.x;
            p = Point { x: v.wrapping_add(1), y: p.y.wrapping_add(3) };
            acc = acc.wrapping_add(v);
            i += 1;
        }
        round += 1;
    }
    emit_u64(acc);
    emit_u64(p.x);
    acc ^ p.y
}
