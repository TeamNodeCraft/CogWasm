//! Deterministic generator for arithmetic/control-flow corpus programs.
//!
//! For each generation seed it writes `<out>/gen_<seed:05>.rs`, all of which
//! include the shared `prelude.rs` (copied into `<out>`), plus a `manifest.tsv`
//! with the source checksum so a run can prove the generated input was the
//! same one that produced a previous result. Generated programs are bounded
//! loops over integer/float/array/slice/Vec operations with wrapping semantics;
//! they never trap and never depend on addresses.
//!
//! Usage: difftest-gen --out DIR --seeds N [--start S] [--prelude PATH]

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{anyhow, bail, Context, Result};

fn usage() -> &'static str {
    "usage: difftest-gen --out DIR --seeds N [--start S] [--prelude PATH]"
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        // Avoid the all-zero state; splitmix handles any input anyway.
        Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0xd1b5_4a32_d192_ed03)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len() as u64) as usize]
    }
}

const LOCALS: [&str; 4] = ["a", "b", "c", "d"];

fn operand(rng: &mut Rng, allow_k: bool) -> String {
    if allow_k && rng.below(3) == 0 {
        "k".into()
    } else {
        (*rng.pick(&LOCALS)).into()
    }
}

fn gen_stmt(rng: &mut Rng, i: usize, in_loop: bool) -> String {
    let lhs = *rng.pick(&LOCALS);
    let k_ok = in_loop;
    match rng.below(18) {
        0 => format!(
            "    {lhs} = {lhs}.wrapping_add({});",
            operand(rng, k_ok)
        ),
        1 => format!(
            "    {lhs} = {lhs}.wrapping_sub({});",
            operand(rng, k_ok)
        ),
        2 => format!(
            "    {lhs} = {lhs}.wrapping_mul(0x{:016x}u64);",
            rng.next() | 1
        ),
        3 => format!(
            "    {lhs} = {lhs}.rotate_left({});",
            rng.below(64)
        ),
        4 => format!(
            "    {lhs} = {lhs} ^ ({} >> {});",
            operand(rng, k_ok),
            rng.below(64)
        ),
        5 => format!(
            "    {lhs} = {lhs}.wrapping_add(0x{:016x}u64.rotate_left({}));",
            rng.next(),
            rng.below(64)
        ),
        6 => format!(
            "    if ({lhs} & 1) == 0 {{ {lhs} = {lhs}.wrapping_add({}); }} \
             else {{ {lhs} = {lhs} ^ {}; }}",
            operand(rng, k_ok),
            operand(rng, k_ok)
        ),
        7 => format!(
            "    match {lhs} & 3 {{ 0 => {lhs} = {lhs}.wrapping_add(0x{:016x}), \
             1 | 2 => {lhs} = {lhs}.wrapping_sub({}), \
             _ => {lhs} = {lhs}.rotate_right({}), }}",
            rng.next(),
            operand(rng, k_ok),
            rng.below(64)
        ),
        8 => format!(
            "    {lhs} = {lhs}.wrapping_add((({} as i64).wrapping_mul(0x{:016x}i64)) as u64);",
            operand(rng, k_ok),
            rng.next() as i64 | 1
        ),
        9 => format!(
            "    {lhs} = {lhs}.wrapping_add(((({} as i32).wrapping_mul(0x{:08x}i32)) as u32) as u64);",
            operand(rng, k_ok),
            rng.next() as u32 | 1
        ),
        10 => format!(
            "    {lhs} = {lhs}.wrapping_add((({} as i128).wrapping_mul(0x{:016x}i128)) as u64);",
            operand(rng, k_ok),
            rng.next()
        ),
        11 => format!(
            "    {lhs} = {lhs}.wrapping_add((({} as f64) / ({} as f64)).to_bits());",
            operand(rng, k_ok),
            (rng.next() & 0xffff) | 1
        ),
        12 => format!(
            "    {lhs} = {lhs}.wrapping_add(arr[({} & 15) as usize] ^ 0x{:016x});",
            operand(rng, k_ok),
            rng.next()
        ),
        13 => {
            // Swap two locals via a temporary. The destination is another
            // local, never `k` (which is an immutable loop binding).
            let other = loop {
                let cand = *rng.pick(&LOCALS);
                if cand != lhs {
                    break cand;
                }
            };
            format!("    let t{i} = {lhs}; {lhs} = {other}; {other} = t{i};")
        }
        14 => format!(
            "    {lhs} = {lhs}.wrapping_add((({}.count_ones() as u64) << 32) ^ ({}.count_ones() as u64));",
            operand(rng, k_ok),
            operand(rng, k_ok)
        ),
        15 => format!(
            "    {lhs} = {lhs}.wrapping_add((({}.leading_zeros() as u64) << 8) ^ {}.trailing_zeros() as u64);",
            operand(rng, k_ok),
            operand(rng, k_ok)
        ),
        16 => format!(
            "    {lhs} = {lhs}.wrapping_add(slice[({} & 7) as usize]);",
            operand(rng, k_ok)
        ),
        _ => format!(
            "    {lhs} = {lhs}.wrapping_add(({} as u128).wrapping_mul(0x{:016x}u128) as u64);",
            operand(rng, k_ok),
            rng.next()
        ),
    }
}

fn gen_program(seed: u64) -> String {
    let mut rng = Rng::new(seed);
    let a0 = rng.next();
    let b0 = rng.next();
    let c0 = rng.next();
    let d0 = rng.next();
    let arr: Vec<u64> = (0..16).map(|_| rng.next()).collect();
    let slice: Vec<u64> = (0..8).map(|_| rng.next()).collect();
    let loop_len: usize = 16 + rng.below(48) as usize;
    let n_top: usize = 10 + rng.below(10) as usize;
    let n_loop: usize = 4 + rng.below(4) as usize;

    let mut s = String::new();
    let _ = writeln!(
        s,
        "// Generated by difftest-gen from seed {seed:#018x}; do not edit."
    );
    let _ = writeln!(s, "#![no_std]");
    let _ = writeln!(
        s,
        "#![allow(dead_code, unused_variables, unused_mut, unused_assignments, \
         arithmetic_overflow, overflowing_literals)]"
    );
    let _ = writeln!(s);
    let _ = writeln!(s, "include!(\"prelude.rs\");");
    let _ = writeln!(s);
    let _ = writeln!(s, "fn body(seed: u64) -> u64 {{");
    // The runtime seed must change the result, otherwise every `--run-seeds`
    // entry would exercise only one path. Fold it into all four locals.
    let _ = writeln!(s, "    let mut a: u64 = 0x{a0:016x} ^ seed;");
    let _ = writeln!(
        s,
        "    let mut b: u64 = 0x{b0:016x} ^ seed.rotate_left(17);"
    );
    let _ = writeln!(
        s,
        "    let mut c: u64 = 0x{c0:016x}u64.wrapping_add(seed.wrapping_mul(0x9e3779b97f4a7c15));"
    );
    let _ = writeln!(
        s,
        "    let mut d: u64 = 0x{d0:016x} ^ seed.rotate_right(29);"
    );
    let _ = write!(s, "    let arr: [u64; 16] = [");
    for (i, v) in arr.iter().enumerate() {
        if i != 0 {
            s.push_str(", ");
        }
        let _ = write!(s, "0x{v:016x}");
    }
    let _ = writeln!(s, "];");
    let _ = write!(s, "    let slice: [u64; 8] = [");
    for (i, v) in slice.iter().enumerate() {
        if i != 0 {
            s.push_str(", ");
        }
        let _ = write!(s, "0x{v:016x}");
    }
    let _ = writeln!(s, "];");
    let _ = writeln!(s);

    for i in 0..n_top {
        let _ = writeln!(s, "{}", gen_stmt(&mut rng, i, false));
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "    let mut acc: u64 = 0;");
    let _ = writeln!(s, "    for i in 0..{loop_len}u64 {{");
    let _ = writeln!(s, "        let k = i;");
    for j in 0..n_loop {
        let _ = writeln!(s, "{}", gen_stmt(&mut rng, n_top + j, true));
    }
    let _ = writeln!(
        s,
        "        acc = acc.wrapping_add(a ^ b.rotate_left(7) ^ c ^ d);"
    );
    let _ = writeln!(s, "        if i & 31 == 0 {{ emit_u64(acc); }}");
    let _ = writeln!(s, "    }}");
    let _ = writeln!(s);

    // A bounded flatten/iterate over a Vec so allocator growth is exercised.
    let vlen: usize = 8 + rng.below(24) as usize;
    let _ = writeln!(
        s,
        "    let mut v: alloc::vec::Vec<u64> = alloc::vec::Vec::with_capacity({vlen});"
    );
    let _ = writeln!(s, "    for i in 0..{vlen}u64 {{");
    let _ = writeln!(s, "        v.push(a.rotate_left((i & 63) as u32) ^ (b.wrapping_add(i.wrapping_mul(0x{:016x}))));", rng.next());
    let _ = writeln!(s, "    }}");
    let _ = writeln!(s, "    v.sort_unstable();");
    let _ = writeln!(s, "    for &x in v.iter() {{ acc = acc.wrapping_add(x); }}");
    let _ = writeln!(s, "    acc ^= (v.len() as u64) << 48;");
    let _ = writeln!(s);

    let _ = writeln!(s, "    emit_u64(a);");
    let _ = writeln!(s, "    emit_u64(b);");
    let _ = writeln!(s, "    emit_u64(c);");
    let _ = writeln!(s, "    emit_u64(d);");
    let _ = writeln!(s, "    emit_u64(acc);");
    let _ = writeln!(s, "    emit_u32(arr[0] as u32 ^ arr[15] as u32);");
    let _ = writeln!(s, "    emit_u32(slice[3] as u32 ^ slice[7] as u32);");
    let _ = writeln!(s);
    let ret = format!(
        "a ^ b.rotate_left({}) ^ c.wrapping_mul(0x{:016x}) ^ d ^ acc",
        rng.below(64),
        rng.next() | 1
    );
    let _ = writeln!(s, "    {ret}");
    let _ = writeln!(s, "}}");
    s
}

fn parse_args() -> Result<(PathBuf, usize, u64, Option<PathBuf>)> {
    let mut args = std::env::args().skip(1);
    let mut out = None;
    let mut seeds = 16usize;
    let mut start = 1u64;
    let mut prelude = None;
    while let Some(a) = args.next() {
        let mut val = |name: &str| -> Result<String> {
            args.next().ok_or_else(|| anyhow!("{name} needs a value"))
        };
        match a.as_str() {
            "--out" => out = Some(PathBuf::from(val("--out")?)),
            "--seeds" => seeds = val("--seeds")?.parse().context("--seeds")?,
            "--start" => start = val("--start")?.parse().context("--start")?,
            "--prelude" => prelude = Some(PathBuf::from(val("--prelude")?)),
            "-h" | "--help" => bail!("{}", usage()),
            other => bail!("unknown option {other}\n{}", usage()),
        }
    }
    Ok((
        out.ok_or_else(|| anyhow!("missing --out\n{}", usage()))?,
        seeds,
        start,
        prelude,
    ))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("difftest-gen: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let (out, seeds, start, prelude) = parse_args()?;
    if seeds == 0 || seeds > 10_000 {
        bail!("--seeds must be in 1..=10000");
    }
    std::fs::create_dir_all(&out).with_context(|| format!("creating {}", out.display()))?;

    // Generated files include "prelude.rs" by a relative path so they can live
    // in a throwaway work directory outside the corpus.
    if let Some(p) = prelude {
        std::fs::copy(&p, out.join("prelude.rs"))
            .with_context(|| format!("copying prelude {} -> {}", p.display(), out.display()))?;
    } else if !out.join("prelude.rs").exists() {
        return Err(anyhow!(
            "no prelude in {}; pass --prelude PATH",
            out.display()
        ));
    }

    let mut manifest = String::from("# name\tkind\texpect_trap\tgeneration seed\tsource fnv1a\n");
    for i in 0..seeds as u64 {
        let seed = start.wrapping_add(i);
        let name = format!("gen_{seed:05}");
        let src = gen_program(seed);
        let checksum = fnv1a(src.as_bytes());
        std::fs::write(out.join(format!("{name}.rs")), &src)
            .with_context(|| format!("writing {}/{name}.rs", out.display()))?;
        let _ = writeln!(
            manifest,
            "{name}\tgenerated\t0\tseed={seed}\tfnv1a={checksum:016x}"
        );
        println!(
            "[difftest-gen] {name} seed={seed:#018x} bytes={} fnv1a={checksum:016x}",
            src.len()
        );
    }
    std::fs::write(out.join("manifest.tsv"), manifest)
        .with_context(|| format!("writing {}/manifest.tsv", out.display()))?;
    println!(
        "[difftest-gen] wrote {seeds} program(s) to {}",
        out.display()
    );
    Ok(())
}
