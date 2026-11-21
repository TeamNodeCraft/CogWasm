// Raw pointer arithmetic, read/write and copy within live allocations.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::vec::Vec;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut buf = [0u8; 512];
    let p = buf.as_mut_ptr();
    unsafe {
        for i in 0..buf.len() {
            *p.add(i) = (i as u64).wrapping_mul(seed).wrapping_add(i as u64 >> 1) as u8;
        }
        emit_u8(*p);
        emit_u8(*p.add(255));
        emit_u8(*p.add(511));
        let mut s = 0u64;
        let mut q = p;
        for _ in 0..16 {
            s = s.wrapping_add(*q as u64);
            q = q.add(32);
        }
        emit_u64(s);

        // offset_from / offset / wrapping_offset within the same allocation.
        let a = p.add(10);
        let b = p.add(300);
        emit_i64(b.offset_from(a) as i64);
        emit_usize(a.offset(5).offset_from(p) as usize);
        emit_u8(*a.offset(5));
        emit_u8(*b.wrapping_offset(-100));
        emit_bool(a < b);

        // write/read round-trip.
        let mut x = 0x1122_3344_5566_7788u64;
        let xp = &mut x as *mut u64;
        let v = core::ptr::read(xp);
        core::ptr::write(xp, v.rotate_left(9));
        emit_u64(x);

        core::ptr::copy_nonoverlapping(p, p.add(256), 128);
        emit_u8(*p.add(256));
        emit_u8(*p.add(383));
        core::ptr::copy(p.add(100), p.add(50), 100);
        emit_u8(*p.add(50) ^ *p.add(149));
    }

    let mut v: Vec<u64> = (0..64u64).map(|i| rng.next_u64() ^ i).collect();
    let vp = v.as_mut_ptr();
    unsafe {
        for i in 0..v.len() {
            *vp.add(i) = (*vp.add(i)).rotate_left((i & 63) as u32);
        }
        let sl = core::slice::from_raw_parts(vp, v.len());
        emit_u64(sl.iter().fold(0u64, |a, &b| a.wrapping_add(b)));
        emit_usize(sl.len());
        emit_bool(core::ptr::eq(sl.as_ptr(), vp));
    }

    // Pointer nil / null distinctions (values, not addresses).
    let null: *const u8 = core::ptr::null();
    emit_bool(null.is_null());
    let dangling = core::ptr::NonNull::<u8>::dangling().as_ptr();
    emit_bool(!dangling.is_null());
    emit_usize(core::mem::size_of::<*const u8>());

    let mut acc = 0u64;
    for &b in buf.iter() {
        acc = acc.wrapping_mul(31).wrapping_add(b as u64);
    }
    emit_u64(acc);
    acc ^ v[63]
}
