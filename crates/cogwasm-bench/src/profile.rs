//! Fixed-cost profiling of one warm edit in the resident serving guest.
//!
//! `-Zself-profile` records rustc's query and generic-activity events. This
//! module runs the same setup as the edit loop (an incremental seed compile,
//! then a warm edit), but passes `-Zself-profile` for the measured edit. It
//! summarises the raw event file with `analyzeme` (measureme's analysis crate)
//! and splits the whole edit into buckets: front end, monomorphisation,
//! incremental save/load, metadata, codegen, link, the remaining query work,
//! and the measured swap/call/unprofiled host time.
//!
//! The buckets are deliberately derived from two independent clocks: the
//! profile's own total, and the host's wall clock around the compile. The gate
//! checks that the two agree after adding the parts rustc does not instrument.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use analyzeme::{ProfilingData, QueryData};
use anyhow::{Context, Result, anyhow, bail};
use cogwasm_supervisor::{Swapper, apply_edit};
use serde::Serialize;

use crate::build::{ServeCompiler, artifact_from_bytes, clear_directory, rustc_args};
use crate::exec::BenchRuntime;
use crate::programs::{DependencySet, Program, externs_for, search_paths};
use crate::toolchain::Toolchain;
use crate::util;

/// Version of the profile document layout.
pub const SCHEMA: u32 = 1;

/// The inputs for one profiled edit.
pub struct ProfileRequest<'a> {
    pub toolchain: &'a Toolchain,
    pub program: &'a Program,
    pub dependencies: &'a DependencySet,
    pub out_dir: PathBuf,
    pub seed: u64,
}

/// One row of the fixed-cost table.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Bucket {
    pub name: String,
    pub ms: f64,
    pub share: f64,
}

/// One profile activity, ordered by self time.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Activity {
    pub label: String,
    pub kind: String,
    pub self_ms: f64,
    pub total_ms: f64,
    pub invocations: usize,
}

/// The gate result: the buckets must account for the measured edit.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Gate {
    pub bucket_sum_ms: f64,
    pub measured_edit_ms: f64,
    pub difference_ms: f64,
    pub difference_fraction: f64,
    pub passed: bool,
}

/// Everything one profiled edit measured.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProfileReport {
    pub schema: u32,
    pub tool: &'static str,
    pub generated_at: String,
    pub program: String,
    pub source_lines: usize,
    pub source: String,
    pub profiled_edit: usize,
    pub unprofiled_edit_ms: Option<f64>,
    pub build_ms: f64,
    pub link_ms: f64,
    pub swap_ms: f64,
    pub call_ms: f64,
    pub edit_ms: f64,
    pub profile_total_ms: f64,
    pub profile_busy_ms: f64,
    pub profile_events: usize,
    pub buckets: Vec<Bucket>,
    pub top_activities: Vec<Activity>,
    pub gate: Gate,
    pub profile_files: Vec<String>,
}

/// Run one unprofiled and one `-Zself-profile` warm edit on the large program.
///
/// # Errors
///
/// Returns an error when a compile fails, the profile directory does not
/// contain exactly one readable profile, or the installed module does not run.
pub fn run(request: &ProfileRequest<'_>) -> Result<ProfileReport> {
    let name = request.program.def.name.clone();
    let work = request.out_dir.join("work");
    fs::create_dir_all(&work)
        .with_context(|| format!("creating profile work directory {}", work.display()))?;
    let source_dir = work.join("source");
    fs::create_dir_all(&source_dir)?;
    let copy = source_dir.join(format!("{name}.rs"));
    fs::copy(&request.program.source, &copy).with_context(|| {
        format!(
            "copying {} to {}",
            request.program.source.display(),
            copy.display()
        )
    })?;
    let original = fs::read_to_string(&copy)
        .with_context(|| format!("reading the copied source {}", copy.display()))?;

    let externs = externs_for(&request.program.def, request.dependencies, "clif")?;
    let search = search_paths(&request.dependencies.clif);
    let incremental = work.join("incremental");
    clear_directory(&incremental)?;
    let profile_dir = work.join("self-profile");
    clear_directory(&profile_dir)?;

    let compiler = cogwasm_host::Compiler::load(
        &request.toolchain.rustc_wasm,
        request.toolchain.precompiled.as_deref(),
        false,
        Some(&request.toolchain.cache_dir),
    )
    .with_context(|| format!("loading {}", request.toolchain.rustc_wasm.display()))?;
    let mut serve = ServeCompiler::new(
        compiler.engine(),
        compiler.module(),
        &request.toolchain.wild,
        vec![work.clone(), request.toolchain.sysroot.clone()],
    )?;

    run_incremental_seed(
        &mut serve,
        request,
        &copy,
        &name,
        &externs,
        &search,
        &incremental,
        &work,
    )?;

    let runtime = Arc::new(BenchRuntime::for_swaps()?);
    let mut swapper = Swapper::new(runtime);
    let text = original;
    let (unprofiled_edit_ms, text) = run_unprofiled_edit(
        &mut serve,
        &mut swapper,
        request,
        &copy,
        &name,
        &externs,
        &search,
        &incremental,
        &work,
        &text,
        1,
        1,
    )?;

    let report = run_profiled_edit(
        &mut serve,
        &mut swapper,
        request,
        &copy,
        &name,
        &externs,
        &search,
        &incremental,
        &work,
        &text,
        2,
        0,
        &profile_dir,
    )?;
    Ok(ProfileReport {
        unprofiled_edit_ms: Some(unprofiled_edit_ms),
        ..report
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "the edit path is the same explicit list of paths rustc_args takes"
)]
fn run_incremental_seed(
    serve: &mut ServeCompiler,
    request: &ProfileRequest<'_>,
    source: &Path,
    name: &str,
    externs: &[String],
    search: &[String],
    incremental: &Path,
    work: &Path,
) -> Result<()> {
    let out = work.join("seed.wasm");
    remove_file_if_exists(&out)?;
    let args = rustc_args(
        request.toolchain,
        source,
        &out,
        name,
        externs,
        search,
        Some(incremental),
        false,
        &[],
    );
    serve
        .compile(&args)
        .with_context(|| format!("the incremental seed compile of {name}"))?;
    Ok(())
}

/// Compile one edit without the profiler and install it, returning its
/// edit -> result time and the edited source.
#[expect(
    clippy::too_many_arguments,
    reason = "one edit needs the source, compiler args, work paths and edit index"
)]
fn run_unprofiled_edit(
    serve: &mut ServeCompiler,
    swapper: &mut Swapper,
    request: &ProfileRequest<'_>,
    source: &Path,
    name: &str,
    externs: &[String],
    search: &[String],
    incremental: &Path,
    work: &Path,
    text: &str,
    index: usize,
    probe: u64,
) -> Result<(f64, String)> {
    let edited = apply_edit(text, index, probe)?;
    fs::write(source, &edited)
        .with_context(|| format!("writing edit {index} to {}", source.display()))?;
    let out = work.join(format!("edit-{index}.wasm"));
    remove_file_if_exists(&out)?;
    let args = rustc_args(
        request.toolchain,
        source,
        &out,
        name,
        externs,
        search,
        Some(incremental),
        false,
        &[],
    );
    let started = Instant::now();
    let outcome = serve
        .compile(&args)
        .with_context(|| format!("unprofiled edit {index} of {name}"))?;
    let build_ms = started.elapsed().as_secs_f64() * 1e3;
    let bytes = fs::read(&out).with_context(|| format!("reading {}", out.display()))?;
    let (swap_ms, call_ms) =
        install_and_call(swapper, &bytes, name, outcome.link_ms, request.seed)?;
    Ok((build_ms + swap_ms + call_ms, edited))
}

#[expect(
    clippy::too_many_arguments,
    reason = "the profiled edit needs the same source, compiler args and output paths"
)]
fn run_profiled_edit(
    serve: &mut ServeCompiler,
    swapper: &mut Swapper,
    request: &ProfileRequest<'_>,
    source: &Path,
    name: &str,
    externs: &[String],
    search: &[String],
    incremental: &Path,
    work: &Path,
    text: &str,
    index: usize,
    probe: u64,
    profile_dir: &Path,
) -> Result<ProfileReport> {
    let edited = apply_edit(text, index, probe)?;
    fs::write(source, &edited)
        .with_context(|| format!("writing edit {index} to {}", source.display()))?;
    let out = work.join(format!("edit-{index}.wasm"));
    remove_file_if_exists(&out)?;
    let args = rustc_args(
        request.toolchain,
        source,
        &out,
        name,
        externs,
        search,
        Some(incremental),
        false,
        &[format!("-Zself-profile={}", profile_dir.display())],
    );
    let started = Instant::now();
    let outcome = serve
        .compile(&args)
        .with_context(|| format!("profiled edit {index} of {name}"))?;
    let build_ms = started.elapsed().as_secs_f64() * 1e3;
    let bytes = fs::read(&out).with_context(|| format!("reading {}", out.display()))?;
    let (swap_ms, call_ms) =
        install_and_call(swapper, &bytes, name, outcome.link_ms, request.seed)?;

    // The profiler's sinks are flushed when its session is dropped. A serving
    // guest keeps the previous session alive until the next request, so make
    // one cheap request after the profiled edit before reading the file.
    let flush_out = work.join("flush.wasm");
    remove_file_if_exists(&flush_out)?;
    let flush_args = rustc_args(
        request.toolchain,
        source,
        &flush_out,
        name,
        externs,
        search,
        Some(incremental),
        false,
        &[],
    );
    serve
        .compile(&flush_args)
        .with_context(|| format!("the post-profile flush compile of {name}"))?;

    let profile_files = profile_files(profile_dir)?;
    let profile = analyse(&profile_files)?;
    let edit_ms = build_ms + swap_ms + call_ms;
    let Analysed {
        mut buckets,
        top_activities,
        profile_total_ms,
        category_sum_ms,
        profile_events,
    } = profile;
    buckets.push(Bucket {
        name: "swap (Wasmtime compile + instantiate)".to_owned(),
        ms: swap_ms,
        share: 0.0,
    });
    buckets.push(Bucket {
        name: "call".to_owned(),
        ms: call_ms,
        share: 0.0,
    });
    buckets.push(Bucket {
        name: "unprofiled host/rustc time".to_owned(),
        ms: (build_ms - category_sum_ms).max(0.0),
        share: 0.0,
    });
    finish_shares(&mut buckets, edit_ms);
    let bucket_sum_ms: f64 = buckets.iter().map(|bucket| bucket.ms).sum();
    let difference_ms = (bucket_sum_ms - edit_ms).abs();
    let difference_fraction = if edit_ms > 0.0 {
        difference_ms / edit_ms
    } else {
        0.0
    };
    let report = ProfileReport {
        schema: SCHEMA,
        tool: "cogwasm-bench profile",
        generated_at: util::format_utc(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |duration| duration.as_secs()),
        ),
        program: name.to_owned(),
        source_lines: request.program.lines,
        source: request.program.source.display().to_string(),
        profiled_edit: index,
        unprofiled_edit_ms: None,
        build_ms,
        link_ms: outcome.link_ms,
        swap_ms,
        call_ms,
        edit_ms,
        profile_total_ms,
        profile_busy_ms: category_sum_ms,
        profile_events,
        buckets,
        top_activities,
        gate: Gate {
            bucket_sum_ms,
            measured_edit_ms: edit_ms,
            difference_ms,
            difference_fraction,
            passed: difference_fraction <= 0.05,
        },
        profile_files: profile_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
    };
    Ok(report)
}

fn install_and_call(
    swapper: &mut Swapper,
    bytes: &[u8],
    name: &str,
    link_ms: f64,
    seed: u64,
) -> Result<(f64, f64)> {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "link time is a non-negative wall-clock duration in whole milliseconds"
    )]
    let link_rounded = link_ms.round() as u64;
    let artifact = artifact_from_bytes(bytes.to_vec(), name, Some(link_rounded))?;
    let started = Instant::now();
    swapper.install(&artifact)?;
    let swap_ms = started.elapsed().as_secs_f64() * 1e3;
    let started = Instant::now();
    swapper.call(seed)?;
    let call_ms = started.elapsed().as_secs_f64() * 1e3;
    Ok((swap_ms, call_ms))
}

struct Analysed {
    buckets: Vec<Bucket>,
    top_activities: Vec<Activity>,
    profile_total_ms: f64,
    category_sum_ms: f64,
    profile_events: usize,
}

fn analyse(files: &[PathBuf]) -> Result<Analysed> {
    let stem = files
        .first()
        .ok_or_else(|| anyhow!("the profiled compile wrote no .mm_profdata file"))?
        .with_extension("");
    let data = ProfilingData::new(&stem)
        .map_err(|error| anyhow!("opening profile data {}: {error}", stem.display()))?;
    let mut kinds: BTreeMap<String, String> = BTreeMap::new();
    for event in data.iter_full() {
        kinds
            .entry(event.label.to_string())
            .or_insert_with(|| event.event_kind.to_string());
    }
    let analysis = data.perform_analysis();
    let profile_total_ms = analysis.total_time.as_secs_f64() * 1e3;
    let mut sums = BTreeMap::<BucketKind, f64>::new();
    for query in &analysis.query_data {
        let kind = kinds.get(&query.label).map_or("", String::as_str);
        *sums.entry(classify(&query.label, kind)).or_default() +=
            query.self_time.as_secs_f64() * 1e3;
    }
    let buckets: Vec<Bucket> = sums
        .into_iter()
        .filter(|(_, ms)| *ms > 0.0)
        .map(|(kind, ms)| Bucket {
            name: kind.name().to_owned(),
            ms,
            share: 0.0,
        })
        .collect();
    let category_sum: f64 = buckets.iter().map(|bucket| bucket.ms).sum();
    let mut activities: Vec<&QueryData> = analysis
        .query_data
        .iter()
        .filter(|query| !query.self_time.is_zero())
        .collect();
    activities.sort_by_key(|query| std::cmp::Reverse(query.self_time));
    let top_activities = activities
        .into_iter()
        .take(10)
        .map(|query| Activity {
            label: query.label.clone(),
            kind: kinds
                .get(&query.label)
                .cloned()
                .unwrap_or_else(|| "unknown".to_owned()),
            self_ms: query.self_time.as_secs_f64() * 1e3,
            total_ms: query.time.as_secs_f64() * 1e3,
            invocations: query.invocation_count,
        })
        .collect();
    Ok(Analysed {
        buckets,
        top_activities,
        profile_total_ms,
        category_sum_ms: category_sum,
        profile_events: analysis.query_data.len(),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BucketKind {
    FrontEnd,
    Monomorphisation,
    Incremental,
    Metadata,
    Codegen,
    Link,
    QueryOther,
}

impl BucketKind {
    const fn name(self) -> &'static str {
        match self {
            Self::FrontEnd => "front end (parse/expand/resolve/typeck/borrowck)",
            Self::Monomorphisation => "monomorphisation / partitioning",
            Self::Incremental => "incremental save/load",
            Self::Metadata => "crate metadata / rmeta",
            Self::Codegen => "codegen",
            Self::Link => "link (rustc activity)",
            Self::QueryOther => "query overhead / other rustc work",
        }
    }
}

fn classify(label: &str, kind: &str) -> BucketKind {
    // A query's label says what it computes, but an incremental result-hash
    // or cache-load event with the same label is persistence work. Attribute
    // those by event kind so the front end does not absorb the incremental
    // machinery's cost.
    if matches!(kind, "IncrementalResultHashing" | "IncrementalLoadResult") {
        return BucketKind::Incremental;
    }
    let lower = label.to_ascii_lowercase();
    let matches = |patterns: &[&str]| patterns.iter().any(|pattern| lower.contains(pattern));
    if lower == "link" || lower.starts_with("link_") || matches(&["run_linker", "link_binary"]) {
        return BucketKind::Link;
    }
    if matches(&[
        "incr_comp",
        "incremental",
        "dep_graph",
        "dep-node",
        "query_cache",
        "work_product",
        "cgu_reuse",
        "serialize",
        "persist",
        "assert_dep",
        "check_clean",
        "encode_query",
        "query_values",
    ]) {
        return BucketKind::Incremental;
    }
    if matches(&[
        "metadata",
        "crate_hash",
        "load_macro",
        "extern_crate",
        "all_traits",
    ]) {
        return BucketKind::Metadata;
    }
    if matches(&[
        "monomorph",
        "partition",
        "mono_item",
        "collect_and_partition",
    ]) {
        return BucketKind::Monomorphisation;
    }
    if matches(&[
        "codegen",
        "llvm",
        "cranelift",
        "write_crate",
        "object",
        "assembly",
        "module_codegen",
        "optimized_mir",
        "mir_opt",
    ]) {
        return BucketKind::Codegen;
    }
    if matches(&[
        "parse",
        "expand",
        "macro",
        "resolve",
        "typeck",
        "type_check",
        "borrowck",
        "borrow_check",
        "wfcheck",
        "writeback",
        "ast_lowering",
        "hir",
        "coherence",
        "trait_selection",
        "evaluate_obligation",
        "normalize",
        "region",
        "privacy",
        "lint",
        "check_crate",
        "check_mod",
        "crate_inherent",
        "collect_mod_item",
        "analysis",
    ]) {
        return BucketKind::FrontEnd;
    }
    BucketKind::QueryOther
}

fn finish_shares(buckets: &mut [Bucket], total: f64) {
    for bucket in buckets.iter_mut() {
        bucket.share = if total > 0.0 { bucket.ms / total } else { 0.0 };
    }
    buckets.sort_by(|left, right| right.ms.total_cmp(&left.ms));
}

fn profile_files(directory: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(directory)
        .with_context(|| format!("reading the profile directory {}", directory.display()))?;
    let mut files = Vec::new();
    for entry in entries {
        let path = entry
            .with_context(|| format!("reading an entry of {}", directory.display()))?
            .path();
        if path
            .extension()
            .is_some_and(|extension| extension == "mm_profdata")
        {
            files.push(path);
        }
    }
    if files.is_empty() {
        bail!(
            "the profiled compile wrote no .mm_profdata file under {}",
            directory.display()
        );
    }
    if files.len() > 1 {
        bail!(
            "the profiled compile wrote {} profile files under {}; expected one",
            files.len(),
            directory.display()
        );
    }
    Ok(files)
}

fn remove_file_if_exists(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("removing {}", path.display())),
    }
}

/// Write `profile.json` and `profile.md` into `out_dir`.
///
/// # Errors
///
/// Returns an error when either file cannot be written.
pub fn write(report: &ProfileReport, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)?;
    let json = serde_json::to_string_pretty(report).context("serialising the profile report")?;
    fs::write(out_dir.join("profile.json"), json)
        .with_context(|| format!("writing {}", out_dir.join("profile.json").display()))?;
    fs::write(out_dir.join("profile.md"), render(report))
        .with_context(|| format!("writing {}", out_dir.join("profile.md").display()))?;
    Ok(())
}

/// The human-readable profile used in the milestone note.
#[must_use]
pub fn render(report: &ProfileReport) -> String {
    let mut text = String::new();
    let _ = writeln!(
        text,
        "# Fixed-cost profile: `{}`, one warm edit\n",
        report.program
    );
    let _ = writeln!(
        text,
        "Generated {} by `{}` ({}). Source: {} ({} lines). Profiled edit {}.\n",
        report.generated_at,
        report.tool,
        report.schema,
        report.source,
        report.source_lines,
        report.profiled_edit
    );
    let _ = writeln!(
        text,
        "Measured edit -> result **{:.1} ms** (build {:.1}, link {:.1}, swap {:.1}, call {:.1}). \
         The bucket work is {:.1} ms (profile span {:.1} ms) over {} activities and queries.\n",
        report.edit_ms,
        report.build_ms,
        report.link_ms,
        report.swap_ms,
        report.call_ms,
        report.profile_busy_ms,
        report.profile_total_ms,
        report.profile_events
    );
    let _ = writeln!(text, "| bucket | ms | share |");
    let _ = writeln!(text, "| --- | ---: | ---: |");
    for bucket in &report.buckets {
        let _ = writeln!(
            text,
            "| {} | {:.1} | {:.1}% |",
            bucket.name,
            bucket.ms,
            bucket.share * 100.0
        );
    }
    let _ = writeln!(
        text,
        "\nGate: buckets sum to {:.1} ms against a measured {:.1} ms edit \
         ({:.2}% difference) — {}.\n",
        report.gate.bucket_sum_ms,
        report.gate.measured_edit_ms,
        report.gate.difference_fraction * 100.0,
        if report.gate.passed { "PASS" } else { "FAIL" }
    );
    let _ = writeln!(text, "## Top 10 activities by self time\n");
    let _ = writeln!(text, "| # | activity | kind | self ms | total ms | calls |");
    let _ = writeln!(text, "| ---: | --- | --- | ---: | ---: | ---: |");
    for (index, activity) in report.top_activities.iter().enumerate() {
        let _ = writeln!(
            text,
            "| {} | `{}` | {} | {:.1} | {:.1} | {} |",
            index + 1,
            activity.label,
            activity.kind,
            activity.self_ms,
            activity.total_ms,
            activity.invocations
        );
    }
    text
}

/// A short stdout summary for the command.
#[must_use]
pub fn render_summary(report: &ProfileReport) -> String {
    format!(
        "profile {}: edit {:.1} ms, profile total {:.1} ms, buckets {:.1} ms \
         (gate {}), top: {:?}",
        report.program,
        report.edit_ms,
        report.profile_total_ms,
        report.gate.bucket_sum_ms,
        if report.gate.passed { "PASS" } else { "FAIL" },
        report
            .top_activities
            .first()
            .map_or("none", |activity| activity.label.as_str())
    )
}

#[cfg(test)]
mod tests {
    use super::{BucketKind, classify, finish_shares};

    #[test]
    fn classifies_the_planned_bucket_names() {
        assert_eq!(classify("typeck", "Query"), BucketKind::FrontEnd);
        assert_eq!(classify("mir_borrowck", "Query"), BucketKind::FrontEnd);
        assert_eq!(
            classify("lower_to_hir", "IncrementalResultHashing"),
            BucketKind::Incremental
        );
        assert_eq!(
            classify("monomorphize", "Query"),
            BucketKind::Monomorphisation
        );
        assert_eq!(
            classify("incr_comp_serialize_result_cache", "GenericActivity"),
            BucketKind::Incremental
        );
        assert_eq!(
            classify("generate_crate_metadata", "GenericActivity"),
            BucketKind::Metadata
        );
        assert_eq!(
            classify("write_crate_metadata", "GenericActivity"),
            BucketKind::Metadata
        );
        assert_eq!(
            classify("LLVM_passes", "GenericActivity"),
            BucketKind::Codegen
        );
        assert_eq!(classify("link", "GenericActivity"), BucketKind::Link);
        assert_eq!(classify("something_new", "Query"), BucketKind::QueryOther);
    }

    #[test]
    fn shares_sum_to_one_and_sort_by_time() {
        let mut buckets = vec![
            super::Bucket {
                name: "a".to_owned(),
                ms: 1.0,
                share: 0.0,
            },
            super::Bucket {
                name: "b".to_owned(),
                ms: 3.0,
                share: 0.0,
            },
        ];
        finish_shares(&mut buckets, 4.0);
        assert_eq!(buckets[0].name, "b");
        assert!((buckets.iter().map(|bucket| bucket.share).sum::<f64>() - 1.0).abs() < 1e-9);
    }
}
