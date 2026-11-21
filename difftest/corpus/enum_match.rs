// Enums with data, pattern matching, guards, @ bindings and exhaustive matches.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

enum Expr {
    Lit(u64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Cond(bool, Box<Expr>, Box<Expr>),
    Fold(Vec<u64>),
}

fn eval(e: &Expr) -> u64 {
    match e {
        Expr::Lit(x) => *x,
        Expr::Add(a, b) => eval(a).wrapping_add(eval(b)),
        Expr::Mul(a, b) => eval(a).wrapping_mul(eval(b)),
        Expr::Neg(a) => eval(a).wrapping_neg(),
        Expr::Cond(c, a, b) => {
            if *c {
                eval(a)
            } else {
                eval(b)
            }
        }
        Expr::Fold(xs) => xs.iter().fold(0u64, |a, &b| a.wrapping_add(b.rotate_left(3))),
    }
}

enum Msg {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
    Nothing,
}

fn handle(m: &Msg) -> u64 {
    match m {
        Msg::Quit => 0,
        Msg::Move { x, y } => (*x as u64).wrapping_mul(31) ^ (*y as u64),
        Msg::Write(s) => s.len() as u64,
        Msg::Color(r, g, b) => (*r as u64) << 16 | (*g as u64) << 8 | *b as u64,
        Msg::Nothing => 0xffff_ffff,
    }
}

#[derive(Clone, Copy)]
enum Tiny {
    A = 1,
    B = 2,
    C = 3,
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;

    for i in 0..12u64 {
        let e = match i % 4 {
            0 => Expr::Add(
                Box::new(Expr::Lit(rng.next_u64() % 1000)),
                Box::new(Expr::Mul(
                    Box::new(Expr::Lit(3)),
                    Box::new(Expr::Lit(rng.next_u64() % 100)),
                )),
            ),
            1 => Expr::Neg(Box::new(Expr::Lit(rng.next_u64() % 1000))),
            2 => Expr::Cond(
                i & 1 == 0,
                Box::new(Expr::Lit(seed)),
                Box::new(Expr::Lit(i)),
            ),
            _ => Expr::Fold((0..8).map(|j| rng.next_u64() ^ j).collect()),
        };
        let v = eval(&e);
        emit_u64(v);
        acc = acc.wrapping_add(v);
    }

    let msgs = [
        Msg::Quit,
        Msg::Move { x: -3, y: 17 },
        Msg::Write(String::from("hello enum")),
        Msg::Color(1, 2, 3),
        Msg::Nothing,
    ];
    for m in &msgs {
        emit_u64(handle(m));
    }

    // Ranges, alternatives, guards and bindings.
    for x in [0u64, 1, 2, 3, 4, 5, 6, 7, 8, 100, u64::MAX] {
        let r = match x {
            0 => 10,
            1..=3 => 20,
            4 | 5 | 6 => 30,
            n if n > 1000 => 40,
            n @ 7..=9 => 50 + n,
            _ => 60,
        };
        emit_u64(r);
    }

    // Nested tuple patterns and optional values.
    let pairs = [(1u64, 2u64), (3, 4), (0, 9), (5, 0)];
    for (a, b) in pairs {
        let label = match (a, b) {
            (0, _) => 0u8,
            (_, 0) => 1,
            (x, y) if x == y => 2,
            (x, y) if x < y => 3,
            _ => 4,
        };
        emit_u8(label);
    }

    // Enum with explicit discriminants.
    for i in 0..6 {
        let t = match i % 3 {
            0 => Tiny::A,
            1 => Tiny::B,
            _ => Tiny::C,
        };
        emit_u8(t as u8);
        emit_u8(match t {
            Tiny::A => 0,
            Tiny::B => 1,
            Tiny::C => 2,
        });
    }

    // while let over a Vec of Options.
    let opts: Vec<Option<u64>> = (0..10)
        .map(|i| if i % 3 == 0 { None } else { Some(i * i) })
        .collect();
    let mut oi = opts.into_iter();
    while let Some(o) = oi.next() {
        if let Some(x) = o {
            emit_u64(x);
            acc ^= x;
        }
    }

    // if let and match on a reference.
    let some = Some(seed);
    if let Some(x) = some {
        emit_u64(x.rotate_left(5));
        acc = acc.wrapping_add(x);
    }
    match &some {
        Some(x) => emit_u64(*x ^ 1),
        None => emit_u64(0),
    }

    acc ^ (msgs.len() as u64)
}
