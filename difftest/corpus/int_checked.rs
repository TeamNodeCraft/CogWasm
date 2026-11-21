// checked_* / overflowing_* across integer widths, including the edges.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn opt_u64(o: Option<u64>) {
    match o {
        Some(v) => {
            emit_bool(true);
            emit_u64(v);
        }
        None => {
            emit_bool(false);
            emit_u64(0xdead_beef_dead_beef);
        }
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for _ in 0..16 {
        let x = rng.next_u64();
        let y = rng.next_u64() | 1;
        opt_u64(x.checked_add(y));
        opt_u64(x.checked_sub(y));
        opt_u64(x.checked_mul(y));
        opt_u64(x.checked_div(y));
        opt_u64(x.checked_rem(y));
        opt_u64(x.checked_pow((y & 7) as u32));
        opt_u64(x.checked_shl((y & 63) as u32));
        opt_u64(x.checked_shr((y & 63) as u32));
        let (v, o) = x.overflowing_add(y);
        emit_u64(v);
        emit_bool(o);

        let sx = x as i64;
        let sy = (y as i64) | 1;
        emit_bool(sx.checked_add(sy).is_some());
        emit_bool(sx.checked_mul(sy).is_some());
        emit_bool(sx.checked_neg().is_some());
        emit_bool(sx.checked_abs().is_some());
        emit_i64(sx.saturating_mul(sy));

        let a8 = x as i8;
        let b8 = (y as i8) | 1;
        emit_bool(a8.checked_add(b8).is_some());
        emit_bool(a8.checked_mul(b8).is_some());
        emit_bool(a8.checked_div(b8).is_some());

        emit_bool(i32::MIN.checked_div(-1).is_some());
        emit_bool(i64::MIN.checked_rem(-1).is_some());

        acc ^= v.rotate_left((x & 63) as u32) ^ (o as u64);
    }
    emit_u64(acc);
    acc
}
