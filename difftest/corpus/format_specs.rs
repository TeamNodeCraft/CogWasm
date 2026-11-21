// core::fmt and alloc::format!: integer radices, float formatting, width,
// precision, debug output. Float formatting is a particularly dense path.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::format;
use alloc::string::String;
use core::fmt::Write as _;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}|{}|{}|{}|{}",
        seed,
        seed as i64,
        seed as i32,
        seed as u8,
        seed as i8
    );
    let _ = write!(out, "\n{:x}|{:X}|{:#x}|{:o}|{:b}", seed, seed, seed, seed, seed);
    let _ = write!(out, "\n{:08x}|{:>12}|{:<12}|{:^12}|", seed as u32, 42, 42, 42);
    let _ = write!(out, "\n{:+}|{:+}|{:+}", 17i32, -17i32, 0i32);
    let _ = write!(out, "\n{:.0}|{:.3}|{:.10}", 3.1415926535f64, 3.1415926535, 0.1f64);
    let _ = write!(out, "\n{:e}|{:E}", 12345.6789f64, 0.000123f64);
    let _ = write!(out, "\n{}|{}|{}", f64::INFINITY, f64::NEG_INFINITY, f64::NAN);
    let _ = write!(out, "\n{}|{}", -0.0f64, 1.0f32 / 3.0);
    let _ = write!(out, "\n{:?}|{:?}|{:?}", (1, "two", 3.5f64), [1, 2, 3], Some(seed));
    emit_str(&out);
    emit_usize(out.len());

    for i in 0..8u64 {
        let x = rng.next_u64();
        let s = format!(
            "{:016x}:{:032b}:{:o}:{}:{:.6}",
            x,
            x,
            x as u32,
            x as i64,
            (x as f64) / 1.0e6
        );
        emit_str(&s);
        emit_usize(s.len());
    }

    // Radix and precision formatting for the wide integers.
    let big = ((rng.next_u64() as u128) << 64) | rng.next_u64() as u128;
    let sb = format!("{:x}|{:o}|{:b}|{}", big, big, big, big);
    emit_str(&sb);
    emit_usize(sb.len());

    // Padding via fill/align.
    let padded = format!("{:*^40}", "difftest");
    emit_str(&padded);

    let mut n = 0u64;
    for b in out.bytes() {
        n = n.wrapping_mul(31).wrapping_add(b as u64);
    }
    emit_u64(n);
    n ^ out.len() as u64
}
