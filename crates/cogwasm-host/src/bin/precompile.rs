//! Precompile rustc.wasm to a Wasmtime `.cwasm` artifact, and measure how
//! long the `.wasm` and `.cwasm` take to load.
//!
//! ```text
//! stage-c-precompile precompile --wasm rustc.wasm --out rustc.cwasm --metadata rustc.cwasm.json
//! stage-c-precompile bench      --wasm rustc.wasm --cwasm rustc.cwasm --runs 5
//! ```
//!
//! The `.cwasm` is native code for this host. It is only valid for the exact
//! Wasmtime version, engine configuration and target recorded in its sidecar
//! metadata; the host verifies all of them and falls back to the `.wasm` when
//! anything does not match.

use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use stage_c_host::config::{engine, engine_config_with, ConfigSummary};
use stage_c_host::precompiled::{
    load_rustc_module, metadata_sidecar, ArtifactMetadata, ModuleSource,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;
use wasmtime::{Engine, Module};

const USAGE: &str = "\
usage: stage-c-precompile <precompile|bench> [options]

precompile options:
  --wasm PATH        rustc.wasm to precompile
  --out PATH         write the .cwasm here
  --metadata PATH    write the sidecar metadata JSON here
  --wasm-simd BOOL   compile with SIMD (default true; false is for testing
                     compatibility-mismatch fallback, not for shipping)

bench options:
  --wasm PATH        rustc.wasm the artifact was built from
  --cwasm PATH       precompiled artifact (metadata is read from PATH.json)
  --runs N           measured runs (default 5)
  --cache-dir DIR    also measure Wasmtime's compiled-module cache
  --verify           also measure the opt-in full-SHA-256 load path
  --skip-cold        skip cold-wasm (reuse a cold baseline measured elsewhere)
  --json PATH        write machine-readable results

The hidden `bench-child` subcommand is what `bench` runs in a child process so
the 137 MiB module teardown is not measured.";

#[derive(Debug, Default)]
struct Args {
    wasm: Option<PathBuf>,
    out: Option<PathBuf>,
    metadata: Option<PathBuf>,
    cwasm: Option<PathBuf>,
    cache_dir: Option<PathBuf>,
    json: Option<PathBuf>,
    mode: Option<String>,
    runs: usize,
    wasm_simd: bool,
    verify: bool,
    skip_cold: bool,
}

impl Args {
    fn parse(raw: &[String]) -> Result<Self> {
        let mut args = Self {
            runs: 5,
            wasm_simd: true,
            ..Self::default()
        };
        let mut iter = raw.iter();
        while let Some(flag) = iter.next() {
            let mut value = |name: &str| -> Result<String> {
                iter.next()
                    .cloned()
                    .ok_or_else(|| anyhow!("{name} needs a value"))
            };
            match flag.as_str() {
                "--wasm" => args.wasm = Some(value("--wasm")?.into()),
                "--out" => args.out = Some(value("--out")?.into()),
                "--metadata" => args.metadata = Some(value("--metadata")?.into()),
                "--cwasm" => args.cwasm = Some(value("--cwasm")?.into()),
                "--cache-dir" => args.cache_dir = Some(value("--cache-dir")?.into()),
                "--json" => args.json = Some(value("--json")?.into()),
                "--mode" => args.mode = Some(value("--mode")?),
                "--verify" => args.verify = true,
                "--skip-cold" => args.skip_cold = true,
                "--runs" => {
                    args.runs = value("--runs")?
                        .parse()
                        .context("--runs must be a number")?;
                }
                "--wasm-simd" => {
                    args.wasm_simd = value("--wasm-simd")?
                        .parse()
                        .context("--wasm-simd must be true or false")?;
                }
                "-h" | "--help" => bail!("{USAGE}"),
                other => bail!("unknown option {other}\n{USAGE}"),
            }
        }
        if args.runs == 0 {
            bail!("--runs must be at least 1");
        }
        Ok(args)
    }

    fn require_wasm(&self) -> Result<&Path> {
        self.wasm
            .as_deref()
            .ok_or_else(|| anyhow!("missing --wasm PATH\n{USAGE}"))
    }
}

fn main() -> Result<()> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let (command, rest) = raw
        .split_first()
        .ok_or_else(|| anyhow!("missing command\n{USAGE}"))?;
    let args = Args::parse(rest)?;
    match command.as_str() {
        "precompile" => precompile(&args),
        "bench" => bench(&args),
        "bench-child" => bench_child(&args),
        other => bail!("unknown command {other}\n{USAGE}"),
    }
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(".tmp");
    let temporary = PathBuf::from(temporary);
    std::fs::write(&temporary, bytes)
        .with_context(|| format!("writing {}", temporary.display()))?;
    std::fs::rename(&temporary, path)
        .with_context(|| format!("renaming {} to {}", temporary.display(), path.display()))?;
    Ok(())
}

fn precompile(args: &Args) -> Result<()> {
    let wasm_path = args.require_wasm()?;
    let out_path = args
        .out
        .as_deref()
        .ok_or_else(|| anyhow!("missing --out PATH\n{USAGE}"))?;
    let metadata_path = args
        .metadata
        .as_deref()
        .ok_or_else(|| anyhow!("missing --metadata PATH\n{USAGE}"))?;

    let summary = ConfigSummary {
        wasm_simd: args.wasm_simd,
        // Wasmtime rejects SIMD off while relaxed SIMD is on.
        wasm_relaxed_simd: args.wasm_simd,
        ..ConfigSummary::shipped()
    };
    let config = engine_config_with(&summary, None)?;
    let engine =
        Engine::new(&config).map_err(|error| anyhow!("creating the Wasmtime engine: {error}"))?;

    let wasm_bytes =
        std::fs::read(wasm_path).with_context(|| format!("reading {}", wasm_path.display()))?;
    let started = Instant::now();
    let cwasm = engine
        .precompile_module(&wasm_bytes)
        .map_err(|error| anyhow!("precompiling {}: {error}", wasm_path.display()))?;
    let precompile_ms = started.elapsed().as_secs_f64() * 1e3;

    write_atomic(out_path, &cwasm)?;
    let metadata = ArtifactMetadata::new(&engine, &summary, wasm_path, out_path)?;
    let expected_metadata = metadata_sidecar(out_path);
    if metadata_path != expected_metadata {
        eprintln!(
            "[stage-c-precompile] warning: --metadata is {}, but the host reads sidecar \
             metadata from {}; copy or write it there before shipping",
            metadata_path.display(),
            expected_metadata.display()
        );
    }
    write_atomic(metadata_path, metadata.to_json()?.as_bytes())?;

    let summary_json = serde_json::json!({
        "wasm": wasm_path.display().to_string(),
        "cwasm": out_path.display().to_string(),
        "metadata": metadata_path.display().to_string(),
        "wasm_bytes": metadata.wasm.bytes,
        "cwasm_bytes": metadata.cwasm.bytes,
        "wasm_sha256": metadata.wasm.sha256,
        "cwasm_sha256": metadata.cwasm.sha256,
        "wasmtime_version": metadata.wasmtime_version,
        "compatibility_hash": metadata.compatibility_hash,
        "target": metadata.target.triple,
        "wasm_simd": summary.wasm_simd,
        "precompile_ms": precompile_ms,
    });
    println!("{}", serde_json::to_string_pretty(&summary_json)?);
    Ok(())
}

/// One measured child run. Kept as its own process because dropping a compiled
/// 137 MiB module is slow and would otherwise pollute the next measurement.
fn bench_child(args: &Args) -> Result<()> {
    let wasm_path = args.require_wasm()?;
    let mode = args
        .mode
        .as_deref()
        .ok_or_else(|| anyhow!("missing --mode"))?;
    let started = Instant::now();
    let (source, fallback_reason, bytes) = match mode {
        "cold-wasm" => {
            let engine = engine(None)?;
            let bytes = std::fs::read(wasm_path)
                .with_context(|| format!("reading {}", wasm_path.display()))?;
            let module = Module::new(&engine, &bytes)
                .map_err(|error| anyhow!("compiling {}: {error}", wasm_path.display()))?;
            let size = bytes.len();
            std::mem::forget(module);
            (ModuleSource::Wasm, None, size)
        }
        "cached-wasm" => {
            let cache_dir = args
                .cache_dir
                .as_deref()
                .ok_or_else(|| anyhow!("cached-wasm needs --cache-dir"))?;
            std::fs::create_dir_all(cache_dir)
                .with_context(|| format!("creating cache dir {}", cache_dir.display()))?;
            // Wasmtime requires an absolute cache directory.
            let cache_dir = std::path::absolute(cache_dir)
                .with_context(|| format!("resolving cache dir {}", cache_dir.display()))?;
            let engine = engine(Some(&cache_dir))?;
            let bytes = std::fs::read(wasm_path)
                .with_context(|| format!("reading {}", wasm_path.display()))?;
            let module = Module::new(&engine, &bytes)
                .map_err(|error| anyhow!("compiling {}: {error}", wasm_path.display()))?;
            let size = bytes.len();
            std::mem::forget(module);
            (ModuleSource::Wasm, None, size)
        }
        "deserialize" => {
            let cwasm_path = args
                .cwasm
                .as_deref()
                .ok_or_else(|| anyhow!("deserialize needs --cwasm"))?;
            let engine = engine(None)?;
            // SAFETY: the caller (`bench`) checked the sidecar's format,
            // Wasmtime version, compatibility hash, target and file size before
            // launching this child, and this is a trusted local artifact
            // produced by the same Wasmtime version. The benchmark process
            // exits without touching the file again.
            let module = unsafe { Module::deserialize_file(&engine, cwasm_path) }
                .map_err(|error| anyhow!("deserializing {}: {error}", cwasm_path.display()))?;
            let size = std::fs::metadata(cwasm_path).map_or(0, |metadata| {
                usize::try_from(metadata.len()).unwrap_or(usize::MAX)
            });
            std::mem::forget(module);
            (ModuleSource::Precompiled, None, size)
        }
        "load-default" | "load-verified" => {
            let cwasm_path = args
                .cwasm
                .as_deref()
                .ok_or_else(|| anyhow!("{mode} needs --cwasm"))?;
            let engine = engine(None)?;
            let verify_digests = mode == "load-verified";
            let loaded = load_rustc_module(&engine, wasm_path, Some(cwasm_path), verify_digests)?;
            let size = std::fs::metadata(wasm_path).map_or(0, |metadata| {
                usize::try_from(metadata.len()).unwrap_or(usize::MAX)
            });
            let result = (loaded.source, loaded.fallback_reason.clone(), size);
            std::mem::forget(loaded);
            result
        }
        other => bail!("unknown --mode {other}"),
    };
    let load_ms = started.elapsed().as_secs_f64() * 1e3;
    println!(
        "{}",
        serde_json::json!({
            "mode": mode,
            "load_ms": load_ms,
            "bytes": bytes,
            "source": source.as_str(),
            "fallback_reason": fallback_reason,
        })
    );
    Ok(())
}

#[derive(Serialize)]
struct Bench {
    wasm: String,
    cwasm: String,
    wasm_bytes: u64,
    cwasm_bytes: u64,
    wasmtime_version: String,
    compatibility_hash: String,
    target: String,
    precompile_config: ConfigSummary,
    runs: usize,
    cold_wasm_ms: Vec<f64>,
    cached_wasm_ms: Vec<f64>,
    deserialize_ms: Vec<f64>,
    load_default_ms: Vec<f64>,
    load_verified_ms: Vec<f64>,
    medians: BTreeMap<String, f64>,
    fallback_reasons: Vec<String>,
}

fn bench(args: &Args) -> Result<()> {
    let wasm_path = args.require_wasm()?;
    let cwasm_path = args
        .cwasm
        .as_deref()
        .ok_or_else(|| anyhow!("missing --cwasm PATH\n{USAGE}"))?;

    // Verify once, up front: a bad artifact makes the whole benchmark
    // meaningless, and the child processes then measure only loading. This is
    // the default, cheap check (version/config/target/sizes); digests are only
    // checked when the verified path is being measured.
    let metadata_path = metadata_sidecar(cwasm_path);
    let metadata = ArtifactMetadata::read(&metadata_path)?;
    let verify_engine = engine(None)?;
    metadata.verify(wasm_path, cwasm_path, &verify_engine, args.verify)?;
    println!(
        "# artifact: wasmtime {}, compatibility hash {}, target {}, cwasm {} bytes",
        metadata.wasmtime_version,
        metadata.compatibility_hash,
        metadata.target.triple,
        metadata.cwasm.bytes
    );

    let mut cold = Vec::with_capacity(args.runs);
    if !args.skip_cold {
        for _ in 0..args.runs {
            cold.push(run_child(args, "cold-wasm", false, None)?.load_ms);
        }
    }

    let mut cached = Vec::new();
    if let Some(cache_dir) = &args.cache_dir {
        std::fs::create_dir_all(cache_dir)
            .with_context(|| format!("creating cache dir {}", cache_dir.display()))?;
        let cache_dir = std::path::absolute(cache_dir)
            .with_context(|| format!("resolving cache dir {}", cache_dir.display()))?;
        // Prime the cache once; that compile is not part of the measurement.
        let _primed = run_child(args, "cached-wasm", true, Some(&cache_dir))?;
        for _ in 0..args.runs {
            cached.push(run_child(args, "cached-wasm", false, Some(&cache_dir))?.load_ms);
        }
    }

    let mut deserialize = Vec::with_capacity(args.runs);
    let mut load_default = Vec::with_capacity(args.runs);
    let mut verified = Vec::with_capacity(args.runs);
    let mut fallback_reasons = Vec::new();
    for _ in 0..args.runs {
        deserialize.push(run_child(args, "deserialize", false, None)?.load_ms);
        let child = run_child(args, "load-default", false, None)?;
        load_default.push(child.load_ms);
        if let Some(reason) = child.fallback_reason {
            fallback_reasons.push(reason);
        }
        if args.verify {
            let child = run_child(args, "load-verified", false, None)?;
            verified.push(child.load_ms);
            if let Some(reason) = child.fallback_reason {
                fallback_reasons.push(reason);
            }
        }
    }

    let mut medians = BTreeMap::new();
    for (name, values) in [
        ("cold_wasm_ms", &cold),
        ("cached_wasm_ms", &cached),
        ("deserialize_ms", &deserialize),
        ("load_default_ms", &load_default),
    ] {
        if !values.is_empty() {
            medians.insert(name.to_owned(), median(values));
        }
    }
    if args.verify {
        medians.insert("load_verified_ms".to_owned(), median(&verified));
    }

    let result = Bench {
        wasm: wasm_path.display().to_string(),
        cwasm: cwasm_path.display().to_string(),
        wasm_bytes: metadata.wasm.bytes,
        cwasm_bytes: metadata.cwasm.bytes,
        wasmtime_version: metadata.wasmtime_version.clone(),
        compatibility_hash: metadata.compatibility_hash.clone(),
        target: metadata.target.triple.clone(),
        precompile_config: metadata.config.clone(),
        runs: args.runs,
        cold_wasm_ms: cold,
        cached_wasm_ms: cached,
        deserialize_ms: deserialize,
        load_default_ms: load_default,
        load_verified_ms: verified,
        medians,
        fallback_reasons,
    };
    let text = serde_json::to_string_pretty(&result)?;
    if let Some(path) = &args.json {
        write_atomic(path, format!("{text}\n").as_bytes())?;
    }
    println!("{text}");
    Ok(())
}

#[derive(serde::Deserialize)]
struct ChildResult {
    load_ms: f64,
    #[serde(default)]
    fallback_reason: Option<String>,
}

fn run_child(
    args: &Args,
    mode: &str,
    quiet: bool,
    cache_dir: Option<&Path>,
) -> Result<ChildResult> {
    let exe = std::env::current_exe().context("locating the stage-c-precompile binary")?;
    let wasm = args.require_wasm()?;
    let mut command = Command::new(exe);
    command
        .arg("bench-child")
        .arg("--mode")
        .arg(mode)
        .arg("--wasm")
        .arg(wasm)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit());
    if let Some(cwasm) = &args.cwasm {
        command.arg("--cwasm").arg(cwasm);
    }
    if let Some(cache_dir) = cache_dir {
        command.arg("--cache-dir").arg(cache_dir);
    }
    let output = command
        .output()
        .with_context(|| format!("running the {mode} benchmark child"))?;
    if !output.status.success() {
        bail!(
            "{mode} child failed: {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let line = text
        .lines()
        .rev()
        .find(|line| line.trim_start().starts_with('{'))
        .ok_or_else(|| anyhow!("{mode} child printed no JSON: {text}"))?;
    if !quiet {
        eprintln!("# {mode}: {}", line.trim());
    }
    serde_json::from_str(line).with_context(|| format!("parsing {mode} child JSON"))
}

fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
    if sorted.len().is_multiple_of(2) {
        f64::midpoint(sorted[sorted.len() / 2 - 1], sorted[sorted.len() / 2])
    } else {
        sorted[sorted.len() / 2]
    }
}
