//! Small program mirroring `guests/a_add`:
//! no_std, no allocator, one exported arithmetic function. It adds the
//! difftest ABI (`run(seed)` plus the output buffer) so the benchmark compares
//! this program's result with the LLVM build.
#![no_std]

include!("../prelude.rs");

#[inline]
fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

fn body(seed: u64) -> u64 {
    let a = seed as i32;
    let b = 20 + (seed >> 32) as i32;
    let sum = add(a, b);
    emit_i32(sum);
    u64::from(sum as u32)
}
