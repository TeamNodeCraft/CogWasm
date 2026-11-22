"""Unit tests for history_point.py's edit-loop summary.

Run from the repository root:

    python3 -m unittest discover -s bench/scripts/tests -v
"""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import history_point


def write(path: Path, median: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps({"median": median, "edits": []}), encoding="utf-8")


class EditLoopSummaryTest(unittest.TestCase):
    def test_fixture_and_per_mode_medians(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write(root / "edit-loop.json", 20.8)
            sources = root / "sources"
            write(sources / "json-1-serve.json", 130.0)
            write(sources / "json-2-job.json", 160.0)
            write(sources / "json-3-job.json", 164.0)
            write(sources / "json-4-serve.json", 134.0)
            write(sources / "expr-1-serve.json", 0)  # not a time: ignored
            write(sources / "expr-2-job.json", 146.0)
            summary = history_point.edit_loop_summary(root)
        self.assertEqual(
            summary,
            {
                "schema": 1,
                "fixture_ms": 20.8,
                "sources": {
                    "expr": {"job_ms": 146.0},
                    "json": {"serve_ms": 132.0, "job_ms": 162.0},
                },
            },
        )

    def test_no_reports_means_no_block(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            self.assertIsNone(history_point.edit_loop_summary(Path(tmp)))


if __name__ == "__main__":
    unittest.main()
