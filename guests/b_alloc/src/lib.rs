//! no_std + alloc: Vec, String, closures, trait objects and f64 maths.
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

// A fixed heap in .bss: wasm memories are zero-filled, so rust-lld stores it
// only in the memory's initial size. It also avoids llvm.wasm.memory.size/grow
// intrinsics (unsupported by clif-to-wasm). A real plugin host gives each
// instance a bounded heap like this. (Wild currently emits this zeroed block as
// a data segment, so the wild builds are ~1 MiB larger; see results.md.)
const HEAP_SIZE: usize = 1 << 20;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

struct Bump;

struct Counter(UnsafeCell<usize>);
unsafe impl Sync for Counter {}
static NEXT: Counter = Counter(UnsafeCell::new(0));

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = core::ptr::addr_of_mut!(HEAP) as *mut u8 as usize;
        let mut p = unsafe { *NEXT.0.get() };
        if p == 0 {
            p = start;
        }
        let aligned = (p + layout.align() - 1) & !(layout.align() - 1);
        let end = aligned + layout.size();
        if end > start + HEAP_SIZE {
            return core::ptr::null_mut();
        }
        unsafe { *NEXT.0.get() = end; }
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

trait Op {
    fn apply(&self, x: f64) -> f64;
}

struct Scale(f64);

impl Op for Scale {
    fn apply(&self, x: f64) -> f64 {
        x * self.0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { ALLOC.alloc(Layout::from_size_align(size.max(1), align.max(1)).unwrap()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn sum_slice(ptr: *const f64, len: usize) -> f64 {
    let xs = unsafe { core::slice::from_raw_parts(ptr, len) };
    xs.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench(n: u32) -> f64 {
    // Exercise Vec, String, format!, closures, trait objects and f64 libm calls.
    let base: Vec<f64> = (0..64).map(|i| i as f64 * 0.25).collect();
    let name = String::from("nodecraft");
    let label = format!("{name}-plugin");
    let bias = label.len() as f64;
    let ops: Vec<Box<dyn Op>> = alloc::vec![Box::new(Scale(1.5)), Box::new(Scale(0.5))];
    let add = |x: f64| x + bias;

    let mut acc = 0.0f64;
    let mut i = 1u32;
    while i <= n {
        let x = (i as f64) * 0.001 + base[(i as usize) & 63];
        let mut y = add(x);
        for op in &ops {
            y = op.apply(y);
        }
        acc += libm::sqrt(y.abs()) + libm::sin(y);
        i = i.wrapping_add(1);
    }
    acc / (n as f64)
}
