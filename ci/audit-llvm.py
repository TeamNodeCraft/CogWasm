#!/usr/bin/env python3
"""Audit rustc.wasm for LLVM symbols/imports.

`stage-c-host inspect` emits the wasm imports, exports and function names from
the name section. This script fails the CI job on the things the pure-Rust path
promises not to have:

  * any import whose module or name mentions LLVM (case-insensitive);
  * any function name containing `rustc_codegen_llvm`, `rustc_llvm` or an
    uppercase `LLVM` prefix/symbol;
  * the marker strings `LLVMInitialize`, `LLVMRust`, `rustc_codegen_llvm` or
    `rustc_llvm` anywhere in the binary.

Lowercase `llvm_*` function names are cg_clif's own intrinsics shims and are
expected; `libLLVM` can appear once in a linker search-path table. Both are
reported but are not failures.

Usage: audit-llvm.py <inspect.json> <rustc.wasm>
"""
import json
import re
import sys

FAIL_NAME = re.compile(r"rustc_codegen_llvm|rustc_llvm|LLVM")
FAIL_STRINGS = (
    b"LLVMInitialize",
    b"LLVMRust",
    b"rustc_codegen_llvm",
    b"rustc_llvm",
)


def main() -> int:
    if len(sys.argv) != 3:
        sys.exit("usage: audit-llvm.py <inspect.json> <rustc.wasm>")
    inspect_path, wasm_path = sys.argv[1], sys.argv[2]
    with open(inspect_path, "r", encoding="utf-8") as f:
        info = json.load(f)
    with open(wasm_path, "rb") as f:
        raw = f.read()

    violations = []
    for imp in info.get("llvm_imports", []):
        violations.append(f"LLVM import: {imp}")

    bad_names = [n for n in info.get("llvm_function_names", []) if FAIL_NAME.search(n)]
    for name in bad_names:
        violations.append(f"forbidden function name: {name}")

    for marker in FAIL_STRINGS:
        if marker in raw:
            violations.append(f"forbidden string in binary: {marker.decode()}")

    imports = {tuple(i) for i in info.get("imports", [])}
    if ("playground", "spawn") not in imports:
        violations.append("missing playground::spawn import")
    if ("wasi", "thread-spawn") in imports or ("wasi", "thread_spawn") in imports:
        violations.append("wasi thread-spawn import present")
    if ("env", "memory") in imports:
        violations.append("env.memory import present")

    llvm_names = info.get("llvm_function_names", [])
    cg_clif_names = [n for n in llvm_names if n not in bad_names and "llvm" in n.lower()]
    libllvm = raw.count(b"libLLVM")
    print(
        f"LLVM audit: imports={len(info.get('llvm_imports', []))} "
        f"forbidden_names={len(bad_names)} "
        f"cg_clif_llvm_intrinsics={len(cg_clif_names)} "
        f"raw_llvm_occurrences={info.get('llvm_raw_occurrences', 0)} "
        f"libLLVM_strings={libllvm} "
        f"bytes={len(raw)}"
    )

    if violations:
        for v in violations:
            print(f"::error title=LLVM-in-rustc.wasm audit::{v}")
        return 1

    print(
        "::notice title=LLVM audit::no LLVM imports, no "
        "rustc_codegen_llvm/rustc_llvm/LLVM symbol names; "
        f"{len(cg_clif_names)} cg_clif llvm_* intrinsic names are allowed, "
        f"{libllvm} libLLVM path string(s) allowed"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
