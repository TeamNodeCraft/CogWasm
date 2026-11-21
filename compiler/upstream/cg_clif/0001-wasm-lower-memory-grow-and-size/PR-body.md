# wasm: lower memory.grow and memory.size intrinsics

> **Draft research note, not submission text.** A human owns any future
> upstream contribution and must write the issue/PR in their own words under
> the project's current AI policy. See `../../README.md` and
> `CogWasm/CONTRIBUTING.md`.

## Problem

`core::arch::wasm32::{memory_grow,memory_size}` resolve to
`InstanceKind::LlvmIntrinsic` with the LLVM names
`llvm.wasm.memory.grow.i32` and `llvm.wasm.memory.size.i32`.

cg_clif's generic LLVM-intrinsic fallback only warns and replaces the call
with a trap. A wasm allocator that grows its heap therefore compiles but
traps at runtime. There is no Cranelift opcode for either instruction.

## Fix

Intercept both intrinsics in `src/intrinsics/llvm.rs` before the generic
fallback and lower them to direct calls to two names reserved by clif-to-wasm:

| wasm | reserved call | signature |
|---|---|---|
| `memory.grow` | `__clif_to_wasm_memory_grow` | `(i32) -> i32` |
| `memory.size` | `__clif_to_wasm_memory_size` | `() -> i32` |

clif-to-wasm recognises those test-case names and emits the real
`memory.grow` / `memory.size` instructions for
`ModuleLayout::memory_index`; see the paired clif-to-wasm patch.

Both LLVM intrinsics take the memory index as their first argument. stdarch
statically requires it to be zero, so only the value operand is forwarded.
The call target uses `ExternalName::TestCase`, which keeps the helper out of
the object's symbol and import tables.

Add `example/wasm-memory-intrinsics.rs`, a compile-only regression that calls
both intrinsics, and enable `aot.wasm_memory_intrinsics` in `config.txt` so a
wasm-target run executes it. The test also asserts that no
"unsupported llvm intrinsic" warning was emitted, because the fallback only
warns and still compiles.

## Tests

Against a scratch tree containing cg_clif `4f78fd3` + the wasm backend series
+ this patch + the paired clif-to-wasm patch (nightly-2026-10-19):

```text
TARGET_TUPLE=wasm32-wasip1 ./test.sh
...
[AOT] f128_wasm_abi
[AOT] wasm_memory_intrinsics
...
EXIT=0
real 79.62
```

The same run built the wasm32-wasip1 sysroot with cg_clif. Only the two
compile-only cases were enabled in the scratch `config.txt`.

Direct backend compile checks for both wasm targets exit 0. Negative check:
without this patch the example still compiles but emits
`unsupported llvm intrinsic llvm.wasm.memory.grow.i32; replacing with trap`
(and the same for `size`), so the new assertion fails. That confirms the test
covers the fix.

The paired clif-to-wasm patch's `tests/memory.rs` covers the actual runtime
semantics: instruction emission, grow/size values, new pages being real
memory, wrong signatures and memory64 rejection.

`rustfmt --check` on the touched files exits 0. Full details:
`test-results.txt`.

## How found

While moving a wasm plugin's allocator off a fixed `.bss` heap. The allocator
called `core::arch::wasm32::memory_grow`; cg_clif emitted a trap for it, and
the plugin failed at the first growth.

## Upstream status

No equivalent change exists on current `main`:

* `src/intrinsics/llvm.rs` at
  `a4af0a9fd1ce26e03364c07ab0c482d084fa97fc` has no
  `llvm.wasm.memory.grow`/`size` arm; it falls through to the generic
  unsupported-intrinsic warning.
* `git log -S'llvm.wasm.memory'` finds no commit adding it.
* GitHub issue/PR searches for `memory.grow` in cg_clif return no equivalent.
  The paired clif-to-wasm repository also has no `MemoryGrow`/`memory_grow`
  history or issue.

Recommended disposition: keep. This is a new change, not a duplicate.

## Conflicts

Rebased from `4f78fd3e5f2ce8a688865b4a6197e6d33211c09c` to
`a4af0a9fd1ce26e03364c07ab0c482d084fa97fc`:

* `src/intrinsics/llvm.rs` is unchanged upstream; the code hunk applies as-is
  and adds the arm before the generic `_ =>` fallback.
* `build_system/tests.rs` and `config.txt` gained unrelated upstream test
  entries (`aot.tls_conflicting_declarations`). The new test insertion is
  after `aot.float-minmax-pass`; the only effect is a line offset. Default
  3-line context picks up the new TLS line in `config.txt`, so applying this
  patch to the older pinned tree needs `git apply -C1`; all code hunks apply.
* `rustfmt` made no changes.

* The f128 and memory-intrinsics patches both add a test entry immediately
  after `aot.float-minmax-pass` in `build_system/tests.rs` and `config.txt`.
  They are independent PRs; if they are stacked, the second one needs a
  trivial rebase of those two lines. The code changes do not conflict.

## Licence

MIT OR Apache-2.0, as a contribution to the Rust project.

## Notes

* **This is a two-PR series with clif-to-wasm.** The cg_clif patch calls
  `clif_to_wasm::WasmIntrinsic` and the reserved helper names, so it cannot
  compile against plain upstream cg_clif, which has no `clif-to-wasm`
  dependency. The refreshed patch still applies cleanly to upstream `main`
  (`git am`), but is only reachable after the cg_clif wasm backend and a
  clif-to-wasm release with `WasmIntrinsic` land. Land clif-to-wasm first,
  then this.
* The helper names are a private contract between the two crates. If
  clif-to-wasm prefers different names or a different public API (for example
  a `ModuleBuilder` option rather than reserved test-case names), the two
  patches need to be updated together.
* `memory64` is rejected with an "unsupported ... 64-bit memory" diagnostic;
  only memory32 is implemented. Tail calls to the helpers are also rejected.
* Best venue: cg_clif PR, coordinated with the clif-to-wasm patch in
  `#wg-cranelift`; do not open this before the wasm backend series is
  accepted.
