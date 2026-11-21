// CRC-32 (runtime-built table), Fletcher checksums and byte folds.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xedb8_8320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
}

fn crc32(table: &[u32; 256], bytes: &[u8]) -> u32 {
    let mut c = 0xffff_ffffu32;
    for &b in bytes {
        c = table[((c ^ b as u32) & 0xff) as usize] ^ (c >> 8);
    }
    c ^ 0xffff_ffff
}

fn fletcher16(bytes: &[u8]) -> u16 {
    let (mut a, mut b) = (0u16, 0u16);
    for &x in bytes {
        a = (a + x as u16) % 255;
        b = (b + a) % 255;
    }
    (b << 8) | a
}

fn fletcher32(words: &[u16]) -> u32 {
    let (mut a, mut b) = (0u32, 0u32);
    for &x in words {
        a = (a + x as u32) % 65535;
        b = (b + a) % 65535;
    }
    (b << 16) | a
}

fn body(seed: u64) -> u64 {
    let table = crc32_table();
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for len in [0usize, 1, 5, 16, 63, 64, 255, 1024, 4096] {
        let bytes: alloc::vec::Vec<u8> = (0..len).map(|_| rng.next_u64() as u8).collect();
        let crc = crc32(&table, &bytes);
        emit_usize(len);
        emit_u32(crc);
        emit_u16(fletcher16(&bytes));
        emit_u64(bytes.iter().fold(0u64, |a, &b| a.wrapping_mul(31).wrapping_add(b as u64)));
        acc ^= (crc as u64).rotate_left((len & 63) as u32) ^ fletcher16(&bytes) as u64;
    }

    let words: alloc::vec::Vec<u16> = (0..128).map(|_| rng.next_u64() as u16).collect();
    emit_u32(fletcher32(&words));
    emit_u32(crc32(&table, &[0x00, 0x01, 0x02, 0x03]));
    emit_u32(crc32(&table, b"123456789"));
    emit_u32(crc32(&table, b"the quick brown fox jumps over the lazy dog"));
    // Streaming CRC equals one-shot.
    let data: alloc::vec::Vec<u8> = (0..1000).map(|_| rng.next_u64() as u8).collect();
    emit_u32(crc32(&table, &data));
    let mut streamed = 0xffff_ffffu32;
    for chunk in data.chunks(7) {
        for &b in chunk {
            streamed = table[((streamed ^ b as u32) & 0xff) as usize] ^ (streamed >> 8);
        }
    }
    emit_u32(streamed ^ 0xffff_ffff);
    emit_bool((streamed ^ 0xffff_ffff) == crc32(&table, &data));

    acc
}
