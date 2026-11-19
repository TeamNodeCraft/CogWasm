#!/usr/bin/env python3
"""Add CI point metadata to a quick benchmark JSON.

The raw quick result already contains the commit sha, subject, machine and
program list. This helper adds the durable fields the chart needs, normalises
the commit date to UTC, and prints the bench-history asset name:

    <commit-date>-<sha7>.json

It is shared by ``publish-history.sh`` (the current run) and
``backfill-history.sh`` (old ci-logs assets), so both write the same schema.
"""

from __future__ import annotations

import argparse
import json
import re
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


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("quick_json", type=Path)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--date", required=True, help="commit date in ISO-8601")
    parser.add_argument("--repo", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        raw = json.loads(args.quick_json.read_text(encoding="utf-8"))
        if not isinstance(raw, dict):
            raise PointError("quick result is not a JSON object")
        point, asset_name = enrich(raw, args.run_id, args.date, args.repo)
    except (OSError, json.JSONDecodeError, PointError) as error:
        print(f"error: {args.quick_json}: {error}", file=sys.stderr)
        return 1
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(point, indent=2) + "\n", encoding="utf-8")
    print(asset_name)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
