// Recursion: fib, gcd, Ackermann, mutual recursion, recursive binary search,
// recursive data structures (tree) and a moderately deep call chain.
#![no_std]
#![allow(dead_code)]

include!("prelude.rs");

fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 1).wrapping_add(fib(n - 2))
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn ackermann(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}

fn is_even(n: u64) -> bool {
    if n == 0 {
        true
    } else {
        is_odd(n - 1)
    }
}

fn is_odd(n: u64) -> bool {
    if n == 0 {
        false
    } else {
        is_even(n - 1)
    }
}

fn bsearch(a: &[u64], key: u64, lo: usize, hi: usize) -> Option<usize> {
    if lo >= hi {
        return None;
    }
    let mid = lo + (hi - lo) / 2;
    if a[mid] == key {
        Some(mid)
    } else if a[mid] < key {
        bsearch(a, key, mid + 1, hi)
    } else {
        bsearch(a, key, lo, mid)
    }
}

#[derive(Clone, Copy)]
struct Node {
    key: u64,
    left: i32,
    right: i32,
}

fn tree_sum(nodes: &[Node], idx: i32) -> u64 {
    if idx < 0 {
        return 0;
    }
    let n = nodes[idx as usize];
    n.key
        .wrapping_add(tree_sum(nodes, n.left))
        .wrapping_add(tree_sum(nodes, n.right))
}

fn depth(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        1 + depth(n - 1)
    }
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let mut acc = 0u64;
    for i in 0..10u64 {
        emit_u64(fib(i + 10));
    }
    emit_u64(fib(25));
    emit_u64(gcd(seed | 1, rng.next_u64() | 1));
    emit_u64(gcd(1_000_000_000, 72));
    emit_u64(ackermann(2, 3));
    emit_u64(ackermann(3, 3));
    emit_bool(is_even(seed & 0xff));
    emit_bool(is_odd(seed & 0xff));

    let mut sorted: alloc::vec::Vec<u64> = (0..256).map(|_| rng.next_u64() % 10_000).collect();
    sorted.sort_unstable();
    for k in [0u64, 1, 42, 9999, 12345] {
        emit_u64(bsearch(&sorted, k, 0, sorted.len()).map(|x| x as u64).unwrap_or(u64::MAX));
    }

    let mut nodes = [Node {
        key: 0,
        left: -1,
        right: -1,
    }; 31];
    for i in 0..31 {
        nodes[i].key = rng.next_u64() % 1000;
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        if l < 31 {
            nodes[i].left = l as i32;
        }
        if r < 31 {
            nodes[i].right = r as i32;
        }
    }
    emit_u64(tree_sum(&nodes, 0));
    emit_u64(depth(400));

    acc = acc.wrapping_add(fib(20)).wrapping_add(tree_sum(&nodes, 0));
    emit_u64(acc);
    acc
}
