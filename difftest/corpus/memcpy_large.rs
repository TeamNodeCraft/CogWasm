// Bulk memory movement: large arrays, slice copies, mem::swap and clones.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::vec::Vec;

#[derive(Clone, Copy)]
struct Big {
    data: [u8; 4096],
    tag: u64,
}

fn body(seed: u64) -> u64 {
    let mut src = [0u8; 8192];
    for i in 0..src.len() {
        src[i] = (seed.rotate_left((i & 63) as u32).wrapping_add(i as u64)) as u8;
    }
    let mut dst = [0u8; 8192];
    dst.copy_from_slice(&src);
    // Overlapping copy (memmove).
    dst.copy_within(0..4096, 2048);
    // Non-overlapping raw copy.
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr().add(1024), dst.as_mut_ptr(), 512);
    }

    let mut sum = 0u64;
    for &b in dst.iter() {
        sum = sum.wrapping_mul(31).wrapping_add(b as u64);
    }
    emit_u64(sum);
    emit_u8(dst[0] ^ dst[2048] ^ dst[8191]);

    let mut v: Vec<u8> = src.to_vec();
    v.extend_from_slice(&src[..2048]);
    v.extend_from_slice(&dst[..1024]);
    emit_usize(v.len());
    emit_u64(v.iter().fold(0u64, |a, &b| a.wrapping_add(b as u64)));

    let mut a = Big {
        data: src[..4096].try_into().unwrap(),
        tag: seed,
    };
    let b = a; // Copy: 4 KiB memcpy.
    let mut c = b.clone();
    c.tag = c.tag.wrapping_add(1);
    c.data[0] = c.data[0].wrapping_add(1);
    core::mem::swap(&mut a, &mut c);
    emit_u64(a.tag);
    emit_u64(c.tag);
    emit_u8(a.data[0]);
    emit_u8(b.data[0]);
    emit_u64(a.data.iter().fold(0u64, |x, &y| x.wrapping_add(y as u64)));

    let old = core::mem::replace(&mut a.tag, 0xdead);
    emit_u64(old);
    emit_u64(a.tag);
    let taken = core::mem::take(&mut c.tag);
    emit_u64(taken);

    // Big moves through Vec growth.
    let mut blocks: Vec<Big> = Vec::new();
    for i in 0..8u64 {
        let mut blk = b;
        blk.tag = i;
        blk.data[7] = i as u8;
        blocks.push(blk);
    }
    emit_usize(blocks.len());
    emit_u64(blocks.iter().map(|x| x.tag).sum());
    emit_u8(blocks[7].data[7]);

    sum ^ a.tag ^ blocks[7].data[7] as u64
}
