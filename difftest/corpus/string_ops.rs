// String and &str operations that live in core/alloc (no Unicode case tables).
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::string::{String, ToString};
use alloc::vec::Vec;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut s = String::from("hello");
    s.push_str(", cranelift-wasm");
    s.push('!');
    emit_str(&s);
    emit_usize(s.len());

    let upper = s.to_ascii_uppercase();
    let lower = s.to_ascii_lowercase();
    emit_str(&upper);
    emit_str(&lower);

    s.insert_str(0, ">> ");
    s.replace_range(0..3, "==");
    emit_str(&s);
    emit_usize(s.find("crane").unwrap_or(usize::MAX));
    emit_usize(s.rfind('l').unwrap_or(usize::MAX));
    emit_bool(s.starts_with("=="));
    emit_bool(s.ends_with('!'));

    let mut acc = 0u64;
    for c in s.chars() {
        acc = acc.wrapping_mul(131).wrapping_add(c as u64);
        emit_u32(c as u32);
    }
    emit_u64(acc);

    let parts: Vec<&str> = s.split(' ').collect();
    emit_usize(parts.len());
    for p in &parts {
        emit_usize(p.len());
        emit_str(p);
    }
    let joined = parts.join("|");
    emit_str(&joined);

    let trimmed = "  \t padded \n ".trim();
    emit_str(trimmed);
    emit_usize(trimmed.len());
    let rep = "ab".repeat(5);
    emit_str(&rep);

    let bytes = b"the quick brown fox".to_vec();
    let text = String::from_utf8(bytes.clone()).unwrap();
    emit_str(&text);
    emit_u8(text.as_bytes()[4]);
    let lossy = String::from_utf8_lossy(&[0x66, 0x6f, 0x80, 0x6f]);
    emit_str(&lossy);

    let mut collected = String::new();
    for (i, c) in "abcxyz".chars().rev().enumerate() {
        if i & 1 == 0 {
            collected.push(c);
        } else {
            collected.push(c.to_ascii_uppercase());
        }
    }
    emit_str(&collected);

    let line = "alpha,beta,,gamma";
    let fields: Vec<&str> = line.split(',').map(|x| x.trim()).collect();
    emit_usize(fields.len());
    emit_usize(fields.iter().map(|x| x.len()).sum());

    let fmt = alloc::format!("{}-{}-{:04}", s.len(), rep, acc as u32);
    emit_str(&fmt);
    // Use the seed so each run is distinct but deterministic.
    emit_u64(seed ^ acc ^ rng.next_u64());
    seed.wrapping_add(acc) ^ fmt.len() as u64
}
