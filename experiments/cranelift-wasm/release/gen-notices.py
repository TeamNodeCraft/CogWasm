#!/usr/bin/env python3
"""Assemble THIRD_PARTY_NOTICES for a CogWasm compiler bundle.

The package list comes from `cargo metadata` for the host/tool crates that are
actually built, so the notices track the dependency graph instead of a
hand-maintained list. Licence and notice files are read from the cargo registry
source or the pinned upstream checkouts. Every package must have licence text:
the script fails with the package name rather than writing a placeholder.

The six components that are not ordinary crates.io dependencies (rustc itself,
Cranelift/Wasmtime, clif-to-wasm, the rust-playground-inbrowser patch series,
Wild and wasi-libc) have explicit sections with their licence texts.

Usage:
    release/gen-notices.py --out release/out/THIRD_PARTY_NOTICES
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

IMPORTANT_FILE = re.compile(r"(copyright|licen[cs]e|author|notice)", re.IGNORECASE)

# Manifests whose dependency graph ends up in a compiler bundle. Missing
# manifests are skipped with a warning, so the script also runs on a partial
# checkout; a present manifest that fails to resolve is fatal.
MANIFESTS = [
    "stage-c/host/Cargo.toml",
    "difftest/host/Cargo.toml",
    "stage-b/a2/harness/Cargo.toml",
]
PROGRAM_GLOB = "stage-c/programs/*/Cargo.toml"

SPDX_TEXT_DIRS = ["toolchain/stage-b/rust/LICENSES", "toolchain/playground/licenses/spdx"]

# The explicit components. `files` are relative to the experiment directory.
NAMED_COMPONENTS = [
    {
        "title": "Rust and rustc (including rustc_codegen_cranelift and the standard library)",
        "licence": "MIT OR Apache-2.0",
        "source": "https://github.com/rust-lang/rust",
        "files": [
            "toolchain/stage-b/rust/LICENSE-MIT",
            "toolchain/stage-b/rust/LICENSE-APACHE",
            "toolchain/stage-b/rust/COPYRIGHT",
        ],
        "note": (
            "rustc.wasm is a modified build of rustc: it uses the "
            "rust-playground-inbrowser rustc-to-wasm patch series plus patches "
            "from this repository, and rustc_codegen_cranelift as its only code "
            "generator. It is not the official Rust compiler."
        ),
    },
    {
        "title": "Cranelift and Wasmtime",
        "licence": "Apache-2.0 WITH LLVM-exception",
        "source": "https://github.com/bytecodealliance/wasmtime",
        # The registry source directory hash varies by host/cargo version, so
        # resolve this path from the lock file at run time.
        "registry_package": "wasmtime",
        "note": "The host links Wasmtime 50.0.2; rustc.wasm contains Cranelift codegen.",
    },
    {
        "title": "clif-to-wasm",
        "licence": "Apache-2.0 WITH LLVM-exception",
        "source": "https://github.com/ryancbuckley/clif-to-wasm",
        "files": ["toolchain/clif-to-wasm/LICENSE"],
        "note": "Lowers Cranelift IR to relocatable wasm objects inside rustc.wasm.",
    },
    {
        "title": "rust-playground-inbrowser rustc-to-wasm patch series",
        "licence": "Apache-2.0",
        "source": "https://github.com/ryancbuckley/rust-playground-inbrowser",
        "files": ["toolchain/playground/LICENSE"],
        "note": (
            "The 23-patch series that makes rustc run on wasm32-wasip1; adapted "
            "by this repository in stage-c/patches/rust/."
        ),
    },
    {
        "title": "Wild",
        "licence": "MIT OR Apache-2.0",
        "source": "https://github.com/wild-linker/wild",
        "files": ["toolchain/wild/LICENSE-MIT", "toolchain/wild/LICENSE-APACHE"],
        "note": "The pure-Rust linker the host runs for rustc.wasm.",
    },
    {
        "title": "wasi-libc",
        "licence": "Apache-2.0 WITH LLVM-exception, Apache-2.0, MIT and others",
        "source": "https://github.com/WebAssembly/wasi-libc",
        "files": sorted(
            str(path.relative_to(Path(__file__).resolve().parent.parent))
            for path in (
                Path(__file__).resolve().parent.parent / "toolchain/playground/licenses/wasi-libc"
            ).glob("*")
            if path.is_file()
        ),
        "note": (
            "wasi-libc's libc.a and crt1-*.o from wasi-sdk are linked into wasm "
            "builds of std. Its license overview is LICENSE; the remaining files "
            "are the licences of its own parts (musl, cloudlibc, musl-fts, "
            "dlmalloc) and its Apache/MIT texts."
        ),
    },
]


def run_metadata(cargo: str, root: Path, manifest: Path) -> dict:
    base = [
        cargo,
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        str(manifest),
        "--locked",
    ]
    # Prefer the local registry. If a dependency graph was never built on this
    # machine, retry once with the network so the notices still cover it;
    # crates.io is the only source allowed here.
    result = subprocess.run(
        [*base, "--offline"], cwd=root, capture_output=True, text=True, check=False
    )
    if result.returncode != 0:
        print(
            f"warning: offline cargo metadata failed for {manifest.relative_to(root)}; "
            "retrying with the network",
            file=sys.stderr,
        )
        result = subprocess.run(base, cwd=root, capture_output=True, text=True, check=False)
    if result.returncode != 0:
        raise SystemExit(
            f"error: cargo metadata failed for {manifest.relative_to(root)}:\n{result.stderr}"
        )
    return json.loads(result.stdout)


def source_label(package: dict, root: Path) -> str:
    source = package.get("source") or ""
    if source.startswith("registry+"):
        return "crates.io"
    if source.startswith("git+"):
        url, _, rev = source.partition("#")
        return f"{url.removeprefix('git+')} ({rev or 'unknown revision'})"
    manifest = Path(package["manifest_path"])
    try:
        return str(manifest.parent.relative_to(root))
    except ValueError:
        return str(manifest.parent)


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError as error:
        raise SystemExit(f"error: {path} is not UTF-8 ({error}); refusing to guess") from error


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def important_files(package_dir: Path, license_file: str | None) -> list[Path]:
    found: list[Path] = []
    if license_file:
        candidate = Path(license_file)
        if not candidate.is_absolute():
            candidate = package_dir / candidate
        if candidate.is_file():
            found.append(candidate)
    if package_dir.is_dir():
        for entry in sorted(package_dir.iterdir()):
            if entry.is_file() and IMPORTANT_FILE.search(entry.name):
                found.append(entry)
            elif entry.is_dir() and IMPORTANT_FILE.search(entry.name):
                found.extend(sorted(child for child in entry.iterdir() if child.is_file()))
    # Deduplicate while keeping order.
    unique: list[Path] = []
    for path in found:
        resolved = path.resolve()
        if resolved not in {item.resolve() for item in unique}:
            unique.append(path)
    return unique


def spdx_text(experiment: Path, identifier: str) -> list[tuple[str, str]]:
    """Return (label, text) for a single SPDX identifier."""
    identifier = identifier.strip()
    if identifier == "Apache-2.0 WITH LLVM-exception":
        candidates = [("Apache-2.0", "Apache-2.0.txt"), ("LLVM-exception", "LLVM-exception.txt")]
    else:
        candidates = [(identifier, f"{identifier}.txt")]
    texts: list[tuple[str, str]] = []
    for label, filename in candidates:
        for directory in SPDX_TEXT_DIRS:
            path = experiment / directory / filename
            if path.is_file():
                texts.append((f"SPDX {label} ({path.relative_to(experiment)})", read_text(path)))
                break
        else:
            return []
    return texts


def spdx_identifiers(expression: str) -> list[str]:
    identifiers = []
    for part in re.split(r"\s+(?:OR|AND)\s+|/", expression):
        part = part.strip().strip("() ")
        if part and part not in identifiers:
            identifiers.append(part)
    return identifiers


def package_notices(experiment: Path, package: dict, root: Path) -> tuple[list[str], int]:
    """Return (lines, number of licence texts) for one package."""
    name = package["name"]
    version = package["version"]
    manifest = Path(package["manifest_path"])
    package_dir = manifest.parent
    license = package.get("license") or "Unspecified"
    lines = [
        f"## {name} {version}",
        f"Licence: {license}",
        f"Source: {source_label(package, root)}",
        "",
    ]
    files = important_files(package_dir, package.get("license_file"))
    text_count = 0
    if files:
        lines.append("Licence and notice files:")
        for path in files:
            lines.append(f"- {path.name} (sha256 {sha256(path)})")
            lines.append("")
            lines.append(read_text(path).rstrip())
            lines.append("")
            text_count += 1
        return lines, text_count

    # Some crates ship no licence file of their own. Use the canonical SPDX
    # text for the declared licence; if that is not available either, stop.
    if license != "Unspecified":
        texts: list[tuple[str, str]] = []
        missing: list[str] = []
        for identifier in spdx_identifiers(license):
            resolved = spdx_text(experiment, identifier)
            if not resolved:
                missing.append(identifier)
            else:
                texts.extend(resolved)
        if texts and not missing:
            lines.append("The package ships no licence file; using the SPDX text:")
            lines.append("")
            for label, text in texts:
                lines.append(f"--- {label} ---")
                lines.append(text.rstrip())
                lines.append("")
                text_count += 1
            return lines, text_count

    raise SystemExit(
        f"error: no licence text found for {name} {version} "
        f"(declared licence: {license}; manifest: {manifest})"
    )


def registry_licence_files(experiment: Path, crate: str) -> list[Path]:
    root = experiment.parents[1]  # CogWasm repository root
    version = read_lock_version(root / "Cargo.lock", crate)
    pattern = f"toolchain/cargo/registry/src/*/{crate}-{version}/LICENSE*"
    matches = sorted(experiment.glob(pattern))
    if not matches:
        raise SystemExit(
            f"error: no licence file for {crate} {version}; tried {experiment / pattern}"
        )
    return matches


def component_section(experiment: Path, component: dict) -> tuple[list[str], int]:
    lines = [
        f"### {component['title']}",
        f"Licence: {component['licence']}",
        f"Source: {component['source']}",
    ]
    if component.get("note"):
        lines.append(component["note"])
    lines.append("")
    if "registry_package" in component:
        files = registry_licence_files(experiment, component["registry_package"])
    else:
        files = [experiment / relative for relative in component["files"]]
    count = 0
    for path in files:
        if not path.is_file():
            raise SystemExit(
                f"error: missing licence text for {component['title']}: {path}"
            )
        try:
            shown = path.relative_to(experiment)
        except ValueError:
            shown = path
        lines.append(f"--- {shown} (sha256 {sha256(path)}) ---")
        lines.append(read_text(path).rstrip())
        lines.append("")
        count += 1
    return lines, count


def git_revision(path: Path) -> str:
    if not (path / ".git").exists():
        return "pinned checkout not present"
    result = subprocess.run(
        ["git", "-C", str(path), "rev-parse", "HEAD"], capture_output=True, text=True, check=False
    )
    return result.stdout.strip() if result.returncode == 0 else "unknown revision"


def read_lock_version(lock: Path, crate: str) -> str:
    text = read_text(lock)
    match = re.search(
        rf'name = "{re.escape(crate)}"\nversion = "([^"]+)"', text
    )
    return match.group(1) if match else "unknown"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path, help="THIRD_PARTY_NOTICES output path")
    parser.add_argument("--cargo", default=os.environ.get("CARGO", "cargo"))
    parser.add_argument(
        "--experiment",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="experiments/cranelift-wasm directory",
    )
    args = parser.parse_args()

    experiment = args.experiment.resolve()
    root = experiment.parents[1]  # CogWasm repository root
    manifests = [experiment / relative for relative in MANIFESTS]
    manifests.extend(sorted(experiment.glob(PROGRAM_GLOB)))

    packages: dict[tuple[str, str, str], dict] = {}
    used_manifests: list[Path] = []
    for manifest in manifests:
        if not manifest.is_file():
            print(f"warning: skipping missing manifest {manifest}", file=sys.stderr)
            continue
        metadata = run_metadata(args.cargo, root, manifest)
        used_manifests.append(manifest)
        for package in metadata["packages"]:
            manifest_path = Path(package["manifest_path"])
            try:
                manifest_path.relative_to(root)
                inside_repo = True
            except ValueError:
                inside_repo = False
            # Our own crates are covered by the repository licence; pinned
            # upstream checkouts under toolchain/ are third-party and stay in.
            if inside_repo and "toolchain" not in manifest_path.parts:
                continue
            key = (package["name"], package["version"], package.get("source") or "")
            packages.setdefault(key, package)

    lines = [
        "THIRD-PARTY NOTICES",
        "===================",
        "",
        "This file is generated by release/gen-notices.py from the actual",
        "dependency graphs of the host and tool crates. Do not edit it by hand;",
        "re-run the generator after changing dependencies.",
        "",
        "This bundle contains a modified build of the Rust compiler. It is not",
        "the official Rust compiler and is not endorsed by the Rust project.",
        "",
        "Named components",
        "----------------",
        "",
    ]
    component_texts = 0
    for component in NAMED_COMPONENTS:
        section, count = component_section(experiment, component)
        lines.extend(section)
        component_texts += count

    lines.extend(["Cargo dependencies", "------------------", ""])
    package_texts = 0
    for key in sorted(packages):
        section, count = package_notices(experiment, packages[key], root)
        lines.extend(section)
        package_texts += count

    lines.extend(
        [
            "Provenance",
            "----------",
            (
                f"rustc source: {git_revision(experiment / 'toolchain/stage-b/rust')} "
                "(nightly-2026-10-19 plus stage-c patches)"
            ),
            f"clif-to-wasm: {git_revision(experiment / 'toolchain/clif-to-wasm')}",
            f"Wild: {git_revision(experiment / 'toolchain/wild')}",
            f"rust-playground-inbrowser: {git_revision(experiment / 'toolchain/playground')}",
            f"Wasmtime: {read_lock_version(root / 'Cargo.lock', 'wasmtime')}",
            "wasi-sdk: "
            + (
                " ".join(
                    (experiment / "toolchain/stage-b/wasi-sdk/VERSION")
                    .read_text(encoding="utf-8")
                    .splitlines()
                )
                if (experiment / "toolchain/stage-b/wasi-sdk/VERSION").is_file()
                else "unknown"
            ),
            "",
        ]
    )

    output = "\n".join(lines).rstrip() + "\n"
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(output, encoding="utf-8")

    print(
        f"wrote {args.out} ({len(output.encode('utf-8'))} bytes, "
        f"{len(packages)} cargo packages, {component_texts} named-component texts, "
        f"{package_texts} package texts) from {len(used_manifests)} manifests"
    )
    for component in NAMED_COMPONENTS:
        title = component["title"]
        if title.split(" (")[0] not in output:
            raise SystemExit(f"error: named component missing from output: {title}")
    print("all six named components are present")


if __name__ == "__main__":
    main()
