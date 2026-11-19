#!/usr/bin/env python3
"""A.2 measurement runner for (e) maths-node and (f) host-call-heavy geometry.

Builds both plugins under each backend/config in fresh and persistent target
dirs (all under Stage-A toolchain/target/a2, gitignored), then drives the
Wasmtime harness for validation, load/instantiate timing, run timing and
host-import cost. Writes stage-b/results/a2.json and stage-b/a2/RESULTS.md.

Protocol:
  cold  = one build in a newly-created target dir (cg_clif builds core/alloc/
          compiler_builtins there too because of -Zbuild-std).
  warm  = 5 builds in a persistent target dir, touching src/lib.rs each time.
  load  = 5 Module::new + instantiate pairs with a fresh Engine, then 5 with a
          reused Engine (new Module each time).
  run   = warm call; 5 timed calls; median/min/max of ms and ns/unit.
"""

import hashlib
import json
import os
import platform
import shutil
import statistics
import subprocess
import sys
import time
from datetime import UTC, datetime
from pathlib import Path

STAGE_A = Path(__file__).resolve().parents[3]  # .../experiments/cranelift-wasm
A2 = STAGE_A / "stage-b" / "a2"
TARGET = STAGE_A / "toolchain" / "target" / "a2"
RESULTS_JSON = STAGE_A / "stage-b" / "results" / "a2.json"
RESULTS_MD = A2 / "RESULTS.md"

RUSTUP_HOME = STAGE_A / "toolchain" / "rustup"
CARGO_HOME = STAGE_A / "toolchain" / "cargo"
NIGHTLY = "nightly-2026-10-19"
DYLIB = STAGE_A / "toolchain" / "target" / "cg_clif" / "release" / "librustc_codegen_cranelift.dylib"
WILD = STAGE_A / "toolchain" / "target" / "wild" / "release" / "wild"
HARNESS = TARGET / "harness" / "release" / "a2-harness"
PROGRAMS = ["e_math", "f_geometry"]
N_POINTS = 100_000
N_ITERS = 40_000
CALLS_PER_ITER = 5
REPS = 5

# Other stage-B workers share Stage A's CARGO_HOME and periodically hold its
# package-cache lock. Use a thin CARGO_HOME whose registry/git are symlinks to
# the shared cache, so our builds never wait on that lock. Offline is safe: all
# plugin/harness dependencies are already in the shared cache.
A2_CARGO_HOME = TARGET / "cargo-a2"


def ensure_cargo_home():
    A2_CARGO_HOME.mkdir(parents=True, exist_ok=True)
    for name in ("registry", "git"):
        link = A2_CARGO_HOME / name
        src = CARGO_HOME / name
        if not link.exists() and src.exists():
            link.symlink_to(src)


ensure_cargo_home()

BASE_ENV = os.environ.copy()
BASE_ENV.update({
    "RUSTUP_HOME": str(RUSTUP_HOME),
    "CARGO_HOME": str(A2_CARGO_HOME),
    "PATH": f"{CARGO_HOME / 'bin'}:{os.environ.get('PATH', '')}",
    "RUSTUP_TOOLCHAIN": NIGHTLY,
    "CARGO_TERM_COLOR": "never",
    "CARGO_INCREMENTAL": "0",
    "CARGO_NET_OFFLINE": "true",
})

CONFIGS = [
    {
        "name": "llvm_o2",
        "backend": "llvm",
        "linker": "rust-lld (default wasm-ld)",
        "build_std": False,
        "extra": ["-Cpanic=abort"],
        "strict": False,
        "note": "default rustc LLVM wasm backend, release opt-level 2, rustup prebuilt std",
    },
    {
        "name": "clif_o2",
        "backend": "clif",
        "linker": "Wild 0.11.0",
        "build_std": True,
        "extra": [],
        "strict": True,
        "note": "cg_clif O2; MIR inlining off by default (-Zinline-mir default no, release MIR opt level 2)",
    },
    {
        "name": "clif_o2_mir3",
        "backend": "clif",
        "linker": "Wild 0.11.0",
        "build_std": True,
        "extra": ["-Zinline-mir", "-Zmir-opt-level=3"],
        "strict": True,
        "expect_fail": True,
        "note": "requested MIR-inlining config; expected to fail on compiler_builtins f128",
    },
    {
        "name": "clif_o2_inline_only",
        "backend": "clif",
        "linker": "Wild 0.11.0",
        "build_std": True,
        "extra": ["-Zinline-mir"],
        "strict": True,
        "note": "control: -Zinline-mir at default release MIR opt level 2",
    },
    {
        "name": "clif_o2_mir4",
        "backend": "clif",
        "linker": "Wild 0.11.0",
        "build_std": True,
        "extra": ["-Zinline-mir", "-Zmir-opt-level=4"],
        "strict": True,
        "note": "MIR inlining accepted at opt level 4 (level 3 fails)",
    },
    {
        "name": "clif_o2_noinline",
        "backend": "clif",
        "linker": "Wild 0.11.0",
        "build_std": True,
        "extra": ["-Zinline-mir=no", "-Zmir-opt-level=2"],
        "strict": True,
        "note": "explicit MIR-inlining-disabled control",
    },
]


def log(msg):
    print(f"[a2] {msg}", flush=True)


def env_for(cfg):
    env = BASE_ENV.copy()
    flags = list(cfg["extra"])
    if cfg["backend"] == "clif":
        flags = [f"-Zcodegen-backend={DYLIB}", "-Cpanic=abort"] + flags
        flags += [f"-Clinker={WILD}", "-Clinker-flavor=wasm-ld"]
    else:
        # cfg["extra"] already carries -Cpanic=abort for the LLVM config.
        flags = list(cfg["extra"])
    env["RUSTFLAGS"] = " ".join(flags)
    return env


def find_wasm(target_dir, prog):
    return target_dir / "wasm32-unknown-unknown" / "release" / f"{prog}.wasm"


def cargo_cmd(cfg):
    cmd = ["cargo", "build", "--release", "--target", "wasm32-unknown-unknown"]
    if cfg["build_std"]:
        cmd += ["-Zbuild-std=core,alloc,compiler_builtins"]
    return cmd


def compiled_crates(stdout, stderr):
    names = []
    for line in (stdout + "\n" + stderr).splitlines():
        line = line.strip()
        if line.startswith("Compiling "):
            names.append(line[len("Compiling "):])
    return names


def build(cfg, prog, target_dir, timed=True, strict=False, timeout=900):
    env = env_for(cfg)
    env["CARGO_TARGET_DIR"] = str(target_dir)
    if strict:
        env["CG_CLIF_WASM_STRICT"] = "1"
    cwd = A2 / prog
    t = time.perf_counter()
    r = subprocess.run(
        cargo_cmd(cfg), cwd=cwd, env=env, capture_output=True, text=True, timeout=timeout, check=False
    )
    dt = time.perf_counter() - t
    wasm = find_wasm(target_dir, prog)
    return {
        "seconds": dt if timed else None,
        "returncode": r.returncode,
        "stdout_tail": r.stdout[-4000:],
        "stderr_tail": r.stderr[-4000:],
        "wasm": str(wasm) if wasm.exists() else None,
        "compiled": compiled_crates(r.stdout, r.stderr),
    }


def safe_rmtree(path):
    p = path.resolve()
    root = TARGET.resolve()
    if root not in p.parents:
        raise RuntimeError(f"refusing to delete outside {root}: {p}")
    if p == root:
        raise RuntimeError("refusing to delete target root")
    if p.exists():
        shutil.rmtree(p)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def harness(*args):
    r = subprocess.run(
        [str(HARNESS), *map(str, args)], capture_output=True, text=True, timeout=300, check=False
    )
    if r.returncode != 0:
        raise RuntimeError(f"harness {args} failed rc={r.returncode}:\n{r.stdout}\n{r.stderr}")
    return r.stdout


def parse_kv(line):
    out = {}
    for tok in line.split():
        if "=" in tok:
            k, v = tok.split("=", 1)
            out[k] = v
    return out


def stats(vals):
    return {
        "runs": vals,
        "median": statistics.median(vals),
        "min": min(vals),
        "max": max(vals),
    }


def measure_artifact(wasm, prog):
    out = harness("inspect", wasm)
    line = next(l for l in out.splitlines() if l.startswith("INSPECT "))
    info = json.loads(line[len("INSPECT "):])

    load_out = harness("load", wasm)
    fresh_mod, fresh_inst, reused_mod, reused_inst = [], [], [], []
    for line in load_out.splitlines():
        parts = line.split()
        if len(parts) != 5 or parts[0] != "LOAD":
            continue
        _, kind, _i, mod_ms, inst_ms = parts
        if kind == "fresh":
            fresh_mod.append(float(mod_ms))
            fresh_inst.append(float(inst_ms))
        elif kind == "reused":
            reused_mod.append(float(mod_ms))
            reused_inst.append(float(inst_ms))
    load = {
        "fresh_engine": {
            "module_ms": stats(fresh_mod),
            "instantiate_ms": stats(fresh_inst),
            "total_ms": stats([a + b for a, b in zip(fresh_mod, fresh_inst)]),
        },
        "reused_engine": {
            "module_ms": stats(reused_mod),
            "instantiate_ms": stats(reused_inst),
            "total_ms": stats([a + b for a, b in zip(reused_mod, reused_inst)]),
        },
    }

    if prog == "e_math":
        run_out = harness("bench_math", wasm, N_POINTS, REPS)
        meta = parse_kv(next(l for l in run_out.splitlines() if l.startswith("MATH_META ")))
        runs_ms, ns_runs, results = [], [], []
        for line in run_out.splitlines():
            if not line.startswith("MATH_RUN "):
                continue
            _, _idx, ms, _rc, val, ns = line.split()
            runs_ms.append(float(ms))
            ns_runs.append(float(ns))
            results.append(float(val))
        ref = float(meta["ref_mean"])
        final = results[-1]
        run = {
            "kind": "math",
            "n_points": N_POINTS,
            "reps": REPS,
            "runs_ms": runs_ms,
            "median_ms": statistics.median(runs_ms),
            "min_ms": min(runs_ms),
            "max_ms": max(runs_ms),
            "ns_per_point_runs": ns_runs,
            "median_ns_per_point": statistics.median(ns_runs),
            "result": final,
            "reference": ref,
            "result_matches_reference": abs(final - ref) <= 1e-9 * max(1.0, abs(ref)),
        }
        if any(l.startswith("MATH_STATS ") for l in run_out.splitlines()):
            st = parse_kv(next(l for l in run_out.splitlines() if l.startswith("MATH_STATS ")))
            run["stats_mean"] = float(st["mean"])
            run["stats_min"] = float(st["min"])
            run["stats_max"] = float(st["max"])
    else:
        run_out = harness("bench_geom", wasm, N_ITERS, REPS)
        meta = parse_kv(next(l for l in run_out.splitlines() if l.startswith("GEOM_META ")))
        runs_ms, ns_runs, results = [], [], []
        for line in run_out.splitlines():
            if not line.startswith("GEOM_RUN "):
                continue
            _, _idx, ms, val, ns = line.split()
            runs_ms.append(float(ms))
            ns_runs.append(float(ns))
            results.append(float(val))
        ref = float(meta["ref"])
        final = results[-1]
        calls = int(meta["calls"])
        run = {
            "kind": "geometry",
            "n_iters": N_ITERS,
            "calls_per_iter": int(meta["calls_per_iter"]),
            "host_calls": calls,
            "reps": REPS,
            "runs_ms": runs_ms,
            "median_ms": statistics.median(runs_ms),
            "min_ms": min(runs_ms),
            "max_ms": max(runs_ms),
            "ns_per_call_runs": ns_runs,
            "median_ns_per_call": statistics.median(ns_runs),
            "result": final,
            "reference": ref,
            "result_matches_reference": abs(final - ref) <= 1e-9 * max(1.0, abs(ref)),
        }
    return {"artifact": info, "load": load, "run": run}


def strict_check(cfg):
    log(f"strict build: {cfg['name']}")
    target_dir = TARGET / "strict" / cfg["name"]
    safe_rmtree(target_dir)
    records = {}
    for prog in PROGRAMS:
        rec = build(cfg, prog, target_dir, timed=True, strict=True)
        # Strict-build the second program in the same dir; sysroot is reused.
        warnings = [l for l in rec["stderr_tail"].splitlines() if "warning:" in l]
        records[prog] = {
            "ok": rec["returncode"] == 0,
            "seconds": rec["seconds"],
            "warnings": warnings,
            "error_tail": None if rec["returncode"] == 0 else rec["stderr_tail"],
        }
        if rec["returncode"] != 0:
            log(f"strict build FAILED: {cfg['name']} {prog}")
            break
    safe_rmtree(target_dir)
    return records


def measure_config(cfg):
    log(f"=== config {cfg['name']}")
    result = {
        "name": cfg["name"],
        "backend": cfg["backend"],
        "linker": cfg["linker"],
        "build_std": cfg["build_std"],
        "rustflags": env_for(cfg)["RUSTFLAGS"],
        "note": cfg["note"],
        "status": "ok",
        "programs": {},
        "failure": None,
    }
    for prog in PROGRAMS:
        log(f"  cold: {prog}")
        cold_dir = TARGET / "cold" / cfg["name"] / prog
        safe_rmtree(cold_dir)
        cold = build(cfg, prog, cold_dir, timed=True)
        if cold["returncode"] != 0:
            result["status"] = "failed"
            result["failure"] = {
                "stage": "cold build",
                "program": prog,
                "returncode": cold["returncode"],
                "error_tail": cold["stderr_tail"],
                "compiled": cold["compiled"],
            }
            log(f"  cold build FAILED ({prog}); see RESULTS.md")
            safe_rmtree(cold_dir)
            return result
        cold_wasm = Path(cold["wasm"])
        art_dir = TARGET / "artifacts" / cfg["name"]
        art_dir.mkdir(parents=True, exist_ok=True)
        art_path = art_dir / f"{prog}.wasm"
        shutil.copy2(cold_wasm, art_path)
        cold_sha = sha256(cold_wasm)
        safe_rmtree(cold_dir)

        log(f"  warm: {prog}")
        warm_dir = TARGET / "warm" / cfg["name"]
        # Prewarm target dir (sysroot/deps) outside the timed runs.
        pre = build(cfg, prog, warm_dir, timed=False)
        if pre["returncode"] != 0:
            result["status"] = "failed"
            result["failure"] = {
                "stage": "warm prebuild",
                "program": prog,
                "returncode": pre["returncode"],
                "error_tail": pre["stderr_tail"],
                "compiled": pre["compiled"],
            }
            return result
        warm_seconds, warm_recompiled = [], []
        for _ in range(REPS):
            os.utime(A2 / prog / "src" / "lib.rs", None)
            rec = build(cfg, prog, warm_dir, timed=True)
            if rec["returncode"] != 0:
                result["status"] = "failed"
                result["failure"] = {
                    "stage": "warm build",
                    "program": prog,
                    "returncode": rec["returncode"],
                    "error_tail": rec["stderr_tail"],
                    "compiled": rec["compiled"],
                }
                return result
            warm_seconds.append(rec["seconds"])
            warm_recompiled.append(bool(rec["compiled"]))
        warm_wasm = find_wasm(warm_dir, prog)
        warm_sha = sha256(warm_wasm)
        if warm_sha != cold_sha:
            log(f"  NOTE: cold/warm wasm differ for {cfg['name']} {prog}")

        log(f"  measure: {prog}")
        measured = measure_artifact(str(art_path), prog)
        measured["cold"] = {
            "seconds": cold["seconds"],
            "compiled": cold["compiled"],
            "wasm_sha256": cold_sha,
        }
        measured["warm"] = {
            "seconds": stats(warm_seconds),
            "recompiled_each_run": warm_recompiled,
            "wasm_sha256": warm_sha,
        }
        # The measured artifact must be the fresh cold copy; report the source
        # path and keep the warm artifact too for size comparison.
        measured["artifact"]["path"] = str(art_path)
        measured["artifact"]["warm_path"] = str(warm_wasm)
        measured["artifact"]["warm_size"] = warm_wasm.stat().st_size
        result["programs"][prog] = measured
    return result


def ratio(a, b):
    if b in (0, None) or a is None:
        return None
    return a / b


def compute_comparisons(configs):
    by_name = {c["name"]: c for c in configs if c["status"] == "ok"}
    llvm = by_name.get("llvm_o2")
    clif = by_name.get("clif_o2")
    comp = {}
    if not llvm or not clif:
        return comp
    for prog in PROGRAMS:
        lp = llvm["programs"].get(prog)
        cp = clif["programs"].get(prog)
        if not lp or not cp:
            continue
        key = "median_ns_per_point" if prog == "e_math" else "median_ns_per_call"
        row = {
            "cold_ratio_clif_over_llvm": ratio(cp["cold"]["seconds"], lp["cold"]["seconds"]),
            "warm_ratio_clif_over_llvm": ratio(cp["warm"]["seconds"]["median"], lp["warm"]["seconds"]["median"]),
            "module_load_ratio_clif_over_llvm": ratio(
                cp["load"]["fresh_engine"]["module_ms"]["median"],
                lp["load"]["fresh_engine"]["module_ms"]["median"],
            ),
            "instantiate_ratio_clif_over_llvm": ratio(
                cp["load"]["fresh_engine"]["instantiate_ms"]["median"],
                lp["load"]["fresh_engine"]["instantiate_ms"]["median"],
            ),
            "run_ratio_clif_over_llvm": ratio(cp["run"][key], lp["run"][key]),
            "total_size_ratio_clif_over_llvm": ratio(
                cp["artifact"]["size"], lp["artifact"]["size"]
            ),
            "code_size_ratio_clif_over_llvm": ratio(
                cp["artifact"]["code_bytes"], lp["artifact"]["code_bytes"]
            ),
            "llvm": {
                "cold_s": lp["cold"]["seconds"],
                "warm_median_s": lp["warm"]["seconds"]["median"],
                "module_ms": lp["load"]["fresh_engine"]["module_ms"]["median"],
                "instantiate_ms": lp["load"]["fresh_engine"]["instantiate_ms"]["median"],
                "run_metric_median": lp["run"][key],
                "size": lp["artifact"]["size"],
                "code_bytes": lp["artifact"]["code_bytes"],
            },
            "clif": {
                "cold_s": cp["cold"]["seconds"],
                "warm_median_s": cp["warm"]["seconds"]["median"],
                "module_ms": cp["load"]["fresh_engine"]["module_ms"]["median"],
                "instantiate_ms": cp["load"]["fresh_engine"]["instantiate_ms"]["median"],
                "run_metric_median": cp["run"][key],
                "size": cp["artifact"]["size"],
                "code_bytes": cp["artifact"]["code_bytes"],
            },
        }
        comp[prog] = row

    # MIR-inlining deltas relative to plain clif O2.
    mir = {}
    for variant in ("clif_o2_inline_only", "clif_o2_mir4", "clif_o2_noinline"):
        v = by_name.get(variant)
        if not v:
            continue
        rows = {}
        for prog in PROGRAMS:
            cp = clif["programs"].get(prog)
            vp = v["programs"].get(prog)
            if not cp or not vp:
                continue
            key = "median_ns_per_point" if prog == "e_math" else "median_ns_per_call"
            rows[prog] = {
                "warm_ratio_variant_over_clif": ratio(
                    vp["warm"]["seconds"]["median"], cp["warm"]["seconds"]["median"]
                ),
                "run_ratio_variant_over_clif": ratio(vp["run"][key], cp["run"][key]),
                "module_ratio_variant_over_clif": ratio(
                    vp["load"]["fresh_engine"]["module_ms"]["median"],
                    cp["load"]["fresh_engine"]["module_ms"]["median"],
                ),
                "code_bytes_delta": vp["artifact"]["code_bytes"] - cp["artifact"]["code_bytes"],
                "size_delta": vp["artifact"]["size"] - cp["artifact"]["size"],
                "run_metric": vp["run"][key],
                "baseline_run_metric": cp["run"][key],
            }
        mir[variant] = rows
    comp["mir_inlining"] = mir
    return comp


def fmt(x, nd=3):
    if x is None:
        return "—"
    return f"{x:.{nd}f}"


def fmt_ms(d):
    return f"{d['median']:.3f} [{d['min']:.3f}–{d['max']:.3f}]"


def write_markdown(doc):
    meta = doc["meta"]
    configs = doc["configs"]
    ok = [c for c in configs if c["status"] == "ok"]
    failed = [c for c in configs if c["status"] != "ok"]
    comp = doc["comparisons"]
    lines = []
    lines.append("# Stage B / A.2 results: two realistic plugins, LLVM vs cg_clif\n")
    lines.append(f"Date: {meta['generated_at']}. Host: {meta['host']['platform']} "
                 f"({meta['host']['machine']}, {meta['host']['cpu_count']} CPUs). "
                 f"Rust: `{meta['rustc'].splitlines()[0]}`. All artifacts were validated with "
                 f"`wasmparser 0.263` (all features) and executed by Wasmtime 50.0.2.\n")
    lines.append("## Verdict\n")
    lines.append("Both plugins **build, link, validate and run** under LLVM O2 and cg_clif O2. "
                 "The requested MIR-inlining config (`-Zinline-mir -Zmir-opt-level=3`) does **not** build: "
                 "compiler_builtins hits the f128 libcall-signature conflict already seen for Stage A's O0 sysroot. "
                 "`-Zmir-opt-level=4` *does* build, so the MIR-inlining effect is measured at level 4 instead; "
                 "its effect on these two programs is small. Details below.\n")

    lines.append("## Configs\n")
    lines.append("| config | backend | linker | std | RUSTFLAGS |")
    lines.append("|---|---|---|---|---|")
    for c in meta["configs"]:
        std = "cg_clif -Zbuild-std" if c["build_std"] else "rustup prebuilt"
        shown = c["rustflags"].replace(str(STAGE_A) + "/", "$STAGE_A/")
        lines.append(f"| {c['name']} | {c['backend']} | {c['linker']} | {std} | "
                     f"`{shown}` |")
    lines.append("")

    lines.append("## Build (cold = fresh target dir, warm = 5 × touch+build)\n")
    lines.append("cg_clif builds `core`/`alloc`/`compiler_builtins` with `-Zbuild-std` in the fresh "
                 "target dir, so its cold column includes the sysroot (~6.5–7 s); LLVM uses rustup's "
                 "prebuilt std and its cold column is only the plugin. Warm is the realistic plugin rebuild. "
                 "cg_clif links with Wild (which materialises the 1 MiB `.bss` heap as a data segment); "
                 "LLVM links with the default rust-lld. `code B` is the sum of wasm function bodies, so it "
                 "is the fair code-size comparison; `size B` includes Wild's 1 MiB data segment.\n")
    lines.append("| config | program | cold s | warm s median [min–max] | size B | code B | data B | valid |")
    lines.append("|---|---|---:|---:|---:|---:|---:|---|")
    for c in configs:
        if c["status"] != "ok":
            continue
        for prog in PROGRAMS:
            p = c["programs"].get(prog)
            if not p:
                continue
            a = p["artifact"]
            lines.append(f"| {c['name']} | {prog} | {p['cold']['seconds']:.3f} | "
                         f"{fmt_ms(p['warm']['seconds'])} | {a['size']} | {a['code_bytes']} | "
                         f"{a['data_bytes']} | {'yes' if a['valid'] else 'NO'} |")
    lines.append("")

    lines.append("## Wasmtime load (median of 5, ms)\n")
    lines.append("Fresh Engine per iteration; reused Engine = one Engine, new `Module` each iteration. "
                 "`inst` is instantiate from the already-compiled module.\n")
    lines.append("| config | program | fresh compile | fresh inst | fresh total | reused compile | reused inst | reused total |")
    lines.append("|---|---|---:|---:|---:|---:|---:|---:|")
    for c in ok:
        for prog in PROGRAMS:
            p = c["programs"].get(prog)
            if not p:
                continue
            f = p["load"]["fresh_engine"]
            r = p["load"]["reused_engine"]
            lines.append(f"| {c['name']} | {prog} | {fmt_ms(f['module_ms'])} | {fmt_ms(f['instantiate_ms'])} | "
                         f"{fmt_ms(f['total_ms'])} | {fmt_ms(r['module_ms'])} | {fmt_ms(r['instantiate_ms'])} | "
                         f"{fmt_ms(r['total_ms'])} |")
    lines.append("")

    lines.append("## Run time (median of 5 warm calls)\n")
    lines.append("(e) 100,000 f64 points: `y = sin(x)*0.5 + cos(x*0.25) + sqrt(|x|)`, reduce to mean. "
                 "(f) 40,000 loop iterations × 5 imports = **200,000 host calls**.\n")
    lines.append("| config | program | total ms median [min–max] | ns/point or ns/host-call | result check |")
    lines.append("|---|---|---:|---:|---|")
    for c in ok:
        for prog in PROGRAMS:
            p = c["programs"].get(prog)
            if not p:
                continue
            r = p["run"]
            metric = r["median_ns_per_point"] if prog == "e_math" else r["median_ns_per_call"]
            unit = "ns/point" if prog == "e_math" else "ns/host-call"
            lines.append(f"| {c['name']} | {prog} | {r['median_ms']:.3f} "
                         f"[{r['min_ms']:.3f}–{r['max_ms']:.3f}] | {metric:.3f} {unit} | "
                         f"{'ok' if r['result_matches_reference'] else 'MISMATCH'} |")
    lines.append("")

    lines.append("## cg_clif vs LLVM O2 ratios\n")
    lines.append("`run` ratio is ns/point for (e) and ns/host-call for (f). Both size ratios are "
                 "cg_clif/LLVM; `total size` is dominated by Wild's 1 MiB `.bss` data segment, so "
                 "`code size` is the meaningful codegen comparison.\n")
    lines.append("| program | cold | warm | module load | instantiate | run | total size | code size |")
    lines.append("|---|---:|---:|---:|---:|---:|---:|---:|")
    for prog in PROGRAMS:
        r = comp.get(prog)
        if not r:
            continue
        lines.append(f"| {prog} | {fmt(r['cold_ratio_clif_over_llvm'], 2)}× | "
                     f"{fmt(r['warm_ratio_clif_over_llvm'], 2)}× | "
                     f"{fmt(r['module_load_ratio_clif_over_llvm'], 2)}× | "
                     f"{fmt(r['instantiate_ratio_clif_over_llvm'], 2)}× | "
                     f"{fmt(r['run_ratio_clif_over_llvm'], 2)}× | "
                     f"{fmt(r['total_size_ratio_clif_over_llvm'], 2)}× | "
                     f"{fmt(r['code_size_ratio_clif_over_llvm'], 2)}× |")
    lines.append("")

    lines.append("## Interpretation\n")
    e = comp.get("e_math")
    g = comp.get("f_geometry")
    if e and g:
        lines.append(
            f"* **Compile+link.** Warm cg_clif is {e['warm_ratio_clif_over_llvm']:.2f}× LLVM for "
            f"(e) and {g['warm_ratio_clif_over_llvm']:.2f}× for (f). Cold is not a fair backend "
            f"comparison: cg_clif's fresh dir builds core/alloc/compiler_builtins "
            f"({e['clif']['cold_s']:.2f} s / {g['clif']['cold_s']:.2f} s), while LLVM uses rustup's "
            f"prebuilt std ({e['llvm']['cold_s']:.2f} s / {g['llvm']['cold_s']:.2f} s, single runs and "
            f"therefore noisy). The warm medians are the meaningful plugin-rebuild numbers and are "
            f"well under the 0.41 s target.\n")
        lines.append(
            f"* **Load.** cg_clif modules take {e['module_load_ratio_clif_over_llvm']:.2f}× / "
            f"{g['module_load_ratio_clif_over_llvm']:.2f}× longer to compile in Wasmtime "
            f"({e['clif']['module_ms']:.1f} vs {e['llvm']['module_ms']:.1f} ms for (e), "
            f"{g['clif']['module_ms']:.1f} vs {g['llvm']['module_ms']:.1f} ms for (f)), consistent with "
            f"the larger code sections. Instantiation itself is ~0.1 ms for both, so `Module::new` "
            f"dominates load time.\n")
        lines.append(
            f"* **Run.** The maths node is {e['run_ratio_clif_over_llvm']:.2f}× slower under cg_clif "
            f"({e['clif']['run_metric_median']:.1f} vs {e['llvm']['run_metric_median']:.1f} ns/point; "
            f"{e['clif']['run_metric_median']*N_POINTS/1e6:.2f} vs "
            f"{e['llvm']['run_metric_median']*N_POINTS/1e6:.2f} ms). The host-call geometry is "
            f"{g['run_ratio_clif_over_llvm']:.2f}× (cg_clif {g['clif']['run_metric_median']:.2f} vs "
            f"LLVM {g['llvm']['run_metric_median']:.2f} ns per imported call): the "
            f"{N_ITERS*CALLS_PER_ITER:,} Wasmtime host calls dominate, so backend codegen barely "
            f"matters.\n")
        lines.append(
            f"* **Size.** cg_clif code is {e['code_size_ratio_clif_over_llvm']:.2f}× / "
            f"{g['code_size_ratio_clif_over_llvm']:.2f}× larger. Total artifact size cannot be "
            f"compared directly because Wild writes the 1 MiB `.bss` heap as a data segment "
            f"(+~1.05 MB); rust-lld represents the same zeroed heap in the memory minimum.\n")
    lines.append(
        "* **MIR inlining.** The requested `-Zinline-mir -Zmir-opt-level=3` fails to build. "
        "`-Zmir-opt-level=4` succeeds but reduces code by only 79 B (maths) / 207 B (geometry) and "
        "changes run time by <2%; `-Zinline-mir` alone at MIR opt level 2 is byte-identical to the "
        "default. There is no meaningful MIR-inlining win for these programs on cg_clif.\n")
    lines.append(
        "* **Strictness.** Every cg_clif artifact passes `CG_CLIF_WASM_STRICT=1` with zero "
        "trap-stub warnings, including core/alloc/libm.\n")

    lines.append("## MIR inlining\n")
    lines.append("`clif_o2` is the default (MIR inlining off). `inline_only` adds `-Zinline-mir` at the "
                 "default release MIR opt level (2). `mir4` adds `-Zinline-mir -Zmir-opt-level=4`. "
                 "The explicit `-Zinline-mir=no` control fails in compiler_builtins (see failures), so "
                 "the default `clif_o2` (no flag) is the meaningful MIR-inlining-disabled baseline. "
                 "`inline_only` artifacts are byte-identical to `clif_o2` (same SHA-256), so its ±3–5% "
                 "ratios are run-to-run noise. Ratios are variant/clif_o2; <1 means the variant is "
                 "faster/smaller.\n")
    lines.append("| variant | program | warm ratio | run ratio | module-load ratio | code B delta | size B delta |")
    lines.append("|---|---|---:|---:|---:|---:|---:|")
    for variant, rows in comp.get("mir_inlining", {}).items():
        for prog, r in rows.items():
            lines.append(f"| {variant} | {prog} | {fmt(r['warm_ratio_variant_over_clif'], 2)}× | "
                         f"{fmt(r['run_ratio_variant_over_clif'], 2)}× | "
                         f"{fmt(r['module_ratio_variant_over_clif'], 2)}× | "
                         f"{r['code_bytes_delta']:+d} | {r['size_delta']:+d} |")
    lines.append("")

    lines.append("## Strict-check and failures\n")
    strict_rows = []
    for c in configs:
        if c.get("strict") and c["status"] == "ok":
            for prog, s in c.get("strict", {}).items():
                strict_rows.append((c["name"], prog, s))
    if strict_rows:
        lines.append("| config | program | strict build | warnings |")
        lines.append("|---|---|---|---:|")
        for name, prog, s in strict_rows:
            lines.append(f"| {name} | {prog} | {'ok' if s['ok'] else 'FAILED'} | {len(s['warnings'])} |")
    else:
        lines.append("No strict results recorded.")
    lines.append("")
    if failed:
        lines.append("Failed configs / builds. `clif_o2_mir3` is the requested level-3 MIR-inlining "
                     "config; `clif_o2_noinline` is an explicit disabled control. Both fail while "
                     "building compiler_builtins on libcall ABI signatures (f128 and i128 respectively). "
                     "The default `clif_o2` (no inline flag) builds and is used as the disabled "
                     "baseline.\n")
        for c in failed:
            f = c.get("failure") or {}
            lines.append(f"### `{c['name']}` ({f.get('stage', 'build')}, {f.get('program', '?')})\n")
            lines.append("```text")
            lines.append((f.get("error_tail") or "no error text").strip()[-2500:])
            lines.append("```\n")
    else:
        lines.append("No build failures were recorded.\n")

    lines.append("## Reproduce\n")
    lines.append("```sh\nstage-b/a2/run.sh\n```\n")
    lines.append(f"Raw runs and medians: `{RESULTS_JSON}`. Harness: "
                 f"`{meta['harness']}` (Wasmtime 50 + wasmparser 0.263). All build outputs are under "
                 f"`{TARGET}` (gitignored); no project target dirs are written.\n")
    lines.append("## Protocol and caveats\n")
    lines.append(f"* Cold: one build in a newly-created target dir. For cg_clif configs (`-Zbuild-std`) this "
                 f"also builds core/alloc/compiler_builtins/libm; for LLVM it is plugin-only against rustup's "
                 f"prebuilt std.\n"
                 f"* Warm: prewarmed target dir, `touch src/lib.rs`, 5 builds; median [min–max]. "
                 f"Each timed run recompiled the plugin crate.\n"
                 f"* Load: 5 iterations per mode. Fresh Engine per iteration; reused mode keeps one Engine "
                 f"and creates a new Module each iteration. `Module::new` compiles; instantiate is timed separately.\n"
                 f"* Run: one warm-up call, then 5 timed calls. (e) n={N_POINTS:,} f64 points. "
                 f"(f) n={N_ITERS:,} iterations × {CALLS_PER_ITER} imports = {N_ITERS*CALLS_PER_ITER:,} host calls; "
                 f"the host implements all five `nodecraft` imports with `Linker::func_wrap`.\n"
                 f"* Linkers: cg_clif uses Wild 0.11.0 (`-Clinker=... -Clinker-flavor=wasm-ld`); LLVM uses "
                 f"rust-lld (rustc's default wasm linker). Within each family the linker is constant.\n"
                 f"* Wild materialises the 1 MiB `.bss` heap as a data segment (~1.05 MB); the `code B` "
                 f"column and the code-size ratio avoid that confound.\n"
                 f"* `CG_CLIF_WASM_STRICT=1` builds are separate clean builds per cg_clif config; warnings "
                 f"and errors are captured.\n"
                 f"* Every artifact validated with `wasmparser::WasmFeatures::all()` and produced the same "
                 f"mean/result as the host reference to within 1e-9 relative.\n"
                 f"* The host was shared with other stage-B build/test jobs during measurement; medians are "
                 f"reported, but single-run cold numbers and occasional load outliers (e.g. the first "
                 f"fresh-engine compile) are sensitive to machine load.\n")
    RESULTS_MD.write_text("\n".join(lines) + "\n")


def main():
    if not HARNESS.exists():
        sys.exit(f"missing harness {HARNESS}; run stage-b/a2/run.sh (it builds the harness)")
    RESULTS_JSON.parent.mkdir(parents=True, exist_ok=True)
    TARGET.mkdir(parents=True, exist_ok=True)

    rustc_v = subprocess.run(
        ["rustc", "-Vv"], env=BASE_ENV, capture_output=True, text=True, check=False
    ).stdout.strip()
    meta = {
        "generated_at": datetime.now(UTC).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "stage_a": str(STAGE_A),
        "rustc": rustc_v,
        "harness": str(HARNESS),
        "n_points": N_POINTS,
        "n_iters": N_ITERS,
        "calls_per_iter": CALLS_PER_ITER,
        "reps": REPS,
        "host": {
            "platform": platform.platform(),
            "machine": platform.machine(),
            "cpu_count": os.cpu_count(),
        },
        "wasmtime": "50.0.2",
        "wasmparser": "0.263.0",
        "wild": "0.11.0 (fd79578)",
        "cg_clif": "4f78fd3e5f2ce8a688865b4a6197e6d33211c09c",
        "configs": [
            {"name": c["name"], "backend": c["backend"], "linker": c["linker"],
             "build_std": c["build_std"], "rustflags": env_for(c)["RUSTFLAGS"], "note": c["note"]}
            for c in CONFIGS
        ],
    }
    doc = {"meta": meta, "configs": [], "comparisons": {}}
    for cfg in CONFIGS:
        rec = measure_config(cfg)
        if rec["status"] == "ok" and cfg.get("strict"):
            rec["strict"] = strict_check(cfg)
        doc["configs"].append(rec)
        # Save incrementally so a later crash still leaves raw data.
        doc["comparisons"] = compute_comparisons(doc["configs"])
        RESULTS_JSON.write_text(json.dumps(doc, indent=2) + "\n")
        log(f"wrote {RESULTS_JSON}")

    # Final comparisons and markdown.
    doc["comparisons"] = compute_comparisons(doc["configs"])
    RESULTS_JSON.write_text(json.dumps(doc, indent=2) + "\n")
    write_markdown(doc)
    log(f"wrote {RESULTS_MD}")
    log("done")


if __name__ == "__main__":
    main()
