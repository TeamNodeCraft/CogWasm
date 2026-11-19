#![no_std]

include!("../prelude.rs");

use alloc::vec::Vec;

const N: usize = 128;
const STEPS: usize = 300;
const DT: f64 = 0.001;
const SOFTENING: f64 = 0.02;

/// Square root by Newton iteration.  The exponent-halving bit trick supplies
/// the initial estimate; the loop is plain IEEE arithmetic, no libm.
#[inline]
fn sqrt_newton(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut y = f64::from_bits((x.to_bits() >> 1) + 0x1ff8_0000_0000_0000);
    let mut i = 0;
    while i < 6 {
        y = 0.5 * (y + x / y);
        i += 1;
    }
    y
}

#[inline]
fn rand_unit(rng: &mut Rng) -> f64 {
    // Uniform in [-1, 1], with a resolution of 1e-6.
    (rng.below(2_000_001) as i64 - 1_000_000) as f64 * 1.0e-6
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut pos: Vec<[f64; 3]> = Vec::with_capacity(N);
    let mut vel: Vec<[f64; 3]> = Vec::with_capacity(N);
    let mut mass: Vec<f64> = Vec::with_capacity(N);
    let mut acc: Vec<[f64; 3]> = Vec::with_capacity(N);

    for _ in 0..N {
        pos.push([
            rand_unit(&mut rng),
            rand_unit(&mut rng),
            rand_unit(&mut rng),
        ]);
        vel.push([
            rand_unit(&mut rng) * 1.0e-2,
            rand_unit(&mut rng) * 1.0e-2,
            rand_unit(&mut rng) * 1.0e-2,
        ]);
        mass.push(0.5 + rng.below(1_000_001) as f64 * 1.0e-6);
        acc.push([0.0, 0.0, 0.0]);
    }

    let soft2 = SOFTENING * SOFTENING;
    let mut step = 0;
    while step < STEPS {
        for a in acc.iter_mut() {
            *a = [0.0, 0.0, 0.0];
        }
        let mut i = 0;
        while i < N {
            let mut j = i + 1;
            while j < N {
                let dx = pos[j][0] - pos[i][0];
                let dy = pos[j][1] - pos[i][1];
                let dz = pos[j][2] - pos[i][2];
                let r2 = dx * dx + dy * dy + dz * dz + soft2;
                let inv_r = 1.0 / sqrt_newton(r2);
                let inv_r3 = inv_r * inv_r * inv_r;
                let fi = mass[j] * inv_r3;
                let fj = mass[i] * inv_r3;
                acc[i][0] += dx * fi;
                acc[i][1] += dy * fi;
                acc[i][2] += dz * fi;
                acc[j][0] -= dx * fj;
                acc[j][1] -= dy * fj;
                acc[j][2] -= dz * fj;
                j += 1;
            }
            i += 1;
        }
        let mut i = 0;
        while i < N {
            vel[i][0] += acc[i][0] * DT;
            vel[i][1] += acc[i][1] * DT;
            vel[i][2] += acc[i][2] * DT;
            pos[i][0] += vel[i][0] * DT;
            pos[i][1] += vel[i][1] * DT;
            pos[i][2] += vel[i][2] * DT;
            i += 1;
        }
        step += 1;
    }

    let mut kinetic = 0.0f64;
    let mut potential = 0.0f64;
    let mut momentum = [0.0f64; 3];
    let mut center = [0.0f64; 3];
    let mut hash = 0u64;
    let mut i = 0;
    while i < N {
        let v2 = vel[i][0] * vel[i][0] + vel[i][1] * vel[i][1] + vel[i][2] * vel[i][2];
        kinetic += 0.5 * mass[i] * v2;
        let m = mass[i];
        momentum[0] += m * vel[i][0];
        momentum[1] += m * vel[i][1];
        momentum[2] += m * vel[i][2];
        center[0] += pos[i][0];
        center[1] += pos[i][1];
        center[2] += pos[i][2];

        let mut j = i + 1;
        while j < N {
            let dx = pos[j][0] - pos[i][0];
            let dy = pos[j][1] - pos[i][1];
            let dz = pos[j][2] - pos[i][2];
            let r = sqrt_newton(dx * dx + dy * dy + dz * dz + soft2);
            potential -= mass[i] * mass[j] / r;
            j += 1;
        }

        hash = hash.rotate_left(7) ^ pos[i][0].to_bits();
        hash = hash.rotate_left(7) ^ pos[i][1].to_bits();
        hash = hash.rotate_left(7) ^ pos[i][2].to_bits();
        hash = hash.rotate_left(7) ^ vel[i][0].to_bits();
        hash = hash.rotate_left(7) ^ vel[i][1].to_bits();
        hash = hash.rotate_left(7) ^ vel[i][2].to_bits();
        i += 1;
    }

    let total = kinetic + potential;
    emit_u64(N as u64);
    emit_u64(STEPS as u64);
    emit_f64(kinetic);
    emit_f64(potential);
    emit_f64(total);
    emit_f64(momentum[0]);
    emit_f64(momentum[1]);
    emit_f64(momentum[2]);
    emit_f64(center[0]);
    emit_f64(center[1]);
    emit_f64(center[2]);
    emit_u64(hash);

    hash ^ kinetic.to_bits().rotate_left(13) ^ potential.to_bits().rotate_left(29)
}
