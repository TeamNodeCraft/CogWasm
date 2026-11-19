#!/usr/bin/env bash
# Rebuild the static README image and the interactive page data from every
# point on the append-only `bench-history` release, then publish them with
# stable names.
#
#   build-chart.sh
#
# Required environment (provided by cranelift_wasm.yml): GH_TOKEN,
# GITHUB_REPOSITORY.
#
# The point JSONs are named <commit-date>-<sha7>.json. data.json and chart.svg
# do not match that pattern, so they are not read back as points.
set -euo pipefail

repo=${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
chart_dir=$(cd "$script_dir/../chart" && pwd)
tag=bench-history

command -v gh >/dev/null || { printf 'error: gh is required\n' >&2; exit 1; }

tmp=$(mktemp -d)
trap 'rm -rf "${tmp:?}"' EXIT
mkdir -p "$tmp/history" "$tmp/output"

if ! gh release view "$tag" >/dev/null 2>&1; then
    printf 'error: the %s release does not exist; nothing to chart\n' "$tag" >&2
    exit 1
fi
gh release download "$tag" --pattern '*-*.json' --dir "$tmp/history" --clobber || true
points=$(find "$tmp/history" -maxdepth 1 -name '*-*.json' -type f | wc -l | tr -d ' ')
if [ "$points" -eq 0 ]; then
    printf 'error: the %s release has no point JSONs; nothing to chart\n' "$tag" >&2
    exit 1
fi

python3 "$chart_dir/chart.py" \
    --history "$tmp/history" \
    --output "$tmp/output" \
    --repo "$repo"

gh release upload "$tag" "$tmp/output/chart.svg" "$tmp/output/data.json" --clobber

chart_url="https://github.com/${repo}/releases/download/${tag}/chart.svg"
owner=$(printf '%s' "${GITHUB_REPOSITORY_OWNER:-teamnodecraft}" | tr '[:upper:]' '[:lower:]')
page_url="https://${owner}.github.io/${repo#*/}/bench/"
printf '::notice title=benchmark chart published::%s\n' "$chart_url"
{
    echo "### Benchmark chart"
    echo
    echo "[chart.svg](${chart_url}) · [$points point(s) in ${tag}](https://github.com/${repo}/releases/tag/${tag})"
    echo
    echo "Interactive page: ${page_url} (published by deploy pages)"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"
