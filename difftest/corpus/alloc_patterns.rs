// Allocation sizes/alignments and reallocation patterns.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    let sizes = [1usize, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 255, 256, 1023, 4096];
    let aligns = [1usize, 2, 4, 8, 16, 32, 64, 256, 1024, 4096];

    for (i, &size) in sizes.iter().enumerate() {
        let align = aligns[i % aligns.len()];
        let layout = Layout::from_size_align(size, align).unwrap();
        let p = unsafe { alloc::alloc::alloc(layout) };
        if p.is_null() {
            emit_bool(false);
            continue;
        }
        emit_bool(true);
        // Alignment is an observable invariant of `alloc`, not a value we
        // compare between backends; both must report 0.
        emit_bool(p as usize % align == 0);
        unsafe {
            for j in 0..size {
                *p.add(j) = (j as u8).wrapping_mul(7).wrapping_add(seed as u8);
            }
            let mut s = 0u64;
            for j in 0..size {
                s = s.wrapping_mul(31).wrapping_add(*p.add(j) as u64);
            }
            acc ^= s;
            alloc::alloc::dealloc(p, layout);
        }
    }
    emit_u64(acc);
    emit_usize(aligns.iter().sum());

    let boxed: alloc::boxed::Box<[u8]> = alloc::vec![0u8; 1000].into_boxed_slice();
    emit_usize(boxed.len());
    let mut v: alloc::vec::Vec<u32> = alloc::vec::Vec::new();
    v.reserve(1000);
    for i in 0..1000u32 {
        v.push(i.wrapping_mul(seed as u32));
    }
    v.shrink_to_fit();
    emit_u32(v[999]);

    let mut s = alloc::string::String::with_capacity(64);
    s.push_str("grow");
    for i in 0..100 {
        s.push((b'a' + (i % 26) as u8) as char);
    }
    emit_usize(s.len());
    s.reserve(4096);
    s.push_str("tail");
    emit_usize(s.len());
    emit_u8(s.as_bytes()[0]);

    let mut rz: alloc::vec::Vec<u16> = alloc::vec::Vec::with_capacity(8);
    for i in 0..8u16 {
        rz.push(i);
    }
    rz.resize(1 << 14, 0xabcd);
    emit_usize(rz.len());
    emit_u16(rz[0] ^ rz[16383]);
    rz.clear();
    emit_usize(rz.len());

    acc ^ (boxed.len() as u64) ^ v[0] as u64 ^ s.len() as u64
}
