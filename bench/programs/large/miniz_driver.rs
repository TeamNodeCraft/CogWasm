//! Large compile workload: a driver linked against the vendored `miniz_oxide`
//! crate (0.9.1, no_std + alloc) and its `adler2` dependency. It compresses and
//! decompresses a seed-derived buffer and checks the round trip.
#![no_std]

extern crate miniz_oxide;

include!("../prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut input = alloc::vec::Vec::with_capacity(32 * 1024);
    for i in 0..32 * 1024u64 {
        let value = (rng.next_u64() ^ i.rotate_left(7)) as u8;
        input.push(value);
    }
    // A repeating prefix compresses well, so the compressor does real work.
    for i in 0..512u64 {
        input[i as usize] = (i % 7) as u8;
    }
    let compressed = miniz_oxide::deflate::compress_to_vec(&input, 6);
    let decompressed = miniz_oxide::inflate::decompress_to_vec(&compressed)
        .unwrap_or_else(|_| alloc::vec::Vec::new());
    let same = decompressed == input;
    let adler = miniz_oxide::mz_adler32_oxide(1, &input);
    emit_u64(compressed.len() as u64);
    emit_u64(decompressed.len() as u64);
    emit_bool(same);
    emit_u32(adler);
    let folded = compressed
        .iter()
        .fold(0u64, |acc, &byte| acc.wrapping_mul(31).wrapping_add(u64::from(byte)));
    folded ^ u64::from(same) ^ u64::from(adler)
}
