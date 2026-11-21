#!/usr/bin/env python3
"""Strict checks for the upstream manifest, PR bundles and Cargo Wasmtime pins.

This is the checking entry point that may change without invalidating the CI
compiler cache. compiler/scripts/upstream.py stays byte-identical to the
version the cache key hashes; this script imports it for manifest parsing and
the original `check`, then adds:

  * the root Cargo.toml Wasmtime pins must equal upstream.wasmtime.version;
  * a compiler/upstream/ bundle .patch that differs from the live patch is an
    error unless the bundle directory has a DIVERGES.md.

Exit status is nonzero on any error. The last line is a one-line summary.
"""

from __future__ import annotations

import contextlib
import importlib.util
import io
import json
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:  # Python < 3.11; use the fallback parser below.
    tomllib = None

HERE = Path(__file__).resolve().parent
COMPILER_DIR = HERE.parent
REPO = COMPILER_DIR.parent
UPSTREAM_SCRIPT = HERE / "upstream.py"
CARGO_TOML = REPO / "Cargo.toml"
WASMTIME_CRATES = ("wasmtime", "wasmtime-wasi", "wasmtime-environ")


def load_upstream_module():
    """Import compiler/scripts/upstream.py by path."""
    spec = importlib.util.spec_from_file_location("upstream_manifest", UPSTREAM_SCRIPT)
    if spec is None or spec.loader is None:
        raise SystemExit(f"error: cannot load {UPSTREAM_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _parse_scalar(upstream, text: str):
    """Parse one TOML value, including inline tables upstream.py does not."""
    text = text.strip()
    if text.startswith('"') and text.endswith('"'):
        return json.loads(text)
    if text.startswith("'") and text.endswith("'"):
        return text[1:-1]
    if text.startswith("[") and text.endswith("]"):
        return [
            _parse_scalar(upstream, item)
            for item in upstream._split_top_level(text[1:-1])
        ]
    if text.startswith("{") and text.endswith("}"):
        table = {}
        for item in upstream._split_top_level(text[1:-1]):
            key, separator, value = item.partition("=")
            if not separator:
                raise ValueError(f"unsupported TOML inline table entry: {item}")
            table[key.strip()] = _parse_scalar(upstream, value)
        return table
    if text in ("true", "false"):
        return text == "true"
    try:
        return int(text, 0)
    except ValueError:
        try:
            return float(text)
        except ValueError:
            raise ValueError(f"unsupported TOML value: {text}") from None


def _parse_simple_toml(upstream, path: Path) -> dict:
    """Minimal TOML parser used when tomllib is unavailable.

    This is the subset the manifest and the root Cargo.toml use: comments,
    [tables], strings, numbers, booleans, single- or multi-line arrays and
    inline tables.
    """
    data: dict = {}
    current = data
    pending_key = None
    pending_text = ""
    pending_table = None

    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = upstream._strip_comment(raw)
        if pending_key is not None:
            pending_text += " " + line
            if not upstream._balanced(pending_text):
                continue
            pending_table[pending_key] = _parse_scalar(upstream, pending_text)
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
        if value.startswith("[") and not upstream._balanced(value):
            pending_key = key
            pending_text = value
            pending_table = current
            continue
        current[key] = _parse_scalar(upstream, value)

    if pending_key is not None:
        raise ValueError(f"{path}: unterminated array for {pending_key}")
    return data


def load_cargo_toml(upstream) -> dict:
    if tomllib is not None:
        with open(CARGO_TOML, "rb") as handle:
            return tomllib.load(handle)
    return _parse_simple_toml(upstream, CARGO_TOML)


def check_wasmtime_pins(upstream, errors: list[str]) -> None:
    """Fail when Cargo.toml's workspace Wasmtime pins differ from the manifest."""
    entries = upstream.upstreams()
    wasmtime = entries.get("wasmtime", {})
    expected = wasmtime.get("version") if isinstance(wasmtime, dict) else None
    if not isinstance(expected, str):
        errors.append("[upstream.wasmtime] has no version to compare with Cargo.toml")
        return

    try:
        data = load_cargo_toml(upstream)
    except (OSError, TypeError, ValueError) as error:
        errors.append(f"cannot read {CARGO_TOML}: {error}")
        return

    workspace = data.get("workspace")
    dependencies = workspace.get("dependencies") if isinstance(workspace, dict) else None
    if not isinstance(dependencies, dict):
        errors.append(f"{CARGO_TOML} has no [workspace.dependencies] table")
        return

    for crate in WASMTIME_CRATES:
        dependency = dependencies.get(crate)
        if isinstance(dependency, dict):
            pin = dependency.get("version")
        else:
            pin = dependency
        if not isinstance(pin, str):
            errors.append(
                f"Cargo.toml [workspace.dependencies] {crate} has no version pin"
            )
            continue
        # An exact `=<version>` pin and a bare version are the same pin; any
        # other operator is a range and is not the manifest's exact version.
        if pin.removeprefix("=") != expected:
            errors.append(
                f"Cargo.toml [workspace.dependencies] {crate} = \"{pin}\" does "
                f"not match upstream.wasmtime.version = \"{expected}\""
            )


def check_bundles(upstream, errors: list[str]) -> tuple[int, int]:
    """Enforce DIVERGES.md for every bundle that differs from the live patch."""
    checked = 0
    explained = 0
    bundle_root = upstream.BUNDLE_ROOT
    patch_root = upstream.PATCH_ROOT
    for bundle in sorted(bundle_root.glob("*/*/*.patch")):
        if not bundle.is_file():
            continue
        parts = bundle.relative_to(bundle_root).parts
        if len(parts) != 3:
            continue
        checked += 1
        name, _, filename = parts
        live = patch_root / name / filename
        if not live.is_file():
            continue
        if bundle.read_bytes() == live.read_bytes():
            continue
        if (bundle.parent / "DIVERGES.md").is_file():
            explained += 1
            continue
        errors.append(
            f"{bundle.relative_to(REPO)} is not byte-identical to "
            f"compiler/patches/{name}/{filename} and has no DIVERGES.md"
        )
    return checked, explained


def main() -> int:
    upstream = load_upstream_module()
    errors: list[str] = []

    # (a) The original checks: patch lists, apply order, bundle presence.
    captured = io.StringIO()
    with contextlib.redirect_stdout(captured), contextlib.redirect_stderr(captured):
        status = upstream.cmd_check([])
    if status != 0:
        sys.stderr.write(captured.getvalue())
        errors.append("upstream.py check failed (patch list or bundle validation)")

    # (b) Root Cargo.toml Wasmtime pins must mirror the manifest.
    check_wasmtime_pins(upstream, errors)

    # (c) A differing bundle needs a DIVERGES.md explaining why.
    bundles, explained = check_bundles(upstream, errors)

    for error in errors:
        print(f"error: {error}", file=sys.stderr)
    if errors:
        print(f"check-upstreams: FAILED ({len(errors)} error(s))", file=sys.stderr)
        return 1
    print(
        f"check-upstreams: OK ({bundles} bundle(s) checked, "
        f"{explained} explained divergence(s))"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
