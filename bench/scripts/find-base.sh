#!/usr/bin/env bash
# Download the JSON of a previously published full benchmark run.
#
#   find-base.sh <destination.json> [commit-prefix]
#
# Without a commit prefix the newest `bench-results` JSON asset is used. With
# one, the newest asset whose name contains the prefix is used. Exits 0 and
# writes nothing when no base exists, so the first full run still succeeds.
set -euo pipefail

destination=${1:?usage: find-base.sh <destination.json> [commit-prefix]}
wanted=${2:-}
tag=bench-results
repo=${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}

tmp=$(mktemp -d)
trap 'rm -rf "${tmp:?}"' EXIT
if ! gh api "repos/${repo}/releases/tags/${tag}" > "$tmp/release.json" 2>/dev/null; then
    printf 'note: no %s release yet; nothing to compare against\n' "$tag" >&2
    exit 0
fi

asset=$(python3 - "$tmp/release.json" "$wanted" <<'PY'
import json
import sys

release = json.load(open(sys.argv[1], encoding="utf-8"))
wanted = sys.argv[2]
assets = [
    asset
    for asset in release.get("assets", [])
    if asset.get("name", "").startswith("bench-")
    and asset.get("name", "").endswith(".json")
]
if wanted:
    assets = [asset for asset in assets if wanted[:12] in asset["name"]]
if assets:
    print(max(assets, key=lambda asset: asset.get("created_at", ""))["name"])
PY
)
if [ -z "$asset" ]; then
    printf 'note: no matching full-run asset for %s\n' "${wanted:-the newest run}" >&2
    exit 0
fi

mkdir -p "$(dirname "$destination")" "$tmp/asset"
gh release download "$tag" --pattern "$asset" --dir "$tmp/asset" --clobber
cp "$tmp/asset/$asset" "$destination"
printf 'base: %s\n' "$asset" >&2
