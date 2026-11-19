#!/usr/bin/env bash
# Publish one full benchmark result (JSON + Markdown + comparison) to the
# dedicated `bench-results` release, one asset set per run.
#
#   publish-results.sh <json> <markdown> [comparison.md]
#
# Required environment (provided by the workflow): GH_TOKEN, GITHUB_REPOSITORY,
# GITHUB_SHA.
set -euo pipefail

json=${1:?usage: publish-results.sh <json> <markdown> [comparison]}
markdown=${2:?usage: publish-results.sh <json> <markdown> [comparison]}
comparison=${3:-}
files=("$json" "$markdown")
if [ -n "$comparison" ]; then
    files+=("$comparison")
fi
for file in "${files[@]}"; do
    [ -f "$file" ] || { printf 'error: %s does not exist\n' "$file" >&2; exit 1; }
done

tag=bench-results
stamp=$(date -u +%Y%m%d)
base="bench-${stamp}-${GITHUB_SHA:0:12}"

tmp=$(mktemp -d)
trap 'rm -rf "${tmp:?}"' EXIT
cp "$json" "$tmp/${base}.json"
cp "$markdown" "$tmp/${base}.md"
assets=("$tmp/${base}.json" "$tmp/${base}.md")
if [ -n "$comparison" ]; then
    cp "$comparison" "$tmp/${base}-compare.md"
    assets+=("$tmp/${base}-compare.md")
fi

if ! gh release view "$tag" >/dev/null 2>&1; then
    gh release create "$tag" \
        --prerelease \
        --title "Benchmark results" \
        --notes "Full CogWasm benchmark runs. One JSON + Markdown asset set per run, named by date and commit." \
        || true
fi
gh release upload "$tag" "${assets[@]}" --clobber

{
    echo "### Published benchmark"
    echo
    for asset in "${assets[@]}"; do
        name=$(basename "$asset")
        echo "- [${name}](https://github.com/${GITHUB_REPOSITORY}/releases/download/${tag}/${name})"
    done
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"
