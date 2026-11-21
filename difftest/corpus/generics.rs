// Generics, trait bounds, operator overloading, generic methods and const
// generics: many monomorphizations of the same source.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

use core::ops::{Add, Mul};

trait Num: Copy + PartialOrd {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn from_u64(x: u64) -> Self;
    fn to_u64(self) -> u64;
}

macro_rules! num_impl {
    ($t:ty, $conv:expr, $back:expr) => {
        impl Num for $t {
            fn zero() -> Self {
                0 as $t
            }
            fn one() -> Self {
                1 as $t
            }
            fn add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
            fn mul(self, other: Self) -> Self {
                self.wrapping_mul(other)
            }
            fn from_u64(x: u64) -> Self {
                let f: fn(u64) -> $t = $conv;
                f(x)
            }
            fn to_u64(self) -> u64 {
                let f: fn($t) -> u64 = $back;
                f(self)
            }
        }
    };
}

num_impl!(u64, |x| x, |x| x);
num_impl!(i64, |x| x as i64, |x: i64| x as u64);
num_impl!(u32, |x| x as u32, |x: u32| x as u64);
num_impl!(i32, |x| x as i32, |x: i32| x as u64);

fn poly_sum<T: Num>(n: u64, seed: u64) -> T {
    let mut acc = T::zero();
    let mut i = 0;
    while i < n {
        acc = acc.add(T::from_u64(seed.wrapping_add(i).wrapping_mul(i + 3)));
        if i % 7 == 0 {
            acc = acc.mul(T::from_u64(3));
        }
        i += 1;
    }
    acc
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair<T> {
    a: T,
    b: T,
}

impl<T: Add<Output = T> + Copy> Add for Pair<T> {
    type Output = Pair<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Pair {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Pair<T> {
    type Output = Pair<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Pair {
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}

fn max_of<T: PartialOrd + Copy>(xs: &[T]) -> Option<T> {
    let mut best = *xs.first()?;
    for &x in &xs[1..] {
        if x > best {
            best = x;
        }
    }
    Some(best)
}

struct Arr<const N: usize> {
    data: [u64; N],
}

impl<const N: usize> Arr<N> {
    fn new(seed: u64) -> Self {
        let mut data = [0u64; N];
        for i in 0..N {
            data[i] = seed.wrapping_mul(i as u64 + 1) ^ (i as u64).rotate_left(13);
        }
        Arr { data }
    }
    fn sum(&self) -> u64 {
        self.data.iter().fold(0u64, |a, &b| a.wrapping_add(b))
    }
    fn first(&self) -> u64 {
        self.data[0]
    }
}

fn generic_pair<T: Num + core::fmt::Debug>(x: T) -> (T, T) {
    (x, x.add(T::one()))
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..8u64 {
        let a: u64 = poly_sum(16 + i, rng.next_u64());
        let b: i64 = poly_sum(16 + i, rng.next_u64());
        let c: u32 = poly_sum(12, rng.next_u64());
        let d: i32 = poly_sum(12, rng.next_u64());
        emit_u64(a);
        emit_i64(b);
        emit_u32(c);
        emit_i32(d);
        acc ^= a ^ b as u64 ^ c as u64 ^ d as u64;
    }

    let p = Pair {
        a: seed,
        b: seed.rotate_left(9),
    };
    let q = p + p;
    let r = q * 3u64;
    emit_u64(r.a);
    emit_u64(r.b);
    emit_bool(p < q);

    let xs = [seed ^ 7, seed, seed.rotate_left(3), 0];
    emit_u64(max_of(&xs).unwrap_or(0));
    let ys = [-5i32, -9, -1, -20];
    emit_i32(max_of(&ys).unwrap_or(0));

    let a3 = Arr::<3>::new(seed);
    let a8 = Arr::<8>::new(seed);
    let a33 = Arr::<33>::new(seed);
    emit_u64(a3.sum());
    emit_u64(a8.sum());
    emit_u64(a33.first() ^ a33.sum());

    let (x, y) = generic_pair(seed as u32);
    emit_u32(x);
    emit_u32(y);

    acc ^ r.a ^ a8.sum()
}
