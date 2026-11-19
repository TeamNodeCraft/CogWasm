#!/usr/bin/env python3
"""Check every local link, asset and anchor in the built CogWasm site.

The checker is deliberately small and uses only the standard library. It reads
the assembled output directory (not the sources), so it catches a missing copy
of the artwork or a chart asset as well as a typo in an anchor.

It checks:

* every local ``href`` and ``src`` resolves to a file in the output;
* every ``#fragment`` resolves to an ``id`` (or ``<a name>``) on the page it
  points at;
* no duplicate ids, no inline event handlers, and an ``alt`` attribute on every
  image;
* the basic document metadata each page needs (lang, title, description,
  viewport).

With ``--check-external`` it also requests each external URL (the default keeps
the check offline and only validates the URLs' shape).

Usage:

    python3 site/tests/link_check.py --root /tmp/cogwasm-site
    python3 site/tests/link_check.py --root public --check-external
"""

from __future__ import annotations

import argparse
import sys
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass, field
from html.parser import HTMLParser
from pathlib import Path

EXTERNAL_SCHEMES = {"http", "https"}
SKIPPED_SCHEMES = {"mailto", "tel"}
TOLERATED_EXTERNAL_STATUSES = {403, 405, 429}
USER_AGENT = "CogWasm-site-link-check/1.0 (+https://github.com/TeamNodeCraft/CogWasm)"


@dataclass
class Page:
    """What the checker needs to know about one HTML file."""

    path: Path
    lang: str = ""
    title: str = ""
    description: str = ""
    viewport: str = ""
    ids: set[str] = field(default_factory=set)
    names: set[str] = field(default_factory=set)
    duplicates: list[str] = field(default_factory=list)
    references: list[tuple[str, str]] = field(default_factory=list)
    event_handlers: list[str] = field(default_factory=list)
    images_without_alt: list[str] = field(default_factory=list)

    def anchor_ids(self) -> set[str]:
        """Every fragment target on the page."""
        return self.ids | self.names


class PageParser(HTMLParser):
    """Collect ids, anchors, local references and metadata from one page."""

    def __init__(self, path: Path) -> None:
        super().__init__(convert_charrefs=True)
        self.page = Page(path=path)
        self._in_title = False

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = {name.lower(): value for name, value in attrs}
        identifier = values.get("id")
        if identifier:
            if identifier in self.page.ids:
                self.page.duplicates.append(f"duplicate id {identifier!r}")
            self.page.ids.add(identifier)
        if tag == "a" and values.get("name"):
            self.page.names.add(values["name"] or "")
        if tag == "html":
            self.page.lang = (values.get("lang") or "").strip()
        if tag == "title":
            self._in_title = True
        if tag == "meta":
            name = (values.get("name") or "").lower()
            content = values.get("content") or ""
            if name == "description":
                self.page.description = content.strip()
            elif name == "viewport":
                self.page.viewport = content.strip()

        for name, value in values.items():
            if name.startswith("on") and value is not None:
                self.page.event_handlers.append(f"<{tag} {name}=...>")
        if tag == "img" and "alt" not in values:
            self.page.images_without_alt.append(values.get("src") or "(no src)")

        if tag in {"a", "link"} and values.get("href") is not None:
            self.page.references.append(("href", values["href"] or ""))
        elif tag in {"img", "script", "source", "iframe"} and values.get("src") is not None:
            self.page.references.append(("src", values["src"] or ""))

    def handle_endtag(self, tag: str) -> None:
        if tag == "title":
            self._in_title = False

    def handle_data(self, data: str) -> None:
        if self._in_title:
            self.page.title += data


def parse_page(path: Path) -> Page:
    parser = PageParser(path)
    parser.feed(path.read_text(encoding="utf-8"))
    parser.close()
    return parser.page


def resolve_local(root: Path, source: Path, path_text: str) -> tuple[Path | None, str]:
    """Resolve a local path against the page, or return an error message."""
    decoded = urllib.parse.unquote(path_text)
    if decoded.startswith("/"):
        # The site is served from /CogWasm/ on GitHub Pages, so a root-absolute
        # path would leave the project prefix and break.
        return None, "root-absolute path (use a relative path)"
    try:
        target = (source.parent / decoded).resolve()
    except (OSError, RuntimeError) as error:
        return None, f"cannot resolve path: {error}"
    try:
        target.relative_to(root)
    except ValueError:
        return None, "path escapes the site root"
    if decoded.endswith("/") or target.is_dir():
        target = target / "index.html"
    return target, ""


@dataclass
class Result:
    """Everything found during one run."""

    local_references: int = 0
    anchors: int = 0
    external_urls: set[str] = field(default_factory=set)
    errors: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)

    def error(self, message: str) -> None:
        self.errors.append(message)

    def warn(self, message: str) -> None:
        self.warnings.append(message)


def check_pages(root: Path, result: Result) -> dict[Path, Page]:
    """Read every page and return them keyed by resolved path."""
    html_files = sorted(path for path in root.rglob("*.html") if path.is_file())
    if not html_files:
        result.error(f"no HTML files found under {root}")
        return {}

    pages: dict[Path, Page] = {}
    for path in html_files:
        page = parse_page(path)
        pages[path.resolve()] = page
        shown = path.relative_to(root)

        for heading, value in (
            ("html lang", page.lang),
            ("title", page.title),
            ("meta description", page.description),
            ("meta viewport", page.viewport),
        ):
            if not value:
                result.error(f"{shown}: missing {heading}")
        result.errors.extend(f"{shown}: {problem}" for problem in page.duplicates)
        result.errors.extend(
            f"{shown}: inline event handler {handler}" for handler in page.event_handlers
        )
        result.errors.extend(
            f"{shown}: image without alt: {image}" for image in page.images_without_alt
        )
    return pages


def check_references(pages: dict[Path, Page], root: Path, result: Result) -> None:
    """Check every reference on every page."""
    for source, page in sorted(pages.items()):
        shown = source.relative_to(root)
        for kind, raw in page.references:
            url = raw.strip()
            if not url:
                result.error(f"{shown}: empty {kind}")
                continue

            parsed = urllib.parse.urlsplit(url)
            if parsed.scheme in EXTERNAL_SCHEMES:
                if not parsed.netloc:
                    result.error(f"{shown}: external URL without a host: {url}")
                elif parsed.scheme == "http":
                    result.error(f"{shown}: external URL is not HTTPS: {url}")
                else:
                    result.external_urls.add(url)
                continue
            if parsed.scheme in SKIPPED_SCHEMES:
                continue
            if parsed.scheme:
                result.error(f"{shown}: unsupported URL scheme {parsed.scheme!r}: {url}")
                continue
            if url.startswith("//"):
                result.error(f"{shown}: protocol-relative URL {url}; write the scheme explicitly")
                continue

            target, problem = resolve_local(root, source, parsed.path)
            if target is None:
                result.error(f"{shown}: {problem}: {url}")
                continue

            result.local_references += 1
            if not target.is_file():
                result.error(f"{shown}: missing file for {url}: {target.relative_to(root)}")
                continue
            if not parsed.fragment:
                continue

            result.anchors += 1
            fragment = urllib.parse.unquote(parsed.fragment)
            if target.suffix.lower() not in {".html", ".htm"}:
                result.error(f"{shown}: fragment {parsed.fragment!r} on a non-HTML file: {url}")
                continue
            target_page = pages.get(target)
            if target_page is None:
                result.error(f"{shown}: anchor target page was not checked: {url}")
            elif fragment not in target_page.anchor_ids():
                result.error(f"{shown}: no id {fragment!r} on {target.relative_to(root)}")


def check_external(urls: set[str], result: Result) -> None:
    """Request each external URL once and report real failures."""
    for url in sorted(urls):
        request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
        try:
            with urllib.request.urlopen(request, timeout=20) as response:
                status = response.status
        except urllib.error.HTTPError as error:
            status = error.code
        except urllib.error.URLError as error:
            result.error(f"external URL could not be fetched: {url} ({error.reason})")
            continue

        if 200 <= status < 400:
            print(f"  {status} {url}")
        elif status in TOLERATED_EXTERNAL_STATUSES:
            result.warn(f"external URL returned {status} (rate limited or refusing HEAD): {url}")
        else:
            result.error(f"external URL returned {status}: {url}")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        required=True,
        type=Path,
        help="assembled site directory to check",
    )
    parser.add_argument(
        "--check-external",
        action="store_true",
        help="also request every external URL over the network",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv if argv is not None else sys.argv[1:])
    root = args.root.resolve()
    if not root.is_dir():
        print(f"FAIL: {root} is not a directory", file=sys.stderr)
        return 2

    result = Result()
    pages = check_pages(root, result)
    if pages:
        check_references(pages, root, result)
    if args.check_external and result.external_urls:
        print(f"Requesting {len(result.external_urls)} external URL(s)")
        check_external(result.external_urls, result)

    print(
        f"Checked {len(pages)} page(s), {result.local_references} local reference(s), "
        f"{result.anchors} anchor(s), {len(result.external_urls)} external URL(s)"
    )
    for warning in result.warnings:
        print(f"warning: {warning}")
    if result.errors:
        print("FAIL")
        for error in result.errors:
            print(f"  {error}")
        return 1
    print("PASS: every local link and anchor resolves")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
