#![no_std]

include!("../prelude.rs");

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const MAX_ITER: u32 = 200;

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    // A tiny seed-dependent offset and zoom keep the image deterministic but
    // distinct for every input seed.
    let x_shift = (rng.below(20_001) as i64 - 10_000) as f64 * 1.0e-6;
    let y_shift = (rng.below(20_001) as i64 - 10_000) as f64 * 1.0e-6;
    let zoom = 1.0 + rng.below(50_001) as f64 * 1.0e-6;

    let mut histogram = [0u32; 32];
    let mut total_iters = 0u64;
    let mut inside = 0u64;
    let mut checksum = 0u64;

    let mut py = 0;
    while py < HEIGHT {
        let ci = (py as f64 / HEIGHT as f64 - 0.5) * (2.4 / zoom) + y_shift;
        let mut px = 0;
        while px < WIDTH {
            let cr = (px as f64 / WIDTH as f64 - 0.62) * (3.2 / zoom) + x_shift;
            let mut zr = 0.0f64;
            let mut zi = 0.0f64;
            let mut iter = 0u32;
            while iter < MAX_ITER {
                let zr2 = zr * zr;
                let zi2 = zi * zi;
                if zr2 + zi2 > 4.0 {
                    break;
                }
                zi = 2.0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;
                iter += 1;
            }

            total_iters += iter as u64;
            if iter >= MAX_ITER {
                inside += 1;
                histogram[31] += 1;
            } else {
                let bucket = (iter as usize * 31) / MAX_ITER as usize;
                histogram[bucket] += 1;
            }
            checksum = checksum.rotate_left(5)
                ^ (iter as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
                ^ (px as u64).rotate_left(11)
                ^ (py as u64).rotate_left(23);
            px += 1;
        }
        py += 1;
    }

    emit_u64(WIDTH as u64);
    emit_u64(HEIGHT as u64);
    emit_u64(MAX_ITER as u64);
    let mut i = 0;
    while i < histogram.len() {
        emit_u32(histogram[i]);
        i += 1;
    }
    emit_u64(total_iters);
    emit_u64(inside);
    emit_u64(checksum);

    checksum ^ total_iters.rotate_left(17) ^ inside.rotate_left(31)
}
