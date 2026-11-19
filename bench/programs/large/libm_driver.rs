//! Large compile workload: a driver linked against the vendored `libm` crate
//! (0.2.16), compiled from source as an rlib by each compiler. The program
//! exercises many libm entry points over seed-derived data, so it also checks
//! that the two pipelines agree on the maths results.
#![no_std]

extern crate libm;

include!("../prelude.rs");

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let n = 20_000usize;
    let mut acc = 0.0f64;
    let mut bits = 0u64;
    for i in 0..n {
        let x = (i as f64 - n as f64 / 2.0) * 0.000_5 + (rng.next_u32() & 0x3ff) as f64 * 1e-9;
        let a = libm::sin(x) + libm::cos(x * 0.25);
        let b = libm::sqrt(libm::fabs(x)) + libm::atan(x);
        let c = libm::exp(-libm::fabs(x) * 0.001) + libm::log1p(libm::fabs(x));
        let d = libm::pow(libm::fabs(x) + 1.0, 0.5);
        acc += a + b + c + d;
        bits ^= (a + b).to_bits() ^ (c * d).to_bits().rotate_left(17);
        if i % 4096 == 0 {
            emit_f64(acc);
        }
    }
    emit_f64(acc);
    emit_u64(bits);
    acc.to_bits() ^ bits
}
