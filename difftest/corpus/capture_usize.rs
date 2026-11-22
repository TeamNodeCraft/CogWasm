// The caller's address is smuggled through `usize` and turned back into a raw
// pointer. The pointer-to-integer cast captures the address, so the hoist must
// not cache the local's field.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[inline(never)]
fn address_of(point: &mut Point) -> usize {
    point as *mut Point as usize
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed.wrapping_mul(0x9e37_79b9), y: seed.rotate_left(17) };
    let address = address_of(&mut p);
    let mut acc = 0u64;
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 7 {
            let v = p.x;
            let pointer = address as *mut Point;
            // SAFETY: `address` is the address of `p`, which is still alive.
            unsafe {
                (*pointer).x = v.wrapping_add(5);
            }
            acc = acc.wrapping_add(v);
            i += 1;
        }
        round += 1;
    }
    emit_u64(acc);
    emit_u64(p.x);
    acc.wrapping_add(p.y)
}
