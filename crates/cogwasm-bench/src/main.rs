//! `cogwasm-bench`: the `CogWasm` compile, size and execution benchmark.

mod build;
mod chart;
mod compare;
mod exec;
mod generator;
mod instrument;
mod llvm;
mod profile;
mod programs;
mod report;
mod result;
mod size;
mod spec;
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
    /// Turn result documents into the site chart's data.json.
    Chart(ChartArgs),
    /// Profile one warm edit in the resident serving guest with -Zself-profile.
    Profile(ProfileArgs),
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
    /// JSON run specification; flags given alongside it win.
    #[arg(long)]
    spec: Option<PathBuf>,
    /// quick, full, large or plugin (default quick).
    #[arg(long)]
    suite: Option<String>,
    /// Program names or class/name (repeat the flag or pass several values).
    #[arg(long, num_args = 1.., action = clap::ArgAction::Append)]
    program: Vec<String>,
    /// Configuration names: default, inline, capture or cgu-per-fn.
    #[arg(long, num_args = 1.., action = clap::ArgAction::Append)]
    config: Vec<String>,
    /// Samples per seed and edits in the edit loop (default 5).
    #[arg(long)]
    runs: Option<usize>,
    /// Output directory (defaults to toolchain/work/bench/<timestamp>).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Print the result JSON on stdout (the human summary goes to stderr).
    #[arg(long)]
    json: bool,
    /// Skip deterministic compiler-fuel measurement (saves a fuel compile).
    #[arg(long)]
    no_fuel: bool,
    /// Also write the chart's data.json into this directory.
    #[arg(long)]
    chart: Option<PathBuf>,
    /// Config to plot in the chart (default `default`).
    #[arg(long)]
    chart_config: Option<String>,
    /// Earlier result.json files or directories of them, plotted before this
    /// run.
    #[arg(long, num_args = 1.., action = clap::ArgAction::Append)]
    history: Vec<PathBuf>,
}

#[derive(Args)]
struct ChartArgs {
    /// result.json files, or directories whose *.json are result documents.
    #[arg(required = true)]
    inputs: Vec<PathBuf>,
    /// Directory for data.json.
    #[arg(long)]
    out: Option<PathBuf>,
    /// Print the chart document on stdout instead of writing a file.
    #[arg(long)]
    stdout: bool,
    /// Config to plot.
    #[arg(long, default_value = "default")]
    config: String,
    /// GitHub repository the commit links point at.
    #[arg(long, default_value = "TeamNodeCraft/CogWasm")]
    repo: String,
    /// Page the chart is published on.
    #[arg(
        long,
        default_value = "https://teamnodecraft.github.io/CogWasm/#benchmarks"
    )]
    page_url: String,
}

#[derive(Args)]
struct ProfileArgs {
    #[command(flatten)]
    toolchain: ToolchainArgs,
    /// Suite whose first program is profiled (default large).
    #[arg(long)]
    suite: Option<String>,
    /// Program name; overrides the suite's first program.
    #[arg(long)]
    program: Option<String>,
    /// Output directory for profile.json, profile.md and the raw profile data.
    #[arg(long)]
    out: Option<PathBuf>,
    /// Seed passed to the installed module's run.
    #[arg(long, default_value_t = 42)]
    seed: u64,
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
        Command::Chart(args) => chart_command(&args),
        Command::Profile(args) => profile_command(args),
    }
}

fn run_command(args: RunArgs) -> Result<()> {
    let toolchain = args.toolchain.resolve()?;
    let spec = args
        .spec
        .as_deref()
        .map(spec::RunSpec::load)
        .transpose()?
        .unwrap_or_default();
    let suite = programs::Suite::parse(
        args.suite
            .as_deref()
            .or(spec.suite.as_deref())
            .unwrap_or("quick"),
    )?;
    let pick = |flag: Vec<String>, from_spec: &[String]| {
        if flag.is_empty() {
            from_spec.to_vec()
        } else {
            flag
        }
    };
    let program_names = pick(args.program, &spec.programs);
    let config_names = pick(args.config, &spec.configs);
    let runs = args.runs.or(spec.runs).unwrap_or(5);
    let fuel = !args.no_fuel && spec.fuel.unwrap_or(true);
    let spec_chart = spec.chart.clone().unwrap_or_default();
    let chart_out = args
        .chart
        .or_else(|| spec.chart.as_ref().map(|c| c.out.clone()));
    let chart_config = args
        .chart_config
        .or(spec_chart.config)
        .unwrap_or_else(|| "default".to_owned());
    let history = if args.history.is_empty() {
        spec_chart.history
    } else {
        args.history
    };
    let out_dir = args.out.or(spec.out).unwrap_or_else(|| {
        toolchain
            .work_root
            .join("bench")
            .join(util::utc_timestamp())
    });
    let options = result::RunOptions {
        suite,
        programs: program_names,
        configs: config_names,
        runs,
        out_dir,
        fuel,
    };
    let artifact = result::run(&toolchain, &options)?;
    if let Some(chart_out) = chart_out {
        // A history directory that does not exist yet is the first run.
        let history: Vec<PathBuf> = history
            .into_iter()
            .filter(|path| {
                let exists = path.exists();
                if !exists {
                    eprintln!(
                        "[cogwasm-bench] chart history {} does not exist yet; skipping it",
                        path.display()
                    );
                }
                exists
            })
            .collect();
        let mut files = if history.is_empty() {
            Vec::new()
        } else {
            chart::collect_inputs(&history)?
        };
        files.retain(|path| !same_file(path, &artifact.result_path));
        files.push(artifact.result_path.clone());
        let options = chart::ChartOptions {
            config: chart_config,
            ..chart::ChartOptions::default()
        };
        let document = chart::chart_files(&files, &options)?;
        let path = chart::write(&document, &chart_out)?;
        eprintln!("[cogwasm-bench] wrote {}", path.display());
    }
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

fn profile_command(args: ProfileArgs) -> Result<()> {
    let toolchain = args.toolchain.resolve()?;
    let suite = programs::Suite::parse(args.suite.as_deref().unwrap_or("large"))?;
    let names: Vec<String> = args.program.into_iter().collect();
    let programs = programs::prepare_programs(&toolchain, suite, &names)?;
    let program = programs
        .first()
        .context("the selected suite has no program to profile")?;
    let dependencies = programs::ensure_dependencies(&toolchain, std::slice::from_ref(program))?;
    let out_dir = util::absolute(&args.out.unwrap_or_else(|| {
        toolchain
            .work_root
            .join("profile")
            .join(util::utc_timestamp())
    }))?;
    let report = profile::run(&profile::ProfileRequest {
        toolchain: &toolchain,
        program,
        dependencies: &dependencies,
        out_dir: out_dir.clone(),
        seed: args.seed,
    })?;
    profile::write(&report, &out_dir)?;
    println!("{}", profile::render_summary(&report));
    eprintln!(
        "[cogwasm-bench] wrote {} and {}",
        out_dir.join("profile.json").display(),
        out_dir.join("profile.md").display()
    );
    Ok(())
}

fn same_file(a: &std::path::Path, b: &std::path::Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn chart_command(args: &ChartArgs) -> Result<()> {
    if args.out.is_none() && !args.stdout {
        bail!("pass --out DIR or --stdout");
    }
    let files = chart::collect_inputs(&args.inputs)?;
    let options = chart::ChartOptions {
        config: args.config.clone(),
        repo: args.repo.clone(),
        page_url: args.page_url.clone(),
    };
    let document = chart::chart_files(&files, &options)?;
    if args.stdout {
        println!(
            "{}",
            serde_json::to_string_pretty(&document).context("serialising the chart")?
        );
    }
    if let Some(out) = &args.out {
        let path = chart::write(&document, out)?;
        eprintln!("wrote {} ({} point(s))", path.display(), files.len());
    }
    Ok(())
}
