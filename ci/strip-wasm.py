#!/usr/bin/env python3
"""Copy a wasm module without its custom sections.

    strip-wasm.py <in.wasm> <out.wasm>

Custom sections (id 0) hold debug info, the name section and producer data;
nothing at run time reads them. Dropping them takes rustc.wasm from 137 MiB to
about 90 MiB for the in-browser REPL's download. Every other section is copied
byte for byte, in order.
"""

from __future__ import annotations

import sys
from pathlib import Path


def read_leb_u32(data: bytes, position: int) -> tuple[int, int]:
    """Decode an unsigned LEB128 value; return it and the next position."""
    result = 0
    shift = 0
    while True:
        byte = data[position]
        position += 1
        result |= (byte & 0x7F) << shift
        if byte < 0x80:
            return result, position
        shift += 7
        if shift > 35:
            raise ValueError("LEB128 value longer than 5 bytes")


def strip_custom_sections(data: bytes) -> bytes:
    """Return the module with every custom section removed."""
    if data[:4] != b"\0asm":
        raise ValueError("not a wasm module")
    output = bytearray(data[:8])
    position = 8
    while position < len(data):
        start = position
        section_id = data[position]
        size, body = read_leb_u32(data, position + 1)
        end = body + size
        if end > len(data):
            raise ValueError(f"section {section_id} at {start} runs past the end")
        if section_id != 0:
            output += data[start:end]
        position = end
    return bytes(output)


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print(__doc__.strip(), file=sys.stderr)
        return 2
    source, destination = (Path(arg) for arg in argv)
    stripped = strip_custom_sections(source.read_bytes())
    destination.write_bytes(stripped)
    print(f"{source.name}: {source.stat().st_size} -> {len(stripped)} bytes")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
