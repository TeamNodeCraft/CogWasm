#!/usr/bin/env bash
# Add one successful quick benchmark result to the append-only `bench-history`
# release, as <commit-date>-<sha7>.json.
#
#   publish-history.sh <quick-result.json>
#
# Required environment (provided by cranelift_wasm.yml): GH_TOKEN,
# GITHUB_REPOSITORY, GITHUB_RUN_ID.
#
# The release is append-only: this script never deletes an asset. A re-run of
# the same commit writes the same <date>-<sha7>.json name, so `--clobber`
# replaces only that commit's point and leaves every other commit alone.
set -euo pipefail

json=${1:?usage: publish-history.sh <quick-result.json>}
repo=${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}
run_id=${GITHUB_RUN_ID:?GITHUB_RUN_ID is required}
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tag=bench-history

[ -f "$json" ] || { printf 'error: %s does not exist\n' "$json" >&2; exit 1; }
command -v gh >/dev/null || { printf 'error: gh is required\n' >&2; exit 1; }

sha=$(python3 -c '
import json, sys
print(json.load(open(sys.argv[1], encoding="utf-8"))["commit"]["sha"])
' "$json")
date_raw=$(gh api "repos/${repo}/commits/${sha}" --jq '.commit.committer.date' 2>/dev/null || true)
if [ -z "$date_raw" ]; then
    date_raw=$(git show -s --format=%cI "$sha")
fi
date=$(python3 -c '
import sys
from datetime import datetime, timezone
value = sys.argv[1].replace("Z", "+00:00")
parsed = datetime.fromisoformat(value)
if parsed.tzinfo is None:
    parsed = parsed.replace(tzinfo=timezone.utc)
print(parsed.astimezone(timezone.utc).isoformat().replace("+00:00", "Z"))
' "$date_raw")

tmp=$(mktemp -d)
trap 'rm -rf "${tmp:?}"' EXIT
# CI's edit-loop steps (ci/build-and-test.sh) leave their reports here.
edit_loop_dir=${EDIT_LOOP_DIR:-toolchain/work/supervisor}
edit_loop_args=()
[ -d "$edit_loop_dir" ] && edit_loop_args=(--edit-loop "$edit_loop_dir")
asset=$(python3 "$script_dir/history_point.py" "$json" \
    --run-id "$run_id" --date "$date" --repo "$repo" \
    --output "$tmp/point.json" ${edit_loop_args[@]+"${edit_loop_args[@]}"})
cp "$tmp/point.json" "$tmp/$asset"

if ! gh release view "$tag" >/dev/null 2>&1; then
    # A concurrent writer can win this race; the upload below is the real
    # check, so a failed create is not fatal.
    gh release create "$tag" \
        --prerelease \
        --title "Benchmark history" \
        --notes "Append-only history of CI quick benchmark points, one JSON per main commit. Never delete assets; a re-run replaces only its commit's point. chart.svg and data.json are the generated trend chart." \
        || true
fi

url="https://github.com/${repo}/releases/download/${tag}/${asset}"
gh release upload "$tag" "$tmp/$asset" --clobber
printf '::notice title=benchmark history published::%s\n' "$url"
{
    echo "### Benchmark history"
    echo
    echo "[${asset}](${url})"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"
