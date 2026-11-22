// A callee returns a raw pointer derived from its argument. The caller keeps
// the pointer and writes through it in a loop while reading the local's field.
// A returned address is a capture, so the hoist must leave this function alone.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[inline(never)]
fn field_ptr(point: &mut Point) -> *mut u64 {
    &mut point.x
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed.wrapping_add(7), y: seed.rotate_left(11) };
    let slot = field_ptr(&mut p);
    let mut acc = 0u64;
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 5 {
            let v = p.x;
            // SAFETY: `slot` points at `p.x` and `p` is alive.
            unsafe {
                *slot = v ^ 0xa5a5_5a5a_1234_5678;
            }
            acc = acc.wrapping_add(v);
            i += 1;
        }
        round += 1;
    }
    emit_u64(acc);
    emit_u64(p.x);
    acc ^ p.y
}
