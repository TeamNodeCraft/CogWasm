#![no_std]

include!("../prelude.rs");

use alloc::vec::Vec;

const N: usize = 128;

#[inline]
fn rand_unit(rng: &mut Rng) -> f64 {
    (rng.below(2_000_001) as i64 - 1_000_000) as f64 * 1.0e-6
}

fn matmul(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    let mut i = 0;
    while i < n {
        let row = i * n;
        let mut j = 0;
        while j < n {
            c[row + j] = 0.0;
            j += 1;
        }
        let mut k = 0;
        while k < n {
            let aik = a[row + k];
            let brow = k * n;
            let mut j = 0;
            while j < n {
                c[row + j] += aik * b[brow + j];
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let total = N * N;
    let mut a: Vec<f64> = Vec::with_capacity(total);
    let mut b: Vec<f64> = Vec::with_capacity(total);
    let mut c: Vec<f64> = Vec::with_capacity(total);

    let mut i = 0;
    while i < total {
        a.push(rand_unit(&mut rng));
        b.push(rand_unit(&mut rng));
        c.push(0.0);
        i += 1;
    }

    matmul(&a, &b, &mut c, N);

    let mut sum = 0.0f64;
    let mut trace = 0.0f64;
    let mut weighted = 0.0f64;
    let mut hash = 0u64;
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let idx = i * N + j;
            let value = c[idx];
            sum += value;
            if i == j {
                trace += value;
            }
            weighted += value * (idx as f64 * 1.0e-4);
            hash = hash.rotate_left(3) ^ value.to_bits();
            j += 1;
        }
        i += 1;
    }

    emit_u64(N as u64);
    emit_f64(sum);
    emit_f64(trace);
    emit_f64(weighted);
    emit_u64(hash);
    emit_f64(c[0]);
    emit_f64(c[N / 2 * N + N / 2]);
    emit_f64(c[total - 1]);

    hash ^ sum.to_bits().rotate_left(11)
        ^ trace.to_bits().rotate_left(31)
        ^ weighted.to_bits().rotate_left(47)
}
