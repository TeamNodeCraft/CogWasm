#!/usr/bin/env python3
"""Read compiler/upstreams.toml: the machine-readable upstream/patch manifest.

Subcommands:
    get <upstream> <key>   print one value (an array prints one item per line)
    patches <upstream>     print the ordered absolute patch paths, one per line
    check                  validate the patch lists and the PR bundles

`check` fails when a listed patch is missing, when a compiler/patches/<name>/
.patch file is not listed, or when the list is not in the sorted order the old
`*.patch` glob used. It only warns when a compiler/upstream/ PR bundle differs
byte-for-byte from the live patch of the same name: the bundles are refreshed
against upstream and are not expected to stay identical.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:  # Python < 3.11; use the parser below.
    tomllib = None

COMPILER_DIR = Path(__file__).resolve().parent.parent
MANIFEST = COMPILER_DIR / "upstreams.toml"
PATCH_ROOT = COMPILER_DIR / "patches"
BUNDLE_ROOT = COMPILER_DIR / "upstream"

USAGE = """usage: upstream.py <command> [args]

commands:
  get <upstream> <key>   print a manifest value
  patches <upstream>     print absolute patch paths, one per line
  check                  validate patch lists and PR bundles
"""


def _strip_comment(text: str) -> str:
    """Drop a TOML comment outside a quoted string."""
    out = []
    quote = None
    escaped = False
    for char in text:
        if quote is not None:
            out.append(char)
            if escaped:
                escaped = False
            elif char == "\\" and quote == '"':
                escaped = True
            elif char == quote:
                quote = None
        elif char in ('"', "'"):
            quote = char
            out.append(char)
        elif char == "#":
            break
        else:
            out.append(char)
    return "".join(out).strip()


def _balanced(text: str) -> bool:
    """True when every array/table bracket and quote in text is closed."""
    depth = 0
    quote = None
    escaped = False
    for char in text:
        if quote is not None:
            if escaped:
                escaped = False
            elif char == "\\" and quote == '"':
                escaped = True
            elif char == quote:
                quote = None
        elif char in ('"', "'"):
            quote = char
        elif char in "[{":
            depth += 1
        elif char in "]}":
            depth -= 1
    return depth == 0 and quote is None


def _split_top_level(text: str) -> list[str]:
    """Split an array body on commas that are not inside a string or nesting."""
    items = []
    buf = []
    quote = None
    escaped = False
    depth = 0
    for char in text:
        if quote is not None:
            buf.append(char)
            if escaped:
                escaped = False
            elif char == "\\" and quote == '"':
                escaped = True
            elif char == quote:
                quote = None
            continue
        if char in ('"', "'"):
            quote = char
            buf.append(char)
        elif char in "[{":
            depth += 1
            buf.append(char)
        elif char in "]}":
            depth -= 1
            buf.append(char)
        elif char == "," and depth == 0:
            item = "".join(buf).strip()
            if item:
                items.append(item)
            buf = []
        else:
            buf.append(char)
    item = "".join(buf).strip()
    if item:
        items.append(item)
    return items


def _parse_scalar(text: str):
    text = text.strip()
    if text.startswith('"') and text.endswith('"'):
        return json.loads(text)
    if text.startswith("'") and text.endswith("'"):
        return text[1:-1]
    if text.startswith("[") and text.endswith("]"):
        return [_parse_scalar(item) for item in _split_top_level(text[1:-1])]
    if text in ("true", "false"):
        return text == "true"
    try:
        return int(text, 0)
    except ValueError:
        try:
            return float(text)
        except ValueError:
            raise ValueError(f"unsupported TOML value: {text}") from None


def _parse_simple_toml(path: Path) -> dict:
    """Minimal parser for the subset of TOML this manifest uses.

    Python 3.11+ uses tomllib; this fallback keeps the script usable on older
    system pythons. It supports comments, [table] headers, strings, integers,
    booleans and single- or multi-line arrays.
    """
    data: dict = {}
    current = data
    pending_key = None
    pending_text = ""
    pending_table = None

    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = _strip_comment(raw)
        if pending_key is not None:
            pending_text += " " + line
            if not _balanced(pending_text):
                continue
            pending_table[pending_key] = _parse_scalar(pending_text)
            pending_key = None
            pending_text = ""
            pending_table = None
            continue
        if not line:
            continue
        if line.startswith("["):
            if not line.endswith("]"):
                raise ValueError(f"{path}:{lineno}: unterminated table header")
            current = data
            for part in line[1:-1].strip().split("."):
                current = current.setdefault(part, {})
            continue
        if "=" not in line:
            raise ValueError(f"{path}:{lineno}: expected key = value")
        key, _, value = line.partition("=")
        key = key.strip()
        value = value.strip()
        if value.startswith("[") and not _balanced(value):
            pending_key = key
            pending_text = value
            pending_table = current
            continue
        current[key] = _parse_scalar(value)

    if pending_key is not None:
        raise ValueError(f"{path}: unterminated array for {pending_key}")
    return data


def load_manifest() -> dict:
    if tomllib is not None:
        with open(MANIFEST, "rb") as handle:
            return tomllib.load(handle)
    return _parse_simple_toml(MANIFEST)


def upstreams() -> dict:
    manifest = load_manifest()
    table = manifest.get("upstream")
    if not isinstance(table, dict):
        raise TypeError(f"{MANIFEST}: missing [upstream] tables")
    return table


def upstream(name: str) -> dict:
    table = upstreams()
    if name not in table:
        known = ", ".join(sorted(table))
        raise ValueError(f"unknown upstream '{name}' (known: {known})")
    entry = table[name]
    if not isinstance(entry, dict):
        raise TypeError(f"[upstream.{name}] is not a table")
    return entry


def cmd_get(args: list[str]) -> int:
    if len(args) != 2:
        print("usage: upstream.py get <upstream> <key>", file=sys.stderr)
        return 2
    name, key = args
    entry = upstream(name)
    if key not in entry:
        known = ", ".join(sorted(entry))
        raise ValueError(f"[upstream.{name}] has no '{key}' (keys: {known})")
    value = entry[key]
    if isinstance(value, list):
        for item in value:
            print(item)
    elif isinstance(value, bool):
        print("true" if value else "false")
    else:
        print(value)
    return 0


def cmd_patches(args: list[str]) -> int:
    if len(args) != 1:
        print("usage: upstream.py patches <upstream>", file=sys.stderr)
        return 2
    name = args[0]
    entry = upstream(name)
    patches = entry.get("patches", [])
    if not isinstance(patches, list):
        raise TypeError(f"[upstream.{name}] patches must be an array")
    for patch in patches:
        print(PATCH_ROOT / name / str(patch))
    return 0


def cmd_check(args: list[str]) -> int:
    if args:
        print("usage: upstream.py check", file=sys.stderr)
        return 2
    entries = upstreams()
    errors: list[str] = []
    warnings: list[str] = []
    total = 0

    if PATCH_ROOT.is_dir():
        for directory in sorted(PATCH_ROOT.iterdir()):
            if not directory.is_dir():
                continue
            actual = sorted(path.name for path in directory.glob("*.patch") if path.is_file())
            if actual and directory.name not in entries:
                errors.append(
                    f"compiler/patches/{directory.name}/ has {len(actual)} .patch "
                    f"file(s) but no [upstream.{directory.name}] table"
                )

    for name in sorted(entries):
        entry = entries[name]
        if not isinstance(entry, dict):
            errors.append(f"[upstream.{name}] is not a table")
            continue
        if not entry.get("repo"):
            errors.append(f"[upstream.{name}] has no repo")
        if "rev" not in entry and "version" not in entry:
            errors.append(f"[upstream.{name}] has neither rev nor version")

        patches = entry.get("patches", [])
        if not isinstance(patches, list):
            errors.append(f"[upstream.{name}] patches must be an array")
            continue
        listed = [str(item) for item in patches]
        total += len(listed)
        if listed != sorted(listed):
            errors.append(f"[upstream.{name}] patches are not in sorted (apply) order")
        if len(set(listed)) != len(listed):
            errors.append(f"[upstream.{name}] lists a patch more than once")

        directory = PATCH_ROOT / name
        actual = sorted(path.name for path in directory.glob("*.patch") if path.is_file())
        for patch in listed:
            if not (directory / patch).is_file():
                errors.append(
                    f"[upstream.{name}] lists {patch}, but "
                    f"compiler/patches/{name}/{patch} is missing"
                )
        for patch in actual:
            if patch not in listed:
                errors.append(
                    f"compiler/patches/{name}/{patch} is not listed in "
                    f"[upstream.{name}] patches"
                )

    if BUNDLE_ROOT.is_dir():
        for bundle in sorted(BUNDLE_ROOT.glob("*/*/*.patch")):
            if not bundle.is_file():
                continue
            parts = bundle.relative_to(BUNDLE_ROOT).parts
            if len(parts) != 3:
                continue
            name, _, filename = parts
            live = PATCH_ROOT / name / filename
            if live.is_file() and bundle.read_bytes() != live.read_bytes():
                warnings.append(
                    f"{bundle.relative_to(COMPILER_DIR.parent)} is not byte-identical to "
                    f"compiler/patches/{name}/{filename}"
                )

    print(f"upstream manifest: {len(entries)} upstreams, {total} listed patches")
    for warning in warnings:
        print(f"warning: {warning}", file=sys.stderr)
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        print(
            f"upstream manifest check FAILED "
            f"({len(errors)} error(s), {len(warnings)} warning(s))",
            file=sys.stderr,
        )
        return 1
    print(f"upstream manifest check OK ({len(warnings)} warning(s))")
    return 0


def main(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        print(USAGE, end="")
        return 0 if argv else 2
    command, args = argv[0], argv[1:]
    try:
        if command == "get":
            return cmd_get(args)
        if command == "patches":
            return cmd_patches(args)
        if command == "check":
            return cmd_check(args)
    except (OSError, TypeError, ValueError) as error:
        print(f"upstream.py: {error}", file=sys.stderr)
        return 1
    print(f"upstream.py: unknown command '{command}'", file=sys.stderr)
    print(USAGE, end="", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
