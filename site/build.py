#!/usr/bin/env python3
"""Assemble the CogWasm site into an output directory.

The site itself is static. This step does the two things that should not be
committed as copies:

* it fills in the commit and date in the Status section, so the page can say
  "as of <sha>, <date>" without a script running in the browser; and
* it copies the logo files from ``artwork/cog-logo/`` under the names the page
  uses, so the artwork stays single-sourced in git.

With ``--include-bench`` it also copies the committed chart page and, when
they exist locally, the generated ``chart.svg`` and ``data.json``. The Pages
workflow does that part itself because those two files are release assets.

Usage:

    python3 site/build.py --output public
    python3 site/build.py --output /tmp/cogwasm-site --include-bench

Requires only the Python standard library.
"""

from __future__ import annotations

import argparse
import datetime as dt
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
SITE_DIR = Path(__file__).resolve().parent
ARTWORK_DIR = REPO_ROOT / "artwork" / "cog-logo"
BENCH_DIR = REPO_ROOT / "bench" / "chart"

# The source page keeps visible placeholders so it is obvious when a build has
# not run. They are replaced here; any placeholder left over is an error.
COMMIT_PLACEHOLDER = "{{COMMIT_SHA7}}"
DATE_PLACEHOLDER = "{{COMMIT_DATE}}"

# Site file -> artwork file. The artwork is copied, never duplicated in git.
ARTWORK_COPIES = {
    "logo.svg": "logo-colour.svg",
    "icon.svg": "logo-colour-dark-wa-inset.svg",
    "favicon.svg": "logo-colour-dark-wa-inset.svg",
}

# Committed files copied unchanged.
SITE_FILES = ("style.css", "script.js")
BENCH_ASSETS = ("chart.svg", "data.json")


class BuildError(RuntimeError):
    """A build input is missing or malformed."""


def git(*arguments: str) -> str:
    """Run git in the repository and return its stripped standard output."""
    result = subprocess.run(
        ["git", "-C", str(REPO_ROOT), *arguments],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def commit_short(explicit: str | None) -> str:
    """Return the seven-character commit to show on the page."""
    value = explicit if explicit is not None else git("rev-parse", "--short=7", "HEAD")
    if not re.fullmatch(r"[0-9a-f]{7,40}", value):
        raise BuildError(
            f"commit {value!r} is not a lowercase hexadecimal git object name"
        )
    return value[:7]


def commit_date(explicit: str | None) -> str:
    """Return the ISO date (YYYY-MM-DD) to show on the page."""
    value = explicit if explicit is not None else git("show", "-s", "--format=%cs", "HEAD")
    try:
        dt.date.fromisoformat(value)
    except ValueError as error:
        raise BuildError(f"date {value!r} is not YYYY-MM-DD") from error
    return value


def copy_file(source: Path, destination: Path) -> None:
    """Copy one file, creating its parent directory."""
    if not source.is_file():
        raise BuildError(f"missing build input: {source}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, destination)


def build_page(output: Path, commit: str, date: str) -> None:
    """Write index.html with the build-time facts filled in."""
    source = SITE_DIR / "index.html"
    text = source.read_text(encoding="utf-8")
    text = text.replace(COMMIT_PLACEHOLDER, commit).replace(DATE_PLACEHOLDER, date)

    leftover = sorted(set(re.findall(r"\{\{[A-Z_]+\}\}", text)))
    if leftover:
        raise BuildError(
            f"{source} still contains unexpanded placeholders: {', '.join(leftover)}"
        )

    (output / "index.html").write_text(text, encoding="utf-8")


def build_artwork(output: Path) -> None:
    """Copy the artwork under the site's stable file names."""
    for name, artwork_name in ARTWORK_COPIES.items():
        copy_file(ARTWORK_DIR / artwork_name, output / name)


def build_site(output: Path, commit: str, date: str, include_bench: bool) -> None:
    """Assemble the whole site into an existing or new output directory."""
    output.mkdir(parents=True, exist_ok=True)

    build_page(output, commit, date)
    for name in SITE_FILES:
        copy_file(SITE_DIR / name, output / name)
    build_artwork(output)

    if include_bench:
        build_bench(output)

    print(f"site built in {output}")
    print(f"  status: as of {commit}, {date}")
    if include_bench:
        print("  chart page: bench/index.html")


def build_bench(output: Path) -> None:
    """Copy the committed chart page and any locally generated chart assets."""
    copy_file(BENCH_DIR / "index.html", output / "bench" / "index.html")
    for name in BENCH_ASSETS:
        source = BENCH_DIR / name
        if source.is_file():
            copy_file(source, output / "bench" / name)
        else:
            print(
                f"warning: {source} is a release asset and is not present; "
                "the page will show its fallback text and the static chart will "
                "be missing. Download it with: gh release download bench-history "
                f"--pattern {name} --dir {BENCH_DIR}",
                file=sys.stderr,
            )


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output",
        required=True,
        type=Path,
        help="directory to write the assembled site into (created if needed)",
    )
    parser.add_argument(
        "--commit",
        help="commit to show, full or short (default: git HEAD)",
    )
    parser.add_argument(
        "--date",
        help="date to show, YYYY-MM-DD (default: git HEAD's commit date)",
    )
    parser.add_argument(
        "--include-bench",
        action="store_true",
        help="also copy bench/chart/index.html and any local chart assets",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv if argv is not None else sys.argv[1:])
    try:
        build_site(
            args.output.resolve(),
            commit_short(args.commit),
            commit_date(args.date),
            args.include_bench,
        )
    except (BuildError, subprocess.CalledProcessError) as error:
        print(f"build failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
