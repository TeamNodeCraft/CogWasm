// A closure captures a raw pointer derived from the caller's local. Building
// the closure retains the address, so the hoist must not cache the local's
// field even though the closure is called inside the loop.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed ^ 0x1234_5678, y: seed.rotate_left(21) };
    let pointer = &mut p as *mut Point;
    let mut bump = move || {
        // SAFETY: the closure is only called while `p` is alive in `body`.
        unsafe {
            (*pointer).x = (*pointer).x.wrapping_add(1);
        }
    };
    let mut acc = 0u64;
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 7 {
            let v = p.x;
            bump();
            acc = acc.wrapping_add(v);
            i += 1;
        }
        round += 1;
    }
    emit_u64(acc);
    emit_u64(p.x);
    acc ^ p.y
}
