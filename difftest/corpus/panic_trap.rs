// Expected trap: an explicit panic! with panic=abort lowers to `unreachable`.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    emit_u64(seed);
    panic!("difftest: this program is expected to trap");
}
