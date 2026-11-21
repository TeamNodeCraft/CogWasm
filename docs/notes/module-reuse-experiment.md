# Module-level object reuse on `generated_stress`

Question, for the tiering/graph design: if the pipeline kept content-addressed
artifacts per codegen unit, how much of a rebuild survives a one-line edit?

## Method

`bench/scripts/gen_stress.py` was changed from one flat file to **8 inline
modules**, one per generated type (`part0` … `part7`), with the same 1,024
`#[inline(never)]` functions, the same function-pointer table and the same
`body`. The program returns identical `run(seed)` values to the flat version
for all four suite seeds.

Each build is linked through a wrapper around Wild that logs the linker
arguments and copies every object rustc passes. After a one-line edit the
object hashes are compared. Both builds of a pair use the **same source path
and work directory**: the root object embeds the source path, so different
temp directories make every object look changed.

## Result

| layout | objects | edit | objects identical | bytes identical |
| --- | ---: | --- | ---: | ---: |
| flat (one module) | 3 | `BENCH_EDIT_TWEAK` 0 -> 1 | 2 / 3 | 1,050 / 175,299 (**0.6%**) |
| 8 inline modules | 11 | one function in `part0` | 10 / 11 | 228,662 / 246,540 (**92.7%**) |

With the flat source rustc emits one 174 KB codegen unit; `-Ccodegen-units`
cannot split a single module (16 and 64 produce the same object). With one
module per type the edit changes one 17.8 KB object and leaves the 102 KB root
unit and the other seven modules byte-identical.

## Cost of the split

| build | compile | link | wasm |
| --- | ---: | ---: | ---: |
| flat, opt-level 2 | 378.2 ms | 23.1 ms | 145,320 B |
| flat, `-C incremental` cold | 109.9 ms | 26.0 ms | 154,847 B |
| modules, opt-level 2 | 383.2 ms | 56.6 ms | 153,576 B |
| modules, `-C incremental` cold | 123.8 ms | 64.7 ms | — |
| modules, incremental after the edit | 129.8 ms | 64.6 ms | — |

Module granularity costs about 5.7% wasm size and 30 ms of link time; compile
time is unchanged. `-C incremental` is about 2x faster than a full build, but
the edit is not cheaper than its own cold build and the link is slower — rustc
reuses the dep graph, not a set of unchanged CGU objects the host can see.

## Conclusion

Module-level artifacts *are* reusable: 92.7% of object bytes survive a module
edit, against 0.6% with the flat layout. Turning that into a host-side graph
needs a compiler boundary that emits and consumes per-codegen-unit objects
without re-running codegen for the whole crate. rustc's incremental mode is
the closest thing today and does not expose that boundary. Until it exists,
the available levers are the tier slice (incremental vs full, opt-level) and
crate/module-level caching of whole artifacts.

The benchmark change is kept: a multi-module crate is closer to the real
programs the compiler targets, and it is what makes this experiment possible.
