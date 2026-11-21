// Option/Result combinators and the `?` operator in helper functions.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::string::String;

#[derive(Clone, Copy)]
enum Err {
    Empty,
    TooBig(u64),
    Bad,
}

fn checked_half(x: u64) -> Option<u64> {
    if x == 0 {
        None
    } else {
        Some(x / 2)
    }
}

fn chain(x: u64) -> Option<u64> {
    let a = checked_half(x)?;
    let b = checked_half(a)?;
    checked_half(b)
}

fn parse_like(x: u64) -> Result<u64, Err> {
    if x == 0 {
        return Err(Err::Empty);
    }
    if x > 1_000_000 {
        return Err(Err::TooBig(x));
    }
    if x % 7 == 0 {
        return Err(Err::Bad);
    }
    Ok(x.rotate_left(3))
}

fn result_chain(x: u64) -> Result<u64, Err> {
    let a = parse_like(x)?;
    let b = parse_like(a)?;
    Ok(a.wrapping_add(b))
}

fn body(seed: u64) -> u64 {
    let mut acc = 0u64;

    for x in [0u64, 1, 2, 8, 16, 64, 1000] {
        match checked_half(x) {
            Some(v) => {
                emit_bool(true);
                emit_u64(v);
            }
            None => emit_bool(false),
        }
    }
    emit_u64(chain(seed).unwrap_or(0));
    emit_u64(checked_half(seed).map(|x| x + 1).unwrap_or(0));
    emit_u64(checked_half(seed).and_then(|x| checked_half(x)).unwrap_or(9));
    emit_u64(checked_half(seed).or_else(|| Some(7)).unwrap_or(0));
    emit_u64(checked_half(seed).filter(|x| x & 1 == 0).unwrap_or(0));
    emit_u64(Some(seed).zip(Some(3)).map(|(a, b)| a ^ b).unwrap_or(0));
    emit_u64(checked_half(seed).xor(Some(5)).unwrap_or(0));

    let mut slot = checked_half(seed);
    let taken = slot.take();
    emit_u64(taken.unwrap_or(0));
    emit_bool(slot.is_none());
    slot.replace(42);
    emit_u64(slot.unwrap_or(0));
    let replaced = slot.replace(43);
    emit_u64(replaced.unwrap_or(0));
    emit_bool(slot.is_some_and(|x| x > 40));
    emit_u64(slot.unwrap_or(0));

    for x in [0u64, 1, 7, 8, 14, 123_456, 2_000_000, 99] {
        match result_chain(x) {
            Ok(v) => {
                emit_bool(true);
                emit_u64(v);
            }
            Err(Err::Empty) => {
                emit_bool(false);
                emit_u8(0);
            }
            Err(Err::TooBig(v)) => {
                emit_bool(false);
                emit_u8(1);
                emit_u64(v);
            }
            Err(Err::Bad) => {
                emit_bool(false);
                emit_u8(2);
            }
        }
        emit_u64(result_chain(x).map_err(|e| match e {
            Err::Empty => 1,
            Err::TooBig(v) => v,
            Err::Bad => 3,
        }).unwrap_or(0));
        emit_bool(result_chain(x).is_ok());
        emit_bool(result_chain(x).is_err());
        emit_u64(result_chain(x).ok().unwrap_or(0));
        emit_u8(match result_chain(x).err() {
            Some(Err::Empty) => 0,
            Some(Err::TooBig(_)) => 1,
            Some(Err::Bad) => 2,
            None => 3,
        });
    }

    let opt: Option<String> = Some(String::from("held"));
    let mapped = opt.as_deref().map(|s| s.len() as u64).unwrap_or(0);
    emit_u64(mapped);
    let none: Option<u64> = None;
    emit_u64(none.unwrap_or_else(|| seed & 0xff));
    emit_u64(none.map_or(11, |x| x + 1));
    emit_u64(opt.is_some().then_some(seed).unwrap_or(0));

    // and_then/or_else chains.
    let chained = checked_half(seed)
        .and_then(checked_half)
        .or_else(|| Some(seed % 13))
        .map(|x| x.wrapping_mul(3));
    emit_u64(chained.unwrap_or(0));

    let (a, b, c): (Option<u8>, Option<u8>, Option<u8>) = (Some(1), None, Some(3));
    emit_u64(a.unwrap_or(0) as u64 + b.unwrap_or(0) as u64 + c.unwrap_or(0) as u64);

    acc ^ mapped ^ chained.unwrap_or(0)
}
