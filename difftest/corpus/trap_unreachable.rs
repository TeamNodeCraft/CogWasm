// Expected trap: unreachable!() panic in an unreachable arm.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    emit_u64(seed);
    // Both paths reach an unreachable!() panic: this traps for every seed.
    match seed {
        0x0123_4567_89ab_cdef => {
            emit_u64(1);
            unreachable!("difftest: unreachable arm reached");
        }
        _ => unreachable!("difftest: unreachable arm reached"),
    }
}
