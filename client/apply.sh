#!/usr/bin/env bash
# Prepare the client source tree: upstream RustDesk at UPSTREAM_TAG + our patches + our files.
#
#   client/apply.sh [DIR] [web]  DIR defaults to client/work (git-ignored)
#
# Then, from DIR: ./build-sctg.py --flutter ...   (brands the tree, then runs upstream's build.py)
#
# `web` prepares the web client instead, from UPSTREAM_TAG_WEB, with web-overlay/ and without patches
# (hbb_common is needed for its .proto files). Brand it with: python3 branding.py --web
# Upstream's open source web JavaScript (flutter/web/js, then v1) only works with the Flutter code of the same
# time: its README says "v1 is not compatible with current Flutter source code". 1.2.5 is the last tag where
# they match (from 1.2.6 on the Dart calls session_start, which the JavaScript does not handle), so the tag
# must not be raised without checking that.
# The upstream tree is never committed here: only patches/ and overlay/ are.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
dest="${1:-$here/work}"
flavor="${2:-desktop}"
case "$flavor" in
    desktop) tag_file=UPSTREAM_TAG; overlay=overlay; patches=1; submodules=(--recurse-submodules --shallow-submodules) ;;
    web)     tag_file=UPSTREAM_TAG_WEB; overlay=web-overlay; patches=0; submodules=(--recurse-submodules --shallow-submodules) ;;
    *) echo "unknown flavor '$flavor' (desktop or web)" >&2; exit 1 ;;
esac
tag="$(tr -d '[:space:]' < "$here/$tag_file")"
upstream="${UPSTREAM_URL:-https://github.com/rustdesk/rustdesk.git}"

[ ! -e "$dest" ] || { echo "$dest already exists; remove it first" >&2; exit 1; }
git clone --quiet --depth 1 --branch "$tag" ${submodules[@]+"${submodules[@]}"} "$upstream" "$dest"

# Patches are plain `git format-patch` files, applied in order on the pristine tag.
if [ "$patches" = 1 ] && compgen -G "$here/patches/*.patch" > /dev/null; then
    git -C "$dest" -c user.name=sctg -c user.email=sctg@localhost am --quiet "$here"/patches/*.patch
fi

# From 1.2.6 upstream keeps the web client in flutter/web/v1, but its tooling (and `flutter build web`, which needs
# web/index.html) expects it in flutter/web: move it up.
if [ "$flavor" = web ] && [ -d "$dest/flutter/web/v1" ]; then
    (cd "$dest/flutter/web" && shopt -s dotglob && mv v1/* . && rmdir v1)
fi

# Files that we add or replace as is (logos, icons).
cp -R "$here/$overlay/." "$dest/"
cp "$here/branding.py" "$here/build-sctg.py" "$dest/"

echo "Client tree ready in $dest (upstream $tag, $flavor)."
echo "Check the branding without writing: (cd $dest && python3 branding.py --check$([ "$flavor" = web ] && echo ' --web'))"
