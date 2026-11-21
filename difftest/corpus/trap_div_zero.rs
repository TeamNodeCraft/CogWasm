// Expected trap: integer division by zero is always checked and panics.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    emit_u64(seed);
    // `& 0` keeps the value opaque enough that this is a runtime division
    // while guaranteeing the divisor is always zero for every seed.
    let zero = seed & 0;
    let x = 100u64;
    emit_u64(x / zero);
    x
}
