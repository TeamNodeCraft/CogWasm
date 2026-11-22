// A callee stores the caller's address in a field of another borrowed struct;
// the loop writes through that field while reading the first local's field.
// The callee captures argument 0, so the hoist must not touch it.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy)]
struct Holder {
    slot: *mut Point,
}

#[inline(never)]
fn stash_in(point: &mut Point, holder: &mut Holder) {
    holder.slot = point as *mut Point;
}

fn body(seed: u64) -> u64 {
    let mut p = Point { x: seed ^ 0x55, y: seed.rotate_left(3) };
    let mut holder = Holder { slot: core::ptr::null_mut() };
    stash_in(&mut p, &mut holder);
    let mut acc = 0u64;
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < 6 {
            let v = p.x;
            // SAFETY: `holder.slot` was set to `&mut p` above and `p` is alive.
            unsafe {
                (*holder.slot).x = v.wrapping_mul(3).wrapping_add(1);
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
