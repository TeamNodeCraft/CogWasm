# Contributing to CogWasm

CogWasm builds a pure-Rust compiler that runs as WebAssembly (see the
[README](README.md)). This file explains how work happens here, how we use AI
tools, and how we relate to the upstream projects we build on.

## How we work: changes live here first

We do **not** send changes to upstream projects while the compiler is still
taking shape. Every fix we need is made here, on the copies we build:

- **Pinned upstreams.** rustc, `rustc_codegen_cranelift`, clif-to-wasm, Wild
  and the rustc-to-wasm patch series are checked out at exact revisions by the
  build scripts (see
  [`docs/pipeline.md`](docs/pipeline.md)).
- **Our changes are patch files** applied on top of those revisions, kept in
  this repository next to the build that uses them (for example
  `compiler/patches/`, or the manual-only LLVM series
  in `legacy/llvm/patches/`). Each patch is small, does one
  thing, has a test, and says in its header that it was changed here.
- **CI builds and tests the patched stack** on every relevant push, so a patch
  that stops applying or breaks something is caught here, not upstream.
- **Upstream-ready drafts** (a patch rebased on the project's current branch,
  its tests, and notes for a pull request) may be kept under
  `compiler/upstream/`. They are drafts only. Nobody opens a
  pull request, issue or discussion upstream without a decision to do so.

The aim is that once the whole pipeline works well, the upstream projects can
look at what we did and take whatever is useful to them, from working code
with tests, in their own way and at their own pace. Until then we keep the
burden on ourselves, not on their maintainers.

## Patch files and licences

A patch file is licensed under the licence of the project it modifies, not
under this repository's MIT OR Apache-2.0 option. The per-project list is in
the README's [Patch files](README.md#patch-files) section. Keep each patch's
header recording that it was changed by the contributors to this repository.

## Using AI tools

AI tools are allowed here, with a human in charge:

- A person reviews and understands every change before it is committed, and
  can explain it. If something works but you don't know why, say so in the
  commit or pull request.
- Keep changes small and focused, with tests that fail before the change and
  pass after it.
- Never weaken or delete a test, or mark it ignored, just to make CI pass.
- Commits made with substantial AI help say so in a trailer (this repository
  uses `Co-Authored-By:` lines naming the tool). That trailer is the only AI
  disclosure in this repository: notes, patch files, patch headers, code
  comments and commit messages carry no LLM-provenance disclaimers. The
  upstream AI policies below apply only to changes we send upstream, and we
  are not upstreaming.

## If we do contribute upstream later

The upstream projects have their own rules, and they are stricter than ours,
mostly about AI-generated text and about the contributor understanding the
change. Before any upstream contribution, re-read the project's current
policy; at the time of writing:

| Project | AI policy (as of November 2026) |
| --- | --- |
| [rust-lang/rust](https://github.com/rust-lang/rust) (rustc) | An LLM policy adopted by the compiler, library, types, rustdoc and bootstrap teams (August 2026). LLM-written text must not be used for pull request descriptions, issues, comments, documentation, non-trivial code comments or diagnostics. LLM-generated code is allowed only on a controlled experimental path: the author and reviewer must fully understand it, it is strongly discouraged for soundness-critical code, reviewers mark such pull requests `ai-assisted`, and new ones are paused if they pass half of merged pull requests over six weeks. AI review is advisory only. |
| [rustc_codegen_cranelift](https://github.com/rust-lang/rustc_codegen_cranelift) | No separate AI policy found. It is a Rust project synced into rust-lang/rust; expect the same rules. |
| [clif-to-wasm](https://github.com/ryancbuckley/clif-to-wasm) | No contributing guide or AI policy found; a single maintainer. Open an issue first. |
| [Wild](https://github.com/wild-linker/wild/blob/main/CONTRIBUTING.md) | AI help is fine with a human in the loop. Pull requests and issues must be written by you, short and concise; no AI agents talking to maintainers and no pasted AI replies to review comments; don't send changes you don't understand, and start small. |
| [LLVM](https://llvm.googlesource.com/llvm-project/+/278d9f76594de1ad0fac669468bf083ad6616a0f/llvm/docs/AIToolPolicy.md) (build-time only for us) | Any tools, but a human reviews all generated code and text and is accountable for it. |

So, when the time comes:

1. **A person owns each contribution.** They understand every line, can defend
   it in review and answer questions themselves.
2. **Write the pull request, issue and review replies yourself**, briefly. Draft
   notes in `upstream/` are background for that person, not text to paste.
3. **No AI agent posts, comments or replies upstream** on anyone's behalf.
4. **Disclose AI assistance** the way each project asks, for example an
   `Assisted-by:` trailer rather than listing a tool as a co-author.
5. **Start with a conversation** (Zulip for Rust, an issue for smaller
   projects), and send one small change at a time.

Sources for the policy summary: the Rust project's LLM policy (reported by
[Linuxiac](https://linuxiac.com/rust-adopts-official-policy-for-ai-generated-contributions/)
and [It's FOSS](https://itsfoss.com/news/rust-code-repo-ai-policy/)), the
[rustc dev guide](https://rustc-dev-guide.rust-lang.org/llm-guidance/reviewing.html),
[Wild's CONTRIBUTING.md](https://github.com/wild-linker/wild/blob/main/CONTRIBUTING.md)
and [LLVM's AI tool policy](https://llvm.googlesource.com/llvm-project/+/278d9f76594de1ad0fac669468bf083ad6616a0f/llvm/docs/AIToolPolicy.md).
Policies change; check the project's own documents before contributing.
