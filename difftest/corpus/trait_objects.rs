// Trait objects and dynamic dispatch through vtables.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use alloc::boxed::Box;
use alloc::vec::Vec;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn kind(&self) -> u8;
    fn scaled_area(&self, factor: f64) -> f64 {
        self.area() * factor
    }
    fn describe(&self) -> u64 {
        self.kind() as u64 * 1000 + self.area() as u64
    }
}

struct Circle {
    r: f64,
}

struct Rect {
    w: f64,
    h: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.141_592_653_589_793 * self.r * self.r
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.141_592_653_589_793 * self.r
    }
    fn kind(&self) -> u8 {
        1
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.w + self.h)
    }
    fn kind(&self) -> u8 {
        2
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula; inputs are a valid triangle.
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).max(0.0)
    }
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
    fn kind(&self) -> u8 {
        3
    }
    fn describe(&self) -> u64 {
        0xdead_0000 | self.kind() as u64
    }
}

trait Accumulate {
    fn add(&mut self, x: u64);
    fn total(&self) -> u64;
    fn doubled(&self) -> u64 {
        self.total().wrapping_mul(2)
    }
}

struct Counter {
    n: u64,
    sum: u64,
}

impl Accumulate for Counter {
    fn add(&mut self, x: u64) {
        self.n += 1;
        self.sum = self.sum.wrapping_add(x ^ self.n);
    }
    fn total(&self) -> u64 {
        self.sum
    }
}

fn use_dyn(s: &dyn Shape) -> u64 {
    emit_f64(s.area());
    emit_f64(s.perimeter());
    emit_u8(s.kind());
    emit_u64(s.describe());
    s.area().to_bits() ^ s.perimeter().to_bits()
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    for i in 0..12u64 {
        let x = (rng.next_u64() % 1000) as f64 / 100.0 + 0.5;
        match i % 3 {
            0 => shapes.push(Box::new(Circle { r: x })),
            1 => shapes.push(Box::new(Rect { w: x, h: x + 1.5 })),
            _ => shapes.push(Box::new(Triangle {
                a: x,
                b: x + 1.0,
                c: x + 2.0,
            })),
        }
    }
    let mut acc = 0u64;
    let mut total_area = 0.0f64;
    for (i, s) in shapes.iter().enumerate() {
        acc ^= use_dyn(s.as_ref());
        total_area += s.area();
        acc ^= s.scaled_area(2.0).to_bits().rotate_left((i & 63) as u32);
    }
    emit_f64(total_area);
    emit_usize(shapes.len());

    // &dyn and &mut dyn.
    let mut c = Counter { n: 0, sum: 0 };
    {
        let d: &mut dyn Accumulate = &mut c;
        for i in 0..32u64 {
            d.add(i.wrapping_mul(seed));
        }
        emit_u64(d.total());
        emit_u64(d.doubled());
    }
    emit_u64(c.n);

    // Heterogeneous vector of trait references.
    let circle = Circle { r: 2.0 };
    let rect = Rect { w: 3.0, h: 4.0 };
    let refs: [&dyn Shape; 2] = [&circle, &rect];
    for s in refs {
        acc ^= s.area().to_bits();
    }

    acc ^ total_area.to_bits()
}
