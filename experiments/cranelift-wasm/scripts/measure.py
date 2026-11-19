#!/usr/bin/env python3
"""Build every program under each config, measure cold/warm compile+link,
validate/run/benchmark with the Wasmtime host, and write results/summary.md."""
import json
import os
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOOL = ROOT / "toolchain"
TARGET = TOOL / "target" / "measure"
HOST = TOOL / "target" / "host" / "release" / "host"
DYLIB = TOOL / "target" / "cg_clif" / "release" / "librustc_codegen_cranelift.dylib"
WILD = TOOL / "target" / "wild" / "release" / "wild"
PROGRAMS = ["a_add", "b_alloc", "c_plugin"]

BASE_ENV = os.environ.copy()
BASE_ENV.update({
    "RUSTUP_HOME": str(TOOL / "rustup"),
    "CARGO_HOME": str(TOOL / "cargo"),
    "PATH": f"{TOOL / 'cargo' / 'bin'}:{os.environ.get('PATH', '')}",
    "RUSTUP_TOOLCHAIN": "nightly-2026-10-19",
})

CONFIGS = [
    # name, backend, linker, opt, build_std
    ("clif-lld-o2", "clif", "lld", 2, True),
    ("clif-lld-o2-prebuilt", "clif", "lld", 2, False),
    ("clif-wild-o2", "clif", "wild", 2, True),
    ("llvm-lld-o2", "llvm", "lld", 2, False),
    ("llvm-wild-o2", "llvm", "wild", 2, False),
    ("clif-lld-o0", "clif", "lld", 0, False),
    ("llvm-lld-o0", "llvm", "lld", 0, False),
]


def env_for(cfg):
    _, backend, linker, opt, _build_std = cfg
    env = BASE_ENV.copy()
    flags = ["-Cpanic=abort"]
    if opt != 2:
        env["CARGO_PROFILE_RELEASE_OPT_LEVEL"] = str(opt)
    if backend == "clif":
        flags.append(f"-Zcodegen-backend={DYLIB}")
    if linker == "wild":
        flags += [f"-Clinker={WILD}", "-Clinker-flavor=wasm-ld"]
    env["RUSTFLAGS"] = " ".join(flags)
    return env


def build(cfg, prog, target_dir, cold, runs=1):
    env = env_for(cfg)
    env["CARGO_TARGET_DIR"] = str(target_dir)
    cwd = ROOT / "programs" / prog
    if cold and target_dir.exists():
        shutil.rmtree(target_dir)
    times = []
    for i in range(runs):
        if not cold:
            os.utime(cwd / "src" / "lib.rs")
        cmd = ["cargo", "build", "--release", "--target", "wasm32-unknown-unknown"]
        if cfg[4]:
            cmd += ["-Zbuild-std=core,alloc,compiler_builtins"]
        t = time.perf_counter()
        r = subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True, check=False)
        dt = time.perf_counter() - t
        if r.returncode != 0:
            print(r.stdout[-4000:])
            print(r.stderr[-4000:], file=sys.stderr)
            raise SystemExit(f"build failed: {cfg[0]} {prog}")
        times.append(dt)
    return statistics.median(times)


def artifact(target_dir, prog):
    return target_dir / "wasm32-unknown-unknown" / "release" / f"{prog}.wasm"


def host(*args, expect_ok=True):
    r = subprocess.run([str(HOST), *map(str, args)], capture_output=True, text=True, check=False)
    if expect_ok and r.returncode != 0:
        raise SystemExit(f"host {args} failed:\n{r.stdout}\n{r.stderr}")
    return r


def validate(path):
    r = host("inspect", path)
    return r.stdout.splitlines()[0]


def bench(path, n=2_000_000, reps=3):
    vals = []
    for _ in range(reps):
        out = host("bench", path, n).stdout
        vals.append(float(out.split("(")[-1].split()[0]))
    return statistics.median(vals)


def main():
    if not HOST.exists():
        raise SystemExit("build the host first (./run.sh tools)")
    records = []
    for cfg in CONFIGS:
        for prog in PROGRAMS:
            td = TARGET / cfg[0] / prog
            cold = build(cfg, prog, td, cold=True, runs=1)
            warm = build(cfg, prog, td, cold=False, runs=3)
            wasm = artifact(td, prog)
            rec = {
                "config": cfg[0], "program": prog,
                "cold_s": round(cold, 3), "warm_s": round(warm, 3),
                "size": wasm.stat().st_size, "validation": validate(wasm),
            }
            if prog == "b_alloc":
                rec["bench_ns_per_iter"] = round(bench(wasm), 2)
            if prog == "a_add":
                rec["add"] = host("add", wasm, 20, 22).stdout.strip()
            if prog == "c_plugin":
                rec["plugin"] = host("plugin", wasm, 1, 2, 3, 4).stdout.strip()
            records.append(rec)
            print(json.dumps(rec))

    (ROOT / "results").mkdir(exist_ok=True)
    (ROOT / "results" / "measurements.json").write_text(json.dumps(records, indent=2) + "\n")

    lines = [
        "| config | program | cold s | warm s | bytes | validation |",
        "|---|---|---:|---:|---:|---|",
    ]
    for r in records:
        lines.append(f"| {r['config']} | {r['program']} | {r['cold_s']:.3f} | {r['warm_s']:.3f} | {r['size']} | {r['validation']} |")
    benches = [r for r in records if "bench_ns_per_iter" in r]
    lines += ["", "| config | b_alloc ns/iter (n=2,000,000) |", "|---|---:|"]
    for r in benches:
        lines.append(f"| {r['config']} | {r['bench_ns_per_iter']:.2f} |")
    (ROOT / "results" / "summary.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
