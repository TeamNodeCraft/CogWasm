// Loop-invariant field reads from a local whose address escapes before the
// loop. `sroa` cannot promote `p` because `blend(&mut p)` takes its address, so
// the fields stay in the stack slot; Goal 1 step 5b's capture-aware loop hoist
// (patch 0008) is what removes the per-iteration reloads. `blend` does not keep
// the address, so the summary allows the hoist. The outputs must match LLVM.
#![no_std]

include!("prelude.rs");

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

#[inline(never)]
fn blend(p: &mut Point) -> u64 {
    p.x = p.x.wrapping_add(1);
    p.x ^ p.y.rotate_left(7) ^ p.z.rotate_left(13)
}

fn body(seed: u64) -> u64 {
    let mut p = Point {
        x: seed,
        y: seed.wrapping_mul(0x9e37_79b9_7f4a_7c15),
        z: seed ^ 0xdead_beef_cafe_f00d,
    };
    let mut acc = blend(&mut p);
    let mut round = 0u64;
    while round < 2 {
        let mut i = 0u64;
        while i < (seed & 31) + 1 {
            acc = acc
                .wrapping_add(p.x)
                .wrapping_add(p.y.rotate_left(1))
                .wrapping_add(p.z.rotate_left(2));
            i += 1;
        }
        round += 1;
    }
    emit_u64(acc);
    acc
}
