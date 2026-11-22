// A callee stashes a raw pointer to the caller's local in a static; the loop
// then writes through that pointer while reading the local's field. A hoist
// that ignores capture would cache the first field read and produce a
// different result. The output must match LLVM with the hoist off and on.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

static mut STASH: *mut Point = core::ptr::null_mut();

#[inline(never)]
fn stash(p: &mut Point) {
    // SAFETY: the test is single-threaded and only uses the pointer while `p`
    // is alive in `body`.
    unsafe {
        STASH = p as *mut Point;
    }
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed, y: seed.rotate_left(9) };
    stash(&mut p);
    let mut acc = 0u64;
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 8 {
            let v = p.x;
            // SAFETY: `STASH` was set to `&mut p` above and `p` is still alive.
            unsafe {
                (*STASH).x = v.wrapping_add(1);
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
