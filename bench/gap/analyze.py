#!/usr/bin/env python3
"""Turn instrumented counts and static sizes into the gap attribution.

Inputs are the JSON files written by ``run.sh``: per-block execution counts,
the static block metadata (category counts, call sites), and ``gap-tool sizes``
output. The output is one deterministic JSON file plus WAT extracts for the
hot functions and a Markdown summary.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

GAP = Path(__file__).resolve().parent
BENCH = GAP.parent
ROOT = BENCH.parent
RESULTS = GAP / "results"
HISTORY = BENCH / "history" / "2026-11-19-darwin-arm64-92f976a.json"

CAUSES = [
    ("software", "Software operations"),
    ("memory", "Values in memory / register moves"),
    ("inlining", "Too little inlining"),
    ("bounds", "Bounds checks"),
    ("loop", "Loop optimisation"),
    ("other", "Other"),
]
CAUSE_LABEL = dict(CAUSES)

SOFT_MEM = re.compile(
    r"compiler_builtins::mem|(^|::)memcpy$|(^|::)memset$|(^|::)memmove$|(^|::)memcmp$|"
    r"copy_forward|copy_backward|set_bytes"
)
SOFT_MATH = re.compile(
    r"libm::math|(^|::)sqrt$|(^|::)floor$|(^|::)ceil$|(^|::)trunc$|(^|::)nearest$|"
    r"(^|::)fabs$|(^|::)copysign$|(^|::)fmin$|(^|::)fmax$|"
    r"widen_mul|rem_pio2|scalbn|goldschmidt|sqrt_round|k_sin|k_cos"
)
SOFT_OTHER = re.compile(r"__multi3|__udivti3|__divti3|__modti3|compiler_builtins::math")


def is_soft(name: str) -> str | None:
    if SOFT_MEM.search(name):
        return "memory"
    if SOFT_MATH.search(name):
        return "math"
    if SOFT_OTHER.search(name):
        return "other"
    return None


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def build_stats(meta: dict, counts: dict) -> dict:
    functions = {function["index"]: function for function in meta["functions"]}
    stats = {
        "total": 0,
        "functions": {index: 0 for index in functions},
        "categories": Counter(),
        "stack_loads": 0,
        "stack_stores": 0,
        "heap_loads": 0,
        "heap_stores": 0,
        "bounds_checks": 0,
        "calls": Counter(),
        "blocks": 0,
    }
    for block in meta["blocks"]:
        executions = counts["totals"][block["id"]]
        if executions == 0:
            continue
        stats["blocks"] += executions
        instructions = executions * block["instructions"]
        stats["total"] += instructions
        stats["functions"][block["func"]] += instructions
        stats["stack_loads"] += executions * block["stack_loads"]
        stats["stack_stores"] += executions * block["stack_stores"]
        stats["heap_loads"] += executions * block["heap_loads"]
        stats["heap_stores"] += executions * block["heap_stores"]
        stats["bounds_checks"] += executions * block["bounds_check_pairs"]
        for category, count in block["categories"].items():
            stats["categories"][category] += executions * count
        for call in block["calls"]:
            stats["calls"][call["target"]] += executions
    return stats


def attribute(meta: dict, counts: dict) -> dict:
    functions = {function["index"]: function for function in meta["functions"]}
    total = Counter()
    detail = defaultdict(Counter)
    for block in meta["blocks"]:
        executions = counts["totals"][block["id"]]
        if executions == 0:
            continue
        function = functions[block["func"]]
        soft = is_soft(function["demangled"])
        if soft is not None:
            total["software"] += executions * block["instructions"]
            detail["software"][f"soft_{soft}"] += executions * block["instructions"]
            continue
        soft_call_sites = sum(
            1 for call in block["calls"] if is_soft(call["demangled"]) is not None
        )
        for category, count in block["categories"].items():
            instructions = executions * count
            if category in ("load.stack", "store.stack", "local.get", "local.set", "local.tee"):
                total["memory"] += instructions
                key = (
                    "stack load/store"
                    if category in ("load.stack", "store.stack")
                    else "local moves (SROA/mem2reg)"
                )
                detail["memory"][key] += instructions
            elif category == "bounds.check":
                total["bounds"] += instructions
            elif category in ("br", "br_if", "br_table"):
                total["loop"] += instructions
            elif category == "call.direct":
                soft_calls = executions * soft_call_sites
                total["software"] += soft_calls
                if soft_calls:
                    detail["software"]["soft_call"] += soft_calls
                total["inlining"] += instructions - soft_calls
            elif category in ("call.indirect", "return"):
                total["inlining"] += instructions
            else:
                total["other"] += instructions
                detail["other"][category] += instructions
    total = {cause: total.get(cause, 0) for cause, _ in CAUSES}
    return {"instructions": total, "detail": {k: dict(v) for k, v in detail.items()}}


def hot_functions(meta: dict, stats: dict, threshold: float = 0.90) -> list[dict]:
    functions = {function["index"]: function for function in meta["functions"]}
    ordered = sorted(
        ((index, value) for index, value in stats["functions"].items() if value),
        key=lambda item: (-item[1], item[0]),
    )
    total = sum(value for _, value in ordered)
    rows = []
    cumulative = 0
    for rank, (index, value) in enumerate(ordered, 1):
        cumulative += value
        function = functions[index]
        rows.append(
            {
                "rank": rank,
                "index": index,
                "name": function["name"],
                "demangled": function["demangled"],
                "instructions": value,
                "share": value / total if total else 0.0,
                "cumulative": cumulative / total if total else 0.0,
                "covered": cumulative <= threshold * total,
                "body_bytes": function["body_bytes"],
            }
        )
        if cumulative > threshold * total and len(rows) >= 3:
            break
    return rows


def extract_function(text: str, name: str) -> str | None:
    needle = f"(func ${name} "
    start = text.find(needle)
    if start < 0:
        return None
    depth = 0
    for offset, character in enumerate(text[start:]):
        if character == "(":
            depth += 1
        elif character == ")":
            depth -= 1
            if depth == 0:
                return text[start : start + offset + 1]
    return None


def wat_for(module: Path, name: str, destination: Path, tool: str) -> bool:
    full = destination.with_suffix(".full.wat")
    destination.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [tool, "wat", "--wasm", str(module), "--out", str(full)],
        check=True,
        capture_output=True,
        text=True,
    )
    text = full.read_text(encoding="utf-8")
    body = extract_function(text, name)
    full.unlink(missing_ok=True)
    if body is None:
        return False
    destination.write_text(body + "\n", encoding="utf-8")
    return True


def classify_extra(name: str) -> str:
    lower = name.lower()
    if any(word in lower for word in ("panic", "unwind", "fmt", "format", "char_count", "str::count")):
        return "panic/fmt machinery"
    if "alloc" in lower or "raw_vec" in lower:
        return "allocator"
    if SOFT_MEM.search(name):
        return "software mem"
    if SOFT_MATH.search(name):
        return "libm software math"
    if "<" in name:
        return "generic instantiation"
    return "other core/compiler_builtins"


def read_leb(data: bytes, offset: int) -> tuple[int, int]:
    value = 0
    shift = 0
    while True:
        byte = data[offset]
        offset += 1
        value |= (byte & 0x7F) << shift
        if not byte & 0x80:
            return value, offset
        shift += 7


def all_data_segments(path: Path) -> list[dict]:
    data = path.read_bytes()
    offset = 8
    segments = []
    while offset < len(data):
        section_id = data[offset]
        offset += 1
        size, offset = read_leb(data, offset)
        end = offset + size
        if section_id == 11:
            count, cursor = read_leb(data, offset)
            for _ in range(count):
                flags, cursor = read_leb(data, cursor)
                memory = 0
                if flags == 0:
                    assert data[cursor] == 0x41
                    cursor += 1
                    address, cursor = read_leb(data, cursor)
                    assert data[cursor] == 0x0B
                    cursor += 1
                elif flags == 2:
                    memory, cursor = read_leb(data, cursor)
                    assert data[cursor] == 0x41
                    cursor += 1
                    address, cursor = read_leb(data, cursor)
                    assert data[cursor] == 0x0B
                    cursor += 1
                elif flags == 1:
                    address = 0
                else:
                    break
                length, cursor = read_leb(data, cursor)
                chunk = data[cursor : cursor + length]
                cursor += length
                segments.append(
                    {
                        "memory": memory,
                        "offset": address,
                        "bytes": length,
                        "zero": bool(chunk) and all(byte == 0 for byte in chunk),
                        "preview": "".join(
                            chr(byte) if 32 <= byte < 127 else "." for byte in chunk[:80]
                        ),
                    }
                )
        offset = end
    return segments


def data_previews(path: Path, limit: int = 12) -> list[dict]:
    previews = [
        {key: item[key] for key in ("memory", "offset", "bytes", "preview")}
        for item in all_data_segments(path)
        if item["bytes"] >= 16
    ]
    previews.sort(key=lambda item: (-item["bytes"], item["offset"]))
    return previews[:limit]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("programs", nargs="*")
    parser.add_argument("--counts", type=Path, default=RESULTS / "counts")
    parser.add_argument("--modules", type=Path, default=GAP / "work" / "modules")
    parser.add_argument("--history", type=Path, default=HISTORY)
    parser.add_argument("--out", type=Path, default=RESULTS / "analysis.json")
    args = parser.parse_args()

    tool = os.environ.get("GAP_TOOL")
    if not tool:
        tool = str(Path(os.environ.get("CARGO_TARGET_DIR", "")) / "release" / "gap-tool")
        if not Path(tool).exists():
            found = shutil.which("gap-tool")
            if not found:
                print("set GAP_TOOL or build gap-tool first", file=sys.stderr)
                return 1
            tool = found

    history = load_json(args.history)
    programs = args.programs or list(history["programs"])
    result = {
        "history": str(args.history),
        "commit": history["commit"],
        "machine": history["machine"],
        "seeds": history["settings"]["seeds"],
        "programs": {},
    }
    hot_root = RESULTS / "hot"
    sizes_root = RESULTS / "sizes"
    sizes_root.mkdir(parents=True, exist_ok=True)
    (RESULTS / "counts").mkdir(parents=True, exist_ok=True)

    for name in programs:
        if name not in history["programs"]:
            print(f"warning: {name} not in {args.history}", file=sys.stderr)
            continue
        entry = {"name": name, "class": history["programs"][name]["class"]}
        execution = history["programs"][name]
        entry["exec_ms"] = {
            "clif": execution["exec"]["clif"]["exec_ms"]["median"],
            "llvm_o2": execution["exec"]["llvm_o2"]["exec_ms"]["median"],
            "ratio": execution["exec"]["clif"]["exec_ms"]["median"]
            / execution["exec"]["llvm_o2"]["exec_ms"]["median"],
        }
        entry["sizes"] = {
            "clif": execution["size"]["clif"]["total_bytes"],
            "llvm_o2": execution["size"]["llvm_o2"]["total_bytes"],
        }
        builds = {}
        for build in ("clif", "llvm_o2"):
            meta_path = args.counts / f"{name}-{build}.meta.json"
            count_path = args.counts / f"{name}-{build}.counts.json"
            if not meta_path.exists() or not count_path.exists():
                print(f"missing counts for {name}/{build}", file=sys.stderr)
                return 1
            meta = load_json(meta_path)
            counts = load_json(count_path)
            stats = build_stats(meta, counts)
            attribution = attribute(meta, counts)
            hot = hot_functions(meta, stats)
            builds[build] = {
                "total_instructions": stats["total"],
                "block_executions": stats["blocks"],
                "functions": {
                    meta["functions"][index]["demangled"]: value
                    for index, value in stats["functions"].items()
                    if value
                },
                "categories": dict(stats["categories"]),
                "stack_loads": stats["stack_loads"],
                "stack_stores": stats["stack_stores"],
                "heap_loads": stats["heap_loads"],
                "heap_stores": stats["heap_stores"],
                "bounds_checks": stats["bounds_checks"],
                "calls": {
                    meta["functions"][index]["demangled"]: value
                    for index, value in stats["calls"].items()
                },
                "attribution": attribution,
                "hot": hot,
                "fuel": [item["fuel"] for item in counts.get("fuel", [])],
                "module": str(meta["source_wasm"]),
            }
        # Correctness gate: the instrumented modules must return exactly the
        # same `run(seed)` and output bytes as LLVM -O2. The originals are
        # byte-identical to the suite history (checked by build.py hashes).
        our_outcomes = [
            (seed["seed"], seed["result"], seed["out_len"], seed["out_hash"])
            for seed in load_json(args.counts / f"{name}-clif.counts.json")["seeds"]
        ]
        llvm_outcomes = [
            (seed["seed"], seed["result"], seed["out_len"], seed["out_hash"])
            for seed in load_json(args.counts / f"{name}-llvm_o2.counts.json")["seeds"]
        ]
        if our_outcomes != llvm_outcomes:
            raise RuntimeError(f"{name}: instrumented outputs differ from LLVM -O2")
        entry["correct"] = True
        entry["outcomes"] = [
            {"seed": seed, "result": result, "out_len": out_len, "out_hash": out_hash}
            for seed, result, out_len, out_hash in our_outcomes
        ]
        entry["builds"] = builds
        entry["instruction_ratio"] = (
            builds["clif"]["total_instructions"] / builds["llvm_o2"]["total_instructions"]
            if builds["llvm_o2"]["total_instructions"]
            else None
        )
        clif = builds["clif"]["attribution"]["instructions"]
        o2 = builds["llvm_o2"]["attribution"]["instructions"]
        entry["attribution"] = {
            cause: {
                "clif": clif[cause],
                "llvm_o2": o2[cause],
                "extra": clif[cause] - o2[cause],
            }
            for cause, _ in CAUSES
        }
        entry["attribution_total_extra"] = (
            builds["clif"]["total_instructions"] - builds["llvm_o2"]["total_instructions"]
        )
        entry["detail"] = {
            "clif": builds["clif"]["attribution"]["detail"],
            "llvm_o2": builds["llvm_o2"]["attribution"]["detail"],
        }
        # Matched hot functions: exact demangled name where LLVM kept the body.
        llvm_functions = builds["llvm_o2"]["functions"]
        for row in builds["clif"]["hot"]:
            row["llvm"] = llvm_functions.get(row["demangled"])
            row["matched"] = row["demangled"] in llvm_functions

        # Static size breakdown and the functions that exist only in our output.
        sizes_path = sizes_root / f"{name}.json"
        subprocess.run(
            [
                tool,
                "sizes",
                "--wasm",
                str(args.modules / "clif" / f"{name}.wasm"),
                "--compare",
                str(args.modules / "llvm_o2" / f"{name}.wasm"),
                "--out",
                str(sizes_path),
            ],
            check=True,
            capture_output=True,
            text=True,
        )
        sizes = load_json(sizes_path)
        segments = all_data_segments(args.modules / "clif" / f"{name}.wasm")
        zero_bytes = sum(item["bytes"] for item in segments if item["zero"] and item["bytes"] >= 1024)
        largest_zero = max(
            (item["bytes"] for item in segments if item["zero"] and item["bytes"] >= 1024),
            default=0,
        )
        dead = 0
        extra_rows = []
        for function in sizes["only_in_first"]:
            dynamic = builds["clif"]["functions"].get(function["demangled"], 0)
            if dynamic == 0:
                dead += function["bytes"]
            extra_rows.append(
                {
                    "bytes": function["bytes"],
                    "dynamic_instructions": dynamic,
                    "category": classify_extra(function["demangled"]),
                    "name": function["demangled"],
                }
            )
        by_category = defaultdict(lambda: {"count": 0, "bytes": 0, "dead_bytes": 0, "live_bytes": 0})
        for row in extra_rows:
            bucket = by_category[row["category"]]
            bucket["count"] += 1
            bucket["bytes"] += row["bytes"]
            if row["dynamic_instructions"] == 0:
                bucket["dead_bytes"] += row["bytes"]
            else:
                bucket["live_bytes"] += row["bytes"]
        entry["size"] = {
            "clif_bytes": sizes["bytes"],
            "llvm_o2_bytes": None,
            "clif_sections": sizes["sections"],
            "llvm_sections": sizes["second_sections"],
            "llvm_data_segments": sizes["second_data_segments"],
            "llvm_data_bytes": sizes["second_data_bytes"],
            "llvm_custom_sections_bytes": sizes["second_custom_sections"],
            "clif_functions_bytes": sizes["first_function_bytes"],
            "llvm_functions_bytes": sizes["second_function_bytes"],
            "data_segments": sizes["data_segments"],
            "data_bytes": sizes["data_bytes"],
            "custom_sections_bytes": sizes["custom_sections"],
            "common_functions": sizes["common_functions"],
            "extra_functions": len(extra_rows),
            "extra_function_bytes": sum(row["bytes"] for row in extra_rows),
            "extra_dead_bytes": dead,
            "extra": sorted(extra_rows, key=lambda row: (-row["bytes"], row["name"]))[:40],
            "extra_by_category": dict(sorted(by_category.items())),
            "data_previews": data_previews(args.modules / "clif" / f"{name}.wasm"),
            "zero_data_bytes": zero_bytes,
            "largest_zero_segment": largest_zero,
        }
        # The second module's total is in the history JSON; keep the static total
        # here too so this file stands on its own.
        entry["size"]["llvm_total_bytes"] = history["programs"][name]["llvm"]["wasm_o2_bytes"]
        entry["size"]["llvm_o2_bytes"] = entry["size"]["llvm_total_bytes"]

        # WAT for the three hottest functions in each build. When the names
        # match, both sides point at the same function body for a direct diff.
        hot_dir = hot_root / name
        for build in ("clif", "llvm_o2"):
            for row in builds[build]["hot"][:3]:
                destination = hot_dir / f"{build}-top{row['rank']}.wat"
                if not wat_for(args.modules / build / f"{name}.wasm", row["name"], destination, tool):
                    destination.write_text(
                        f";; function {row['demangled']} has no body in this build "
                        f"(LLVM inlined or removed it)\n",
                        encoding="utf-8",
                    )
        # A side-by-side diff for every hot function LLVM kept under the same
        # name, plus the common case where LLVM inlined the whole program into
        # `run` while our build keeps a named callee.
        llvm_hot = {row["demangled"]: row for row in builds["llvm_o2"]["hot"]}
        for row in builds["clif"]["hot"][:3]:
            counterpart = llvm_hot.get(row["demangled"])
            if counterpart is None:
                continue
            ours = hot_dir / f"clif-top{row['rank']}.wat"
            theirs = hot_dir / f"llvm_o2-top{counterpart['rank']}.wat"
            (hot_dir / f"diff-top{row['rank']}.md").write_text(
                f"# {row['demangled']}\n\n"
                f"- ours: {row['instructions']:,} executed instructions "
                f"({row['share'] * 100:.1f}% of the program)\n"
                f"- LLVM -O2: {counterpart['instructions']:,} executed instructions "
                f"({counterpart['share'] * 100:.1f}%)\n\n"
                f"## Ours\n\n```wat\n{ours.read_text(encoding='utf-8')}```\n\n"
                f"## LLVM -O2\n\n```wat\n{theirs.read_text(encoding='utf-8')}```\n",
                encoding="utf-8",
            )
        llvm_top = builds["llvm_o2"]["hot"]
        if llvm_top and llvm_top[0]["demangled"] == "run" and llvm_top[0]["share"] > 0.9:
            ours = next(
                (row for row in builds["clif"]["hot"] if is_soft(row["demangled"]) is None),
                None,
            )
            if ours is not None and ours["rank"] > 3:
                destination = hot_dir / "clif-inlined-counterpart.wat"
                wat_for(args.modules / "clif" / f"{name}.wasm", ours["name"], destination, tool)
                ours_file = destination
            elif ours is not None:
                ours_file = hot_dir / f"clif-top{ours['rank']}.wat"
            else:
                ours_file = None
            if ours is not None:
                (hot_dir / "diff-inlined.md").write_text(
                    f"# {ours['demangled']} vs LLVM's inlined `run`\n\n"
                    f"LLVM -O2 inlined the whole program into `run`; this is the "
                    f"hot function that survives as a separate body in our module.\n\n"
                    f"## Ours ({ours['instructions']:,} instructions, "
                    f"{ours['share'] * 100:.1f}%)\n\n```wat\n"
                    f"{ours_file.read_text(encoding='utf-8')}```\n\n"
                    f"## LLVM -O2 `run` ({llvm_top[0]['instructions']:,} instructions, "
                    f"{llvm_top[0]['share'] * 100:.1f}%)\n\n```wat\n"
                    f"{(hot_dir / 'llvm_o2-top1.wat').read_text(encoding='utf-8')}```\n",
                    encoding="utf-8",
                )
        result["programs"][name] = entry
        print(f"analysed {name}", file=sys.stderr)

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    summary = markdown_summary(result)
    (args.out.parent / "SUMMARY.md").write_text(summary, encoding="utf-8")
    print(f"wrote {args.out} and {args.out.parent / 'SUMMARY.md'}", file=sys.stderr)
    return 0


def fmt(value: float | None, digits: int = 2) -> str:
    return "—" if value is None else f"{value:.{digits}f}"


def human(value: int) -> str:
    if value >= 1_000_000_000:
        return f"{value / 1e9:.2f}G"
    if value >= 1_000_000:
        return f"{value / 1e6:.2f}M"
    if value >= 1_000:
        return f"{value / 1e3:.1f}k"
    return str(value)


def markdown_summary(result: dict) -> str:
    lines = [
        f"# Gap attribution summary ({result['machine']['kind']})",
        "",
        f"History: `{result['history']}` — commit `{result['commit']['sha'][:12]}`",
        "",
        "| program | exec ours/O2 | instr ours | instr O2 | instr ratio | top cause (extra) |",
        "| --- | ---: | ---: | ---: | ---: | --- |",
    ]
    for name, entry in result["programs"].items():
        causes = sorted(
            ((cause, value["extra"]) for cause, value in entry["attribution"].items()),
            key=lambda item: -item[1],
        )
        top = causes[0]
        lines.append(
            f"| {name} | {fmt(entry['exec_ms']['ratio'])}× | "
            f"{human(entry['builds']['clif']['total_instructions'])} | "
            f"{human(entry['builds']['llvm_o2']['total_instructions'])} | "
            f"{fmt(entry['instruction_ratio'])}× | "
            f"{CAUSE_LABEL[top[0]]} +{human(top[1])} |"
        )
    lines.append("")
    return "\n".join(lines)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:  # noqa: BLE001 - top-level CLI boundary
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
