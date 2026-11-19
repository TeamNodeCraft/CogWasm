# Skill: measuring performance

Every speed claim needs a number. This is how to get one that means something.

## Rules

- **Median of at least 5 runs**, after one warm-up run. Report the median and
  the spread (min/max).
- **Same machine, same build profile, same pinned revisions** for before and
  after. Write the machine down (CPU, OS) with the results.
- **Change one thing at a time.**
- **Check correctness in the same run**: the difftest corpus must still pass.
  A faster wrong answer is a regression.
- Note that CI runners are slower and noisier than a developer machine; compare
  CI with CI and local with local.

## What we measure, and with what

| Question | Tool | Where |
| --- | --- | --- |
| How fast is generated code? | The product suite runs kernels plus the A.2 maths (e) and host-call (f) programs against LLVM `-O0` / `-O2` | `bench/run.sh`, `experiments/cranelift-wasm/stage-b/a2/` |
| How fast is the compiler? | The product suite: module load (cold, cached, precompiled), instantiate, compile inside `rustc.wasm` (warm and incremental, with phases), link, edit → result | `bench/run.sh --quick` / `--full` ([README](../../bench/README.md)) |
| How big is the output? | Total/code/data bytes from the benchmark host | `bench/run.sh` |
| Is it still correct? | Differential tests | `experiments/cranelift-wasm/difftest/` |

## Reporting

Put results in `experiments/cranelift-wasm/results.md` (or the stage's own
`results.md`) as a table: what changed, before, after, ratio, machine, commit.
If the README's Status numbers change, update them in the same commit.

For product-level numbers use the suite and keep the JSON:

```sh
bench/run.sh --full --out bench/history/<machine>-<commit>.json
bench/compare.py bench/history/<before>.json bench/history/<after>.json
```

`compare.py` flags only changes larger than the run-to-run spread and refuses
to compare different machine kinds unless forced. Commit meaningful points in
`bench/history/`; do not compare CI numbers with a developer machine.

## Profiling generated code

- Look at the wasm first: `wasm-tools print` and search the hot function for
  redundant `local.get`/`local.set` pairs, loads and stores of the same
  address, calls that should have been inlined, and bounds checks in loops.
- Compare with LLVM's output for the same function to see what it removed.
- For native profiles of Wasmtime-compiled code, use Wasmtime's `--profile`
  support (perf/jitdump on Linux, or the guest profiler) and read the hot spots
  back to wasm functions.
