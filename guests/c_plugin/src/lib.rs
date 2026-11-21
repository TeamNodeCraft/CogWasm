//! NodeCraft-style plugin: host import, f64 parameter list, allocation, result
//! in linear memory, panic=abort, no_std.
//!
//! This stage-C copy replaces the fixed 1 MiB `.bss` heap with a growable bump
//! allocator over `memory.grow`. The heap starts at the current end of linear
//! memory (after all static data and the linker-provided shadow stack), so
//! every heap byte comes from pages that `memory.grow` added. `dealloc` is a
//! no-op: the bump pointer only moves forward, which is enough for a plugin
//! that is instantiated, evaluated, and dropped.
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32::{memory_grow, memory_size};
use core::cell::UnsafeCell;

const PAGE_SIZE: usize = 65536;

struct BumpState {
    /// Whether `next`/`end` have been initialized from `memory.size`.
    initialized: bool,
    /// Next free byte.
    next: usize,
    /// End of the memory obtained from `memory.grow` so far.
    end: usize,
}

struct Bump(UnsafeCell<BumpState>);

// Plugins are single-threaded per instance; see results.md on atomics.
unsafe impl Sync for Bump {}

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(1);
        let align = layout.align().max(1);
        let state = unsafe { &mut *self.0.get() };

        if !state.initialized {
            let mut end = memory_size(0) * PAGE_SIZE;
            if end == 0 {
                // A zero-page memory would make the first allocation return
                // address 0, which Rust reserves for null. Give the heap one
                // page first.
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
static GLOBAL: Bump = Bump::new();

impl Bump {
    const fn new() -> Self {
        Bump(UnsafeCell::new(BumpState {
            initialized: false,
            next: 0,
            end: 0,
        }))
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// The host API this plugin expects. Imported from wasm module "nodecraft",
// not the default "env".
#[link(wasm_import_module = "nodecraft")]
unsafe extern "C" {
    fn host_bias(x: f64) -> f64;
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { GLOBAL.alloc(Layout::from_size_align(size.max(1), align.max(1)).unwrap()) }
}

/// `eval` receives `len` f64 parameters at `params`, and writes one f64 result
/// to `out`. Returns 0 on success.
#[unsafe(no_mangle)]
pub extern "C" fn eval(params: *const f64, len: usize, out: *mut f64) -> i32 {
    if params.is_null() || out.is_null() {
        return 1;
    }
    let xs = unsafe { core::slice::from_raw_parts(params, len) };
    let mut adjusted: Vec<f64> = Vec::with_capacity(xs.len());
    for &x in xs {
        adjusted.push(unsafe { host_bias(x) });
    }
    let sum: f64 = adjusted.iter().sum();
    let mean = if adjusted.is_empty() {
        0.0
    } else {
        sum / adjusted.len() as f64
    };
    unsafe { *out = mean };
    0
}
