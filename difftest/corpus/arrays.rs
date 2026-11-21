// Arrays, multidimensional arrays, overlapping copy_within and array maps.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn transpose(m: [[u32; 4]; 4]) -> [[u32; 4]; 4] {
    let mut t = [[0u32; 4]; 4];
    let mut i = 0;
    while i < 4 {
        let mut j = 0;
        while j < 4 {
            t[j][i] = m[i][j];
            j += 1;
        }
        i += 1;
    }
    t
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut m = [[0u32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            m[i][j] = rng.next_u32() ^ ((i as u32) << 8) ^ (j as u32);
        }
    }
    let t = transpose(m);
    for i in 0..4 {
        for j in 0..4 {
            emit_u32(t[i][j]);
        }
    }

    let mut a = [0u8; 1024];
    for i in 0..a.len() {
        a[i] = (seed.wrapping_mul(i as u64 + 1) >> 7) as u8;
    }
    // Overlapping copy: memmove semantics, forward and backward overlap.
    a.copy_within(0..512, 256);
    a.copy_within(600..800, 100);
    let b = a.map(|x| x.rotate_left(3));
    let mut c = b;
    c.reverse();
    c.rotate_left(17);
    let mut sum = 0u64;
    for i in 0..c.len() {
        sum = sum.wrapping_add(c[i] as u64).rotate_left(1);
    }
    emit_u64(sum);
    emit_u8(c[0]);
    emit_u8(c[512]);
    emit_u8(c[1023]);

    let mut small = [0i16; 32];
    for i in 0..small.len() {
        small[i] = (i as i16).wrapping_mul(seed as i16).wrapping_sub(7);
    }
    small.swap(0, 31);
    small[4..12].reverse();
    small[8..16].sort_unstable();
    emit_i16(small.iter().fold(0i16, |x, &y| x.wrapping_add(y)));
    emit_i16(small[0] ^ small[31]);

    emit_usize(core::mem::size_of::<[u8; 1024]>());
    emit_usize(core::mem::size_of::<[[u32; 4]; 4]>());
    emit_usize(core::mem::align_of::<[u64; 3]>());
    emit_usize(core::mem::size_of_val(&c));

    sum ^ (small[0] as u64)
}
