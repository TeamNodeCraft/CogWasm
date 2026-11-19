#!/usr/bin/env bash
# Publish the quick benchmark JSON + Markdown to the rolling `ci-logs`
# pre-release. Asset names carry the run id so old runs stay readable.
#
#   publish-ci.sh <json> <markdown>
#
# Required environment (provided by the workflow): GH_TOKEN, GITHUB_REPOSITORY,
# GITHUB_RUN_ID.
set -euo pipefail

json=${1:?usage: publish-ci.sh <json> <markdown>}
markdown=${2:?usage: publish-ci.sh <json> <markdown>}
for file in "$json" "$markdown"; do
    [ -f "$file" ] || { printf 'error: %s does not exist\n' "$file" >&2; exit 1; }
done

tag=ci-logs
json_asset="bench-quick-${GITHUB_RUN_ID}.json"
md_asset="bench-quick-${GITHUB_RUN_ID}.md"
url_base="https://github.com/${GITHUB_REPOSITORY}/releases/download/${tag}"

tmp=$(mktemp -d)
trap 'rm -rf "${tmp:?}"' EXIT
cp "$json" "$tmp/$json_asset"
cp "$markdown" "$tmp/$md_asset"

if ! gh release view "$tag" >/dev/null 2>&1; then
    gh release create "$tag" \
        --prerelease \
        --title "CI build logs" \
        --notes "Rolling pre-release with build logs and quick benchmark results." \
        || true
fi
gh release upload "$tag" "$tmp/$json_asset" "$tmp/$md_asset" --clobber

{
    echo "### Quick benchmark"
    echo
    echo "[${json_asset}](${url_base}/${json_asset}) · [${md_asset}](${url_base}/${md_asset})"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"
