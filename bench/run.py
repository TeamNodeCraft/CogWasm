#!/usr/bin/env python3
"""Run the CogWasm compile-time and execution benchmark suite.

The shell entry point is ``bench/run.sh``. This script owns the measurement
plan, the correctness check (every program's outputs must match LLVM) and the
JSON/Markdown result files. All timing numbers are medians of ``--runs``
measured runs after a warm-up.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import time
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BENCH = ROOT / "bench"
WORK = BENCH / "work"
RESULTS = BENCH / "results"
PROGRAMS_FILE = BENCH / "programs.json"
PRELUDE = BENCH / "programs" / "prelude.rs"
PROBE = """
// Added by the benchmark harness: a one-line edit whose two spellings are both
// zero, so an incremental recompile never changes the result.
const BENCH_EDIT_TWEAK: u64 = 0u64;
#[unsafe(no_mangle)]
pub extern "C" fn bench_edit_probe() -> u64 { BENCH_EDIT_TWEAK }
"""


class BenchError(RuntimeError):
    """A measurement failed; the suite must not publish a result."""


def run(command: list[str], env: dict | None = None, quiet: bool = False) -> str:
    """Run a command, returning stdout and raising with both streams on error."""
    proc = subprocess.run(
        command,
        capture_output=True,
        text=True,
        env=env,
        check=False,
    )
    if proc.returncode != 0:
        details = f"command failed ({proc.returncode}): {' '.join(command)}"
        raise BenchError(f"{details}\nstdout:\n{proc.stdout}\nstderr:\n{proc.stderr}")
    if not quiet and proc.stderr.strip():
        print(proc.stderr.rstrip(), file=sys.stderr)
    return proc.stdout


def run_json(command: list[str], env: dict | None = None) -> dict:
    output = run(command, env=env, quiet=True)
    try:
        return json.loads(output)
    except json.JSONDecodeError as error:
        raise BenchError(f"no JSON from {' '.join(command)}: {error}\n{output}") from error


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def stats(values: list[float]) -> dict:
    if not values:
        return {"median": None, "min": None, "max": None, "runs": 0}
    ordered = sorted(values)
    middle = len(ordered) // 2
    if len(ordered) % 2 == 0:
        median = (ordered[middle - 1] + ordered[middle]) / 2.0
    else:
        median = ordered[middle]
    return {
        "median": median,
        "min": ordered[0],
        "max": ordered[-1],
        "runs": len(ordered),
    }


def median(values: list[float]) -> float | None:
    result = stats(values)["median"]
    return result


def machine_info() -> dict:
    os_name = platform.system().lower()
    arch = platform.machine().lower()
    cpu = platform.processor()
    cores = os.cpu_count()
    if os_name == "darwin":
        cpu = run(["sysctl", "-n", "machdep.cpu.brand_string"], quiet=True).strip() or cpu
    elif os_name == "linux":
        try:
            with open("/proc/cpuinfo", encoding="utf-8") as handle:
                for line in handle:
                    if line.startswith("model name"):
                        cpu = line.split(":", 1)[1].strip()
                        break
        except OSError:
            pass
    if os.environ.get("GITHUB_ACTIONS") == "true":
        runner = os.environ.get("RUNNER_OS", "github-actions").lower()
        kind = f"{os_name}-{arch}-github-actions-{runner}"
    else:
        kind = f"{os_name}-{arch}-local"
    kind = os.environ.get("BENCH_MACHINE_KIND", kind)
    return {
        "kind": kind,
        "os": f"{platform.system()} {platform.release()}",
        "arch": arch,
        "cpu": cpu,
        "cores": cores,
        "ram_bytes": memory_bytes(os_name),
    }


def memory_bytes(os_name: str) -> int | None:
    if os_name == "darwin":
        try:
            return int(run(["sysctl", "-n", "hw.memsize"], quiet=True).strip())
        except (BenchError, ValueError):
            return None
    try:
        with open("/proc/meminfo", encoding="utf-8") as handle:
            for line in handle:
                if line.startswith("MemTotal:"):
                    return int(line.split()[1]) * 1024
    except OSError:
        pass
    return None


def git_commit() -> dict:
    root = Path(os.environ.get("BENCH_COMMIT_ROOT", ROOT))
    try:
        sha = run(["git", "-C", str(root), "rev-parse", "HEAD"], quiet=True).strip()
        subject = run(
            ["git", "-C", str(root), "log", "-1", "--format=%s"], quiet=True
        ).strip()
        return {"sha": sha, "subject": subject}
    except BenchError:
        return {"sha": "unknown", "subject": "unknown"}


def find_prelude_include(text: str) -> str | None:
    for line in text.splitlines():
        if 'include!("' in line and "prelude.rs" in line:
            return line.strip()
    return None


def prepare_source(program: dict, destination: Path) -> Path:
    source = ROOT / program["source"]
    if not source.exists():
        raise BenchError(f"benchmark source {source} does not exist")
    text = source.read_text(encoding="utf-8")
    include = find_prelude_include(text)
    if include is not None:
        text = text.replace(include, f'include!("{PRELUDE}");')
    if "BENCH_EDIT_TWEAK" not in text:
        text += PROBE
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(text, encoding="utf-8")
    return destination


def find_library(deps_root: Path, crate: str, suffix: str) -> Path | None:
    candidates = list(deps_root.rglob(f"lib{crate}-*.{suffix}"))
    if not candidates:
        candidates = list(deps_root.rglob(f"lib{crate}.{suffix}"))
    if not candidates:
        return None
    return max(candidates, key=lambda path: path.stat().st_mtime)


def find_artifact(deps_root: Path, crate: str) -> tuple[Path, Path | None]:
    rlib = find_library(deps_root, crate, "rlib")
    if rlib is None:
        raise BenchError(f"no {crate} rlib under {deps_root}")
    rmeta = find_library(deps_root, crate, "rmeta")
    return rlib, rmeta


def clif_env(toolchain: Path) -> dict:
    env = os.environ.copy()
    env["RUSTUP_HOME"] = str(toolchain / "rustup")
    env["CARGO_HOME"] = str(toolchain / "cargo")
    env["STAGE_C_HOST_TRIPLE"] = os.environ.get(
        "BENCH_HOST_TRIPLE", detect_host_triple(toolchain)
    )
    env["STAGE_C_SYSROOT"] = str(toolchain / "stage-c" / "sysroot-pure")
    env["STAGE_C_RUSTC_WASM"] = str(toolchain / "stage-c" / "artifacts" / "rustc.wasm")
    env["STAGE_C_HOST"] = str(toolchain / "target" / "stage-c-host" / "release" / "stage-c-host")
    env["STAGE_C_WILD"] = str(toolchain / "target" / "wild" / "release" / "wild")
    env["STAGE_C_STAGE1"] = str(
        toolchain / "stage-b" / "rust" / "build" / "host" / "stage1" / "bin" / "rustc"
    )
    env["STAGE_C_CACHE"] = str(toolchain / "stage-c" / "cache")
    env["RUSTC"] = str(
        ROOT / "experiments" / "cranelift-wasm" / "stage-c" / "scripts" / "rustc-wasm-wrapper.sh"
    )
    env["RUSTUP_TOOLCHAIN"] = os.environ.get("BENCH_HOST_TOOLCHAIN", "1.99.1")
    objcopy = sorted((toolchain / "rustup" / "toolchains").glob("nightly-*/lib/rustlib/*/bin"))
    if objcopy:
        env["PATH"] = f"{objcopy[0]}:{env.get('PATH', '')}"
    return env


def llvm_env() -> dict:
    env = os.environ.copy()
    env.pop("RUSTC", None)
    env["RUSTUP_TOOLCHAIN"] = os.environ.get("BENCH_NIGHTLY", "nightly-2026-10-19")
    return env


def detect_host_triple(toolchain: Path) -> str:
    stage1 = toolchain / "stage-b" / "rust" / "build" / "host" / "stage1" / "bin" / "rustc"
    if stage1.exists():
        output = run([str(stage1), "-vV"], quiet=True)
        for line in output.splitlines():
            if line.startswith("host: "):
                return line.removeprefix("host: ").strip()
    return f"{platform.machine()}-apple-darwin" if platform.system() == "Darwin" else (
        f"{platform.machine()}-unknown-linux-gnu"
    )


def build_deps(toolchain: Path, work: Path) -> dict[str, Path]:
    cargo = toolchain / "cargo" / "bin" / "cargo"
    manifest = BENCH / "vendor" / "Cargo.toml"
    roots = {}
    for pipeline in ("clif", "llvm"):
        target = work / "deps" / pipeline
        rlib = find_library(target, "libm", "rlib")
        miniz = find_library(target, "miniz_oxide", "rlib")
        if (
            rlib is not None
            and miniz is not None
            and not os.environ.get("BENCH_REBUILD_DEPS")
        ):
            roots[pipeline] = target
            continue
        if os.environ.get("BENCH_REBUILD_DEPS"):
            # Cargo cannot see that the compiler behind the RUSTC wrapper
            # changed, so an explicit rebuild must start from a clean target.
            shutil.rmtree(target, ignore_errors=True)
        target.mkdir(parents=True, exist_ok=True)
        command = [
            str(cargo),
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--manifest-path",
            str(manifest),
            "-p",
            "libm",
            "-p",
            "miniz_oxide",
        ]
        env = clif_env(toolchain) if pipeline == "clif" else llvm_env()
        env["CARGO_TARGET_DIR"] = str(target)
        if pipeline == "llvm":
            env["RUSTUP_TOOLCHAIN"] = os.environ.get("BENCH_NIGHTLY", "nightly-2026-10-19")
        print(f"building vendored benchmark dependencies with {pipeline}", file=sys.stderr)
        run(command, env=env, quiet=True)
        roots[pipeline] = target
    return roots


def dependency_search_paths(deps_root: Path) -> list[str]:
    return sorted({str(path.parent) for path in deps_root.rglob("*.rlib")})


def extern_args(program: dict, deps: dict[str, Path], pipeline: str) -> list[str]:
    args = []
    for crate in program.get("externs", []):
        rlib, rmeta = find_artifact(deps[pipeline], crate)
        args.extend(["--extern", f"{crate}={rlib}"])
        if rmeta is not None:
            args.extend(["--extern", f"{crate}={rmeta}"])
    return args


def compile_llvm(
    rustc: Path,
    program: dict,
    source: Path,
    out: Path,
    opt_level: int,
    externs: list[str],
    runs: int,
    search_paths: list[str],
) -> dict:
    out.parent.mkdir(parents=True, exist_ok=True)
    command = [
        str(rustc),
        "--edition=2021",
        "--target=wasm32-unknown-unknown",
        "--crate-type",
        "cdylib",
        "-C",
        "panic=abort",
        "-C",
        f"opt-level={opt_level}",
        "-C",
        "strip=debuginfo",
    ]
    command.extend(externs)
    for directory in search_paths:
        command.extend(["-L", f"dependency={directory}"])
    command.extend([str(source), "-o", str(out)])

    def once() -> float:
        started = time.perf_counter()
        run(command, quiet=True)
        return (time.perf_counter() - started) * 1e3

    once()  # warm-up
    values = [once() for _ in range(runs)]
    return {"compile_ms": stats(values), "wasm": str(out), "wasm_sha256": sha256_file(out)}


def compile_clif(
    host: Path,
    toolchain: Path,
    program: dict,
    source: Path,
    out: Path,
    work: Path,
    runs: int,
    phase_runs: int,
    load: str,
    precompiled: Path | None,
    externs: list[str],
    search_paths: list[str],
    fuel: bool,
) -> dict:
    out.parent.mkdir(parents=True, exist_ok=True)
    program_work = work / "compile" / program["name"]
    program_work.mkdir(parents=True, exist_ok=True)
    command = [
        str(host),
        "compile",
        "--rustc",
        str(toolchain / "stage-c" / "artifacts" / "rustc.wasm"),
        "--sysroot",
        str(toolchain / "stage-c" / "sysroot-pure"),
        "--linker",
        str(toolchain / "target" / "wild" / "release" / "wild"),
        "--src",
        str(source),
        "--prelude",
        str(PRELUDE),
        "--out",
        str(out),
        "--work",
        str(program_work),
        "--crate-name",
        program["name"],
        "--runs",
        str(runs),
        "--phase-runs",
        str(phase_runs),
        "--load",
        load,
        "--cache-dir",
        str(toolchain / "stage-c" / "cache"),
        "--json",
        str(work / "json" / f"{program['name']}-compile.json"),
    ]
    if precompiled is not None:
        command.extend(["--precompiled", str(precompiled)])
    if os.environ.get("BENCH_MIR_HOIST") == "1":
        # Goal 1 step 5b experiment: opt the clif compile into the loop-field
        # hoist. The LLVM reference compiler does not have the pass, so this is
        # not added to compile_llvm.
        command.extend(["--rustc-arg", "-Zmir-hoist-loop-field-reads"])
    if fuel:
        # Two samples per program: the host fails if they disagree, so a
        # non-deterministic compiler work count stops the run.
        command.extend(["--fuel", "--fuel-runs", "2"])
    if program.get("incremental", True):
        command.extend(
            [
                "--incremental",
                "--incremental-dir",
                str(work / "incremental" / program["name"]),
            ]
        )
    command.extend(externs)
    for directory in search_paths:
        command.extend(["--search-path", directory])
    (work / "json").mkdir(parents=True, exist_ok=True)
    result = run_json(command)
    result["wasm"] = str(out)
    result["wasm_bytes"] = out.stat().st_size
    result["wasm_sha256"] = sha256_file(out)
    return result


def execute(host: Path, wasm: Path, runs: int, seeds: list[int], work: Path, tag: str) -> dict:
    result = run_json(
        [
            str(host),
            "run",
            "--wasm",
            str(wasm),
            "--runs",
            str(runs),
            "--seeds",
            ",".join(str(seed) for seed in seeds),
            "--json",
            str(work / "json" / f"{tag}-run.json"),
        ]
    )
    if not result.get("ok"):
        raise BenchError(f"execution failed for {tag}")
    return result


def sizes(host: Path, wasm: Path, work: Path, tag: str) -> dict:
    return run_json(
        [
            str(host),
            "sizes",
            "--wasm",
            str(wasm),
            "--json",
            str(work / "json" / f"{tag}-sizes.json"),
        ]
    )


def exec_stats(run: dict) -> dict:
    return {
        metric: stats([item[metric] for item in run["runs"]])
        for metric in ("exec_ms", "instantiate_ms")
    }


def compile_phase_summary(compile_result: dict) -> dict:
    phase_runs = compile_result.get("phase_runs") or []
    phases: dict[str, list[float]] = {}
    for item in phase_runs:
        for name, value in item.get("phases", {}).items():
            phases.setdefault(name, []).append(value)
    medians = {name: median(values) for name, values in phases.items()}
    total = medians.get("total")
    codegen = medians.get("codegen_crate")
    link = medians.get("link")
    frontend = None
    if total is not None:
        frontend = max(total - (codegen or 0.0) - (link or 0.0), 0.0)
    return {
        "total_ms": total,
        "frontend_ms": frontend,
        "codegen_ms": codegen,
        "link_ms": link,
        "raw": medians,
    }


def compile_stats(runs: list[dict], key: str = "compile_ms") -> dict:
    return stats([item[key] for item in runs])


def run_program(
    program: dict,
    host: Path,
    toolchain: Path,
    llvm_rustc: Path,
    deps: dict[str, Path],
    work: Path,
    runs: int,
    llvm_runs: int,
    phase_runs: int,
    load: str,
    precompiled: Path | None,
    fuel: bool,
) -> dict:
    name = program["name"]
    print(f"== {program['class']}/{name}", file=sys.stderr)
    source = prepare_source(program, work / "sources" / f"{name}.rs")
    clif_externs = extern_args(program, deps, "clif")
    llvm_externs = extern_args(program, deps, "llvm")
    clif_search = dependency_search_paths(deps["clif"])
    llvm_search = dependency_search_paths(deps["llvm"])

    clif_wasm = work / "wasm" / "clif" / f"{name}.wasm"
    clif = compile_clif(
        host,
        toolchain,
        program,
        source,
        clif_wasm,
        work,
        runs,
        phase_runs,
        load,
        precompiled,
        clif_externs,
        clif_search,
        fuel,
    )

    llvm_o0 = compile_llvm(
        llvm_rustc,
        program,
        source,
        work / "wasm" / "llvm_o0" / f"{name}.wasm",
        0,
        llvm_externs,
        llvm_runs,
        llvm_search,
    )
    llvm_o2 = compile_llvm(
        llvm_rustc,
        program,
        source,
        work / "wasm" / "llvm_o2" / f"{name}.wasm",
        2,
        llvm_externs,
        llvm_runs,
        llvm_search,
    )

    seeds = [int(seed) for seed in program["_seeds"]]
    clif_run = execute(host, clif_wasm, runs, seeds, work, f"{name}-clif")
    llvm_o0_run = execute(host, Path(llvm_o0["wasm"]), runs, seeds, work, f"{name}-llvm-o0")
    llvm_o2_run = execute(host, Path(llvm_o2["wasm"]), runs, seeds, work, f"{name}-llvm-o2")
    if not (
        clif_run["outcomes"] == llvm_o0_run["outcomes"] == llvm_o2_run["outcomes"]
    ):
        raise BenchError(
            f"{name}: outputs differ between the Cranelift pipeline and LLVM\n"
            f"clif:    {json.dumps(clif_run['outcomes'], sort_keys=True)}\n"
            f"llvm O0: {json.dumps(llvm_o0_run['outcomes'], sort_keys=True)}\n"
            f"llvm O2: {json.dumps(llvm_o2_run['outcomes'], sort_keys=True)}"
        )

    clif_sizes = sizes(host, clif_wasm, work, f"{name}-clif")
    llvm_o0_sizes = sizes(host, Path(llvm_o0["wasm"]), work, f"{name}-llvm-o0")
    llvm_o2_sizes = sizes(host, Path(llvm_o2["wasm"]), work, f"{name}-llvm-o2")

    incremental = clif.get("incremental")
    phases = compile_phase_summary(clif)
    return {
        "name": name,
        "class": program["class"],
        "source": program["source"],
        "source_lines": len(source.read_text(encoding="utf-8").splitlines()),
        "source_bytes": source.stat().st_size,
        "correct": True,
        "outcomes": clif_run["outcomes"],
        "clif": {
            "module_load_ms": clif["module_load_ms"],
            "load_source": clif["load_source"],
            "warmup_ms": clif["warmup"]["compile_ms"] if clif.get("warmup") else None,
            "compile_warm": compile_stats(clif["runs"]),
            "compile_incremental": (
                compile_stats(incremental["runs"]) if incremental else None
            ),
            "incremental_cold_ms": incremental["cold_ms"] if incremental else None,
            "incremental_first_edit_ms": (
                incremental["first_edit_ms"] if incremental else None
            ),
            "instantiate": stats([item["instantiate_ms"] for item in clif["runs"]]),
            "link": stats([item["link_ms"] for item in clif["runs"]]),
            "peak_memory_bytes": stats(
                [float(item["peak_memory_bytes"]) for item in clif["runs"]]
            ),
            "phases": phases,
            "compiler_fuel": (
                {
                    "median": clif["compiler_fuel"],
                    "runs": [item["fuel"] for item in clif.get("fuel_runs", [])],
                }
                if clif.get("compiler_fuel") is not None
                else None
            ),
            "wasm_bytes": clif["wasm_bytes"],
            "wasm_sha256": clif["wasm_sha256"],
        },
        "llvm": {
            "compile_o0": llvm_o0["compile_ms"],
            "compile_o2": llvm_o2["compile_ms"],
            "wasm_o0_bytes": Path(llvm_o0["wasm"]).stat().st_size,
            "wasm_o2_bytes": Path(llvm_o2["wasm"]).stat().st_size,
        },
        "exec": {
            "clif": exec_stats(clif_run),
            "llvm_o0": exec_stats(llvm_o0_run),
            "llvm_o2": exec_stats(llvm_o2_run),
        },
        "size": {
            "clif": clif_sizes,
            "llvm_o0": llvm_o0_sizes,
            "llvm_o2": llvm_o2_sizes,
        },
    }


def fmt(value: float | None, digits: int = 2) -> str:
    return "—" if value is None else f"{value:.{digits}f}"


def ratio(value: float | None, base: float | None) -> str:
    if value is None or base is None or base == 0:
        return "—"
    return f"{value / base:.2f}×"


def markdown_report(result: dict) -> str:
    lines = [
        f"# CogWasm benchmark ({result['kind']})",
        "",
        f"- commit: `{result['commit']['sha'][:12]}` {result['commit']['subject']}",
        (
            f"- machine: {result['machine']['kind']} — {result['machine']['cpu']} "
            f"({result['machine']['cores']} cores, {result['machine']['os']})"
        ),
        (
            f"- runs: {result['settings']['runs']} (median, after a warm-up); "
            f"seeds: {', '.join(str(seed) for seed in result['settings']['seeds'])}"
        ),
        (
            f"- rustc.wasm: {result['settings']['rustc_wasm_bytes']:,} bytes, "
            f"sha256 `{result['settings']['rustc_wasm_sha256'][:16]}…`"
        ),
        f"- wasmtime: {result['pins']['wasmtime']}",
        "",
        "## Compile",
        "",
        "| program | class | lines | clif warm (ms) | clif incr. (ms) | frontend | codegen | link | LLVM O0 (ms) | LLVM O2 (ms) | compiler fuel | peak MiB |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for program in result["programs"].values():
        clif = program["clif"]
        warm = clif["compile_warm"]
        incremental = clif["compile_incremental"]
        phases = clif["phases"]
        peak = clif["peak_memory_bytes"]["median"]
        fuel = clif.get("compiler_fuel")
        lines.append(
            "| {name} | {class} | {lines} | {warm} | {incr} | {frontend} | {codegen} | "
            "{link} | {o0} | {o2} | {fuel} | {peak} |".format(
                name=program["name"],
                **{"class": program["class"]},
                lines=program["source_lines"],
                warm=fmt(warm["median"]),
                incr=fmt(incremental["median"] if incremental else None),
                frontend=fmt(phases["frontend_ms"]),
                codegen=fmt(phases["codegen_ms"]),
                link=fmt(phases["link_ms"]),
                o0=fmt(program["llvm"]["compile_o0"]["median"]),
                o2=fmt(program["llvm"]["compile_o2"]["median"]),
                fuel=f"{(fuel or {}).get('median', 0):,}" if fuel else "—",
                peak=fmt(None if peak is None else peak / (1 << 20)),
            )
        )
    lines.extend(
        [
            "",
            "## Execution and size",
            "",
            "| program | clif exec (ms) | LLVM O0 (ms) | LLVM O2 (ms) | clif/O0 | clif/O2 | clif total/code/data (B) | LLVM O2 total/code/data (B) |",
            "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for program in result["programs"].values():
        run = program["exec"]
        clif_exec = run["clif"]["exec_ms"]["median"]
        o0_exec = run["llvm_o0"]["exec_ms"]["median"]
        o2_exec = run["llvm_o2"]["exec_ms"]["median"]
        clif_size = program["size"]["clif"]
        llvm_size = program["size"]["llvm_o2"]
        lines.append(
            "| {name} | {clif} | {o0} | {o2} | {r0} | {r2} | {ct}/{cc}/{cd} | {lt}/{lc}/{ld} |".format(
                name=program["name"],
                clif=fmt(clif_exec, 3),
                o0=fmt(o0_exec, 3),
                o2=fmt(o2_exec, 3),
                r0=ratio(clif_exec, o0_exec),
                r2=ratio(clif_exec, o2_exec),
                ct=clif_size["total_bytes"],
                cc=clif_size["code_bytes"],
                cd=clif_size["data_bytes"],
                lt=llvm_size["total_bytes"],
                lc=llvm_size["code_bytes"],
                ld=llvm_size["data_bytes"],
            )
        )
    lines.extend(
        [
            "",
            "All programs returned identical `run(seed)` values and output buffers for every",
            "seed in the Cranelift pipeline and both LLVM builds. Compile time excludes",
            "instantiation and linker-subprocess time, which are reported separately in the",
            "JSON. `frontend` is `total - codegen - link` from rustc's `-Ztime-passes`.",
            "",
        ]
    )
    return "\n".join(lines)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mode", choices=["quick", "full"], default="full")
    parser.add_argument("--programs", default="", help="comma-separated names or class/name")
    parser.add_argument("--runs", type=int, default=5)
    parser.add_argument("--llvm-runs", type=int, default=3)
    parser.add_argument("--phase-runs", type=int, default=1)
    parser.add_argument("--cold-runs", type=int, default=5)
    parser.add_argument("--host", type=Path, required=True)
    parser.add_argument("--toolchain", type=Path, required=True)
    parser.add_argument("--llvm-rustc", type=Path, required=True)
    parser.add_argument("--precompiled", type=Path)
    parser.add_argument(
        "--startup-precompiled",
        type=Path,
        help="measure startup from this .cwasm but keep program compiles on the cached path",
    )
    parser.add_argument("--out", type=Path)
    parser.add_argument("--no-incremental", action="store_true")
    parser.add_argument(
        "--fuel",
        action="store_true",
        help="also measure deterministic compiler work with Wasmtime fuel",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    if args.runs < 1:
        raise BenchError("--runs must be at least 1")
    if args.precompiled is not None and args.startup_precompiled is not None:
        raise BenchError("--precompiled and --startup-precompiled are mutually exclusive")
    result_path = args.out or (
        RESULTS
        / f"{args.mode}-{git_commit()['sha'][:12]}-"
        f"{datetime.now(UTC).strftime('%Y%m%dT%H%M%SZ')}.json"
    )
    result_path.parent.mkdir(parents=True, exist_ok=True)
    WORK.mkdir(parents=True, exist_ok=True)
    RESULTS.mkdir(parents=True, exist_ok=True)

    manifest = json.loads(PROGRAMS_FILE.read_text(encoding="utf-8"))
    programs = manifest["programs"]
    seeds = [int(seed) for seed in manifest["seeds"]]
    selected = programs
    if args.programs:
        wanted = {item.strip() for item in args.programs.split(",") if item.strip()}
        selected = [
            program
            for program in programs
            if program["name"] in wanted or f"{program['class']}/{program['name']}" in wanted
        ]
        missing = wanted - {
            program["name"] for program in selected
        } - {f"{program['class']}/{program['name']}" for program in selected}
        if missing:
            raise BenchError(f"unknown benchmark program(s): {', '.join(sorted(missing))}")
    elif args.mode == "quick":
        selected = [program for program in programs if program.get("quick")]
    if not selected:
        raise BenchError("no benchmark programs selected")

    if any(program.get("generated") for program in selected):
        generated = WORK / "generated_stress.rs"
        run(
            [
                sys.executable,
                str(BENCH / "scripts" / "gen_stress.py"),
                "--prelude",
                str(PRELUDE),
                "--out",
                str(generated),
            ]
        )

    if not args.no_incremental:
        for program in selected:
            if program.get("incremental", True):
                program["incremental"] = True
            else:
                program["incremental"] = False
    else:
        for program in selected:
            program["incremental"] = False

    deps = build_deps(args.toolchain, WORK)
    host = args.host.resolve()
    toolchain = args.toolchain.resolve()
    llvm_rustc = args.llvm_rustc.resolve()

    precompiled = args.precompiled
    if precompiled is not None:
        precompiled = precompiled.resolve()
        if not precompiled.exists():
            precompiled = None
    startup_precompiled = args.startup_precompiled
    if startup_precompiled is not None:
        startup_precompiled = startup_precompiled.resolve()
        if not startup_precompiled.exists():
            raise BenchError(
                f"--startup-precompiled {startup_precompiled} does not exist"
            )
    load = "precompiled" if precompiled is not None else "cached"

    print("measuring compiler startup", file=sys.stderr)
    startup_command = [
        str(host),
        "startup",
        "--rustc",
        str(toolchain / "stage-c" / "artifacts" / "rustc.wasm"),
        "--cache-dir",
        str(toolchain / "stage-c" / "cache"),
        "--runs",
        str(args.cold_runs if args.mode == "full" else min(args.cold_runs, 1)),
        "--mode",
        "all",
        "--json",
        str(WORK / "startup.json"),
    ]
    startup_artifact = precompiled if precompiled is not None else startup_precompiled
    if startup_artifact is not None:
        startup_command.extend(["--precompiled", str(startup_artifact)])
    startup = run_json(startup_command)

    measured = {}
    for program in selected:
        program["_seeds"] = seeds
        measured[program["name"]] = run_program(
            program,
            host,
            toolchain,
            llvm_rustc,
            deps,
            WORK,
            args.runs,
            args.llvm_runs,
            args.phase_runs,
            load,
            precompiled,
            args.fuel,
        )

    fuel_values = [
        float(program["clif"]["compiler_fuel"]["median"])
        for program in measured.values()
        if program["clif"].get("compiler_fuel") is not None
    ]
    deterministic = {
        "compiler_fuel_median": median(fuel_values),
        "compiler_fuel_programs": len(fuel_values),
    }

    rustc_version = run([str(llvm_rustc), "--version"], quiet=True).strip()
    result = {
        "schema": 1,
        "kind": args.mode,
        "generated_at": datetime.now(UTC).isoformat(),
        "machine": machine_info(),
        "commit": git_commit(),
        "pins": {
            "rustc": rustc_version,
            "nightly": os.environ.get("BENCH_NIGHTLY", "nightly-2026-10-19"),
            "wasmtime": startup.get("wasmtime_version", "unknown"),
            "libm": "0.2.16",
            "adler2": "2.0.1",
            "miniz_oxide": "0.9.1",
            "wild_sha256": sha256_file(toolchain / "target" / "wild" / "release" / "wild"),
        },
        "settings": {
            "runs": args.runs,
            "llvm_runs": args.llvm_runs,
            "phase_runs": args.phase_runs,
            "cold_runs": args.cold_runs if args.mode == "full" else min(args.cold_runs, 1),
            "seeds": seeds,
            "programs": list(measured),
            "load": load,
            "startup_precompiled": startup_artifact is not None,
            "fuel": args.fuel,
            "rustc_wasm_bytes": startup["rustc_bytes"],
            "rustc_wasm_sha256": startup["rustc_sha256"],
        },
        "startup": startup["modes"],
        "deterministic": deterministic,
        "programs": measured,
        "ok": True,
    }
    result_path.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    report_path = result_path.with_suffix(".md")
    report_path.write_text(markdown_report(result), encoding="utf-8")
    print(f"wrote {result_path} and {report_path}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except BenchError as error:
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
