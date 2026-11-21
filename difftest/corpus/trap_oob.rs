// Expected trap: bounds-checked indexing always out of range.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let a = [1u8, 2, 3, 4];
    // Index 4..=7: always past the 4-element array for every seed.
    let i = 4 + (seed as usize & 3);
    emit_u8(a[0]);
    emit_u8(a[i]);
    seed
}
