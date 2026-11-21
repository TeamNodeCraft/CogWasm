//! (f) Host-call-heavy geometry stub.
//!
//! A tiny fixed point set is pushed through five `nodecraft` imports per
//! iteration:
//!
//! ```text
//! dot2(x1, y1, x2, y2) -> f64
//! length2(x, y)        -> f64
//! clamp01(x)           -> f64
//! scale(x, k)          -> f64
//! mix(a, b, t)         -> f64
//! ```
//!
//! `run(n)` executes the loop `n` times and therefore makes **exactly 5*n**
//! imported host calls. The host harness reports ns per imported call.
//!
//! `no_std` + `alloc`, `panic = "abort"`, growable bump heap over
//! `memory.grow`; the only imports are the five geometry functions from module
//! `nodecraft`.
#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32::{memory_grow, memory_size};
use core::cell::UnsafeCell;

const PAGE_SIZE: usize = 65536;

struct BumpState {
    initialized: bool,
    next: usize,
    end: usize,
}

struct Counter(UnsafeCell<BumpState>);
unsafe impl Sync for Counter {}
static NEXT: Counter = Counter(UnsafeCell::new(BumpState {
    initialized: false,
    next: 0,
    end: 0,
}));

struct Bump;

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(1);
        let align = layout.align().max(1);
        let state = unsafe { &mut *NEXT.0.get() };
        if !state.initialized {
            let mut end = memory_size(0) * PAGE_SIZE;
            if end == 0 {
                if memory_grow(0, 1) == usize::MAX {
                    return core::ptr::null_mut();
                }
                end = PAGE_SIZE;
            }
            state.next = end;
            state.end = end;
            state.initialized = true;
        }
        let Some(aligned) = state.next.checked_add(align - 1).map(|p| p & !(align - 1)) else {
            return core::ptr::null_mut();
        };
        let Some(end) = aligned.checked_add(size) else {
            return core::ptr::null_mut();
        };
        if end > state.end {
            let pages = (end - state.end).div_ceil(PAGE_SIZE);
            let old_pages = memory_grow(0, pages);
            if old_pages == usize::MAX {
                return core::ptr::null_mut();
            }
            state.end = (old_pages + pages) * PAGE_SIZE;
        }
        state.next = end;
        aligned as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOC: Bump = Bump;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// Five small f64 imports. All live in the explicit "nodecraft" module, like the
// Stage-A plugin, rather than the default "env".
#[link(wasm_import_module = "nodecraft")]
unsafe extern "C" {
    fn dot2(x1: f64, y1: f64, x2: f64, y2: f64) -> f64;
    fn length2(x: f64, y: f64) -> f64;
    fn clamp01(x: f64) -> f64;
    fn scale(x: f64, k: f64) -> f64;
    fn mix(a: f64, b: f64, t: f64) -> f64;
}

// Small fixed array, deliberately not heap-allocated: the point is the host
// boundary, not data movement.
static PTS: [f64; 16] = [
    0.0, 1.0, -1.0, 0.5, -0.5, 2.0, -2.0, 0.25,
    0.75, -0.75, 1.5, -1.5, 0.125, -0.125, 3.0, -3.0,
];

#[unsafe(no_mangle)]
pub extern "C" fn wasm_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { ALLOC.alloc(Layout::from_size_align(size.max(1), align.max(1)).unwrap()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_reset() {
    unsafe { (*NEXT.0.get()).initialized = false; }
}

/// Number of imported calls made per loop iteration. The harness uses this to
/// compute ns/call instead of hard-coding the constant.
#[unsafe(no_mangle)]
pub extern "C" fn calls_per_iter() -> i32 {
    5
}

/// Run `n` iterations of the geometry pipeline. Exactly `5*n` host imports.
#[unsafe(no_mangle)]
pub extern "C" fn run(n: u32) -> f64 {
    let mut acc = 0.0f64;
    let mut i = 0u32;
    while i < n {
        let j = (i as usize) & 7;
        let x = unsafe { *PTS.get_unchecked(j) };
        let y = unsafe { *PTS.get_unchecked(j + 8) };
        // Varying input keeps the calls from looking loop-invariant; external
        // calls are not pure, so this is belt-and-braces.
        let t = (i as f64) * 1.0e-6;

        let a = unsafe { dot2(x, y, 0.5, 0.25) };
        let b = unsafe { length2(a + t, x) };
        let c = unsafe { clamp01(b * 0.001) };
        let d = unsafe { scale(c, 2.0) };
        let e = unsafe { mix(d, y, 0.5) };
        acc += e;
        i += 1;
    }
    acc
}

/// Proof that `alloc` is linked; not on the timed path.
#[unsafe(no_mangle)]
pub extern "C" fn alloc_smoke(n: usize) -> f64 {
    use alloc::vec::Vec;
    let mut v: Vec<f64> = Vec::new();
    for i in 0..n.min(1024) {
        v.push(i as f64);
    }
    v.iter().sum()
}
