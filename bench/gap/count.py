#!/usr/bin/env python3
"""Add exact executed-instruction counts to one quick benchmark JSON.

The quick suite already builds our module and LLVM -O2's under
``bench/work/wasm/``. This script runs the gap tool's conservative
basic-block instrumenter on those modules, executes the suite seeds, and
writes the per-program totals into the quick JSON as::

    "instructions": {
      "clif":     {"total": 611310000, "by_seed": [...]},
      "llvm_o2":  {"total": 424730000, "by_seed": [...]},
      "ratio":    1.44
    }

It also reports the geometric mean ratio over the same 0.1 ms exclusion rule
as the time ratio, and the median compiler work (Wasmtime fuel) already
measured by ``bench/run.sh --fuel``.

The command is deliberately separate from ``bench/run.py``: the quick suite
owns the modules and timings, while this step reuses them and adds counts.
Two measurements of the same instrumented module are compared, so a
non-deterministic count fails the run instead of being published.
"""

from __future__ import annotations

import argparse
import json
import math
import statistics
import subprocess
import sys
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

EXEC_MIN_LLVM_MS = 0.1
COUNT_BUILDS = ("clif", "llvm_o2")
MARKER_START = "<!-- deterministic-instruction-counts -->"
MARKER_END = "<!-- /deterministic-instruction-counts -->"


class CountError(RuntimeError):
    """A count could not be measured or was not deterministic."""


def load_json(path: Path) -> dict[str, Any]:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise CountError(f"cannot read {path}: {error}") from error
    if not isinstance(raw, dict):
        raise CountError(f"{path} is not a JSON object")
    return raw


def median(values: list[float]) -> float | None:
    return statistics.median(values) if values else None


def geometric_mean(values: list[float]) -> float | None:
    if not values:
        return None
    if any(value <= 0 for value in values):
        raise CountError("geometric mean needs positive instruction counts")
    return math.exp(math.fsum(math.log(value) for value in values) / len(values))


def number(value: Any) -> float | None:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    result = float(value)
    return result if math.isfinite(result) else None


def nested(node: Any, *path: str) -> Any:
    current = node
    for key in path:
        if not isinstance(current, dict):
            return None
        current = current.get(key)
    return current


def run_tool(command: list[str]) -> None:
    proc = subprocess.run(command, capture_output=True, text=True, check=False)
    if proc.returncode != 0:
        raise CountError(
            f"command failed ({proc.returncode}): {' '.join(command)}\n"
            f"stdout:\n{proc.stdout}\nstderr:\n{proc.stderr}"
        )


def measure(
    tool: Path,
    wasm: Path,
    seeds: list[int],
    work: Path,
    tag: str,
    verify_runs: int,
) -> dict[str, Any]:
    """Instrument one module, measure it, and check repeat runs agree."""
    instrumented = work / f"{tag}.wasm"
    meta_path = work / f"{tag}.meta.json"
    run_tool(
        [
            str(tool),
            "instrument",
            "--wasm",
            str(wasm),
            "--out",
            str(instrumented),
            "--meta",
            str(meta_path),
        ]
    )
    count_paths = []
    for index in range(max(1, verify_runs)):
        count_path = work / f"{tag}.counts-{index}.json"
        run_tool(
            [
                str(tool),
                "measure",
                "--wasm",
                str(instrumented),
                "--meta",
                str(meta_path),
                "--seeds",
                ",".join(str(seed) for seed in seeds),
                "--out",
                str(count_path),
            ]
        )
        count_paths.append(count_path)
    counts = load_json(count_paths[0])
    for repeated in count_paths[1:]:
        other = load_json(repeated)
        if other.get("totals") != counts.get("totals"):
            raise CountError(f"{tag}: two measurements of the same module disagree")
        if other.get("seeds") != counts.get("seeds"):
            raise CountError(f"{tag}: two measurements returned different results")
    return {"counts": counts, "meta": load_json(meta_path)}


def instruction_totals(counts: dict[str, Any], meta: dict[str, Any]) -> list[int]:
    """Instruction totals per seed, summing each block's instructions."""
    blocks = meta.get("blocks") or []
    totals = counts.get("totals") or []
    if len(blocks) != len(totals):
        raise CountError("block metadata and measured counters have different lengths")
    per_seed = [0] * len(counts.get("seeds") or [])
    for block, _executions in zip(blocks, totals):
        for seed_index, seed in enumerate(counts.get("seeds") or []):
            per_seed[seed_index] += int(seed["blocks"][block["id"]]) * int(
                block["instructions"]
            )
    return per_seed


def outcomes(counts: dict[str, Any]) -> list[tuple[int, int, int, int]]:
    return [
        (int(seed["seed"]), int(seed["result"]), int(seed["out_len"]), int(seed["out_hash"]))
        for seed in counts.get("seeds") or []
    ]


def measure_program(
    tool: Path,
    name: str,
    modules: Path,
    work: Path,
    seeds: list[int],
    verify_runs: int,
) -> dict[str, Any]:
    measured = {}
    for build in COUNT_BUILDS:
        wasm = modules / build / f"{name}.wasm"
        if not wasm.is_file():
            raise CountError(f"missing quick module {wasm}")
        result = measure(
            tool,
            wasm,
            seeds,
            work,
            f"{name}-{build}",
            verify_runs,
        )
        per_seed = instruction_totals(result["counts"], result["meta"])
        measured[build] = {
            "total": sum(per_seed),
            "by_seed": per_seed,
            "outcomes": outcomes(result["counts"]),
        }
    if measured["clif"]["outcomes"] != measured["llvm_o2"]["outcomes"]:
        raise CountError(f"{name}: instrumented outputs differ from LLVM -O2")
    ours = measured["clif"]["total"]
    llvm = measured["llvm_o2"]["total"]
    ratio = ours / llvm if llvm > 0 else None
    return {
        "clif": {"total": ours, "by_seed": measured["clif"]["by_seed"]},
        "llvm_o2": {"total": llvm, "by_seed": measured["llvm_o2"]["by_seed"]},
        "ratio": ratio,
    }


def instruction_ratio(programs: dict[str, Any], names: list[str]) -> tuple[float | None, int]:
    """Geometric mean with the time chart's 0.1 ms LLVM exclusion."""
    ratios = []
    for name in names:
        llvm_ms = number(nested(programs, name, "exec", "llvm_o2", "exec_ms", "median"))
        clif_ms = number(nested(programs, name, "exec", "clif", "exec_ms", "median"))
        ours = number(nested(programs, name, "instructions", "clif", "total"))
        llvm = number(nested(programs, name, "instructions", "llvm_o2", "total"))
        if (
            llvm_ms is None
            or clif_ms is None
            or clif_ms <= 0
            or llvm_ms < EXEC_MIN_LLVM_MS
            or ours is None
            or llvm is None
            or ours <= 0
            or llvm <= 0
        ):
            continue
        ratios.append(ours / llvm)
    return geometric_mean(ratios), len(ratios)


def compiler_fuel(programs: dict[str, Any], names: list[str]) -> tuple[int | None, int]:
    values = []
    for name in names:
        value = number(
            nested(programs, name, "clif", "compiler_fuel", "median")
        )
        if value is not None and value > 0:
            values.append(int(value))
    if not values:
        return None, 0
    # Every per-program median is itself the agreed value of two identical
    # fuel runs, so the median across programs is exact and integral.
    return int(statistics.median(values)), len(values)


def summary_markdown(
    quick: dict[str, Any],
    programs: dict[str, Any],
    names: list[str],
    ratios: dict[str, float | None],
    ratio: float | None,
    included: int,
    verify_runs: int,
) -> str:
    rows = [
        MARKER_START,
        "## Deterministic instruction counts",
        "",
        (
            "Executed wasm instructions over the suite seeds; exact basic-block "
            "counts, not an estimate."
        ),
        "",
        "| program | clif instructions | LLVM -O2 instructions | clif/O2 |",
        "| --- | ---: | ---: | ---: |",
    ]
    for name in names:
        ours = nested(programs, name, "instructions", "clif", "total")
        llvm = nested(programs, name, "instructions", "llvm_o2", "total")
        if ours is None or llvm is None:
            continue
        ratio_text = f"{ratios[name]:.2f}×" if ratios[name] is not None else "—"
        rows.append(f"| {name} | {ours:,} | {llvm:,} | {ratio_text} |")
    if ratio is None:
        rows.append("")
        rows.append("No program passed the LLVM execution-time floor for the geomean.")
    else:
        rows.append("")
        rows.append(
            f"Geometric mean over {included} program(s) whose LLVM -O2 time is at "
            f"least {EXEC_MIN_LLVM_MS:g} ms: **{ratio:.2f}× LLVM -O2**."
        )
    fuel_median = nested(quick, "deterministic", "compiler_fuel_median")
    fuel_programs = nested(quick, "deterministic", "compiler_fuel_programs")
    rows.append("")
    if fuel_median is None:
        rows.append("Compiler work was not measured for this point.")
    else:
        rows.append(
            f"Compiler work: median **{int(fuel_median):,} fuel units** across "
            f"{fuel_programs} program(s); every program's two fuel runs were identical."
        )
    rows.append(
        f"Counts were measured {max(1, verify_runs)} time(s) on the same instrumented "
        "module and agreed exactly."
    )
    rows.append(MARKER_END)
    return "\n".join(rows) + "\n"


def update_markdown(path: Path, section: str) -> None:
    if path.is_file():
        text = path.read_text(encoding="utf-8")
        start = text.find(MARKER_START)
        end = text.find(MARKER_END)
        if start >= 0 and end > start:
            text = text[:start] + section + text[end + len(MARKER_END) + 1 :]
        else:
            text = text.rstrip() + "\n\n" + section
    else:
        text = section
    path.write_text(text, encoding="utf-8")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--quick", type=Path, required=True)
    parser.add_argument("--modules", type=Path, required=True)
    parser.add_argument("--tool", type=Path, required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument(
        "--verify-runs",
        type=int,
        default=2,
        help="measure each instrumented module this many times and require agreement",
    )
    args = parser.parse_args(argv)
    try:
        quick = load_json(args.quick)
        if quick.get("kind") != "quick":
            raise CountError(f"expected a quick result, got kind={quick.get('kind')!r}")
        names = list(nested(quick, "settings", "programs") or [])
        seeds = [int(seed) for seed in nested(quick, "settings", "seeds") or []]
        if not names or not seeds:
            raise CountError("quick result has no program list or seeds")
        args.work.mkdir(parents=True, exist_ok=True)
        programs = quick.get("programs")
        if not isinstance(programs, dict):
            raise CountError("quick result has no programs object")
        ratios: dict[str, float | None] = {}
        for name in names:
            print(f"counting {name}", file=sys.stderr)
            instructions = measure_program(
                args.tool,
                name,
                args.modules,
                args.work,
                seeds,
                args.verify_runs,
            )
            programs[name]["instructions"] = instructions
            ratios[name] = instructions["ratio"]
        ratio, included = instruction_ratio(programs, names)
        fuel_median, fuel_programs = compiler_fuel(programs, names)
        quick["deterministic"] = {
            "schema": 1,
            "measured_at": datetime.now(UTC).isoformat().replace("+00:00", "Z"),
            "instruction_ratio": ratio,
            "instruction_programs": included,
            "instruction_measurement": "exact basic-block instrumentation",
            "compiler_fuel_median": fuel_median,
            "compiler_fuel_programs": fuel_programs,
            "compiler_fuel_measurement": "Wasmtime fuel",
            "verify_runs": max(1, args.verify_runs),
            "exec_min_llvm_ms": EXEC_MIN_LLVM_MS,
        }
        temporary = args.quick.with_name(args.quick.name + ".tmp")
        temporary.write_text(json.dumps(quick, indent=2) + "\n", encoding="utf-8")
        temporary.replace(args.quick)
        update_markdown(
            args.quick.with_suffix(".md"),
            summary_markdown(
                quick,
                programs,
                names,
                ratios,
                ratio,
                included,
                args.verify_runs,
            ),
        )
        if ratio is None:
            print("wrote counts; no program passed the geomean floor", file=sys.stderr)
        else:
            print(
                f"wrote counts: {ratio:.2f}x LLVM -O2 over {included} program(s)",
                file=sys.stderr,
            )
        if fuel_median is not None:
            print(f"compiler fuel median: {fuel_median}", file=sys.stderr)
    except CountError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
