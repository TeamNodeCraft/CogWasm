#!/usr/bin/env bash
# Anti-pattern review scan for CogWasm's own code (agents/skills/anti-pattern-review.md).
#
#   agents/scripts/review-scan.sh            scan everything we own
#   agents/scripts/review-scan.sh --changed  only crates/files changed against origin/main
#   agents/scripts/review-scan.sh --strict   also fail on fmt/clippy/shellcheck/ruff findings
#
# ast-grep error-level rules always fail the scan. Other tools report findings
# and fail only with --strict, until existing findings are fixed (see
# agents/ROADMAP.md, Quality work). A tool that isn't installed is skipped with
# a message, never silently.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir:?}"' EXIT

changed=0
strict=0
for arg in "$@"; do
  case "$arg" in
    --changed) changed=1 ;;
    --strict) strict=1 ;;
    *) echo "unknown option: $arg" >&2; exit 2 ;;
  esac
done

failed=0      # findings that always fail
soft=0        # findings that fail only with --strict
blocking_reasons=()  # one line per blocking failure, repeated in the summary
note() { printf '\n== %s\n' "$*"; }
blocking() { blocking_reasons+=("$*"); printf '   BLOCKING: %s\n' "$*"; }
# Read lines from stdin into the named array (bash 3.2 has no mapfile).
# Callers assign the array first so a reader that isn't shellcheck-aware can
# still see the assignment: `eval` is the only way to name an array in 3.2.
read_into() { local _name="$1" _line; while IFS= read -r _line; do
  [[ -n "$_line" ]] && eval "$_name+=(\"\$_line\")"; done; }
skip() { printf '   skipped: %s\n' "$*"; }

# Our own code: everything tracked except patches for upstream projects,
# upstream drafts, vendored third-party sources (which follow their own style
# and licences), the legacy LLVM series and the rule fixtures.
own_files() {
  git ls-files -- "$@" \
    | grep -v -E '^(legacy/llvm/patches/|.*/upstream/|.*/vendor/|.*/toolchain/|agents/scripts/rule-fixtures/)' || true
}
if [[ $changed -eq 1 ]]; then
  base="$(git merge-base HEAD origin/main 2>/dev/null || echo HEAD)"
  # Materialize the list once, then read it as a file. Piping it into `grep -q`
  # let grep exit at its first match, the writer die of SIGPIPE, and `pipefail`
  # turn the inverted test into "not changed"; every host crate was skipped.
  changed_file="$tmpdir/changed.txt"
  { git diff --name-only "$base" -- . ; git ls-files --others --exclude-standard; } \
    >"$changed_file"
  in_scope() { grep -F -x -f "$changed_file" || true; }
else
  in_scope() { cat; }
fi

# True when the changed-file list contains a path under crate directory $1.
# $2 is the list to search; it defaults to the list --changed built.
dir_changed() {
  local dir="$1" list="${2:-$changed_file}"
  grep -q "^$dir/" "$list"
}

if [[ $changed -eq 1 ]]; then
  # The scope filter must find a crate with one changed file and ignore one
  # with none. This guards the matcher itself; the SIGPIPE failure came from
  # the pipe that used to feed it.
  note "changed-scope self-test"
  scope_fixture="$tmpdir/scope.txt"
  printf 'bench/gap/src/main.rs\nagents/README.md\n' >"$scope_fixture"
  if ! dir_changed bench/gap "$scope_fixture" \
    || dir_changed bench/host "$scope_fixture" \
    || ! dir_changed agents "$scope_fixture"; then
    blocking "the --changed scope filter does not match changed crate directories"
    failed=1
  else
    echo "   changed crates are found"
  fi
fi

# 0. The machine-readable upstream/patch manifest still matches the tree.
note "upstream manifest"
if ! command -v python3 >/dev/null; then
  blocking "python3 not installed; cannot check compiler/upstreams.toml"
  failed=1
elif ! python3 "$root/compiler/scripts/upstream.py" check; then
  blocking "compiler/upstreams.toml does not match compiler/patches/ (run python3 compiler/scripts/upstream.py check)"
  failed=1
fi

# 1. The project rules still match their known examples.
note "ast-grep rule self-test"
if command -v ast-grep >/dev/null; then
  expected="allow-needs-expect no-dbg-macro no-ignored-tests no-process-exit-in-library no-sleep-to-fix-ordering no-todo-in-code"
  self_test_json="$tmpdir/self-test.json"
  self_test_err="$tmpdir/self-test.err"
  status=0
  ast-grep scan --json=compact agents/scripts/rule-fixtures/bad.rs \
    >"$self_test_json" 2>"$self_test_err" || status=$?
  found=""
  have_python=0
  if command -v python3 >/dev/null; then
    have_python=1
    found="$(python3 -I -c 'import json,sys; print(" ".join(sorted({x["ruleId"] for x in json.load(sys.stdin)})))' \
      <"$self_test_json" 2>/dev/null || true)"
  fi
  if [[ $have_python -eq 0 ]]; then
    blocking "python3 not installed; cannot verify that the ast-grep rules match bad.rs"
    failed=1
  elif [[ -z "$found" ]]; then
    reason="$(head -n 1 "$self_test_err")"
    blocking "ast-grep self-test produced no rule ids (ast-grep exit $status${reason:+; stderr: $reason})"
    failed=1
  elif [[ "$found" != "$expected" ]]; then
    blocking "rules no longer match their fixtures: expected [$expected], got [$found]"
    failed=1
  else
    echo "   all rules match bad.rs"
  fi
else
  skip "ast-grep not installed"
fi

# 2. Project anti-pattern rules.
note "ast-grep scan"
if command -v ast-grep >/dev/null; then
  rs_files=()
  read_into rs_files < <(own_files '*.rs' | in_scope)
  if [[ ${#rs_files[@]} -gt 0 ]]; then
    scan_out="$tmpdir/ast-grep-scan.out"
    status=0
    ast-grep scan "${rs_files[@]}" >"$scan_out" 2>&1 || status=$?
    cat "$scan_out"
    if [[ $status -ne 0 ]]; then
      failed=1
      error_count="$(grep -cE '^error\[[^]]+\]' "$scan_out" || true)"
      error_rules="$(grep -oE '^error\[[^]]+\]' "$scan_out" \
        | sed -E 's/^error\[([^]]+)\]$/\1/' | sort -u | tr '\n' ' ' || true)"
      if [[ ${error_count:-0} -gt 0 ]]; then
        blocking "ast-grep found $error_count error-level finding(s) from: ${error_rules% }"
      else
        blocking "ast-grep scan failed with exit $status and no error-level findings (see output above)"
      fi
    fi
  else
    echo "   no Rust files in scope"
  fi
else
  skip "ast-grep not installed"
fi

# 3. Host-side Rust crates: formatting and clippy. Guest crates built for wasm
# (no_std, custom panic handlers) are checked by the build itself.
note "cargo fmt / clippy (host crates)"
host_crates=()
for m in $(own_files '*Cargo.toml' | sort -u); do
  # The root manifest is a virtual workspace, not a crate: lint its members
  # through the per-member loop below.
  if ! grep -q '^\[package\]' "$m"; then continue; fi
  if grep -q -E '^\s*wasmtime\s*=|^\s*clap\s*=' "$m" || [[ "$m" == legacy/llvm/config_rewriter/Cargo.toml ]]; then
    dir="$(dirname "$m")"
    if [[ $changed -eq 1 ]] && ! dir_changed "$dir"; then continue; fi
    host_crates+=("$m")
  fi
done
if [[ ${#host_crates[@]} -eq 0 ]]; then
  echo "   no host crates in scope"
elif ! cargo clippy --version >/dev/null 2>&1; then
  skip "cargo clippy not installed (rustup component add clippy rustfmt)"
else
  export CLIPPY_CONF_DIR="$root"
  for m in "${host_crates[@]}"; do
    echo "   $m"
    if ! cargo fmt --manifest-path "$m" --check >/dev/null 2>&1; then
      echo "     fmt: needs formatting (cargo fmt --manifest-path $m)"; soft=1
    fi
    if ! cargo clippy --quiet --manifest-path "$m" --all-targets -- \
        -W clippy::all -W clippy::pedantic \
        -W clippy::excessive_nesting -W clippy::unwrap_used -W clippy::dbg_macro -W clippy::todo \
        -W clippy::undocumented_unsafe_blocks -W unsafe_op_in_unsafe_fn \
        -A clippy::module_name_repetitions -A clippy::must_use_candidate \
        -A clippy::missing_errors_doc -A clippy::missing_panics_doc \
        -D warnings 2>&1 | tail -n 40; then
      soft=1
    fi
  done
fi

# 4. Shell scripts.
note "shellcheck"
sh_files=()
read_into sh_files < <(own_files '*.sh' | in_scope)
if [[ ${#sh_files[@]} -eq 0 ]]; then
  echo "   no shell scripts in scope"
elif command -v shellcheck >/dev/null; then
  shellcheck -S warning "${sh_files[@]}" || soft=1
else
  skip "shellcheck not installed"
fi

# 5. Python scripts.
note "ruff"
py_files=()
read_into py_files < <(own_files '*.py' | in_scope)
if [[ ${#py_files[@]} -eq 0 ]]; then
  echo "   no Python files in scope"
elif command -v ruff >/dev/null; then
  ruff check "${py_files[@]}" || soft=1
else
  skip "ruff not installed"
fi

note "summary"
if [[ $failed -ne 0 ]]; then
  echo "   FAIL: blocking findings"
  for reason in "${blocking_reasons[@]}"; do printf '   %s\n' "$reason"; done
  exit 1
fi
if [[ $soft -ne 0 ]]; then
  if [[ $strict -eq 1 ]]; then echo "   FAIL (--strict): findings above"; exit 1; fi
  echo "   findings reported above (not blocking yet; use --strict to enforce)"
  exit 0
fi
echo "   clean"
