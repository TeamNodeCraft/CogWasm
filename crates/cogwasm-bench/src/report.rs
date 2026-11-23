//! Markdown rendering for a benchmark result.

use crate::result::{BenchResult, ConfigResult};
use crate::stats::Metric;
use std::fmt::Write as _;

/// Render the full Markdown report, including the V2 target table.
#[must_use]
pub fn render(result: &BenchResult) -> String {
    let mut out = String::new();
    header(result, &mut out);
    toolchain(result, &mut out);
    build_speed(result, &mut out);
    edit_samples(result, &mut out);
    build_size(result, &mut out);
    llvm_compile(result, &mut out);
    execution(result, &mut out);
    binary_size(result, &mut out);
    suite_geomeans(result, &mut out);
    v2_targets(result, &mut out);
    notes(result, &mut out);
    out
}

fn header(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "# CogWasm benchmark ({})", result.suite);
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "- commit: `{}` {}",
        short(&result.compiler.git_commit),
        result.compiler.git_subject
    );
    let _ = writeln!(
        out,
        "- machine: {} {} ({} cores) — {}",
        result.machine.os,
        result.machine.arch,
        result.machine.cores,
        result.machine.cpu.as_deref().unwrap_or("unknown CPU")
    );
    let _ = writeln!(
        out,
        "- runs: {} (median, after a warm-up); seeds: {}",
        result.settings.runs,
        result
            .settings
            .seeds
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(
        out,
        "- regime: **{}** ({}); requested mode {}",
        result.regime.as_str(),
        result.regime_reason,
        result.settings.mode.as_str()
    );
    let worst = result.idle.worst_fraction().map_or_else(
        || "no timed windows".to_owned(),
        |fraction| format!("{:.1}% of one core at the busiest window", fraction * 100.0),
    );
    let _ = writeln!(
        out,
        "- idle: **{}** ({worst}; threshold {:.1}%, {} sample(s), method `{}`)",
        if result.idle.qualified() {
            "qualified"
        } else {
            "not qualified"
        },
        result.idle.threshold_fraction * 100.0,
        result.idle.samples.len(),
        result.idle.method
    );
    let _ = writeln!(
        out,
        "- rustc.wasm: {} bytes, sha256 `{}…`",
        result.compiler.rustc_wasm_bytes,
        short(&result.compiler.rustc_wasm_sha256)
    );
    let _ = writeln!(
        out,
        "- compiler: {}; wasmtime {}; module load {:.1} ms ({}), serve start {:.1} ms",
        result.compiler.rustc_version,
        result.compiler.wasmtime_version,
        result.compiler.module_load_ms,
        result.compiler.precompiled_source,
        result.compiler.serve_start_ms
    );
    if let Some(warning) = &result.machine.load_warning {
        let _ = writeln!(out, "- **warning:** {warning}");
    }
    if let Some(reason) = &result.compiler.precompiled_fallback_reason {
        let _ = writeln!(out, "- precompiled fallback: {reason}");
    }
    let _ = writeln!(out);
}

fn toolchain(result: &BenchResult, out: &mut String) {
    let footprint = &result.toolchain;
    render_toolchain(footprint, out);
}

/// Render the toolchain-footprint section on its own (the `toolchain` command).
#[must_use]
pub fn render_toolchain_section(footprint: &crate::toolchain::ToolchainFootprint) -> String {
    let mut out = String::new();
    render_toolchain(footprint, &mut out);
    out
}

fn render_toolchain(footprint: &crate::toolchain::ToolchainFootprint, out: &mut String) {
    let _ = writeln!(out, "## Build size: toolchain footprint");
    let _ = writeln!(out);
    let _ = writeln!(out, "| asset | bytes | MiB |\n| --- | ---: | ---: |");
    let rows = [
        ("rustc.wasm raw", footprint.rustc_wasm_bytes),
        (
            "rustc.wasm, custom sections stripped",
            footprint.rustc_wasm_stripped_bytes,
        ),
        ("rustc.wasm, gzipped", footprint.rustc_wasm_gzip_bytes),
        (
            "rustc.wasm, stripped + gzipped",
            footprint.rustc_wasm_stripped_gzip_bytes,
        ),
        ("precompiled .cwasm", footprint.cwasm_bytes.unwrap_or(0)),
        ("sysroot", footprint.sysroot.bytes),
        (
            "REPL assets",
            footprint
                .repl_assets
                .as_ref()
                .map_or(0, |asset| asset.bytes),
        ),
    ];
    for (name, bytes) in rows {
        let _ = writeln!(out, "| {name} | {bytes} | {:.2} |", mib(bytes));
    }
    if let Some(repl) = &footprint.repl_assets {
        let _ = writeln!(
            out,
            "\nREPL assets: {} file(s) under `toolchain/work/repl-assets`.",
            repl.files.unwrap_or(0)
        );
    }
    let _ = writeln!(out);
}

fn build_speed(result: &BenchResult, out: &mut String) {
    let configs = &result.settings.configs;
    let _ = writeln!(out, "## Build speed");
    let _ = writeln!(out);
    if configs.len() > 1 {
        let _ = writeln!(out, "Configs are side by side: {}.\n", configs.join(", "));
    }
    let _ = writeln!(out, "| program | metric | {} |", configs.join(" | "));
    let _ = writeln!(
        out,
        "| --- | --- | {} |",
        configs
            .iter()
            .map(|_| "---:")
            .collect::<Vec<_>>()
            .join(" | ")
    );
    for program in result.programs.values() {
        let metrics = [
            ("cold compile ms", MetricKind::ColdCompile),
            ("warm compile ms", MetricKind::WarmCompile),
            ("edit p50 ms", MetricKind::EditP50),
            ("edit p90 ms", MetricKind::EditP90),
            ("edit build ms", MetricKind::EditBuild),
            ("edit link ms", MetricKind::EditLink),
            ("edit swap ms", MetricKind::EditSwap),
            ("compiler fuel", MetricKind::Fuel),
            ("warm fuel", MetricKind::WarmFuel),
            ("edit fuel", MetricKind::EditFuel),
            ("host instructions", MetricKind::HostInstructions),
            ("host cycles", MetricKind::HostCycles),
            (
                "cold Wild link instructions",
                MetricKind::ColdLinkInstructions,
            ),
            (
                "warm Wild link instructions",
                MetricKind::WarmLinkInstructions,
            ),
            (
                "edit Wild link instructions",
                MetricKind::EditLinkInstructions,
            ),
            (
                "edit swap compile instructions",
                MetricKind::EditSwapInstructions,
            ),
            ("peak guest MiB", MetricKind::PeakGuest),
            ("host RSS MiB", MetricKind::HostRss),
        ];
        for (label, kind) in metrics {
            let cells = configs
                .iter()
                .map(|config| {
                    program
                        .configs
                        .get(config)
                        .map_or_else(|| "—".to_owned(), |result| kind.cell(result))
                })
                .collect::<Vec<_>>()
                .join(" | ");
            let _ = writeln!(out, "| {} | {label} | {cells} |", program.name);
        }
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "`cold compile` is a fresh guest with no incremental state; `warm compile` is a \
         resident serving guest. `edit build/link/swap` are the components of edit-to-result."
    );
    let _ = writeln!(out);
}

#[derive(Clone, Copy)]
enum MetricKind {
    ColdCompile,
    WarmCompile,
    EditP50,
    EditP90,
    EditBuild,
    EditLink,
    EditSwap,
    Fuel,
    WarmFuel,
    EditFuel,
    HostInstructions,
    HostCycles,
    ColdLinkInstructions,
    WarmLinkInstructions,
    EditLinkInstructions,
    EditSwapInstructions,
    PeakGuest,
    HostRss,
}

impl MetricKind {
    fn cell(self, result: &ConfigResult) -> String {
        match self {
            Self::ColdCompile => fmt_metric(&result.build.cold_compile_ms, 2),
            Self::WarmCompile => fmt_metric(&result.build.warm_compile_ms, 2),
            Self::EditP50 => format!("{:.2}", result.build.edit.p50_ms),
            Self::EditP90 => format!("{:.2}", result.build.edit.p90_ms),
            Self::EditBuild => fmt_metric(&result.build.edit.build_ms, 2),
            Self::EditLink => fmt_metric(&result.build.edit.link_ms, 2),
            Self::EditSwap => fmt_metric(&result.build.edit.swap_ms, 2),
            Self::Fuel => result
                .build
                .fuel
                .as_ref()
                .map_or_else(|| "—".to_owned(), |fuel| format!("{:.0}", fuel.fuel.median)),
            Self::WarmFuel => result
                .build
                .warm_fuel
                .as_ref()
                .map_or_else(|| "—".to_owned(), |fuel| format!("{:.0}", fuel.fuel.median)),
            Self::EditFuel => result
                .build
                .edit
                .fuel
                .as_ref()
                .map_or_else(|| "—".to_owned(), |fuel| format!("{:.0}", fuel.fuel.median)),
            Self::HostInstructions => counter_cell(&result.build.native.host_instructions),
            Self::HostCycles => counter_cell(&result.build.native.host_cycles),
            Self::ColdLinkInstructions => counter_cell(&result.build.native.cold_link_instructions),
            Self::WarmLinkInstructions => counter_cell(&result.build.native.warm_link_instructions),
            Self::EditLinkInstructions => counter_cell(&result.build.edit.link_instructions),
            Self::EditSwapInstructions => counter_cell(&result.build.edit.swap_instructions),
            Self::PeakGuest => format!(
                "{:.1}",
                mib_f64(result.build.peak_guest_memory_bytes.median)
            ),
            Self::HostRss => result.build.host_rss_peak_bytes.as_ref().map_or_else(
                || "—".to_owned(),
                |metric| format!("{:.1}", mib_f64(metric.median)),
            ),
        }
    }
}

fn edit_samples(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## Edit loop samples");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| program | config | edit | probe | build ms | link ms | swap ms | call ms | edit-to-result ms | fuel | link instr | swap instr | byte-identical |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :---: |"
    );
    for program in result.programs.values() {
        for (config, measurement) in &program.configs {
            for row in &measurement.build.edit.edits {
                let _ = writeln!(
                    out,
                    "| {} | {config} | {} | {} | {:.2} | {:.2} | {:.2} | {:.3} | {:.2} | {} | {} | {} | {} |",
                    program.name,
                    row.edit,
                    row.probe,
                    row.build_ms,
                    row.link_ms,
                    row.swap_ms,
                    row.call_ms,
                    row.edit_to_result_ms,
                    fmt_opt(row.fuel.map(|value| value as f64), 0),
                    fmt_opt(row.link_instructions.map(|value| value as f64), 0),
                    fmt_opt(row.swap_instructions.map(|value| value as f64), 0),
                    if measurement.build.edit.byte_identical_to_cold {
                        "yes"
                    } else {
                        "no"
                    }
                );
            }
        }
    }
    let _ = writeln!(out);
}

fn build_size(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## Build size");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| program | config | total B | code B | data B | custom B | name B | import B | export B | functions | incremental B | objects B | peak MiB |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    for program in result.programs.values() {
        for (config, measurement) in &program.configs {
            let size = &measurement.size;
            let _ = writeln!(
                out,
                "| {} | {config} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.1} |",
                program.name,
                size.total_bytes,
                size.code_bytes,
                size.data_bytes,
                size.custom_bytes,
                size.name_bytes,
                size.import_bytes,
                size.export_bytes,
                size.functions,
                measurement.build.incremental_dir_bytes,
                measurement.build.intermediate_objects_bytes,
                mib_f64(measurement.build.peak_guest_memory_bytes.median)
            );
        }
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "`objects B` is the `.o`/`.obj`/`.rlib`/`.rmeta` subset of the incremental directory \
         ({} files in the reported run).",
        result
            .programs
            .values()
            .flat_map(|program| program.configs.values())
            .map(|measurement| measurement.build.intermediate_object_files)
            .sum::<u64>()
    );
    let _ = writeln!(out);
}

fn llvm_compile(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## LLVM compile time");
    let _ = writeln!(out);
    let _ = writeln!(out, "| program | LLVM -O0 ms | LLVM -O2 ms | LLVM -Oz ms |");
    let _ = writeln!(out, "| --- | ---: | ---: | ---: |");
    for program in result.programs.values() {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            program.name,
            fmt_metric(&program.llvm.o0.compile_ms, 2),
            fmt_metric(&program.llvm.o2.compile_ms, 2),
            fmt_metric(&program.llvm.oz.compile_ms, 2)
        );
    }
    let _ = writeln!(out);
}

fn execution(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## Compiled binary speed");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| program | config | clif ms | LLVM -O0 ms | LLVM -O2 ms | clif/O0 | clif/O2 | instructions clif | instructions O0 | instructions O2 | instr ratio O2 |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    for program in result.programs.values() {
        let llvm_o0 = program.llvm.exec.get("o0");
        let llvm_o2 = program.llvm.exec.get("o2");
        let instr_o0 = program.llvm.instructions.get("o0");
        let instr_o2 = program.llvm.instructions.get("o2");
        for (config, measurement) in &program.configs {
            let clif_ms = measurement.exec.exec_ms.median;
            let o0_ms = llvm_o0.map(|exec| exec.exec_ms.median);
            let o2_ms = llvm_o2.map(|exec| exec.exec_ms.median);
            let ours = measurement.instructions.instructions.median;
            let theirs = instr_o2.map(|instr| instr.instructions.median);
            let _ = writeln!(
                out,
                "| {} | {config} | {clif_ms:.4} | {} | {} | {} | {} | {ours:.0} | {} | {} | {} |",
                program.name,
                fmt_opt(o0_ms, 4),
                fmt_opt(o2_ms, 4),
                ratio(clif_ms, o0_ms),
                ratio(clif_ms, o2_ms),
                instr_o0.map_or_else(
                    || "—".to_owned(),
                    |instr| format!("{:.0}", instr.instructions.median)
                ),
                fmt_opt(theirs, 0),
                ratio(ours, theirs)
            );
        }
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "LLVM execution is measured on the same source with the toolchain's native rustc \
         (`-Copt-level=0/2`). Instructions are exact basic-block counts summed over the suite's \
         seeds; the result and output bytes must match LLVM before a timing is accepted."
    );
    let _ = writeln!(out);
}

fn binary_size(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## Compiled binary size");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| program | config | clif total B | clif code B | clif data B | LLVM -O2 total B | LLVM -Oz total B | clif/O2 | clif/Oz | clif functions |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    for program in result.programs.values() {
        for (config, measurement) in &program.configs {
            let size = &measurement.size;
            let _ = writeln!(
                out,
                "| {} | {config} | {} | {} | {} | {} | {} | {} | {} | {} |",
                program.name,
                size.total_bytes,
                size.code_bytes,
                size.data_bytes,
                program.llvm.o2.size.total_bytes,
                program.llvm.oz.size.total_bytes,
                ratio_u64(size.total_bytes, program.llvm.o2.size.total_bytes),
                ratio_u64(size.total_bytes, program.llvm.oz.size.total_bytes),
                size.functions
            );
        }
    }
    let _ = writeln!(out);
}

fn suite_geomeans(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## Suite geomeans");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| config | exec time / LLVM -O2 | instructions / LLVM -O2 | size / LLVM -O2 | size / LLVM -Oz | warm compile median ms | edit p50 median ms | programs |"
    );
    let _ = writeln!(
        out,
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    for (config, set) in &result.geomeans.by_config {
        let _ = writeln!(
            out,
            "| {config} | {} | {} | {} | {} | {} | {} | {} |",
            fmt_ratio(set.exec_ratio),
            fmt_ratio(set.instructions_ratio),
            fmt_ratio(set.size_ratio_o2),
            fmt_ratio(set.size_ratio_oz),
            fmt_opt(set.warm_compile_median_ms, 2),
            fmt_opt(set.edit_p50_median_ms, 2),
            set.programs
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Execution and instruction ratios use programs whose LLVM -O2 execution time is at least \
         0.1 ms, the same floor as the historical suite; size ratios cover every program."
    );
    let _ = writeln!(out);
}

fn v2_targets(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## V2 targets");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| axis | CogWasm V1 today | V2 target | measured | met |"
    );
    let _ = writeln!(out, "| --- | --- | --- | --- | :---: |");
    for target in &result.v2_targets {
        let met = target.met.map_or("—", |met| if met { "yes" } else { "no" });
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {met} |",
            target.axis, target.v1, target.target, target.measured
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "The target column is copied from `agents/plans/v2.md`; `measured` is filled only where \
         this run collected the metric. p90 is shown for large-program edits because the plan's \
         p99 needs more samples than the default run."
    );
    let _ = writeln!(out);
}

fn notes(result: &BenchResult, out: &mut String) {
    let _ = writeln!(out, "## Notes and limitations");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "- Compile time excludes instantiation and the native Wild subprocess; both are reported \
         separately in the JSON."
    );
    let _ = writeln!(
        out,
        "- Peak guest memory is the largest linear-memory size requested by a fresh compile; host \
         RSS is the process high-water mark, not a per-build delta."
    );
    let _ = writeln!(
        out,
        "- `-Ztime-passes` is a separate cold phase run plus a warm incremental run, so the headline \
         edit numbers do not pay for the extra instrumentation. The `incremental` bucket is zero \
         when the pinned rustc emits no incremental save/load timer."
    );
    let _ = writeln!(
        out,
        "- Wall-clock rows are noisy on shared machines. In counters mode they are unqualified by \
         construction; in full mode each row carries the idle evidence above and only an \
         idle-qualified run may call a wall-clock change a regression."
    );
    let _ = writeln!(
        out,
        "- Metric classes: exact (fuel, executed wasm instructions, bytes) compare exactly; \
         counter (native instructions retired, cycles) compare with a relative tolerance; wall \
         (milliseconds) compare only when both runs are idle-qualified and outside the spread."
    );
    let _ = writeln!(
        out,
        "- LLVM `-O0`/`-O2` builds link the same prebuilt `libm`/`miniz_oxide` rlibs; those large-crate \
         numbers isolate driver codegen, not the dependency's own optimisation level."
    );
    if !result.settings.fuel {
        let _ = writeln!(out, "- Compiler fuel was not collected (`--no-fuel`).");
    }
    let _ = writeln!(out);
}

fn counter_cell(metric: &Metric) -> String {
    if metric.is_available() {
        format!("{:.0}", metric.median)
    } else {
        "unavailable".to_owned()
    }
}

fn fmt_metric(metric: &Metric, digits: usize) -> String {
    format!("{:.digits$}", metric.median)
}

fn fmt_opt(value: Option<f64>, digits: usize) -> String {
    value.map_or_else(|| "—".to_owned(), |value| format!("{value:.digits$}"))
}

fn fmt_ratio(value: Option<f64>) -> String {
    value.map_or_else(|| "—".to_owned(), |value| format!("{value:.2}x"))
}

fn ratio(value: f64, base: Option<f64>) -> String {
    match base {
        Some(base) if base > 0.0 => format!("{:.2}x", value / base),
        _ => "—".to_owned(),
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "wasm module sizes are far below 2^53 bytes"
)]
fn ratio_u64(value: u64, base: u64) -> String {
    if base == 0 {
        "—".to_owned()
    } else {
        format!("{:.2}x", value as f64 / base as f64)
    }
}

fn short(value: &str) -> &str {
    value.get(..12).unwrap_or(value)
}

#[expect(
    clippy::cast_precision_loss,
    reason = "wasm module sizes are far below 2^53 bytes"
)]
fn mib(bytes: u64) -> f64 {
    mib_f64(bytes as f64)
}

fn mib_f64(bytes: f64) -> f64 {
    bytes / (1024.0 * 1024.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_has_one_section_per_measurement_and_the_v2_table() {
        let result = crate::result::tests::minimal_result();
        let text = render(&result);
        for section in [
            "# CogWasm benchmark (quick)",
            "## Build size: toolchain footprint",
            "## Build speed",
            "## Edit loop samples",
            "## Build size",
            "## LLVM compile time",
            "## Compiled binary speed",
            "## Compiled binary size",
            "## Suite geomeans",
            "## V2 targets",
            "## Notes and limitations",
        ] {
            assert!(text.contains(section), "missing section {section:?}");
        }
        assert!(text.contains("| a_add |"));
        assert!(text.contains("| default |"));
        assert!(text.contains("| test | old | new | measured | yes |"));
    }

    #[test]
    fn toolchain_section_is_reusable_on_its_own() {
        let result = crate::result::tests::minimal_result();
        let section = render_toolchain_section(&result.toolchain);
        assert!(section.contains("rustc.wasm raw"));
        assert!(section.contains("REPL assets"));
        assert!(!section.contains("## Build speed"));
    }
}
