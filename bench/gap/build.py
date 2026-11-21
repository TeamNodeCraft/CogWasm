#!/usr/bin/env python3
"""Build the benchmark modules the gap analysis instruments.

This deliberately reuses ``bench/run.py`` for source preparation, dependency
resolution and the two compile paths, so the modules analysed here are the
same ones the suite measures. It only skips the timing/startup machinery.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import sys
from pathlib import Path

GAP = Path(__file__).resolve().parent
BENCH = GAP.parent
ROOT = BENCH.parent


def load_run():
    spec = importlib.util.spec_from_file_location("bench_run", BENCH / "run.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("programs", nargs="*", help="program names (default: all)")
    parser.add_argument("--toolchain", type=Path)
    parser.add_argument("--llvm-rustc", type=Path)
    parser.add_argument("--out", type=Path, default=GAP / "work" / "modules")
    parser.add_argument("--reuse", action="store_true", help="keep a module whose source hash matches")
    args = parser.parse_args()

    run = load_run()
    toolchain = (args.toolchain or Path(os.environ.get(
        "BENCH_TOOLCHAIN",
        ROOT / "toolchain",
    ))).resolve()
    llvm_rustc = args.llvm_rustc
    if llvm_rustc is None:
        nightly = os.environ.get("BENCH_NIGHTLY", "nightly-2026-10-19")
        candidates = sorted((toolchain / "rustup" / "toolchains").glob(f"{nightly}-*/bin/rustc"))
        if not candidates:
            print(f"no {nightly} rustc under {toolchain}/rustup", file=sys.stderr)
            return 1
        llvm_rustc = candidates[0]
    llvm_rustc = llvm_rustc.resolve()
    host = toolchain / "target" / "cogwasm-host" / "release" / "bench-host"
    if not host.exists():
        print(f"missing {host}; run bench/run.sh once", file=sys.stderr)
        return 1

    manifest = json.loads((BENCH / "programs.json").read_text(encoding="utf-8"))
    wanted = set(args.programs)
    selected = [
        program for program in manifest["programs"]
        if not wanted or program["name"] in wanted
    ]
    if wanted - {program["name"] for program in selected}:
        print(f"unknown programs: {sorted(wanted - {p['name'] for p in selected})}", file=sys.stderr)
        return 1

    os.environ.setdefault("BENCH_HOST_TRIPLE", run.detect_host_triple(toolchain))
    os.environ.setdefault("BENCH_NIGHTLY", "nightly-2026-10-19")
    os.environ.setdefault("BENCH_HOST_TOOLCHAIN", "1.99.1")
    # Reuse the suite's work directory (and therefore the same absolute source
    # paths) so the compiler produces the same module the suite measured.
    work = BENCH / "work"
    sources = work / "sources"
    sources.mkdir(parents=True, exist_ok=True)
    out = args.out
    (out / "clif").mkdir(parents=True, exist_ok=True)
    (out / "llvm_o2").mkdir(parents=True, exist_ok=True)

    if any(program.get("generated") for program in selected):
        run.run([
            sys.executable,
            str(BENCH / "scripts" / "gen_stress.py"),
            "--prelude",
            str(BENCH / "programs" / "prelude.rs"),
            "--out",
            str(work / "generated_stress.rs"),
        ])
        for program in selected:
            if program.get("generated"):
                program["source"] = str(work / "generated_stress.rs")

    deps = run.build_deps(toolchain, BENCH / "work")
    records = {}
    for program in selected:
        name = program["name"]
        source = run.prepare_source(program, sources / f"{name}.rs")
        source_hash = sha256(source)
        record = {"source": str(source), "source_sha256": source_hash}
        clif_out = out / "clif" / f"{name}.wasm"
        clif = run.compile_clif(
            host,
            toolchain,
            program,
            source,
            clif_out,
            work,
            1,
            0,
            "cached",
            None,
            run.extern_args(program, deps, "clif"),
            run.dependency_search_paths(deps["clif"]),
            False,
        )
        llvm_out = out / "llvm_o2" / f"{name}.wasm"
        run.compile_llvm(
            llvm_rustc,
            program,
            source,
            llvm_out,
            2,
            run.extern_args(program, deps, "llvm"),
            1,
            run.dependency_search_paths(deps["llvm"]),
        )
        record["clif"] = {"path": str(clif_out), "sha256": clif["wasm_sha256"]}
        record["llvm_o2"] = {"path": str(llvm_out), "sha256": sha256(llvm_out)}
        records[name] = record
        print(f"built {name}", file=sys.stderr)
    (out / "manifest.json").write_text(json.dumps(records, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {out / 'manifest.json'}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:  # noqa: BLE001 - top-level CLI boundary
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
