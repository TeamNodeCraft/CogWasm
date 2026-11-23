//! `cogwasm-bench`: the `CogWasm` compile, size and execution benchmark.

mod build;
mod compare;
mod exec;
mod generator;
mod instrument;
mod llvm;
mod programs;
mod report;
mod result;
mod size;
mod stats;
mod toolchain;
mod util;

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use std::fmt::Write as _;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "cogwasm-bench",
    version,
    about = "Measure CogWasm compile speed, size and execution against LLVM"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a benchmark suite and write result.json + report.md.
    Run(RunArgs),
    /// Compare two result documents with spread-aware timing rules.
    Compare(CompareArgs),
    /// Re-render a result document's Markdown report.
    Report(ReportArgs),
    /// Measure the toolchain footprint only (fast, no compiles).
    Toolchain(ToolchainCommand),
}

#[derive(Args, Clone, Default)]
struct ToolchainArgs {
    /// Repository root (defaults to this crate's repository).
    #[arg(long)]
    root: Option<PathBuf>,
    /// Toolchain root (defaults to <root>/toolchain).
    #[arg(long)]
    toolchain: Option<PathBuf>,
    /// rustc.wasm path (defaults to <toolchain>/artifacts/rustc.wasm).
    #[arg(long)]
    rustc: Option<PathBuf>,
    /// Precompiled rustc.wasm .cwasm (defaults to the work/precompiled one).
    #[arg(long)]
    precompiled: Option<PathBuf>,
    /// wasm32-unknown-unknown sysroot.
    #[arg(long)]
    sysroot: Option<PathBuf>,
    /// Wild linker binary.
    #[arg(long)]
    wild: Option<PathBuf>,
    /// Native LLVM rustc used for the -O0/-O2/-Oz baselines.
    #[arg(long)]
    native_rustc: Option<PathBuf>,
    /// Wasmtime module cache directory.
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    /// Work root (defaults to <toolchain>/work).
    #[arg(long)]
    work_root: Option<PathBuf>,
    /// Prebuilt vendored dependency rlib root (defaults to bench/work/deps).
    #[arg(long)]
    deps_root: Option<PathBuf>,
}

impl ToolchainArgs {
    fn resolve(&self) -> Result<toolchain::Toolchain> {
        toolchain::Toolchain::resolve(&toolchain::ToolchainOverrides {
            root: self.root.clone(),
            toolchain: self.toolchain.clone(),
            rustc_wasm: self.rustc.clone(),
            precompiled: self.precompiled.clone(),
            sysroot: self.sysroot.clone(),
            wild: self.wild.clone(),
            native_rustc: self.native_rustc.clone(),
            cache_dir: self.cache_dir.clone(),
            work_root: self.work_root.clone(),
            deps_root: self.deps_root.clone(),
        })
    }
}

#[derive(Args)]
struct RunArgs {
    #[command(flatten)]
    toolchain: ToolchainArgs,
    /// quick, full, large or plugin.
    #[arg(long, default_value = "quick")]
    suite: String,
    /// Program names or class/name (repeat the flag or pass several values).
    #[arg(long, num_args = 1.., action = clap::ArgAction::Append)]
    program: Vec<String>,
    /// Configuration names: default, inline, capture or cgu-per-fn.
    #[arg(long, num_args = 1.., action = clap::ArgAction::Append)]
    config: Vec<String>,
    /// Samples per seed and edits in the edit loop.
    #[arg(long, default_value_t = 5)]
    runs: usize,
    /// Output directory (defaults to toolchain/work/bench/<timestamp>).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Print the result JSON on stdout (the human summary goes to stderr).
    #[arg(long)]
    json: bool,
    /// Skip deterministic compiler-fuel measurement (saves a fuel compile).
    #[arg(long)]
    no_fuel: bool,
}

#[derive(Args)]
struct CompareArgs {
    baseline: PathBuf,
    current: PathBuf,
    /// Exit 1 when a regression is found.
    #[arg(long)]
    fail_on_regression: bool,
}

#[derive(Args)]
struct ReportArgs {
    result: PathBuf,
    /// Where to write the Markdown (defaults to the sibling .md).
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args)]
struct ToolchainCommand {
    #[command(flatten)]
    toolchain: ToolchainArgs,
    /// Print the footprint as JSON.
    #[arg(long)]
    json: bool,
    /// Also write the Markdown section to this file.
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Run(args) => run_command(args),
        Command::Compare(args) => compare_command(&args),
        Command::Report(args) => report_command(args),
        Command::Toolchain(args) => toolchain_command(&args),
    }
}

fn run_command(args: RunArgs) -> Result<()> {
    let toolchain = args.toolchain.resolve()?;
    let suite = programs::Suite::parse(&args.suite)?;
    let out_dir = args.out.unwrap_or_else(|| {
        toolchain
            .work_root
            .join("bench")
            .join(util::utc_timestamp())
    });
    let options = result::RunOptions {
        suite,
        programs: args.program,
        configs: args.config,
        runs: args.runs,
        out_dir,
        fuel: !args.no_fuel,
    };
    let artifact = result::run(&toolchain, &options)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&artifact.result).context("serialising the result")?
        );
    }
    let summary = summary(&artifact.result);
    if args.json {
        eprintln!("{summary}");
    } else {
        println!("{summary}");
    }
    eprintln!(
        "[cogwasm-bench] wrote {} and {}",
        artifact.result_path.display(),
        artifact.report_path.display()
    );
    Ok(())
}

fn summary(result: &result::BenchResult) -> String {
    let mut text = format!(
        "cogwasm-bench {}: {} program(s), {} config(s), {} runs\n",
        result.suite,
        result.programs.len(),
        result.settings.configs.len(),
        result.settings.runs
    );
    for (config, set) in &result.geomeans.by_config {
        let _ = writeln!(
            text,
            "  {config}: warm compile {} ms, edit p50 {} ms, exec {}x LLVM -O2, instructions {}x, size {}x O2 / {}x Oz",
            set.warm_compile_median_ms
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.2}")),
            set.edit_p50_median_ms
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.2}")),
            set.exec_ratio
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.2}")),
            set.instructions_ratio
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.2}")),
            set.size_ratio_o2
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.2}")),
            set.size_ratio_oz
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.2}")),
        );
    }
    if let Some(warning) = &result.machine.load_warning {
        writeln!(text, "  warning: {warning}").expect("writing to a String cannot fail");
    }
    write!(text, "  result: {}", result.settings.out_dir).expect("writing to a String cannot fail");
    text
}

fn compare_command(args: &CompareArgs) -> Result<()> {
    let baseline = result::load_result(&args.baseline)?;
    let current = result::load_result(&args.current)?;
    let comparison = compare::compare(&baseline, &current);
    print!(
        "{}",
        compare::render(
            &comparison,
            &args.baseline.display().to_string(),
            &args.current.display().to_string()
        )
    );
    if args.fail_on_regression && comparison.has_regressions() {
        std::process::exit(1);
    }
    Ok(())
}

fn report_command(args: ReportArgs) -> Result<()> {
    let result = result::load_result(&args.result)?;
    let markdown = report::render(&result);
    let path = args
        .out
        .unwrap_or_else(|| result::report_path_for(&args.result));
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&path, markdown).with_context(|| format!("writing {}", path.display()))?;
    println!("wrote {}", path.display());
    Ok(())
}

fn toolchain_command(args: &ToolchainCommand) -> Result<()> {
    let toolchain = args.toolchain.resolve()?;
    if !toolchain.rustc_wasm.is_file() {
        bail!(
            "missing {}; pass --rustc or build the toolchain",
            toolchain.rustc_wasm.display()
        );
    }
    let footprint = toolchain::measure_footprint(&toolchain)?;
    let markdown = report::render_toolchain_section(&footprint);
    if let Some(path) = &args.out {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        std::fs::write(path, &markdown).with_context(|| format!("writing {}", path.display()))?;
        eprintln!("wrote {}", path.display());
    }
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&footprint).context("serialising the footprint")?
        );
    } else {
        print!("{markdown}");
    }
    Ok(())
}
