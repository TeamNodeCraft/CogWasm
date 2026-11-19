#!/usr/bin/env python3
"""Compare two CogWasm benchmark JSON files.

Only changes larger than the run-to-run spread (and the configured threshold)
are flagged. Cross-machine comparisons are refused unless ``--force`` is given,
because CI runners and developer machines are not comparable.

Exit status is 0 unless ``--fail-on-regression`` is set and a regression larger
than ``--fail-threshold`` is found. The benchmark workflows keep it advisory at
first; see bench/README.md for how to make it blocking.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

LOWER_IS_BETTER = True


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", type=Path)
    parser.add_argument("new", type=Path)
    parser.add_argument(
        "--threshold",
        type=float,
        default=0.05,
        help="minimum relative change to flag (default 0.05 = 5%%)",
    )
    parser.add_argument(
        "--fail-threshold",
        type=float,
        default=0.10,
        help="regression size that fails with --fail-on-regression (default 0.10)",
    )
    parser.add_argument(
        "--fail-on-regression",
        action="store_true",
        help="exit non-zero on a flagged regression beyond --fail-threshold",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="compare across different machine kinds",
    )
    return parser.parse_args(argv)


def load(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise SystemExit(f"error: cannot read {path}: {error}") from error


def metric(entry: dict | None, *keys: str) -> dict | None:
    current = entry
    for key in keys:
        if not isinstance(current, dict):
            return None
        current = current.get(key)
        if current is None:
            return None
    if isinstance(current, (int, float)):
        return {"median": float(current), "min": float(current), "max": float(current), "runs": 1}
    if isinstance(current, dict) and current.get("median") is not None:
        return current
    return None


def relative_spread(stats: dict) -> float:
    median = stats["median"]
    if not median:
        return 0.0
    return max((stats["max"] - stats["min"]) / abs(median), 0.0)


def rows(base: dict, new: dict, threshold: float) -> list[dict]:
    result = []

    def add(scope: str, name: str, base_stats: dict | None, new_stats: dict | None) -> None:
        if base_stats is None or new_stats is None:
            return
        base_median = base_stats["median"]
        new_median = new_stats["median"]
        if base_median in (None, 0) or new_median is None:
            return
        delta = (new_median - base_median) / abs(base_median)
        noise = max(relative_spread(base_stats), relative_spread(new_stats))
        flagged = abs(delta) > max(noise, threshold)
        regression = flagged and (
            (delta > 0) if LOWER_IS_BETTER else (delta < 0)
        )
        result.append(
            {
                "scope": scope,
                "name": name,
                "base": base_median,
                "new": new_median,
                "delta": delta,
                "noise": noise,
                "flagged": flagged,
                "regression": regression,
            }
        )

    startup_base = base.get("startup", {})
    startup_new = new.get("startup", {})
    for mode in ("cold", "cached", "precompiled"):
        for field in ("module_load", "instantiate"):
            add(
                "startup",
                f"{mode}.{field}",
                metric(startup_base.get(mode), field),
                metric(startup_new.get(mode), field),
            )

    programs_base = base.get("programs", {})
    programs_new = new.get("programs", {})
    for name in sorted(set(programs_base) | set(programs_new)):
        old = programs_base.get(name)
        fresh = programs_new.get(name)
        if old is None or fresh is None:
            continue
        p = f"{fresh.get('class', '?')}/{name}"
        add(p, "compile.warm", metric(old, "clif", "compile_warm"), metric(fresh, "clif", "compile_warm"))
        add(
            p,
            "compile.incremental",
            metric(old, "clif", "compile_incremental"),
            metric(fresh, "clif", "compile_incremental"),
        )
        add(
            p,
            "compile.peak_memory",
            metric(old, "clif", "peak_memory_bytes"),
            metric(fresh, "clif", "peak_memory_bytes"),
        )
        add(p, "exec.clif", metric(old, "exec", "clif", "exec_ms"), metric(fresh, "exec", "clif", "exec_ms"))
        add(
            p,
            "size.total",
            metric(old, "size", "clif", "total_bytes"),
            metric(fresh, "size", "clif", "total_bytes"),
        )
        add(
            p,
            "size.code",
            metric(old, "size", "clif", "code_bytes"),
            metric(fresh, "size", "clif", "code_bytes"),
        )
        add(
            p,
            "size.data",
            metric(old, "size", "clif", "data_bytes"),
            metric(fresh, "size", "clif", "data_bytes"),
        )
    return result


def fmt(value: float) -> str:
    if abs(value) >= 1000:
        return f"{value:,.1f}"
    if abs(value) >= 10:
        return f"{value:.2f}"
    return f"{value:.4f}"


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    base = load(args.base)
    new = load(args.new)
    base_kind = base.get("machine", {}).get("kind", "unknown")
    new_kind = new.get("machine", {}).get("kind", "unknown")
    if base_kind != new_kind and not args.force:
        print(
            f"error: refusing to compare {base_kind} with {new_kind}; "
            "record both on the same machine kind, or pass --force",
            file=sys.stderr,
        )
        return 2
    if base_kind != new_kind:
        print(f"warning: forced comparison across {base_kind} and {new_kind}", file=sys.stderr)

    base_pins = base.get("pins", {})
    new_pins = new.get("pins", {})
    for pin in ("wasmtime", "rustc"):
        if base_pins.get(pin) != new_pins.get(pin):
            print(
                f"note: {pin} differs ({base_pins.get(pin)!r} -> {new_pins.get(pin)!r}); "
                "treat execution and compile-time deltas with care",
                file=sys.stderr,
            )

    print(f"base: {base.get('commit', {}).get('sha', '?')[:12]} ({args.base})")
    print(f"new:  {new.get('commit', {}).get('sha', '?')[:12]} ({args.new})")
    print()
    print("| program | metric | base | new | delta | spread | flag |")
    print("| --- | --- | ---: | ---: | ---: | ---: | --- |")
    rows_out = rows(base, new, args.threshold)
    flagged = [row for row in rows_out if row["flagged"]]
    regressions = [row for row in flagged if row["regression"]]
    for row in rows_out:
        if not row["flagged"]:
            continue
        flag = "regression" if row["regression"] else "improvement"
        print(
            f"| {row['scope']} | {row['name']} | {fmt(row['base'])} | "
            f"{fmt(row['new'])} | {row['delta'] * 100:+.1f}% | "
            f"±{row['noise'] * 100:.1f}% | {flag} |"
        )
    print()
    print(
        f"{len(flagged)} flagged change(s) out of {len(rows_out)} metrics; "
        f"threshold {args.threshold * 100:.1f}%, noise = run-to-run spread."
    )
    if not flagged:
        print("No change larger than the run-to-run spread.")
    if regressions:
        print()
        print("Regressions beyond the noise threshold:")
        for row in regressions:
            print(
                f"  {row['scope']} {row['name']}: {row['delta'] * 100:+.1f}% "
                f"({fmt(row['base'])} -> {fmt(row['new'])})"
            )
    if args.fail_on_regression:
        failing = [
            row
            for row in regressions
            if row["delta"] > args.fail_threshold
        ]
        if failing:
            print(
                f"error: {len(failing)} regression(s) above "
                f"{args.fail_threshold * 100:.1f}%",
                file=sys.stderr,
            )
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
