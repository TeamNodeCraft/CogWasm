//! Small program mirroring `guests/b_alloc`:
//! no_std + alloc with `Vec`, `String`, `format!`, closures and trait objects.
//! The difftest prelude supplies the growable bump allocator and the output
//! buffer, so LLVM and the Cranelift pipeline build the same source.
#![no_std]


use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

include!("../prelude.rs");

trait Op {
    fn apply(&self, x: f64) -> f64;
}

struct Scale(f64);

impl Op for Scale {
    fn apply(&self, x: f64) -> f64 {
        x * self.0
    }
}

fn bench(n: u32) -> f64 {
    let base: Vec<f64> = (0..64).map(|i| i as f64 * 0.25).collect();
    let name = String::from("cogwasm");
    let label = format!("{name}-plugin");
    let bias = label.len() as f64;
    let ops: Vec<Box<dyn Op>> = alloc::vec![Box::new(Scale(1.5)), Box::new(Scale(0.5))];
    let add = |x: f64| x + bias;

    let mut acc = 0.0f64;
    let mut i = 1u32;
    while i <= n {
        let x = (i as f64) * 0.001 + base[(i as usize) & 63];
        let mut y = add(x);
        for op in &ops {
            y = op.apply(y);
        }
        // Same shape as the stage-C program without the libm dependency; the
        // A.2 maths program covers libm calls.
        acc += (y.abs() * 0.5 + y) * 0.999;
        i = i.wrapping_add(1);
    }
    acc / (n as f64)
}

fn body(seed: u64) -> u64 {
    let n = 1_000 + (seed % 2_000) as u32;
    let value = bench(n);
    emit_f64(value);
    value.to_bits()
}
