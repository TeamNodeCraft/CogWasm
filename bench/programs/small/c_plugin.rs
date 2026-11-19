//! Small program mirroring `experiments/cranelift-wasm/stage-c/programs/c_plugin`:
//! a NodeCraft-style plugin with an explicit `nodecraft::host_bias` import, an
//! f64 parameter list and a result written through linear memory. The difftest
//! prelude provides the growable heap and output buffer.
#![no_std]

include!("../prelude.rs");

#[link(wasm_import_module = "nodecraft")]
unsafe extern "C" {
    fn host_bias(x: f64) -> f64;
}

#[inline]
fn eval(params: &[f64]) -> f64 {
    let mut adjusted = [0.0f64; 4];
    for (i, &p) in params.iter().enumerate() {
        adjusted[i] = unsafe { host_bias(p) };
    }
    let sum: f64 = adjusted.iter().sum();
    sum / (params.len() as f64)
}

fn body(seed: u64) -> u64 {
    let bias = (seed % 97) as f64 * 0.125;
    let params = [1.0 + bias, 2.0 - bias, 3.0 + bias, 4.0 - bias];
    let value = eval(&params);
    emit_f64(value);
    value.to_bits()
}
