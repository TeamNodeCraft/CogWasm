//! End-to-end test of `memory.size` / `memory.grow`, lowered by cg_clif to the
//! reserved clif-to-wasm helper calls and by clif-to-wasm to the wasm
//! instructions.
//!
//! The harness calls `pages`, `grow`, and `probe` on one instance:
//!
//! * `pages()` is `memory.size`,
//! * `grow(delta)` is `memory.grow` and returns the previous size or `usize::MAX`,
//! * `probe(addr, value)` writes and reads back through a pointer, which the
//!   harness points into a freshly grown page.
#![no_std]

use core::arch::wasm32::{memory_grow, memory_size};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// Current memory size in 64 KiB pages.
#[unsafe(no_mangle)]
pub extern "C" fn pages() -> usize {
    memory_size(0)
}

/// Grow memory by `delta` pages, returning the previous size or `usize::MAX`.
#[unsafe(no_mangle)]
pub extern "C" fn grow(delta: usize) -> usize {
    memory_grow(0, delta)
}

/// Write `value` through `addr` and read it back.
#[unsafe(no_mangle)]
pub extern "C" fn probe(addr: *mut u32, value: u32) -> u32 {
    unsafe {
        addr.write(value);
        addr.read()
    }
}
