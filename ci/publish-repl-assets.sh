#!/usr/bin/env bash
# Publish the in-browser REPL's compiler (ci/build-repl-assets.sh output) to
# the `repl` release, which the Pages deploy copies to repl/assets/.
#
#   ci/publish-repl-assets.sh [assets-dir]   (default toolchain/work/repl-assets)
#
# Needs GH_TOKEN and GITHUB_REPOSITORY (cranelift_wasm.yml provides them).
# Uploads every file with --clobber and manifest.json last, so a reader never
# sees a manifest that names a missing file; then deletes assets the new
# manifest no longer uses (sysroot file names carry a hash that changes when
# the sysroot is rebuilt).
set -euo pipefail
REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
DIR=${1:-$REPO/toolchain/work/repl-assets}
repo=${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}
tag=repl

[ -f "$DIR/manifest.json" ] || { echo "no $DIR/manifest.json; run ci/build-repl-assets.sh" >&2; exit 1; }
command -v gh >/dev/null || { echo "gh is required" >&2; exit 1; }

if ! gh release view "$tag" --repo "$repo" >/dev/null 2>&1; then
    gh release create "$tag" --repo "$repo" --prerelease \
        --title "In-browser REPL compiler" \
        --notes "rustc.wasm, a single-threaded wild.wasm and the no_std sysroot, gzipped, for the site's live REPL (site/repl/). Replaced on every successful main build of cranelift_wasm; the site downloads them at deploy time." \
        || true # a concurrent run may have created it; the upload below is the real check
fi

wanted=()
for path in "$DIR"/*; do
    name=$(basename "$path")
    [ "$name" = manifest.json ] || wanted+=("$name")
done
for name in "${wanted[@]}"; do
    gh release upload "$tag" "$DIR/$name" --repo "$repo" --clobber
done
gh release upload "$tag" "$DIR/manifest.json" --repo "$repo" --clobber

while IFS= read -r name; do
    [ "$name" = manifest.json ] && continue
    keep=false
    for want in "${wanted[@]}"; do [ "$want" = "$name" ] && keep=true; done
    if [ "$keep" = false ]; then
        echo "removing stale asset $name"
        gh release delete-asset "$tag" "$name" --repo "$repo" --yes
    fi
done < <(gh release view "$tag" --repo "$repo" --json assets --jq '.assets[].name')

python3 -c 'import json,sys; m=json.load(open(sys.argv[1])); print("::notice title=REPL compiler published::%.1f MB for commit %s" % (m["download_bytes"]/1e6, m["commit"][:7]))' "$DIR/manifest.json"
