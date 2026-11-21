// Integer and f64 matrix multiplication, transpose, trace and determinant.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

const N: usize = 8;

fn matmul_u64(a: &[[u64; N]; N], b: &[[u64; N]; N]) -> [[u64; N]; N] {
    let mut c = [[0u64; N]; N];
    for i in 0..N {
        for j in 0..N {
            let mut s = 0u64;
            for k in 0..N {
                s = s.wrapping_add(a[i][k].wrapping_mul(b[k][j]));
            }
            c[i][j] = s;
        }
    }
    c
}

fn transpose(a: &[[u64; N]; N]) -> [[u64; N]; N] {
    let mut t = [[0u64; N]; N];
    for i in 0..N {
        for j in 0..N {
            t[j][i] = a[i][j];
        }
    }
    t
}

fn matmul_f64(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut c = [[0.0f64; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let mut s = 0.0;
            for k in 0..4 {
                s += a[i][k] * b[k][j];
            }
            c[i][j] = s;
        }
    }
    c
}

fn determinant(mut a: [[f64; 4]; 4]) -> f64 {
    let mut det = 1.0;
    for col in 0..4 {
        let mut pivot = col;
        for r in col + 1..4 {
            if a[r][col].abs() > a[pivot][col].abs() {
                pivot = r;
            }
        }
        if a[pivot][col] == 0.0 {
            return 0.0;
        }
        if pivot != col {
            a.swap(pivot, col);
            det = -det;
        }
        det *= a[col][col];
        let p = a[col][col];
        for r in col + 1..4 {
            let f = a[r][col] / p;
            for c in col..4 {
                a[r][c] -= f * a[col][c];
            }
        }
    }
    det
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut a = [[0u64; N]; N];
    let mut b = [[0u64; N]; N];
    for i in 0..N {
        for j in 0..N {
            a[i][j] = rng.next_u64() % 1000;
            b[i][j] = rng.next_u64() % 1000;
        }
    }
    let c = matmul_u64(&a, &b);
    let t = transpose(&a);
    let mut checksum = 0u64;
    for i in 0..N {
        for j in 0..N {
            checksum = checksum.wrapping_add(c[i][j].rotate_left((i + j) as u32));
            emit_u64(c[i][j]);
        }
    }
    emit_u64(checksum);
    emit_u64(t[0][7] ^ t[7][0]);
    emit_u64((0..N).map(|i| a[i][i]).sum());

    let mut fa = [[0.0f64; 4]; 4];
    let mut fb = [[0.0f64; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            fa[i][j] = ((rng.next_u64() % 2001) as f64 - 1000.0) / 256.0;
            fb[i][j] = ((rng.next_u64() % 2001) as f64 - 1000.0) / 512.0;
        }
    }
    let fc = matmul_f64(&fa, &fb);
    for i in 0..4 {
        for j in 0..4 {
            emit_f64(fc[i][j]);
        }
    }
    emit_f64(determinant(fa));
    emit_f64(determinant(fb));
    emit_f64(determinant(fc));

    checksum ^ fc[0][0].to_bits() ^ fc[3][3].to_bits()
}
