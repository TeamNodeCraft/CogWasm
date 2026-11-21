//! Differential test runner: load an LLVM-built and a Cranelift-built wasm
//! module for each corpus program, call `run(seed)` in both, and compare the
//! returned u64 *and* the bytes in the exported output buffer bit-for-bit.
//!
//! Usage:
//!   difftest-host compare --manifest FILE --llvm DIR --clif DIR \
//!       [--run-seeds LIST] [--log FILE] [--expect-trap-from FILE]
//!
//! Manifest format (tab-separated, `#` comments allowed):
//!   name <TAB> kind <TAB> expect_trap(0|1) [<TAB> extra...]
//!
//! `name` is the file stem in both directories. A trap in both modules is a
//! match only when `expect_trap` is 1; a trap in one is always a mismatch.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{anyhow, bail, Context, Result};
use wasmtime::{Config, Engine, Instance, Memory, Module, Store};

#[derive(Clone, Debug)]
struct Entry {
    name: String,
    kind: String,
    expect_trap: bool,
}

#[derive(Clone, Debug)]
struct ConfigCli {
    manifest: PathBuf,
    llvm: PathBuf,
    clif: PathBuf,
    run_seeds: Vec<u64>,
    log: Option<PathBuf>,
}

fn usage() -> &'static str {
    "usage: difftest-host compare --manifest FILE --llvm DIR --clif DIR \\
     [--run-seeds 0,1,0x...] [--log FILE]"
}

fn parse_seeds(s: &str) -> Result<Vec<u64>> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let v = if let Some(hex) = part.strip_prefix("0x").or_else(|| part.strip_prefix("0X")) {
            u64::from_str_radix(hex, 16).with_context(|| format!("seed {part}"))?
        } else {
            part.parse::<u64>()
                .with_context(|| format!("seed {part}"))?
        };
        out.push(v);
    }
    if out.is_empty() {
        bail!("--run-seeds must not be empty");
    }
    Ok(out)
}

fn parse_args() -> Result<ConfigCli> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    if cmd != "compare" {
        bail!("unknown command {cmd:?}\n{}", usage());
    }
    let mut manifest = None;
    let mut llvm = None;
    let mut clif = None;
    let mut run_seeds = vec![0, 1, 0x9e37_79b9_7f4a_7c15, 0xffff_ffff_ffff_ffff];
    let mut log = None;
    while let Some(a) = args.next() {
        let mut val = |name: &str| -> Result<String> {
            args.next().ok_or_else(|| anyhow!("{name} needs a value"))
        };
        match a.as_str() {
            "--manifest" => manifest = Some(PathBuf::from(val("--manifest")?)),
            "--llvm" => llvm = Some(PathBuf::from(val("--llvm")?)),
            "--clif" => clif = Some(PathBuf::from(val("--clif")?)),
            "--run-seeds" => run_seeds = parse_seeds(&val("--run-seeds")?)?,
            "--log" => log = Some(PathBuf::from(val("--log")?)),
            "-h" | "--help" => bail!("{}", usage()),
            other => bail!("unknown option {other}\n{}", usage()),
        }
    }
    Ok(ConfigCli {
        manifest: manifest.ok_or_else(|| anyhow!("missing --manifest\n{}", usage()))?,
        llvm: llvm.ok_or_else(|| anyhow!("missing --llvm\n{}", usage()))?,
        clif: clif.ok_or_else(|| anyhow!("missing --clif\n{}", usage()))?,
        run_seeds,
        log,
    })
}

fn load_manifest(path: &Path) -> Result<Vec<Entry>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading manifest {}", path.display()))?;
    let mut out = Vec::new();
    for (lineno, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut cols = line.split('\t');
        let name = cols
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow!("{}:{}: missing name", path.display(), lineno + 1))?;
        let kind = cols
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow!("{}:{}: missing kind", path.display(), lineno + 1))?;
        let trap = cols
            .next()
            .ok_or_else(|| anyhow!("{}:{}: missing expect_trap", path.display(), lineno + 1))?;
        let expect_trap = match trap.trim() {
            "0" | "false" => false,
            "1" | "true" => true,
            other => bail!(
                "{}:{}: bad expect_trap {other:?}",
                path.display(),
                lineno + 1
            ),
        };
        out.push(Entry {
            name: name.to_string(),
            kind: kind.to_string(),
            expect_trap,
        });
    }
    if out.is_empty() {
        bail!("manifest {} has no entries", path.display());
    }
    Ok(out)
}

struct Reporter {
    file: Option<File>,
}

impl Reporter {
    fn new(path: Option<&Path>) -> Result<Self> {
        let file = match path {
            Some(p) => {
                if let Some(parent) = p.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                Some(
                    OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(p)
                        .with_context(|| format!("opening log {}", p.display()))?,
                )
            }
            None => None,
        };
        Ok(Reporter { file })
    }

    fn line(&mut self, s: &str) {
        println!("{s}");
        if let Some(f) = self.file.as_mut() {
            let _ = writeln!(f, "{s}");
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Value {
        value: u64,
        out: Vec<u8>,
    },
    Trap {
        message: String,
    },
    /// Harness/ABI failure (missing export, bad memory range, ...). Always a
    /// mismatch: a module that cannot expose the contract has failed the test.
    Invalid {
        message: String,
    },
}

impl Outcome {
    fn describe(&self) -> String {
        match self {
            Outcome::Value { value, out } => {
                format!(
                    "value={value:#018x} out_len={} out_sha={}",
                    out.len(),
                    short_hash(out)
                )
            }
            Outcome::Trap { message } => format!("TRAP: {message}"),
            Outcome::Invalid { message } => format!("INVALID: {message}"),
        }
    }
}

/// A small non-cryptographic but well-mixed digest, only for compact logs.
fn short_hash(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

fn hex_window(bytes: &[u8], start: usize, len: usize) -> String {
    let end = (start + len).min(bytes.len());
    let mut s = String::new();
    for b in &bytes[start.min(bytes.len())..end] {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Run one module once with `seed`, fresh store/instance. A trap is reported as
/// `Outcome::Trap`; a broken ABI/harness condition as `Outcome::Invalid`.
fn call_module(engine: &Engine, module: &Module, seed: u64) -> Outcome {
    match call_module_inner(engine, module, seed) {
        Ok(o) => o,
        Err(e) => Outcome::Invalid {
            message: format!("{e:#}"),
        },
    }
}

fn call_module_inner(engine: &Engine, module: &Module, seed: u64) -> Result<Outcome> {
    let mut store = Store::new(engine, ());
    let instance = Instance::new(&mut store, module, &[])
        .map_err(|e| anyhow!("instantiating module: {e:#}"))?;
    let memory: Memory = instance
        .get_memory(&mut store, "memory")
        .ok_or_else(|| anyhow!("module does not export `memory`"))?;
    let run = instance
        .get_typed_func::<u64, u64>(&mut store, "run")
        .map_err(|e| anyhow!("module does not export run(u64) -> u64: {e:#}"))?;
    let out_ptr = instance
        .get_typed_func::<(), u32>(&mut store, "out_ptr")
        .map_err(|e| anyhow!("module does not export out_ptr() -> u32: {e:#}"))?;
    let out_len = instance
        .get_typed_func::<(), u32>(&mut store, "out_len")
        .map_err(|e| anyhow!("module does not export out_len() -> u32: {e:#}"))?;

    let value = match run.call(&mut store, seed) {
        Ok(v) => v,
        Err(e) => {
            return Ok(Outcome::Trap {
                message: format!("{e:#}"),
            })
        }
    };
    let p = out_ptr
        .call(&mut store, ())
        .map_err(|e| anyhow!("out_ptr trapped after a successful run: {e:#}"))?
        as usize;
    let n = out_len
        .call(&mut store, ())
        .map_err(|e| anyhow!("out_len trapped after a successful run: {e:#}"))?
        as usize;
    let mem_len = memory.data_size(&store);
    if p > mem_len || n > mem_len - p {
        bail!("out buffer out of bounds: ptr={p:#x} len={n} memory={mem_len}");
    }
    let mut out = vec![0u8; n];
    memory
        .read(&store, p, &mut out)
        .with_context(|| format!("reading {n} output bytes at {p:#x}"))?;
    Ok(Outcome::Value { value, out })
}

struct Counts {
    programs: usize,
    handwritten: usize,
    generated: usize,
    calls: usize,
    value_matches: usize,
    trap_matches: usize,
    mismatches: usize,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("difftest-host: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = parse_args()?;
    let entries = load_manifest(&cli.manifest)?;
    let mut rep = Reporter::new(cli.log.as_deref())?;

    let mut cfg = Config::new();
    cfg.wasm_threads(false).wasm_bulk_memory(true);
    let engine = Engine::new(&cfg).map_err(|e| anyhow!("creating Wasmtime engine: {e:#}"))?;

    rep.line(&format!(
        "[difftest] runner: wasmtime 50.0.2, {} programs, {} run seeds {:?}",
        entries.len(),
        cli.run_seeds.len(),
        cli.run_seeds
    ));

    let mut counts = Counts {
        programs: entries.len(),
        handwritten: 0,
        generated: 0,
        calls: 0,
        value_matches: 0,
        trap_matches: 0,
        mismatches: 0,
    };
    // name -> mismatch detail lines
    let mut mismatch_details: BTreeMap<String, Vec<String>> = BTreeMap::new();

    // Cache compiled modules per backend: each pair is used for every run seed.
    for e in &entries {
        match e.kind.as_str() {
            "handwritten" => counts.handwritten += 1,
            "generated" => counts.generated += 1,
            _ => {}
        }
        let llvm_path = cli.llvm.join(format!("{}.wasm", e.name));
        let clif_path = cli.clif.join(format!("{}.wasm", e.name));
        let llvm_mod = Module::from_file(&engine, &llvm_path)
            .map_err(|err| anyhow!("loading {}: {err:#}", llvm_path.display()))?;
        let clif_mod = Module::from_file(&engine, &clif_path)
            .map_err(|err| anyhow!("loading {}: {err:#}", clif_path.display()))?;

        let mut prog_mismatches = 0usize;
        let mut prog_traps = 0usize;
        let mut out_sizes = Vec::new();
        let mut seeds_line = String::new();
        for &seed in &cli.run_seeds {
            counts.calls += 1;
            let a = call_module(&engine, &llvm_mod, seed);
            let b = call_module(&engine, &clif_mod, seed);

            let differs = match (&a, &b) {
                (Outcome::Invalid { .. }, _) | (_, Outcome::Invalid { .. }) => true,
                (Outcome::Trap { .. }, Outcome::Trap { .. }) => {
                    if e.expect_trap {
                        counts.trap_matches += 1;
                        prog_traps += 1;
                        false
                    } else {
                        true
                    }
                }
                (Outcome::Trap { .. }, Outcome::Value { .. })
                | (Outcome::Value { .. }, Outcome::Trap { .. }) => true,
                (Outcome::Value { value: va, out: oa }, Outcome::Value { value: vb, out: ob }) => {
                    if e.expect_trap {
                        // expected a trap, both returned normally
                        true
                    } else if va == vb && oa == ob {
                        counts.value_matches += 1;
                        out_sizes.push(oa.len());
                        false
                    } else {
                        true
                    }
                }
            };

            if differs {
                counts.mismatches += 1;
                prog_mismatches += 1;
                let det = vec![
                    format!(
                        "MISMATCH program={} kind={} seed={seed:#018x} expect_trap={}",
                        e.name, e.kind, e.expect_trap
                    ),
                    format!("  llvm: {}", a.describe()),
                    format!("  clif: {}", b.describe()),
                    format!("  {}", diff_line(&a, &b)),
                ];
                for l in det {
                    rep.line(&l);
                }
                mismatch_details
                    .entry(e.name.clone())
                    .or_default()
                    .push(format!(
                        "seed={seed:#018x}\n  llvm: {}\n  clif: {}",
                        a.describe(),
                        b.describe()
                    ));
            } else {
                if let Outcome::Value { out, .. } = &a {
                    let _ = write!(seeds_line, "{seed:#x}({}B) ", out.len());
                } else {
                    let _ = write!(seeds_line, "{seed:#x}(trap) ");
                }
            }
        }

        let status = if prog_mismatches == 0 { "PASS" } else { "FAIL" };
        let extra = if e.expect_trap {
            format!("expected-trap matches={prog_traps}/{}", cli.run_seeds.len())
        } else {
            let min = out_sizes.iter().min().copied().unwrap_or(0);
            let max = out_sizes.iter().max().copied().unwrap_or(0);
            if min == max {
                format!("out_bytes={min}")
            } else {
                format!("out_bytes={min}..{max}")
            }
        };
        rep.line(&format!(
            "[difftest] {status} {:<28} {:<11} {} {}",
            e.name,
            e.kind,
            extra,
            seeds_line.trim_end()
        ));
    }

    rep.line(&format!(
        "[difftest] SUMMARY programs={} (handwritten={}, generated={}) calls={} value_matches={} trap_matches={} mismatches={}",
        counts.programs, counts.handwritten, counts.generated, counts.calls,
        counts.value_matches, counts.trap_matches, counts.mismatches
    ));
    if !mismatch_details.is_empty() {
        rep.line("[difftest] FAILURES:");
        for (name, details) in &mismatch_details {
            rep.line(&format!("  {name}: {} mismatched seed(s)", details.len()));
            for d in details {
                for l in d.lines() {
                    rep.line(&format!("    {l}"));
                }
            }
        }
        rep.line("[difftest] RESULT: FAIL");
        return Ok(ExitCode::FAILURE);
    }
    rep.line("[difftest] RESULT: PASS");
    Ok(ExitCode::SUCCESS)
}

fn diff_line(a: &Outcome, b: &Outcome) -> String {
    match (a, b) {
        (Outcome::Value { value: va, out: oa }, Outcome::Value { value: vb, out: ob }) => {
            let mut parts = Vec::new();
            if va != vb {
                parts.push(format!(
                    "return {va:#018x} != {vb:#018x} (xor {:#018x})",
                    va ^ vb
                ));
            }
            if oa != ob {
                let first = oa.iter().zip(ob.iter()).position(|(x, y)| x != y);
                match first {
                    Some(i) => parts.push(format!(
                        "output first differs at byte {i} (llvm_len={}, clif_len={}); \
                         llvm[{}..]={} clif[{}..]={}",
                        oa.len(),
                        ob.len(),
                        i,
                        hex_window(oa, i, 16),
                        i,
                        hex_window(ob, i, 16)
                    )),
                    None => parts.push(format!(
                        "output length differs: llvm_len={}, clif_len={}",
                        oa.len(),
                        ob.len()
                    )),
                }
            }
            if parts.is_empty() {
                "outputs agree".into()
            } else {
                parts.join("; ")
            }
        }
        (Outcome::Invalid { message }, v) => {
            format!(
                "llvm ABI/harness failure: {message}; clif: {}",
                v.describe()
            )
        }
        (v, Outcome::Invalid { message }) => {
            format!(
                "clif ABI/harness failure: {message}; llvm: {}",
                v.describe()
            )
        }
        (Outcome::Trap { message: a }, Outcome::Trap { message: b }) => {
            format!("both trapped; llvm={a}; clif={b}")
        }
        (Outcome::Trap { message }, v) => {
            format!("llvm trapped ({message}); clif: {}", v.describe())
        }
        (v, Outcome::Trap { message }) => {
            format!("clif trapped ({message}); llvm: {}", v.describe())
        }
    }
}
