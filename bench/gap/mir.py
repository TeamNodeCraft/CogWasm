#!/usr/bin/env python3
"""Measure rustc MIR optimisation levels in the CogWasm pipeline.

The product default is untouched: each variant passes extra `-Z` arguments to
the compiler inside `rustc.wasm` through `bench-host compile --rustc-arg`.
Every module is executed with the suite's seeds and its outcomes are compared
with the LLVM -O2 build; a mismatch aborts the experiment.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path

GAP = Path(__file__).resolve().parent
BENCH = GAP.parent
ROOT = BENCH.parent

VARIANTS = {
    "baseline": [],
    "mir3": ["-Zmir-opt-level=3"],
    "inline": [
        "-Zmir-opt-level=3",
        "-Zinline-mir=yes",
        "-Zinline-mir-threshold=100",
    ],
}


def load_run():
    spec = importlib.util.spec_from_file_location("bench_run", BENCH / "run.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def run_json(command: list[str], env: dict | None = None) -> dict:
    proc = subprocess.run(command, capture_output=True, text=True, env=env, check=False)
    if proc.returncode != 0:
        raise RuntimeError(
            f"command failed ({proc.returncode}): {' '.join(command)}\n"
            f"stdout:\n{proc.stdout}\nstderr:\n{proc.stderr}"
        )
    return json.loads(proc.stdout)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("programs", nargs="*")
    parser.add_argument("--runs", type=int, default=3, help="compile runs per variant")
    parser.add_argument("--exec-runs", type=int, default=5)
    parser.add_argument("--out", type=Path, default=GAP / "results" / "mir.json")
    args = parser.parse_args()

    run = load_run()
    toolchain = Path(
        os.environ.get(
            "BENCH_TOOLCHAIN", ROOT / "toolchain"
        )
    ).resolve()
    nightly = os.environ.get(
        "BENCH_NIGHTLY", run.manifest_value("rust-nightly", "version")
    )
    llvm_candidates = sorted(
        (toolchain / "rustup" / "toolchains").glob(f"{nightly}-*/bin/rustc")
    )
    if not llvm_candidates:
        raise RuntimeError(f"no {nightly} rustc")
    llvm_rustc = llvm_candidates[0]
    host = toolchain / "target" / "cogwasm-host" / "release" / "bench-host"
    manifest = json.loads((BENCH / "programs.json").read_text(encoding="utf-8"))
    wanted = set(args.programs)
    selected = [
        program
        for program in manifest["programs"]
        if not wanted or program["name"] in wanted
    ]
    if wanted - {program["name"] for program in selected}:
        raise RuntimeError(f"unknown programs: {sorted(wanted)}")
    seeds = [int(seed) for seed in manifest["seeds"]]

    os.environ.setdefault("BENCH_HOST_TRIPLE", run.detect_host_triple(toolchain))
    os.environ.setdefault("BENCH_NIGHTLY", nightly)
    os.environ.setdefault(
        "BENCH_HOST_TOOLCHAIN", run.manifest_value("rust-stable", "version")
    )
    work = BENCH / "work"
    gap_work = GAP / "work" / "mir"
    deps = run.build_deps(toolchain, work)
    if any(program.get("generated") for program in selected):
        run.run(
            [
                sys.executable,
                str(BENCH / "scripts" / "gen_stress.py"),
                "--prelude",
                str(BENCH / "programs" / "prelude.rs"),
                "--out",
                str(work / "generated_stress.rs"),
            ]
        )
        for program in selected:
            if program.get("generated"):
                program["source"] = str(work / "generated_stress.rs")

    results = {"variants": list(VARIANTS), "programs": {}, "seeds": seeds}
    for program in selected:
        name = program["name"]
        print(f"MIR experiment: {name}", file=sys.stderr)
        source = run.prepare_source(program, work / "sources" / f"{name}.rs")
        llvm_out = gap_work / "llvm_o2" / f"{name}.wasm"
        run.compile_llvm(
            llvm_rustc,
            program,
            source,
            llvm_out,
            2,
            run.extern_args(program, deps, "llvm"),
            1,
            run.dependency_search_paths(deps["llvm"]),
        )
        reference = run_json(
            [
                str(host),
                "run",
                "--wasm",
                str(llvm_out),
                "--runs",
                "1",
                "--seeds",
                ",".join(str(seed) for seed in seeds),
                "--json",
                str(gap_work / f"{name}-llvm.json"),
            ]
        )["outcomes"]
        entry = {"llvm_o2_bytes": llvm_out.stat().st_size, "variants": {}}
        for variant, extra_args in VARIANTS.items():
            out = gap_work / variant / f"{name}.wasm"
            out.parent.mkdir(parents=True, exist_ok=True)
            command = [
                str(host),
                "compile",
                "--rustc",
                str(toolchain / "artifacts" / "rustc.wasm"),
                "--sysroot",
                str(toolchain / "sysroot"),
                "--linker",
                str(toolchain / "target" / "wild" / "release" / "wild"),
                "--src",
                str(source),
                "--prelude",
                str(BENCH / "programs" / "prelude.rs"),
                "--out",
                str(out),
                "--work",
                str(gap_work / "compile" / variant / name),
                "--crate-name",
                name,
                "--runs",
                str(args.runs),
                "--phase-runs",
                "0",
                "--load",
                "cached",
                "--cache-dir",
                str(toolchain / "cache"),
                "--json",
                str(gap_work / variant / f"{name}-compile.json"),
            ]
            for argument in extra_args:
                command.append(f"--rustc-arg={argument}")
            command.extend(run.extern_args(program, deps, "clif"))
            for directory in run.dependency_search_paths(deps["clif"]):
                command.extend(["--search-path", directory])
            compiled = run_json(command)
            execution = run_json(
                [
                    str(host),
                    "run",
                    "--wasm",
                    str(out),
                    "--runs",
                    str(args.exec_runs),
                    "--seeds",
                    ",".join(str(seed) for seed in seeds),
                    "--json",
                    str(gap_work / variant / f"{name}-run.json"),
                ]
            )
            sizes = run_json(
                [
                    str(host),
                    "sizes",
                    "--wasm",
                    str(out),
                    "--json",
                    str(gap_work / variant / f"{name}-sizes.json"),
                ]
            )
            correct = execution["outcomes"] == reference
            if not correct:
                raise RuntimeError(f"{name}/{variant}: outputs differ from LLVM -O2")
            entry["variants"][variant] = {
                "compile_ms": compiled["medians"]["compile_ms"],
                "exec_ms": execution["medians"]["exec_ms"],
                "bytes": sizes["total_bytes"],
                "code_bytes": sizes["code_bytes"],
                "data_bytes": sizes["data_bytes"],
                "correct": correct,
                "wasm_sha256": hashlib.sha256(out.read_bytes()).hexdigest(),
            }
        results["programs"][name] = entry
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
    summary = mir_summary(results)
    (args.out.parent / "MIR.md").write_text(summary, encoding="utf-8")
    print(f"wrote {args.out} and {args.out.parent / 'MIR.md'}", file=sys.stderr)
    return 0


def mir_summary(results: dict) -> str:
    lines = [
        "# MIR optimisation experiment",
        "",
        "Compile and execution times are medians over the same seeds as the suite;",
        "`baseline` is the unmodified pipeline, `mir3` adds `-Zmir-opt-level=3`, and",
        "`inline` adds `-Zmir-opt-level=3 -Zinline-mir=yes -Zinline-mir-threshold=100`.",
        "All variants produced the same `run(seed)` values and output bytes as LLVM -O2.",
        "",
        "| program | baseline exec | mir3 | inline | baseline compile | mir3 | inline | baseline B | mir3 | inline |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for name, entry in results["programs"].items():
        variants = entry["variants"]
        base = variants["baseline"]
        lines.append(
            f"| {name} | {base['exec_ms']:.4f} | {variants['mir3']['exec_ms']:.4f} | "
            f"{variants['inline']['exec_ms']:.4f} | {base['compile_ms']:.1f} | "
            f"{variants['mir3']['compile_ms']:.1f} | {variants['inline']['compile_ms']:.1f} | "
            f"{base['bytes']} | {variants['mir3']['bytes']} | {variants['inline']['bytes']} |"
        )
    lines.append("")
    return "\n".join(lines)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:  # noqa: BLE001 - top-level CLI boundary
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
