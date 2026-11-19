//! A.2 (f) host-call-heavy geometry, in the benchmark ABI: 40k iterations with
//! exactly five `nodecraft` imports each (200k calls). The runner provides the
//! same five host functions to LLVM and to the Cranelift build.
#![no_std]

include!("../prelude.rs");

#[link(wasm_import_module = "nodecraft")]
unsafe extern "C" {
    fn dot2(x1: f64, y1: f64, x2: f64, y2: f64) -> f64;
    fn length2(x: f64, y: f64) -> f64;
    fn clamp01(x: f64) -> f64;
    fn scale(x: f64, k: f64) -> f64;
    fn mix(a: f64, b: f64, t: f64) -> f64;
}

static PTS: [f64; 16] = [
    0.0, 1.0, -1.0, 0.5, -0.5, 2.0, -2.0, 0.25, 0.75, -0.75, 1.5, -1.5, 0.125, -0.125, 3.0, -3.0,
];

fn body(seed: u64) -> u64 {
    let n = 40_000u32;
    let offset = (seed % 17) as f64 * 1.0e-6;
    let mut acc = 0.0f64;
    let mut i = 0u32;
    while i < n {
        let j = (i as usize) & 7;
        let x = PTS[j];
        let y = PTS[j + 8];
        let t = (i as f64) * 1.0e-6 + offset;
        let a = unsafe { dot2(x, y, 0.5, 0.25) };
        let b = unsafe { length2(a + t, x) };
        let c = unsafe { clamp01(b * 0.001) };
        let d = unsafe { scale(c, 2.0) };
        let e = unsafe { mix(d, y, 0.5) };
        acc += e;
        i += 1;
    }
    emit_f64(acc);
    acc.to_bits()
}
