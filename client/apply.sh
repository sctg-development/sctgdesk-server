#!/usr/bin/env bash
# Prepare the client source tree: upstream RustDesk at UPSTREAM_TAG + our patches + our files.
#
#   client/apply.sh [DIR]        DIR defaults to client/work (git-ignored)
#
# Then, from DIR: ./build-sctg.py --flutter ...   (brands the tree, then runs upstream's build.py)
# The upstream tree is never committed here: only patches/ and overlay/ are.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
tag="$(tr -d '[:space:]' < "$here/UPSTREAM_TAG")"
dest="${1:-$here/work}"
upstream="${UPSTREAM_URL:-https://github.com/rustdesk/rustdesk.git}"

[ ! -e "$dest" ] || { echo "$dest already exists; remove it first" >&2; exit 1; }
git clone --quiet --depth 1 --branch "$tag" --recurse-submodules --shallow-submodules "$upstream" "$dest"

# Patches are plain `git format-patch` files, applied in order on the pristine tag.
if compgen -G "$here/patches/*.patch" > /dev/null; then
    git -C "$dest" -c user.name=sctg -c user.email=sctg@localhost am --quiet "$here"/patches/*.patch
fi

# Files that we add or replace as is (logos, icons).
cp -R "$here/overlay/." "$dest/"
cp "$here/branding.py" "$here/build-sctg.py" "$dest/"

echo "Client tree ready in $dest (upstream $tag)."
echo "Check the branding without writing: (cd $dest && python3 branding.py --check)"
