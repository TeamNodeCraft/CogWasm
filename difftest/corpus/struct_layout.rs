// Struct/enum layout, sizes/alignments, field access and byte serialization.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

#[repr(C)]
#[derive(Clone, Copy)]
struct Mix {
    a: u8,
    b: u64,
    c: u32,
    d: [u8; 3],
}

#[repr(C)]
struct Nested {
    x: Mix,
    y: i16,
    z: u8,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Tag {
    A = 1,
    B = 7,
    C = 200,
}

#[repr(C)]
union Bits {
    u: u32,
    f: f32,
}

fn body(seed: u64) -> u64 {
    emit_usize(core::mem::size_of::<Mix>());
    emit_usize(core::mem::align_of::<Mix>());
    emit_usize(core::mem::size_of::<Nested>());
    emit_usize(core::mem::align_of::<Nested>());
    emit_usize(core::mem::size_of::<u8>());
    emit_usize(core::mem::size_of::<u128>());
    emit_usize(core::mem::size_of::<*const u8>());
    emit_usize(core::mem::size_of::<Option<core::num::NonZeroU64>>());
    emit_usize(core::mem::size_of::<Option<&u64>>());
    emit_usize(core::mem::size_of::<fn(u64) -> u64>());

    let m = Mix {
        a: seed as u8,
        b: seed.rotate_left(13),
        c: (seed >> 7) as u32,
        d: [1, 2, 3],
    };
    emit_u8(m.a);
    emit_u64(m.b);
    emit_u32(m.c);
    emit_u8(m.d[0] ^ m.d[2]);

    let n = Nested {
        x: m,
        y: -1234,
        z: 0x5a,
    };
    emit_u64(n.x.b ^ n.x.c as u64);
    emit_i16(n.y);
    emit_u8(n.z);

    let mut ms = [m; 8];
    for i in 0..ms.len() {
        ms[i].a = ms[i].a.wrapping_add(i as u8);
        ms[i].b ^= i as u64;
    }
    let mut h = 0u64;
    for e in &ms {
        h = h.wrapping_mul(131).wrapping_add(e.b ^ e.a as u64);
    }
    emit_u64(h);

    // Manual serialization of the layout (endianness via to_le_bytes).
    let mut bytes = [0u8; 16];
    bytes[0] = m.a;
    bytes[1..9].copy_from_slice(&m.b.to_le_bytes());
    bytes[9..12].copy_from_slice(&m.d);
    bytes[12..16].copy_from_slice(&m.c.to_le_bytes());
    emit_bytes(&bytes);
    emit_u32(u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]));

    emit_u8(Tag::A as u8);
    emit_u8(Tag::B as u8);
    emit_u8(Tag::C as u8);
    let t = if seed & 1 == 0 { Tag::B } else { Tag::C };
    match t {
        Tag::A => emit_u8(0),
        Tag::B => emit_u8(1),
        Tag::C => emit_u8(2),
    }

    let u = Bits { u: 0x3f80_0000 };
    emit_u32(unsafe { u.u });
    emit_f32(f32::from_bits(0x3f80_0000));
    let f = Bits { f: 1.5 };
    emit_u32(unsafe { f.u });

    emit_usize(core::mem::offset_of!(Mix, a));
    emit_usize(core::mem::offset_of!(Mix, b));
    emit_usize(core::mem::offset_of!(Mix, c));
    emit_usize(core::mem::offset_of!(Mix, d));

    h ^ m.b ^ n.y as u64
}
