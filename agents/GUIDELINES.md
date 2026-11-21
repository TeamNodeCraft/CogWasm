# Guidelines

Conventions for every change to CogWasm, by people and AI agents alike.
The [review scan](skills/anti-pattern-review.md) checks the mechanical parts;
reviewers check the rest.

These rules apply to **our own code**: hosts, harnesses, difftest, scripts,
CI and docs. Patch files that modify an upstream project follow **that
project's** style and lints instead (see [CONTRIBUTING.md](../CONTRIBUTING.md)).

## Fix root causes

- Find the shared source of a bug and fix it there. No special cases, per-input
  exceptions, magic offsets or constants, or one-off branches.
- No `sleep` or retry loops to hide an ordering or invalidation problem.
- No silent fallbacks: if something can't be done, return a clear error.
- If a genuine exception is unavoidable, write down why next to the code and
  add a test that covers it.

## Rust style

- `cargo fmt` with the default style. Edition 2024 for new crates.
- **Nesting: at most 5 levels** inside a function (`clippy::excessive_nesting`,
  threshold in [`clippy.toml`](../clippy.toml)). Prefer early returns, `let ...
  else`, small helper functions and iterator chains.
- **Functions: at most about 100 lines** (`clippy::too_many_lines`) and **at
  most 7 arguments** (`clippy::too_many_arguments`); group related arguments
  into a struct.
- **Lints:** code must build clean under `clippy::all` and `clippy::pedantic`
  with the small allow-list in [`agents/scripts/review-scan.sh`](scripts/review-scan.sh).
  To silence a lint, use `#[expect(clippy::name, reason = "...")]` at the
  narrowest scope, never a crate-wide `allow` and never without a reason.
- Names say what things are. No abbreviations a newcomer would have to guess.
- Comments explain *why*, not *what*. Public items get doc comments.

## Errors and panics

- Library code returns `Result` with a typed error (an `enum` with
  `std::error::Error`); binaries may use a single top-level error type.
- No `unwrap()` / `expect()` on anything that comes from outside the program
  (files, the network, user input, wasm modules, environment). `expect` is
  fine for true invariants, with a message saying what the invariant is.
- No `todo!()`, `unimplemented!()` or `dbg!()` in committed code.
- Never discard an error silently (`let _ = fallible();`). Handle it, return
  it, or log it with the reason it's safe to continue.
- Error messages say what failed, what was expected and what to do next.

## `unsafe`

- Avoid it. When it's needed, keep the block as small as possible and put a
  `// SAFETY:` comment on it explaining why every requirement holds
  (`clippy::undocumented_unsafe_blocks`).
- Inside `unsafe fn`, still use explicit `unsafe {}` blocks
  (`unsafe_op_in_unsafe_fn`).
- Every `unsafe` block needs a test that exercises it.

## Performance

- **Measure, don't guess.** A performance change comes with before/after
  numbers (median of at least 5 runs, same machine, same build profile). See
  [skills/measuring-performance.md](skills/measuring-performance.md).
- **Correctness first:** the differential tests must stay green. A faster
  pipeline that changes results is a bug.
- Prefer removing work to speeding it up: cache, reuse, avoid copies, avoid
  allocation in hot loops.
- Write hot loops so they can be vectorised. See
  [skills/vectorisation.md](skills/vectorisation.md).

## Tests

- Every bug fix has a regression test that **fails before** the fix and
  **passes after** it. Say in the commit how you checked that it failed.
- Every new feature has tests for the normal case and the edge cases.
- Programs that exercise code generation go into the
  [difftest corpus](../difftest/), so LLVM and our
  pipeline are compared on them.
- Never delete, weaken, `#[ignore]` or `cfg`-skip a test to make CI pass.
- Tests are deterministic: no clocks, no unseeded randomness, no network.

## Builds, CI and dependencies

- Pin everything: exact crate versions in `Cargo.lock`, exact git revisions for
  upstream checkouts, exact versions for downloaded tools.
- Downloads only from official sources (the project's git host, crates.io,
  static.rust-lang.org, GitHub releases of the project itself).
- One build script runs on a developer's machine and in CI; fix portability in
  the script, not with separate per-OS logic.
- CI never passes by matching a failure message. If a step is expected to fail,
  fix the cause.
- Shell scripts start with `set -euo pipefail`, quote every expansion, and
  guard destructive paths (`rm -rf "${dir:?}"`).

## Commits

- One logical change per commit. The message says what changed and why; a bug
  fix also says the root cause and how it was verified.
- Commits made with substantial AI help say so in a trailer
  (`Co-Authored-By:` naming the tool).
- Don't commit build outputs, toolchains or large binaries.
- Experiments that only touch docs can use `[skip ci]`; anything that changes
  the build or tests must let CI run.

## Patches to upstream projects

- Small, one change each, with a test, in the upstream project's style.
- Stored here as patch files, licensed as the project they modify, with a
  header saying they were changed here.
- Not sent upstream without a decision (see [CONTRIBUTING.md](../CONTRIBUTING.md)).

## Review checklist

Before asking for review (or before an agent hands work back):

- [ ] `agents/scripts/review-scan.sh` is clean for the files you touched.
- [ ] The root cause is fixed, not a symptom.
- [ ] A test fails before the change and passes after.
- [ ] Difftest is green.
- [ ] Performance claims have numbers.
- [ ] No new `unwrap()` on outside input, no new `unsafe` without `// SAFETY:`.
- [ ] Docs (README, ROADMAP, this file) are updated if behaviour or goals changed.
