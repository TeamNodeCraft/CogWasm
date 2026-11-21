// Shared prelude, textually included by every difftest corpus program.
//
// The *same* source is compiled by both backends (LLVM via rustup's prebuilt
// wasm32-unknown-unknown std, cg_clif via the CogWasm rustc.wasm and the pure
// sysroot), so the allocator and output-buffer behaviour cannot differ just
// because the harness was written twice.
//
// ABI (see difftest/README.md):
//
//   run(seed: u64) -> u64          entry point; calls the program's body(seed)
//   out_ptr() -> u32               address of the output byte buffer
//   out_len() -> u32               number of bytes written by body()
//   memory                         exported linear memory
//
// Nothing here exports or observes an address: `out_ptr()` is used by the host
// only to address memory, never as a value the guest computes with.

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32::{memory_grow, memory_size};
use core::cell::UnsafeCell;

const PAGE_SIZE: usize = 65536;

// ---------------------------------------------------------------------------
// Growable bump allocator.
//
// It starts at `memory.size() * 64 KiB`, i.e. after every static and the
// linker-provided shadow stack, and calls `memory.grow` by whole pages. This is
// the same design as guests/c_plugin: both backends support it (the
// the CogWasm patch lowers the wasm memory intrinsics), and the heap never lives in
// static data, so no allocation-layout differences leak from `.bss` sizing.
// `dealloc` is a no-op; every corpus program is a short-lived instance.
// ---------------------------------------------------------------------------

struct BumpState {
    initialized: bool,
    next: usize,
    end: usize,
}

struct Bump(UnsafeCell<BumpState>);

// One instance per test; no threads.
unsafe impl Sync for Bump {}

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(1);
        let align = layout.align().max(1);
        let state = unsafe { &mut *self.0.get() };

        if !state.initialized {
            let mut end = memory_size(0) * PAGE_SIZE;
            if end == 0 {
                // Address 0 is null; make sure the first allocation is non-null.
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
static GLOBAL: Bump = Bump(UnsafeCell::new(BumpState {
    initialized: false,
    next: 0,
    end: 0,
}));

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // panic=abort: every panic, including the implicit bounds/div checks,
    // becomes the wasm `unreachable` trap.
    core::arch::wasm32::unreachable()
}

// ---------------------------------------------------------------------------
// Output buffer: a fixed byte array in static data. Programs append to it with
// the emit helpers; the host reads [out_ptr(), out_ptr()+out_len()) after the
// call. Contents are compared bit-for-bit, so only values (never addresses)
// may be emitted.
// ---------------------------------------------------------------------------

pub const OUT_CAP: usize = 16384;

struct OutBuf(UnsafeCell<([u8; OUT_CAP], usize)>);

unsafe impl Sync for OutBuf {}

static OUT: OutBuf = OutBuf(UnsafeCell::new(([0u8; OUT_CAP], 0)));

#[inline]
fn out_state() -> &'static mut ([u8; OUT_CAP], usize) {
    unsafe { &mut *OUT.0.get() }
}

#[inline]
pub fn emit_bytes(bytes: &[u8]) {
    let st = out_state();
    let len = st.1;
    let n = bytes.len().min(OUT_CAP.saturating_sub(len));
    st.0[len..len + n].copy_from_slice(&bytes[..n]);
    st.1 = len + n;
}

#[inline]
pub fn emit_u8(v: u8) {
    emit_bytes(&[v]);
}

#[inline]
pub fn emit_u16(v: u16) {
    emit_bytes(&v.to_le_bytes());
}

#[inline]
pub fn emit_u32(v: u32) {
    emit_bytes(&v.to_le_bytes());
}

#[inline]
pub fn emit_u64(v: u64) {
    emit_bytes(&v.to_le_bytes());
}

#[inline]
pub fn emit_usize(v: usize) {
    emit_u32(v as u32);
}

#[inline]
pub fn emit_i8(v: i8) {
    emit_u8(v as u8);
}

#[inline]
pub fn emit_i16(v: i16) {
    emit_u16(v as u16);
}

#[inline]
pub fn emit_i32(v: i32) {
    emit_u32(v as u32);
}

#[inline]
pub fn emit_i64(v: i64) {
    emit_u64(v as u64);
}

#[inline]
pub fn emit_i128(v: i128) {
    emit_bytes(&v.to_le_bytes());
}

#[inline]
pub fn emit_u128(v: u128) {
    emit_bytes(&v.to_le_bytes());
}

#[inline]
pub fn emit_bool(v: bool) {
    emit_u8(v as u8);
}

#[inline]
pub fn emit_f32(v: f32) {
    emit_u32(v.to_bits());
}

#[inline]
pub fn emit_f64(v: f64) {
    emit_u64(v.to_bits());
}

#[inline]
pub fn emit_str(s: &str) {
    emit_bytes(s.as_bytes());
}

/// A tiny deterministic PRNG (splitmix64). Programs may use it to derive
/// inputs from `seed`; it never touches pointers or host state.
pub struct Rng(u64);

impl Rng {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Rng(splitmix64(seed ^ 0x9e37_79b9_7f4a_7c15))
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        splitmix64(self.0)
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    pub fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next_u64() % n
        }
    }
}

#[inline]
pub fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

// ---------------------------------------------------------------------------
// Exports.
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn out_ptr() -> u32 {
    out_state().0.as_ptr() as u32
}

#[no_mangle]
pub extern "C" fn out_len() -> u32 {
    out_state().1 as u32
}

#[no_mangle]
pub extern "C" fn run(seed: u64) -> u64 {
    body(seed)
}
