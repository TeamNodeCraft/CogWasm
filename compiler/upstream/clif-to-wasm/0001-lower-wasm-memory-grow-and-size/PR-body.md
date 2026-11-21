# Lower wasm memory.grow and memory.size calls

> **Draft research note, not submission text.** A human owns any future
> upstream contribution and must write the issue/PR in their own words under
> the project's current AI policy. See `../../README.md` and
> `CogWasm/CONTRIBUTING.md`.

## Problem

Cranelift's IR has no opcode for WebAssembly's `memory.grow` or `memory.size`.
A frontend that targets wasm through clif-to-wasm therefore cannot express
either instruction. cg_clif needs both to implement
`core::arch::wasm32::{memory_grow,memory_size}` and a growable allocator, and
`compiler_builtins`/`alloc` need them for a heap that can grow.

## Fix

Add `WasmIntrinsic`, a small public enum for wasm instructions that have no
Cranelift opcode. A frontend selects one by making a direct call to a reserved
`ExternalName::TestCase` symbol:

| wasm | reserved call | signature |
|---|---|---|
| `memory.grow` | `__clif_to_wasm_memory_grow` | `(i32) -> i32` |
| `memory.size` | `__clif_to_wasm_memory_size` | `() -> i32` |

`Emitter::lower` recognises a `call`/`return_call` to one of those names,
validates the signature and the memory type, and emits
`Instruction::MemoryGrow`/`MemorySize` for
`ModuleLayout::memory_index` instead of a call. The helper never appears in
the object's symbol or import tables because it is a test-case name.

Restrictions are explicit and diagnosed as unsupported:

* the call must match the exact signature above;
* it must not be a tail call (`return_call` cannot return the instruction's
  result);
* the memory must be 32-bit (memory64 is rejected).

The public API is `WasmIntrinsic::{symbol_name, from_symbol_name}` plus a
`#[non_exhaustive]` enum, so more instructions can be added later without
breaking callers.

## Tests

`tests/memory.rs` compiles CLIF that calls both helpers, parses the emitted
wasm and checks that the bodies contain real `memory.size`/`memory.grow`
instructions and no calls. It instantiates the module in Wasmtime and checks:

* `memory.size` returns the initial page count;
* `memory.grow` returns the previous size and the memory actually grows;
* a subsequent `memory.size` observes the new size;
* newly grown pages are readable and writable.

It also checks that a wrong signature is reported as unsupported and that
memory64 is rejected. A separate test in the same file checks the rejection
diagnostics.

Run locally with Rust 1.99.1:

```text
cargo test --all-features
...
test wrong_signature_is_rejected ... ok
test memory64_is_rejected ... ok
test memory_size_and_grow_instructions ... ok
test result: ok. 3 passed; 0 failed
...
all doctests passed
EXIT=0
real 95.46
```

`cargo fmt --check` passes with nightly rustfmt. Full log:
`test-results.txt`.

## How found

While implementing a growable wasm allocator in cg_clif. clif-to-wasm had no
way to express `memory.grow`, and adding an ISA opcode to Cranelift would have
been a much larger change. Reserved helper calls keep the frontend and backend
in sync without touching Cranelift.

## Upstream status

Upstream `main` is exactly the pinned revision
`0e4e0458f5a190a7fcdbb2298d9ad780678c44bc` (2026-11-14, "Cargo upgrade"), so
the patch applies cleanly with no rebase.

No equivalent change exists:

* `src/lower.rs` has no `MemoryGrow`/`MemorySize` handling at this revision
  (the only `Opcode::Call` arm forwards to `Env`).
* `git log --all -S'MemoryGrow'` and `-S'memory_grow'` return no commits.
* GitHub issue/PR search for `memory.grow` in
  `ryancbuckley/clif-to-wasm` returns no results.

Recommended disposition: keep. This is a new change, not a duplicate.

## Conflicts

None. Upstream `main` had not moved since the pinned `0e4e0458`, so the
original patch applied with `git am` unchanged, apart from a test-comment typo
fix and a licence line.

`git apply --check` against a clean snapshot of the pinned revision passes.

## Licence

Apache-2.0 WITH LLVM-exception, as a contribution to clif-to-wasm.

## Notes

* **Paired API change.** cg_clif's companion patch calls these reserved names
  and uses `WasmIntrinsic`; land clif-to-wasm first (or release a version with
  it) and then bump cg_clif. The two patches should be reviewed together.
* **Design choice.** Using `ExternalName::TestCase` avoids adding a new
  Cranelift instruction or a new environment callback, and keeps the helper
  out of the symbol table. The trade-off is that the names are a private
  contract rather than a typed API on `ModuleBuilder`. If the project prefers
  a `ModuleBuilder`/`Config` option, the lowering and the cg_clif call site
  need to change together.
* **Memory index.** Both instructions use `Config::module.memory_index`, which
  is the memory the rest of the module uses; multi-memory is not otherwise
  supported by this crate today.
* **Venue.** clif-to-wasm has no CONTRIBUTING file or CI workflow; the
  project convention is `cargo test --all-features` and `cargo fmt --check`.
  A PR against the repository, with a note in `#wg-cranelift`, is the natural
  venue; coordinate the merge order with the cg_clif patch.
