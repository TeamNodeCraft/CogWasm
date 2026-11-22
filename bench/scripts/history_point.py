#!/usr/bin/env python3
"""Add CI point metadata to a quick benchmark JSON.

The raw quick result already contains the commit sha, subject, machine and
program list. This helper adds the durable fields the chart needs, normalises
the commit date to UTC, and prints the bench-history asset name:

    <commit-date>-<sha7>.json

It is shared by ``publish-history.sh`` (the current run) and
``backfill-history.sh`` (old ci-logs assets), so both write the same schema.

With ``--edit-loop DIR`` (the supervisor work directory CI's edit-loop steps
write, ``toolchain/work/supervisor``), the point also gets an ``edit_loop``
block: the resident supervisor's edit -> result for the small fixture and for
each plugin-sized source, serving guest and one guest per job.
"""

from __future__ import annotations

import argparse
import json
import re
import statistics
import sys
from datetime import UTC, datetime
from pathlib import Path
from typing import Any


class PointError(Exception):
    """The quick result cannot be turned into a history point."""


def parse_date(value: str) -> datetime:
    """Parse an ISO-8601 date or datetime into an aware UTC datetime."""
    try:
        parsed = datetime.fromisoformat(value)
    except ValueError as error:
        raise PointError(f"invalid commit date {value!r}: {error}") from error
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=UTC)
    return parsed.astimezone(UTC)


def enrich(raw: dict[str, Any], run_id: str, date: str, repo: str) -> tuple[dict[str, Any], str]:
    """Return the enriched quick result and its history asset name."""
    if raw.get("kind") != "quick":
        raise PointError(f"expected a quick benchmark result, got kind={raw.get('kind')!r}")
    commit = raw.get("commit")
    commit = commit if isinstance(commit, dict) else {}
    machine = raw.get("machine")
    machine = machine if isinstance(machine, dict) else {}
    sha = str(commit.get("sha") or "")
    if not re.fullmatch(r"[0-9a-fA-F]{7,40}", sha):
        raise PointError(f"quick result has no usable commit sha: {sha!r}")
    sha = sha.lower()
    subject = str(commit.get("subject") or "(no subject)")
    parsed_date = parse_date(date)
    runner = str(machine.get("kind") or machine.get("os") or "unknown")
    point = dict(raw)
    point["ci_point"] = {
        "schema": 1,
        "sha": sha,
        "subject": subject,
        "date": parsed_date.isoformat().replace("+00:00", "Z"),
        "run_id": str(run_id),
        "workflow": "cranelift_wasm",
        "runner": runner,
        "repo": repo,
    }
    return point, f"{parsed_date:%Y-%m-%d}-{sha[:7]}.json"


def _median_field(path: Path, field: str) -> float | None:
    """One numeric field of an edit-loop report, or None if it is unusable."""
    try:
        report = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    value = report.get(field) if isinstance(report, dict) else None
    return float(value) if isinstance(value, (int, float)) and value > 0 else None


def edit_loop_summary(directory: Path) -> dict[str, Any] | None:
    """Summarise CI's edit-loop reports under ``directory``, or None if absent.

    ``edit-loop.json`` is the small fixture (its ``median`` edit -> result in
    ms). ``sources/<name>-<slot>-<mode>.json`` are the plugin-sized runs that
    ``ci/edit-loop-sources.sh`` writes, two per mode; each source's value is
    the median of its runs' medians, as that script's notice reports it.
    """
    summary: dict[str, Any] = {"schema": 1}
    fixture = _median_field(directory / "edit-loop.json", "median")
    if fixture is not None:
        summary["fixture_ms"] = fixture
    runs: dict[str, dict[str, list[float]]] = {}
    for path in sorted((directory / "sources").glob("*-*-*.json")):
        name, _slot, mode = path.stem.split("-", 2)
        value = _median_field(path, "median")
        if value is not None and mode in ("serve", "job"):
            runs.setdefault(name, {}).setdefault(mode, []).append(value)
    sources = {
        name: {f"{mode}_ms": statistics.median(values) for mode, values in modes.items()}
        for name, modes in runs.items()
    }
    if sources:
        summary["sources"] = sources
    return summary if len(summary) > 1 else None


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("quick_json", type=Path)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--date", required=True, help="commit date in ISO-8601")
    parser.add_argument("--repo", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument(
        "--edit-loop",
        type=Path,
        help="supervisor work directory with CI's edit-loop reports",
    )
    args = parser.parse_args(argv)
    try:
        raw = json.loads(args.quick_json.read_text(encoding="utf-8"))
        if not isinstance(raw, dict):
            raise PointError("quick result is not a JSON object")
        point, asset_name = enrich(raw, args.run_id, args.date, args.repo)
        if args.edit_loop is not None:
            edit_loop = edit_loop_summary(args.edit_loop)
            if edit_loop is not None:
                point["edit_loop"] = edit_loop
    except (OSError, json.JSONDecodeError, PointError) as error:
        print(f"error: {args.quick_json}: {error}", file=sys.stderr)
        return 1
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(point, indent=2) + "\n", encoding="utf-8")
    print(asset_name)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
