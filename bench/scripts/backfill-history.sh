#!/usr/bin/env bash
# Import quick benchmark results that already exist on the rolling `ci-logs`
# release into the append-only `bench-history` release, once per commit.
#
#   backfill-history.sh
#
# Required environment (provided by cranelift_wasm.yml): GH_TOKEN,
# GITHUB_REPOSITORY.
#
# This catches the quick result for 8476eeb that was published before the
# bench-history release existed, and any later main run whose publish step did
# not get a chance to record it. It only imports successful main-branch
# cranelift_wasm runs, and only commits that have no point yet, so running it
# on every CI run is idempotent. It never deletes an asset.
set -euo pipefail

repo=${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
history_tag=bench-history
ci_tag=ci-logs

command -v gh >/dev/null || { printf 'error: gh is required\n' >&2; exit 1; }

tmp=$(mktemp -d)
trap 'rm -rf "${tmp:?}"' EXIT
mkdir -p "$tmp/history" "$tmp/download"
: > "$tmp/known.txt"
: > "$tmp/known-runs.txt"

ensure_history_release() {
    if ! gh release view "$history_tag" >/dev/null 2>&1; then
        gh release create "$history_tag" \
            --prerelease \
            --title "Benchmark history" \
            --notes "Append-only history of CI quick benchmark points, one JSON per main commit. Never delete assets; a re-run replaces only its commit's point. chart.svg and data.json are the generated trend chart." \
            || true
    fi
}

if gh release view "$history_tag" >/dev/null 2>&1; then
    gh release download "$history_tag" --pattern '*-*.json' \
        --dir "$tmp/history" --clobber >/dev/null 2>&1 || true
    python3 - "$tmp/history" > "$tmp/known-runs.txt" <<'PY'
import json
import sys
from pathlib import Path

for path in sorted(Path(sys.argv[1]).glob("*-*.json")):
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        continue
    ci = raw.get("ci_point") if isinstance(raw, dict) else None
    run_id = str(ci.get("run_id") or "") if isinstance(ci, dict) else ""
    if run_id.isdigit():
        print(run_id)
PY
    python3 - "$tmp/history" > "$tmp/known.txt" <<'PY'
import json
import sys
from pathlib import Path

for path in sorted(Path(sys.argv[1]).glob("*-*.json")):
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        continue
    ci = raw.get("ci_point") if isinstance(raw, dict) else None
    commit = raw.get("commit") if isinstance(raw, dict) else None
    sha = ""
    if isinstance(ci, dict):
        sha = str(ci.get("sha") or "")
    if not sha and isinstance(commit, dict):
        sha = str(commit.get("sha") or "")
    if sha:
        print(sha[:7])
PY
fi

if ! gh release view "$ci_tag" >/dev/null 2>&1; then
    printf 'note: no %s release; nothing to backfill\n' "$ci_tag" >&2
    exit 0
fi

gh release view "$ci_tag" --json assets --jq '.assets[].name' > "$tmp/ci-assets.txt"
python3 - "$tmp/ci-assets.txt" "$tmp/known-runs.txt" > "$tmp/candidates.txt" <<'PY'
import re
import sys

with open(sys.argv[2], encoding="utf-8") as known_file:
    known_runs = {line.strip() for line in known_file if line.strip()}
run_ids = []
for line in open(sys.argv[1], encoding="utf-8"):
    match = re.fullmatch(r"bench-quick-([0-9]+)\.json", line.strip())
    if match and match.group(1) not in known_runs:
        run_ids.append(int(match.group(1)))
for run_id in sorted(run_ids):
    print(f"bench-quick-{run_id}.json")
PY

imported=0
while IFS= read -r asset; do
    [ -n "$asset" ] || continue
    run_id=${asset#bench-quick-}
    run_id=${run_id%.json}
    info=$(gh api "repos/${repo}/actions/runs/${run_id}" \
        --jq '[.head_branch, .head_sha, .conclusion] | @tsv' 2>/dev/null || true)
    if [ -z "$info" ]; then
        printf 'warning: cannot read run %s (%s); leaving it for a later run\n' \
            "$run_id" "$asset" >&2
        continue
    fi
    IFS=$'\t' read -r branch sha conclusion <<< "$info"
    if [ "$branch" != main ] || [ "$conclusion" != success ]; then
        printf 'note: skipping %s from %s (conclusion %s)\n' \
            "$asset" "${branch:-unknown}" "${conclusion:-unknown}" >&2
        continue
    fi
    short=${sha:0:7}
    if grep -F -x -q "$short" "$tmp/known.txt"; then
        continue
    fi
    gh release download "$ci_tag" --pattern "$asset" --dir "$tmp/download" --clobber
    quick="$tmp/download/$asset"
    date_raw=$(gh api "repos/${repo}/commits/${sha}" \
        --jq '.commit.committer.date' 2>/dev/null || true)
    if [ -z "$date_raw" ]; then
        date_raw=$(python3 -c '
import json, sys
print(json.load(open(sys.argv[1], encoding="utf-8")).get("generated_at", ""))
' "$quick")
    fi
    if [ -z "$date_raw" ]; then
        printf 'warning: no date for %s (%s); leaving it for a later run\n' \
            "$short" "$asset" >&2
        continue
    fi
    date=$(python3 -c '
import sys
from datetime import datetime, timezone
parsed = datetime.fromisoformat(sys.argv[1].replace("Z", "+00:00"))
if parsed.tzinfo is None:
    parsed = parsed.replace(tzinfo=timezone.utc)
print(parsed.astimezone(timezone.utc).isoformat().replace("+00:00", "Z"))
' "$date_raw")
    point_asset=$(python3 "$script_dir/history_point.py" "$quick" \
        --run-id "$run_id" --date "$date" --repo "$repo" \
        --output "$tmp/point.json")
    cp "$tmp/point.json" "$tmp/$point_asset"
    ensure_history_release
    gh release upload "$history_tag" "$tmp/$point_asset" --clobber
    printf 'backfilled %s from %s (run %s)\n' "$point_asset" "$ci_tag" "$run_id"
    printf '%s\n' "$short" >> "$tmp/known.txt"
    printf '%s\n' "$run_id" >> "$tmp/known-runs.txt"
    imported=$((imported + 1))
done < "$tmp/candidates.txt"

printf 'backfill: imported %s point(s)\n' "$imported"
