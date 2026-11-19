#!/usr/bin/env python3
"""Build the CogWasm benchmark trend chart from the CI history points.

    python3 bench/chart/chart.py --history bench/chart/work/history \
        --output bench/chart/work

Each JSON in --history is one commit's quick benchmark result plus the
``ci_point`` metadata written by ``bench/scripts/publish-history.sh`` (or
``bench/scripts/backfill-history.sh``). The script writes:

* ``chart.svg`` - the static image linked from the README (light theme,
  fixed 800x420), and
* ``data.json`` - the same series for the interactive page.

Only the Python standard library is used so CI and a developer machine run
the same code without a setup step.

What is plotted (lower is better throughout):

Panel 1 "Against LLVM -O2 (lower is better, 1.0 = parity)"
  * execution time ours/LLVM -O2, geometric mean over programs whose LLVM
    time is at least 0.1 ms (the same exclusion rule for every commit),
  * module size ours/LLVM -O2, geometric mean over the quick programs, and
  * a dashed reference line at 1.0 labelled "LLVM -O2".

Panel 2 "Compile speed (ms, lower is better)"
  * median warm compile across the quick programs,
  * median incremental (edit-to-result) compile, when the point has it, and
  * precompiled compiler load, when the point has it.

The x axis is commit order by commit date, not elapsed time. A commit whose
quick program subset differs from its predecessor is marked with a dashed
vertical rule, because a different subset would otherwise silently mix
different measurements.
"""

from __future__ import annotations

import argparse
import html
import json
import math
import os
import re
import statistics
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable

EXEC_MIN_LLVM_MS = 0.1
SHORT_SHA_LENGTH = 7
DEFAULT_REPO = "TeamNodeCraft/CogWasm"
DEFAULT_PAGE_URL = "https://teamnodecraft.github.io/CogWasm/#benchmarks"

# Okabe-Ito, a colour-blind-safe palette. Markers are distinct as well, so the
# lines stay distinguishable when colour is not available.
SERIES: tuple[dict[str, Any], ...] = (
    {
        "key": "exec_ratio",
        "panel": 1,
        "label": "Execution",
        "unit": "x LLVM -O2",
        "digits": 2,
        "color": "#0072B2",
        "marker": "circle",
    },
    {
        "key": "size_ratio",
        "panel": 1,
        "label": "Module size",
        "unit": "x LLVM -O2",
        "digits": 2,
        "color": "#D55E00",
        "marker": "square",
    },
    {
        "key": "warm_compile_ms",
        "panel": 2,
        "label": "Warm compile",
        "unit": "ms",
        "digits": 1,
        "color": "#009E73",
        "marker": "triangle",
    },
    {
        "key": "incremental_ms",
        "panel": 2,
        "label": "Incremental",
        "unit": "ms",
        "digits": 1,
        "color": "#CC79A7",
        "marker": "diamond",
    },
    {
        "key": "precompiled_load_ms",
        "panel": 2,
        "label": "Precompiled load",
        "unit": "ms",
        "digits": 1,
        "color": "#E69F00",
        "marker": "plus",
    },
)

PANELS: tuple[dict[str, Any], ...] = (
    {
        "id": 1,
        "title": "Against LLVM -O2 (lower is better, 1.0 = parity)",
    },
    {
        "id": 2,
        "title": "Compile speed (ms, lower is better)",
    },
)

REFERENCE = {"panel": 1, "value": 1.0, "label": "LLVM -O2"}


class ChartError(Exception):
    """A history point or output request that cannot be charted."""


@dataclass
class Point:
    """One commit's CI quick benchmark result, reduced to chart values."""

    sha: str
    subject: str
    date: datetime
    run_id: str
    runner: str
    programs: list[str]
    metrics: dict[str, float | None]
    exec_programs: int
    size_programs: int
    subset_changed: bool = False
    subset_added: list[str] = field(default_factory=list)
    subset_removed: list[str] = field(default_factory=list)
    source: str = ""

    @property
    def short_sha(self) -> str:
        return self.sha[:SHORT_SHA_LENGTH]

    def to_dict(self, index: int) -> dict[str, Any]:
        return {
            "index": index,
            "sha": self.sha,
            "short_sha": self.short_sha,
            "subject": self.subject,
            "date": self.date.isoformat().replace("+00:00", "Z"),
            "run_id": self.run_id,
            "runner": self.runner,
            "programs": self.programs,
            "program_count": len(self.programs),
            "subset_changed": self.subset_changed,
            "subset_added": self.subset_added,
            "subset_removed": self.subset_removed,
            "metrics": self.metrics,
            "exec_programs": self.exec_programs,
            "size_programs": self.size_programs,
            "source": self.source,
        }


def _get(node: Any, *path: str) -> Any:
    """Return a nested mapping value, or None when any step is absent."""
    current = node
    for key in path:
        if not isinstance(current, dict):
            return None
        current = current.get(key)
    return current


def _number(value: Any) -> float | None:
    """Return a finite float for a JSON number, otherwise None."""
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    number = float(value)
    return number if math.isfinite(number) else None


def geometric_mean(values: Iterable[float]) -> float | None:
    """Geometric mean of positive values, or None for an empty series."""
    positive = list(values)
    if not positive:
        return None
    if any(value <= 0 for value in positive):
        raise ChartError("geometric mean needs positive values")
    return math.exp(math.fsum(math.log(value) for value in positive) / len(positive))


def execution_ratio(
    programs: dict[str, Any],
    names: Iterable[str],
    min_llvm_ms: float = EXEC_MIN_LLVM_MS,
) -> tuple[float | None, int]:
    """Geometric mean of ours/LLVM -O2, excluding LLVM times under the floor.

    Tiny LLVM timings are dominated by process and timer noise, so the same
    floor is applied to every commit. Returns the ratio and how many programs
    contributed to it.
    """
    ratios = []
    for name in names:
        clif = _number(_get(programs, name, "exec", "clif", "exec_ms", "median"))
        llvm = _number(_get(programs, name, "exec", "llvm_o2", "exec_ms", "median"))
        if clif is None or llvm is None or llvm < min_llvm_ms or clif <= 0:
            continue
        ratios.append(clif / llvm)
    return geometric_mean(ratios), len(ratios)


def size_ratio(
    programs: dict[str, Any], names: Iterable[str]
) -> tuple[float | None, int]:
    """Geometric mean of our module size over LLVM -O2's, over the subset."""
    ratios = []
    for name in names:
        ours = _number(_get(programs, name, "size", "clif", "total_bytes"))
        llvm = _number(_get(programs, name, "size", "llvm_o2", "total_bytes"))
        if ours is None or llvm is None or ours <= 0 or llvm <= 0:
            continue
        ratios.append(ours / llvm)
    return geometric_mean(ratios), len(ratios)


def median_metric(
    programs: dict[str, Any], names: Iterable[str], *path: str
) -> float | None:
    """Median across programs of one per-program median metric."""
    values = []
    for name in names:
        value = _number(_get(programs, name, *path))
        if value is not None:
            values.append(value)
    return statistics.median(values) if values else None


def parse_datetime(value: Any) -> datetime | None:
    """Parse an ISO-8601 timestamp into an aware UTC datetime."""
    if not isinstance(value, str):
        return None
    try:
        parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def _program_names(raw: dict[str, Any]) -> list[str]:
    """The quick program list recorded in the point, in recorded order."""
    recorded = _get(raw, "settings", "programs")
    if isinstance(recorded, list) and recorded:
        return [str(name) for name in recorded]
    programs = raw.get("programs")
    if isinstance(programs, dict) and programs:
        return sorted(str(name) for name in programs)
    raise ChartError("point has no settings.programs or programs")


def point_from_json(raw: dict[str, Any], source: str) -> Point:
    """Reduce one bench-history JSON document to a chart point."""
    if not isinstance(raw, dict):
        raise ChartError(f"{source}: point is not a JSON object")
    ci = raw.get("ci_point")
    ci = ci if isinstance(ci, dict) else {}
    commit = raw.get("commit")
    commit = commit if isinstance(commit, dict) else {}
    machine = raw.get("machine")
    machine = machine if isinstance(machine, dict) else {}

    sha = str(ci.get("sha") or commit.get("sha") or "")
    if not re.fullmatch(r"[0-9a-fA-F]{7,40}", sha):
        raise ChartError(f"{source}: no usable commit sha")
    sha = sha.lower()
    subject = str(ci.get("subject") or commit.get("subject") or "(no subject)")
    date = (
        parse_datetime(ci.get("date"))
        or parse_datetime(commit.get("date"))
        or parse_datetime(raw.get("generated_at"))
    )
    if date is None:
        raise ChartError(f"{source}: no commit date (ci_point.date, commit.date, generated_at)")
    run_id = str(ci.get("run_id") or "")
    runner = str(
        ci.get("runner") or machine.get("kind") or machine.get("os") or "unknown"
    )

    names = _program_names(raw)
    programs = raw.get("programs")
    programs = programs if isinstance(programs, dict) else {}
    exec_value, exec_count = execution_ratio(programs, names)
    size_value, size_count = size_ratio(programs, names)
    precompiled_mode = _get(raw, "startup", "precompiled")
    precompiled_value = None
    if (
        isinstance(precompiled_mode, dict)
        and precompiled_mode.get("source") == "precompiled"
    ):
        precompiled_value = _number(
            _get(precompiled_mode, "module_load", "median_ms")
        )
    return Point(
        sha=sha,
        subject=subject,
        date=date,
        run_id=run_id,
        runner=runner,
        programs=names,
        metrics={
            "exec_ratio": exec_value,
            "size_ratio": size_value,
            "warm_compile_ms": median_metric(
                programs, names, "clif", "compile_warm", "median"
            ),
            "incremental_ms": median_metric(
                programs, names, "clif", "compile_incremental", "median"
            ),
            "precompiled_load_ms": precompiled_value,
        },
        exec_programs=exec_count,
        size_programs=size_count,
        source=source,
    )


def mark_subsets(points: list[Point]) -> None:
    """Mark points whose quick program subset differs from the previous one."""
    for index, point in enumerate(points):
        if index == 0:
            point.subset_changed = False
            continue
        previous = set(points[index - 1].programs)
        current = set(point.programs)
        point.subset_changed = current != previous
        point.subset_added = sorted(current - previous)
        point.subset_removed = sorted(previous - current)


def _run_order(point: Point) -> tuple[datetime, int]:
    try:
        run_id = int(point.run_id)
    except ValueError:
        run_id = 0
    return point.date, run_id


def _deduplicate(points: list[Point]) -> list[Point]:
    """Keep one point per commit; a re-run replaces that commit's point."""
    newest: dict[str, Point] = {}
    for point in points:
        previous = newest.get(point.sha)
        if previous is None or _run_order(point) > _run_order(previous):
            newest[point.sha] = point
    return list(newest.values())


def load_points(history_dir: Path) -> list[Point]:
    """Read every history JSON in a directory, ordered by commit date."""
    if not history_dir.is_dir():
        raise ChartError(f"history directory does not exist: {history_dir}")
    files = sorted(
        path
        for path in history_dir.iterdir()
        if path.is_file() and path.suffix == ".json"
    )
    if not files:
        raise ChartError(f"no history JSON files in {history_dir}")
    points = []
    for path in files:
        try:
            raw = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            raise ChartError(f"{path.name}: cannot read point: {error}") from error
        points.append(point_from_json(raw, path.name))
    points = _deduplicate(points)
    points.sort(key=lambda point: (point.date, point.sha))
    mark_subsets(points)
    return points


def build_data(
    points: list[Point],
    repo: str = DEFAULT_REPO,
    page_url: str = DEFAULT_PAGE_URL,
) -> dict[str, Any]:
    """Build the JSON document shared by the static SVG and the web page."""
    return {
        "schema": 1,
        "generated_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "repo": repo,
        "page_url": page_url,
        "exec_min_llvm_ms": EXEC_MIN_LLVM_MS,
        "panels": list(PANELS),
        "series": list(SERIES),
        "reference": REFERENCE,
        "points": [point.to_dict(index) for index, point in enumerate(points)],
    }


# ---------------------------------------------------------------------------
# Static SVG (the README image)
# ---------------------------------------------------------------------------

SVG_WIDTH = 800
SVG_HEIGHT = 420
FONT = (
    "-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif"
)
INK = "#1f2328"
MUTED = "#59636e"
GRID = "#e6e8eb"
BORDER = "#d8dee4"
ACCENT = "#8250df"


def _x_positions(count: int, left: float, right: float) -> list[float]:
    if count <= 1:
        return [(left + right) / 2.0]
    step = (right - left) / (count - 1)
    return [left + index * step for index in range(count)]


def _nice_ticks(high: float, intervals: int = 5) -> tuple[float, list[float]]:
    """Pick a clean upper bound and tick values for a zero-based axis."""
    high = max(high, 1e-9)
    raw_step = high / intervals
    exponent = math.floor(math.log10(raw_step))
    fraction = raw_step / (10**exponent)
    for candidate in (1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 7.5, 10.0):
        if fraction <= candidate:
            step = candidate * (10**exponent)
            break
    else:
        step = 10.0 * (10**exponent)
    top = math.ceil(high / step - 1e-9) * step
    count = int(round(top / step))
    return top, [index * step for index in range(count + 1)]


def _panel_high(points: list[Point], panel: int) -> float:
    values = [
        point.metrics[str(series["key"])]
        for point in points
        for series in SERIES
        if series["panel"] == panel
        and point.metrics.get(str(series["key"])) is not None
    ]
    high = max(values) * 1.06 if values else 1.0
    if panel == REFERENCE["panel"]:
        # The dashed parity line must stay on the panel even after the ratios
        # drop below 1.0, which is exactly the progress the chart is for.
        high = max(high, float(REFERENCE["value"]) * 1.05)
    return high


def _marker(kind: str, x: float, y: float, color: str, size: float = 3.2) -> str:
    """Small distinct marker; the fill is the series colour."""
    if kind == "circle":
        return (
            f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{size:.1f}" '
            f'fill="{color}" stroke="#ffffff" stroke-width="1"/>'
        )
    if kind == "square":
        side = size * 1.8
        return (
            f'<rect x="{x - side / 2:.1f}" y="{y - side / 2:.1f}" '
            f'width="{side:.1f}" height="{side:.1f}" fill="{color}" '
            f'stroke="#ffffff" stroke-width="1"/>'
        )
    if kind == "triangle":
        return (
            f'<path d="M {x:.1f} {y - size:.1f} L {x + size:.1f} '
            f'{y + size:.1f} L {x - size:.1f} {y + size:.1f} Z" '
            f'fill="{color}" stroke="#ffffff" stroke-width="1"/>'
        )
    if kind == "diamond":
        return (
            f'<path d="M {x:.1f} {y - size:.1f} L {x + size:.1f} {y:.1f} '
            f'L {x:.1f} {y + size:.1f} L {x - size:.1f} {y:.1f} Z" '
            f'fill="{color}" stroke="#ffffff" stroke-width="1"/>'
        )
    if kind == "plus":
        return (
            f'<path d="M {x - size:.1f} {y:.1f} L {x + size:.1f} {y:.1f} '
            f'M {x:.1f} {y - size:.1f} L {x:.1f} {y + size:.1f}" '
            f'stroke="{color}" stroke-width="1.8" stroke-linecap="round"/>'
        )
    raise ChartError(f"unknown marker kind: {kind}")


def _segments(
    points: list[Point],
    key: str,
    xs: list[float],
    y_for: Any,
) -> list[list[tuple[float, float]]]:
    """Polyline segments, split where a point has no value for the series."""
    segments: list[list[tuple[float, float]]] = []
    current: list[tuple[float, float]] = []
    for point, x in zip(points, xs):
        value = point.metrics.get(key)
        if value is None:
            if len(current) > 1:
                segments.append(current)
            current = []
        else:
            current.append((x, y_for(value)))
    if len(current) > 1:
        segments.append(current)
    return segments


def _series_has_data(points: list[Point], key: str) -> bool:
    return any(point.metrics.get(key) is not None for point in points)


def _legend(
    x: float, y: float, panel: int, points: list[Point]
) -> str:
    """Legend row under one panel, including the reference line for panel 1."""
    parts = []
    cursor = x
    for series in SERIES:
        if series["panel"] != panel:
            continue
        has_data = _series_has_data(points, str(series["key"]))
        color = str(series["color"]) if has_data else "#afb8c1"
        label = str(series["label"])
        title = "" if has_data else '<title>No values for this line in these points yet</title>'
        parts.append(
            f'<g>{title}{_marker(str(series["marker"]), cursor + 4, y - 3, color, 2.8)}'
            f'<text x="{cursor + 11:.1f}" y="{y:.1f}" font-size="9" '
            f'fill="{MUTED}">{html.escape(label)}</text></g>'
        )
        cursor += 16 + len(label) * 5.0
    if panel == 1:
        parts.append(
            f'<line x1="{cursor:.1f}" y1="{y - 3:.1f}" x2="{cursor + 10:.1f}" '
            f'y2="{y - 3:.1f}" stroke="{MUTED}" stroke-dasharray="3 2" '
            f'stroke-width="1"/>'
        )
        parts.append(
            f'<text x="{cursor + 14:.1f}" y="{y:.1f}" font-size="9" '
            f'fill="{MUTED}">LLVM -O2</text>'
        )
    return "".join(parts)


def _subset_title(point: Point) -> str:
    changes = []
    if point.subset_added:
        changes.append("added " + ", ".join(point.subset_added))
    if point.subset_removed:
        changes.append("removed " + ", ".join(point.subset_removed))
    detail = "; ".join(changes) if changes else "program list changed"
    return (
        f"Quick program subset changed at {point.short_sha} "
        f"({len(point.programs)} programs): {detail}"
    )


def render_svg(data: dict[str, Any], width: int = SVG_WIDTH, height: int = SVG_HEIGHT) -> str:
    """Render the static, light-theme 800x420 README image."""
    points = [
        Point(
            sha=item["sha"],
            subject=item["subject"],
            date=parse_datetime(item["date"]) or datetime.min.replace(tzinfo=timezone.utc),
            run_id=item["run_id"],
            runner=item["runner"],
            programs=list(item["programs"]),
            metrics=dict(item["metrics"]),
            exec_programs=int(item["exec_programs"]),
            size_programs=int(item["size_programs"]),
            subset_changed=bool(item["subset_changed"]),
            subset_added=list(item["subset_added"]),
            subset_removed=list(item["subset_removed"]),
        )
        for item in data["points"]
    ]
    count = len(points)
    margin = 24
    gap = 28
    panel_width = (width - 2 * margin - gap) / 2
    panel_top = 56
    panel_height = 322
    plot_top = 96
    plot_bottom = 300
    label_step = max(1, math.ceil(count / 8)) if count else 1
    label_indices = set(range(0, count, label_step)) if count else set()
    if count:
        label_indices.add(count - 1)

    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" '
        f'height="{height}" viewBox="0 0 {width} {height}" role="img" '
        f'aria-labelledby="chart-title chart-desc">',
        "<title id=\"chart-title\">CogWasm benchmark trend by commit</title>",
        '<desc id="chart-desc">Execution time and module size against LLVM -O2, '
        "and compile speed, one point per CI commit on main. Lower is better.</desc>",
        f'<rect width="{width}" height="{height}" fill="#ffffff"/>',
        f'<g font-family="{FONT}" fill="{INK}">',
        f'<text x="{margin}" y="24" font-size="16" font-weight="650">'
        "CogWasm benchmark trend</text>",
        f'<text x="{margin}" y="42" font-size="10" fill="{MUTED}">'
        "Each point is one CI commit on main &#183; lower is better</text>",
    ]
    if any(point.subset_changed for point in points):
        out.append(
            f'<text x="{width - margin}" y="42" font-size="9" fill="{ACCENT}" '
            'text-anchor="end">&#9615; quick program subset changed</text>'
        )

    for panel in PANELS:
        panel_id = int(panel["id"])
        panel_x = margin if panel_id == 1 else margin + panel_width + gap
        plot_left = panel_x + 52
        plot_right = panel_x + panel_width - 14
        xs = _x_positions(count, plot_left, plot_right)
        high = _panel_high(points, panel_id)
        top, ticks = _nice_ticks(high)
        def y_for(value: float, top: float = top) -> float:
            return plot_bottom - (value / top) * (plot_bottom - plot_top)
        out.append(
            f'<rect x="{panel_x:.1f}" y="{panel_top}" width="{panel_width:.1f}" '
            f'height="{panel_height}" rx="8" fill="#ffffff" stroke="{BORDER}"/>'
        )
        out.append(
            f'<text x="{panel_x + 12:.1f}" y="76" font-size="11.5" '
            f'font-weight="600">{html.escape(str(panel["title"]))}</text>'
        )
        for tick in ticks:
            y = y_for(tick)
            out.append(
                f'<line x1="{plot_left:.1f}" y1="{y:.1f}" x2="{plot_right:.1f}" '
                f'y2="{y:.1f}" stroke="{GRID}" stroke-width="1"/>'
            )
            label = f"{tick:g}"
            out.append(
                f'<text x="{plot_left - 8:.1f}" y="{y + 3:.1f}" font-size="9" '
                f'fill="{MUTED}" text-anchor="end">{label}</text>'
            )
        out.append(
            f'<line x1="{plot_left:.1f}" y1="{plot_bottom:.1f}" '
            f'x2="{plot_right:.1f}" y2="{plot_bottom:.1f}" stroke="{BORDER}" '
            'stroke-width="1"/>'
        )
        if count:
            for index, point in enumerate(points):
                if index not in label_indices:
                    continue
                x = xs[index]
                out.append(
                    f'<text x="{x:.1f}" y="{plot_bottom + 14:.1f}" font-size="8" '
                    f'fill="{MUTED}" text-anchor="end" '
                    f'transform="rotate(-40 {x:.1f} {plot_bottom + 14:.1f})">'
                    f"{html.escape(point.short_sha)}</text>"
                )
        for point_index, point in enumerate(points):
            if not point.subset_changed:
                continue
            x = xs[point_index]
            out.append(f"<g><title>{html.escape(_subset_title(point))}</title>")
            out.append(
                f'<line x1="{x:.1f}" y1="{plot_top:.1f}" x2="{x:.1f}" '
                f'y2="{plot_bottom:.1f}" stroke="{ACCENT}" stroke-width="1" '
                'stroke-dasharray="2 3" opacity="0.75"/>'
            )
            out.append(
                f'<path d="M {x - 3:.1f} {plot_top - 4:.1f} L {x + 3:.1f} '
                f'{plot_top - 4:.1f} L {x:.1f} {plot_top + 2:.1f} Z" '
                f'fill="{ACCENT}"/>'
            )
            out.append("</g>")
        if panel_id == 1:
            y = y_for(REFERENCE["value"])
            out.append(
                f'<line x1="{plot_left:.1f}" y1="{y:.1f}" x2="{plot_right:.1f}" '
                f'y2="{y:.1f}" stroke="{MUTED}" stroke-width="1" '
                'stroke-dasharray="4 3"/>'
            )
            out.append(
                f'<text x="{plot_right - 2:.1f}" y="{y - 4:.1f}" font-size="9" '
                f'fill="{MUTED}" text-anchor="end">LLVM -O2</text>'
            )
        for series in SERIES:
            if series["panel"] != panel_id:
                continue
            key = str(series["key"])
            if not _series_has_data(points, key):
                continue
            for segment in _segments(points, key, xs, y_for):
                coordinates = " ".join(f"{x:.1f},{y:.1f}" for x, y in segment)
                out.append(
                    f'<polyline points="{coordinates}" fill="none" '
                    f'stroke="{series["color"]}" stroke-width="1.8" '
                    'stroke-linejoin="round" stroke-linecap="round"/>'
                )
            for point, x in zip(points, xs):
                value = point.metrics.get(key)
                if value is None:
                    continue
                out.append(
                    _marker(
                        str(series["marker"]),
                        x,
                        y_for(value),
                        str(series["color"]),
                    )
                )
        out.append(_legend(panel_x + 12, 364, panel_id, points))
    out.extend(["</g>", "</svg>"])
    return "\n".join(out) + "\n"


def write_outputs(data: dict[str, Any], output: Path) -> tuple[Path, Path]:
    """Write chart.svg and data.json, returning their paths."""
    output.mkdir(parents=True, exist_ok=True)
    svg_path = output / "chart.svg"
    data_path = output / "data.json"
    svg_path.write_text(render_svg(data), encoding="utf-8")
    data_path.write_text(
        json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    return svg_path, data_path


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--history",
        type=Path,
        required=True,
        help="directory of bench-history point JSONs",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(__file__).resolve().parent,
        help="directory for chart.svg and data.json",
    )
    parser.add_argument(
        "--repo",
        default=os.environ.get("GITHUB_REPOSITORY", DEFAULT_REPO),
        help="owner/name used for commit links",
    )
    parser.add_argument("--page-url", default=DEFAULT_PAGE_URL)
    args = parser.parse_args(argv)
    try:
        points = load_points(args.history)
        data = build_data(points, repo=args.repo, page_url=args.page_url)
        svg_path, data_path = write_outputs(data, args.output)
    except ChartError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print(
        f"wrote {svg_path} and {data_path} "
        f"for {len(points)} commit(s)",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
