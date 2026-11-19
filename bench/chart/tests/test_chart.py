"""Unit tests for the benchmark chart generator.

Run from the repository root:

    python3 -m unittest discover -s bench/chart/tests -v
"""

from __future__ import annotations

import json
import math
import re
import sys
import tempfile
import unittest
from pathlib import Path

CHART_DIR = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(CHART_DIR))

import chart  # noqa: E402  (the test adds the script directory to sys.path)

FIXTURES = Path(__file__).resolve().parent / "fixtures"


def programs_with(*entries: tuple[str, float | None, float | None]) -> dict:
    """Build a minimal programs mapping of (name, clif_ms, llvm_ms)."""
    programs = {}
    for name, clif_ms, llvm_ms in entries:
        program = {}
        if clif_ms is not None:
            program["exec"] = {"clif": {"exec_ms": {"median": clif_ms}}}
        if llvm_ms is not None:
            program.setdefault("exec", {})["llvm_o2"] = {
                "exec_ms": {"median": llvm_ms}
            }
        programs[name] = program
    return programs


class GeometricMeanTest(unittest.TestCase):
    def test_multiplies_through_logs(self) -> None:
        self.assertAlmostEqual(chart.geometric_mean([1.0, 4.0, 16.0]), 4.0)
        self.assertAlmostEqual(chart.geometric_mean([2.0]), 2.0)
        self.assertIsNone(chart.geometric_mean([]))

    def test_rejects_non_positive_values(self) -> None:
        with self.assertRaises(chart.ChartError):
            chart.geometric_mean([1.0, 0.0])


class ExecutionRatioTest(unittest.TestCase):
    def test_excludes_llvm_times_under_the_floor(self) -> None:
        programs = programs_with(
            ("tiny", 0.10, 0.05),  # excluded: LLVM is under 0.1 ms
            ("ok", 0.40, 0.20),  # ratio 2
            ("also_ok", 0.60, 0.30),  # ratio 2
        )
        ratio, count = chart.execution_ratio(programs, ["tiny", "ok", "also_ok"])
        self.assertEqual(count, 2)
        self.assertAlmostEqual(ratio, 2.0)

    def test_floor_is_inclusive(self) -> None:
        programs = programs_with(("edge", 0.20, 0.10))
        ratio, count = chart.execution_ratio(programs, ["edge"])
        self.assertEqual(count, 1)
        self.assertAlmostEqual(ratio, 2.0)

    def test_missing_metrics_are_skipped(self) -> None:
        programs = programs_with(
            ("no_llvm", 0.20, None),
            ("no_clif", None, 0.20),
            ("no_exec", None, None),
        )
        ratio, count = chart.execution_ratio(
            programs, ["no_llvm", "no_clif", "no_exec"]
        )
        self.assertIsNone(ratio)
        self.assertEqual(count, 0)

    def test_geometric_mean_of_the_surviving_ratios(self) -> None:
        programs = programs_with(
            ("two", 0.40, 0.20),
            ("eight", 1.60, 0.20),
            ("excluded", 0.10, 0.01),
        )
        ratio, count = chart.execution_ratio(programs, ["two", "eight", "excluded"])
        self.assertEqual(count, 2)
        self.assertAlmostEqual(ratio, 4.0)


class SizeRatioTest(unittest.TestCase):
    def test_skips_a_program_missing_either_size(self) -> None:
        programs = {
            "both": {
                "size": {
                    "clif": {"total_bytes": 2000},
                    "llvm_o2": {"total_bytes": 1000},
                }
            },
            "missing_clif": {"size": {"llvm_o2": {"total_bytes": 1000}}},
            "missing_llvm": {"size": {"clif": {"total_bytes": 1000}}},
        }
        ratio, count = chart.size_ratio(
            programs, ["both", "missing_clif", "missing_llvm"]
        )
        self.assertEqual(count, 1)
        self.assertAlmostEqual(ratio, 2.0)

    def test_uses_all_quick_programs_not_the_execution_subset(self) -> None:
        programs = {
            "small": {
                "size": {
                    "clif": {"total_bytes": 400},
                    "llvm_o2": {"total_bytes": 100},
                }
            },
            "large": {
                "size": {
                    "clif": {"total_bytes": 900},
                    "llvm_o2": {"total_bytes": 100},
                }
            },
        }
        ratio, count = chart.size_ratio(programs, ["small", "large"])
        self.assertEqual(count, 2)
        self.assertAlmostEqual(ratio, 6.0)


class FixtureTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.points = chart.load_points(FIXTURES / "history")

    def test_points_are_in_commit_date_order(self) -> None:
        self.assertEqual(len(self.points), 5)
        self.assertEqual(
            [point.short_sha for point in self.points],
            ["aaaaaaa", "bbbbbbb", "ccccccc", "ddddddd", "eeeeeee"],
        )

    def test_subset_change_marks_only_the_changed_commit(self) -> None:
        marked = [point.short_sha for point in self.points if point.subset_changed]
        self.assertEqual(marked, ["ccccccc"])
        changed = self.points[2]
        self.assertEqual(changed.subset_added, ["sha256"])
        self.assertEqual(changed.subset_removed, ["expr"])
        self.assertFalse(self.points[0].subset_changed)

    def test_execution_exclusion_is_applied_per_point(self) -> None:
        first = self.points[0]
        self.assertEqual(first.exec_programs, 2)
        self.assertAlmostEqual(first.metrics["exec_ratio"], 2.0)

    def test_missing_metric_is_none_not_zero(self) -> None:
        missing = self.points[3]
        self.assertIsNone(missing.metrics["incremental_ms"])
        self.assertIsNone(missing.metrics["precompiled_load_ms"])
        self.assertEqual(missing.size_programs, 2)
        self.assertEqual(self.points[4].metrics["precompiled_load_ms"], 16.0)

    def test_data_json_has_every_series_and_value(self) -> None:
        data = chart.build_data(self.points, repo="TeamNodeCraft/CogWasm")
        self.assertEqual(len(data["points"]), 5)
        self.assertEqual(
            [series["key"] for series in data["series"]],
            [
                "exec_ratio",
                "size_ratio",
                "warm_compile_ms",
                "incremental_ms",
                "precompiled_load_ms",
            ],
        )
        self.assertEqual(data["reference"]["value"], 1.0)
        self.assertEqual(data["exec_min_llvm_ms"], 0.1)
        json.dumps(data)  # the page consumes this as JSON

    def test_static_svg_is_the_readme_size_and_has_the_markers(self) -> None:
        data = chart.build_data(self.points)
        svg = chart.render_svg(data)
        self.assertIn('width="800"', svg)
        self.assertIn('height="420"', svg)
        self.assertIn("LLVM -O2", svg)
        self.assertIn("Quick program subset changed at ccccccc", svg)
        self.assertIn("Compile speed (ms, lower is better)", svg)
        self.assertNotIn("<script", svg)

    def test_outputs_are_written(self) -> None:
        data = chart.build_data(self.points)
        with tempfile.TemporaryDirectory() as directory:
            svg_path, data_path = chart.write_outputs(data, Path(directory))
            self.assertTrue(svg_path.is_file())
            self.assertTrue(data_path.is_file())
            self.assertTrue(svg_path.read_text(encoding="utf-8").startswith("<svg"))
            parsed = json.loads(data_path.read_text(encoding="utf-8"))
            self.assertEqual(len(parsed["points"]), 5)


class SinglePointTest(unittest.TestCase):
    def test_one_point_still_charts(self) -> None:
        points = chart.load_points(FIXTURES / "single")
        self.assertEqual(len(points), 1)
        point = points[0]
        self.assertFalse(point.subset_changed)
        self.assertEqual(point.metrics["exec_ratio"], 1.0)
        self.assertAlmostEqual(point.metrics["size_ratio"], 2.0)
        data = chart.build_data(points)
        svg = chart.render_svg(data)
        self.assertIn("ffffff", svg)
        self.assertIn("<circle", svg)
        self.assertEqual(len(data["points"]), 1)

    def test_one_point_centres_the_marker(self) -> None:
        points = chart.load_points(FIXTURES / "single")
        data = chart.build_data(points)
        svg = chart.render_svg(data)
        # Panel 1's plot spans x=76..372, so a single point belongs at 224.
        # Drawing it on an edge would clip the marker and its sha label.
        match = re.search(r'<circle cx="([0-9.]+)"', svg)
        self.assertIsNotNone(match)
        self.assertAlmostEqual(float(match.group(1)), 224.0, delta=0.5)
        self.assertTrue(math.isfinite(points[0].metrics["warm_compile_ms"]))


if __name__ == "__main__":
    unittest.main()
