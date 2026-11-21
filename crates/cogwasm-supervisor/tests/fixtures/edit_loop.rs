//! Edit-loop fixture: no imports, no allocator, and a `run(u64) -> u64`
//! entry point the supervisor's Wasmtime runtime can call.
//!
//! The body is small but not free (512 rounds of integer mixing), so a build
//! and a call are both visible in the measurement. The benchmark probe is
//! read through `black_box` and immediately cancelled by wrapping arithmetic,
//! so toggling it between `0u64` and `1u64` changes the source bytes without
//! changing `run`'s result.
#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const BENCH_EDIT_TWEAK: u64 = 0u64;

const MIX: u64 = 0x9e37_79b9_7f4a_7c15;
const ROUNDS: u64 = 512;

#[unsafe(no_mangle)]
pub extern "C" fn run(seed: u64) -> u64 {
    let tweak = core::hint::black_box(BENCH_EDIT_TWEAK);
    let mut state = seed ^ MIX;
    let mut round = 0u64;
    while round < ROUNDS {
        state ^= state >> 30;
        state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        state ^= state >> 27;
        state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
        state ^= state >> 31;
        state = state.wrapping_add(round.wrapping_mul(MIX));
        round += 1;
    }
    state.wrapping_add(tweak).wrapping_sub(tweak)
}
