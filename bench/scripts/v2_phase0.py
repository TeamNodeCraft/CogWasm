#!/usr/bin/env python3
"""Phase 0 cacheability measurement for the V2 plan.

For each realistic edit to the generated large program (``gen_large.py``), this
script measures:

  (a) how many functions' final optimised MIR actually changed, by comparing a
      cold ``-Zdump-mir=SimplifyCfg-final`` dump of the base source with the
      same dump of the edited source (same file path, same crate name);
  (b) which CGUs rustc rebuilt and how many function bodies they contain, from
      ``-Zquery-dep-graph -Zprint-mono-items`` (``CGU_REUSE`` / ``MONO_ITEM``);
  (c) the incremental time split from ``-Ztime-passes`` and ``-Zself-profile``
      (front end, per-CGU codegen, incremental save/load, link).

The measurement uses the native stage-1 compiler built by the compiler scripts
with the wasm32-wasip1 target, the only wasm target that compiler has. Its
pointer width and aggregate layout match wasm32-unknown-unknown, and the CGU
partitioner and incremental machinery are target independent. The product
pipeline's Wild link and Wasmtime swap are measured separately by
``cogwasm-supervisor edit-loop``; this script reports rustc's rlib link.

Dumps and logs go under ``toolchain/work/v2`` (git-ignored). Delete the bulk
MIR dump directories when the numbers are recorded.

Usage:

    python3 bench/scripts/v2_phase0.py \
        --large toolchain/work/v2/large/large.rs \
        --out toolchain/work/v2/cacheability \
        --rounds 2
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
STAGE1 = ROOT / "toolchain/rust/build/host/stage1"
RUSTC = STAGE1 / "bin/rustc"
PROFSUM = ROOT / "toolchain/work/v2/tools/profsum/target/release/profsum"


def block(text: str, name: str) -> tuple[int, int]:
    start = text.index(f"// >>> {name}\n")
    end = text.index(f"// <<< {name}\n") + len(f"// <<< {name}\n")
    return start, end


def scoped_replace(
    text: str, module: int, old: str, new: str, count: int = 1
) -> str:
    start, end = block(text, f"m{module:03}")
    segment = text[start:end]
    found = segment.count(old)
    if found != count:
        raise SystemExit(f"m{module:03}: expected {count} x {old!r}, found {found}")
    return text[:start] + segment.replace(old, new) + text[end:]


def function_replace(
    text: str,
    module: int,
    fn_marker: str,
    next_marker: str,
    old: str,
    new: str,
    count: int = 1,
) -> str:
    start, end = block(text, f"m{module:03}")
    segment = text[start:end]
    fn_start = segment.index(fn_marker)
    fn_end = segment.index(next_marker, fn_start)
    part = segment[fn_start:fn_end]
    found = part.count(old)
    if found != count:
        raise SystemExit(f"m{module:03} {fn_marker}: expected {count} x {old!r}, found {found}")
    return text[:start] + segment[:fn_start] + part.replace(old, new) + segment[fn_end:] + text[end:]


def edit_one_function(text: str) -> str:
    """Change one instruction inside m064::hot_edit (one function body)."""
    return scoped_replace(
        text,
        64,
        "        acc = acc.rotate_left(13).wrapping_add(0x1234_5678);",
        "        acc = acc.rotate_left(13).wrapping_add(0x1234_5679);",
    )


def edit_add_function(text: str) -> str:
    """Add a used function at the end of the crate, called from m032::run.

    Appending at the end keeps every existing span's line number unchanged, so
    a line/column shift cannot masquerade as an MIR change.
    """
    text = scoped_replace(
        text,
        32,
        "        let mut acc = combine(seed, &samples);",
        "        let mut acc = combine(seed, &samples).wrapping_add(super::added_new(seed));",
    )
    return text + (
        "\n// @edit:add-function (appended at the end: existing spans do not move)\n"
        "#[inline(never)]\n"
        "pub fn added_new(seed: u64) -> u64 {\n"
        "    let words = [seed, seed.rotate_left(9), seed ^ 0x55aa];\n"
        "    crate::support::checksum::<3>(seed, &words)\n"
        "}\n"
    )


def edit_signature(text: str) -> str:
    """Change m016::stage_06's parameter type and update its one caller."""
    text = function_replace(
        text,
        16,
        "    fn stage_06(",
        "    #[inline(never)]\n    fn stage_07",
        "fn stage_06(seed: u64, depth: u32) -> u64",
        "fn stage_06(seed: u32, depth: u32) -> u64",
    )
    text = function_replace(
        text,
        16,
        "    fn stage_06(",
        "    #[inline(never)]\n    fn stage_07",
        "        let mut acc = seed ^",
        "        let mut acc = (seed as u64) ^",
    )
    text = function_replace(
        text,
        16,
        "    fn stage_06(",
        "    #[inline(never)]\n    fn stage_07",
        "seed.rotate_left",
        "(seed as u64).rotate_left",
    )
    text = scoped_replace(
        text,
        16,
        "stage_06(acc, depth.saturating_sub(1))",
        "stage_06(acc as u32, depth.saturating_sub(1))",
    )
    return scoped_replace(text, 16, "stage_06(acc, 1)", "stage_06(acc as u32, 1)")


def edit_struct_layout(text: str) -> str:
    """Add a field to m048::Packet48 and its one construction."""
    text = scoped_replace(
        text,
        48,
        "        pub payload: u64,\n    }",
        "        pub payload: u64, pub extra: u16,\n    }",
    )
    return scoped_replace(
        text,
        48,
        "        let packet = Packet48 { tag: (seed as u16) ^ 48, payload: acc }; "
        "acc = acc.wrapping_add(packet.payload ^ packet.tag as u64);",
        "        let packet = Packet48 { tag: (seed as u16) ^ 48, payload: acc, extra: 1 }; "
        "acc = acc.wrapping_add(packet.payload ^ packet.extra as u64);",
    )


def edit_generic_insert(text: str) -> str:
    """Change support::Table::insert's body; it is instantiated once per module."""
    start, end = block(text, "support")
    segment = text[start:end]
    old = "                    self.entries[index].1 = value;"
    new = (
        "                    self.entries[index].1 = value; "
        "let last = self.entries.len() - 1; self.entries.swap(index, last);"
    )
    found = segment.count(old)
    if found != 1:
        raise SystemExit(f"support insert: expected 1 match, found {found}")
    return text[:start] + segment.replace(old, new) + text[end:]


# Distinctive MonoItem substrings for the functions each edit changes. The
# cold MIR dump is per generic definition, so a changed generic definition
# shows up once; this list lets us count the monomorphised instances in the
# rebuilt CGUs, which is what codegen actually pays for.
CHANGED_MARKERS = {
    "one_function": ("hot_edit",),
    "add_function": ("added_new", "m032::run"),
    "signature": ("m016::stage_06", "m016::stage_07", "m016::run"),
    "struct_layout": ("m048::run",),
    "generic_insert": ("::insert",),
}

EDITS = {
    "one_function": edit_one_function,
    "add_function": edit_add_function,
    "signature": edit_signature,
    "struct_layout": edit_struct_layout,
    "generic_insert": edit_generic_insert,
}
EDIT_ORDER = list(EDITS)


# ---------------------------------------------------------------------------
# Parsers
# ---------------------------------------------------------------------------

def normalise_mir_name(name: str) -> str:
    # Closure def paths embed a source span. Spans do not move between these
    # edits, but normalise anyway so a future edit that does move them cannot
    # fake a change.
    return re.sub(r":\d+:\d+", ":L:C", name)


def mir_hashes(directory: Path) -> dict[str, list[str]]:
    suffix = ".SimplifyCfg-final.after.mir"
    hashes: dict[str, list[str]] = {}
    for path in sorted(directory.glob(f"*{suffix}")):
        name = normalise_mir_name(path.name[: -len(suffix)])
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        start = next((i for i, line in enumerate(lines) if line.startswith("fn ")), 0)
        body = "\n".join(lines[start:])
        # Allocation, promoted-constant and static ids are numbering, not
        # optimised code. An edit that adds one allocation renumbers every
        # later one and would otherwise look like a crate-wide MIR change.
        body = re.sub(r"\balloc\d+\b", "alloc", body)
        body = re.sub(r"\bpromoted\[\d+\]", "promoted[]", body)
        body = re.sub(r"\bstatic\[\d+\]", "static[]", body)
        digest = hashlib.sha256(body.encode()).hexdigest()
        hashes.setdefault(name, []).append(digest)
    for values in hashes.values():
        values.sort()
    return hashes


def compare_mir(base: dict[str, list[str]], edited: dict[str, list[str]]) -> dict[str, int]:
    changed = added = removed = 0
    for name in set(base) & set(edited):
        left, right = base[name], edited[name]
        changed += sum(1 for a, b in zip(left, right) if a != b)
        changed += abs(len(left) - len(right))
    added = sum(len(v) for k, v in edited.items() if k not in base)
    removed = sum(len(v) for k, v in base.items() if k not in edited)
    return {
        "base_functions": sum(len(v) for v in base.values()),
        "changed": changed,
        "added": added,
        "removed": removed,
        "changed_or_added": changed + added,
    }


TIME_RE = re.compile(r"^time:\s+([0-9.]+);.*\t([^\t]+)\s*$")


def parse_time_passes(text: str) -> dict[str, float]:
    totals: dict[str, float] = {}
    for line in text.splitlines():
        match = TIME_RE.match(line)
        if match:
            totals[match.group(2)] = totals.get(match.group(2), 0.0) + float(match.group(1)) * 1000.0
    return totals


def parse_mono_items(text: str) -> tuple[dict[str, str], dict[str, set[str]]]:
    reuse: dict[str, str] = {}
    cgu_functions: dict[str, set[str]] = {}
    for line in text.splitlines():
        if line.startswith("CGU_REUSE "):
            _, cgu, kind = line.split()
            reuse[cgu] = kind
        elif line.startswith("MONO_ITEM fn "):
            rest = line[len("MONO_ITEM fn ") :]
            if " @@ " not in rest:
                continue
            item, cgus = rest.split(" @@ ", 1)
            for token in cgus.split():
                cgu = token.split("[", 1)[0]
                cgu_functions.setdefault(cgu, set()).add(item)
    return reuse, cgu_functions


def parse_profile(stem: Path) -> tuple[dict[str, float], dict[str, float]]:
    """Return (per-CGU codegen ns, aggregate label ns) from a self-profile."""
    if not PROFSUM.exists():
        return {}, {}
    result = subprocess.run(
        [str(PROFSUM), str(stem)], capture_output=True, text=True, check=False
    )
    if result.returncode != 0:
        return {}, {}
    per_cgu: dict[str, float] = {}
    aggregate: dict[str, float] = {}
    for line in result.stdout.splitlines():
        parts = line.split(None, 3)
        if len(parts) < 4:
            continue
        nanos, _count, _kind, rest = parts
        match = re.match(r'^(\S+)(?: \["([^"]*)"\])?$', rest)
        if not match:
            continue
        label, arg = match.group(1), match.group(2)
        value = float(nanos)
        aggregate[label] = aggregate.get(label, 0.0) + value
        if label == "codegen_module_optimize" and arg:
            per_cgu[arg] = per_cgu.get(arg, 0.0) + value
    return per_cgu, aggregate


def run_rustc(
    source: Path,
    out: Path,
    log: Path,
    extra: list[str],
    incremental: Path | None = None,
) -> tuple[str, str, float]:
    command = [
        str(RUSTC),
        "--edition=2021",
        "--crate-name",
        "large",
        "--target",
        "wasm32-wasip1",
        "--sysroot",
        str(STAGE1),
        "--crate-type",
        "lib",
        "-C",
        "panic=abort",
        "-C",
        "opt-level=2",
        "-Zmir-opt-level=2",
        "-o",
        str(out),
    ]
    if incremental is not None:
        command.append(f"-Cincremental={incremental}")
    command.extend(extra)
    command.append(str(source))
    started = time.perf_counter()
    result = subprocess.run(command, capture_output=True, text=True, check=False)
    elapsed = (time.perf_counter() - started) * 1000.0
    log.write_text(
        "$ " + " ".join(command) + "\n\n== stdout ==\n" + result.stdout + "\n== stderr ==\n" + result.stderr,
        encoding="utf-8",
    )
    if result.returncode != 0:
        raise SystemExit(f"rustc failed for {source} (see {log})")
    return result.stdout, result.stderr, elapsed


def dump_mir(source: Path, out: Path, dump_dir: Path, log: Path) -> Path:
    if dump_dir.exists():
        shutil.rmtree(dump_dir)
    dump_dir.mkdir(parents=True)
    run_rustc(
        source,
        out,
        log,
        [
            "-Zdump-mir=SimplifyCfg-final",
            f"-Zdump-mir-dir={dump_dir}",
            "-Zdump-mir-exclude-pass-number",
        ],
    )
    return dump_dir


def measure_variant(
    base_text: str,
    variant: str,
    source: Path,
    out_root: Path,
    base_mir: dict[str, list[str]],
    rounds: int,
    cgu_per_fn: bool,
) -> dict:
    edited_text = EDITS[variant](base_text)
    variant_dir = out_root / variant
    variant_dir.mkdir(parents=True, exist_ok=True)
    logs = out_root / "logs"
    logs.mkdir(parents=True, exist_ok=True)

    # (a) cold full-MIR dump of the edited source.
    source.write_text(edited_text, encoding="utf-8")
    mir_dir = dump_mir(source, variant_dir / "dump.rlib", out_root / "mir" / variant, logs / f"{variant}-dump.log")
    edited_mir = mir_hashes(mir_dir)
    mir = compare_mir(base_mir, edited_mir)
    source.write_text(base_text, encoding="utf-8")

    # (b)/(c) incremental base -> edit, repeated for a timing spread.
    samples = []
    for round_index in range(rounds):
        incr = out_root / "incr" / f"{variant}-{round_index}"
        if incr.exists():
            shutil.rmtree(incr)
        incr.mkdir(parents=True)
        shared = variant_dir / "large.rlib"
        extra = [
            "-Zprint-mono-items",
            "-Zquery-dep-graph",
            "-Ztime-passes",
        ]
        if cgu_per_fn:
            extra.append("-Zcogwasm-cgu-per-fn")
        run_rustc(source, shared, logs / f"{variant}-base-{round_index}.log", extra, incr)
        source.write_text(edited_text, encoding="utf-8")
        profile_dir = variant_dir / f"prof-edit-{round_index}"
        edit_extra = extra + [
            f"-Zself-profile={profile_dir}",
            "-Zself-profile-events=default,args",
        ]
        stdout, stderr, elapsed = run_rustc(
            source, shared, logs / f"{variant}-edit-{round_index}.log", edit_extra, incr
        )
        source.write_text(base_text, encoding="utf-8")

        reuse, cgu_functions = parse_mono_items(stdout)
        rebuilt = {cgu for cgu, kind in reuse.items() if kind not in ("PreLto", "PostLto")}
        rebuilt_bodies = sum(len(cgu_functions.get(cgu, ())) for cgu in rebuilt)
        rebuilt_distinct = len(set().union(*(cgu_functions.get(cgu, set()) for cgu in rebuilt)))
        profile_files = sorted(profile_dir.glob("*.mm_profdata"))
        profile_stem = profile_files[0].with_suffix("") if profile_files else profile_dir
        per_cgu_ns, aggregate_ns = parse_profile(profile_stem)
        rebuilt_codegen_ms = sum(
            per_cgu_ns.get(cgu, 0.0) for cgu in rebuilt
        ) / 1e6
        samples.append(
            {
                "_rebuilt": rebuilt,
                "_cgu_functions": cgu_functions,
                "round": round_index,
                "wall_ms": elapsed,
                "cgu_total": len(reuse),
                "cgu_rebuilt": len(rebuilt),
                "cgu_reused": len(reuse) - len(rebuilt),
                "rebuilt_function_bodies": rebuilt_bodies,
                "rebuilt_distinct_functions": rebuilt_distinct,
                "time_passes_ms": parse_time_passes(stderr),
                "profile_ms": {k: v / 1e6 for k, v in aggregate_ns.items()},
                "rebuilt_cgu_codegen_ms": rebuilt_codegen_ms,
            }
        )
        shutil.rmtree(incr, ignore_errors=True)

    # Keep the smallest-wall sample as the representative one for the split.
    best = min(samples, key=lambda sample: sample["wall_ms"])
    rebuilt_bodies = best["rebuilt_function_bodies"]
    markers = CHANGED_MARKERS.get(variant, ())
    changed = min(
        len(
            {
                item
                for cgu in best.get("_rebuilt", set())
                for item in best.get("_cgu_functions", {}).get(cgu, set())
                if any(marker in item for marker in markers)
            }
        ),
        rebuilt_bodies,
    )
    for sample in samples:
        sample.pop("_rebuilt", None)
        sample.pop("_cgu_functions", None)
    unchanged = max(rebuilt_bodies - changed, 0)
    change_share = changed / rebuilt_bodies if rebuilt_bodies else 0.0
    codegen = best["rebuilt_cgu_codegen_ms"]
    best["codegen_split"] = {
        "changed_bodies": changed,
        "unchanged_bodies_in_rebuilt_cgus": unchanged,
        "changed_share": change_share,
        "changed_ms": codegen * change_share,
        "unchanged_ms": codegen * (1.0 - change_share),
        "method": "per-CGU codegen from -Zself-profile, apportioned by function-body count",
    }
    return {"mir": mir, "samples": samples, "representative": best}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--large", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--rounds", type=int, default=2)
    parser.add_argument("--cgu-per-fn", action="store_true")
    args = parser.parse_args()
    if not RUSTC.exists():
        raise SystemExit(f"missing native stage-1 rustc: {RUSTC}")
    base_text = args.large.read_text(encoding="utf-8")
    out_root: Path = args.out
    out_root.mkdir(parents=True, exist_ok=True)
    # Base and every variant live at the same path: span identity must not
    # change just because the source file moved.
    source = out_root / "large.rs"
    source.write_text(base_text, encoding="utf-8")
    (out_root / "logs").mkdir(exist_ok=True)

    print("dumping base MIR...", flush=True)
    base_mir = mir_hashes(
        dump_mir(source, out_root / "base.rlib", out_root / "mir" / "base", out_root / "logs/base-dump.log")
    )
    print(f"base: {sum(len(v) for v in base_mir.values())} function MIR dumps", flush=True)

    results: dict[str, dict] = {}
    for variant in EDIT_ORDER:
        print(f"measuring {variant}...", flush=True)
        results[variant] = measure_variant(
            base_text, variant, source, out_root, base_mir, args.rounds, args.cgu_per_fn
        )
        print(
            f"  MIR changed {results[variant]['mir']['changed_or_added']}, "
            f"CGUs rebuilt {results[variant]['representative']['cgu_rebuilt']}, "
            f"bodies {results[variant]['representative']['rebuilt_function_bodies']}",
            flush=True,
        )

    payload = {
        "large_source": str(args.large),
        "cgu_per_fn": args.cgu_per_fn,
        "base_functions": sum(len(v) for v in base_mir.values()),
        "variants": results,
    }
    (out_root / "results.json").write_text(json.dumps(payload, indent=2), encoding="utf-8")

    # Markdown table for the note.
    lines = [
        "| edit | MIR changed | CGUs rebuilt / total | fn bodies rebuilt (distinct) | unchanged share | wall ms |",
        "| --- | ---: | ---: | ---: | ---: | ---: |",
    ]
    for variant in EDIT_ORDER:
        result = results[variant]
        rep = result["representative"]
        wall = [sample["wall_ms"] for sample in result["samples"]]
        share = rep["codegen_split"]["changed_share"]
        lines.append(
            f"| `{variant}` | {result['mir']['changed_or_added']} | "
            f"{rep['cgu_rebuilt']} / {rep['cgu_total']} | "
            f"{rep['rebuilt_function_bodies']} ({rep['rebuilt_distinct_functions']}) | "
            f"{1.0 - share:.0%} | {min(wall):.0f} (min of {len(wall)}) |"
        )
    print("\n".join(lines))
    (out_root / "summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
