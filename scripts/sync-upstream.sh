#!/usr/bin/env bash
# Merge one upstream rustdesk-server release tag into a sync branch, then build and test.
#
#   scripts/sync-upstream.sh 1.1.17
#
# Upstream tags are fetched into refs/upstream-tags/* so they never collide with ours.
# The script never pushes and never touches the current branch: it works on sync/upstream-<tag>.
# See docs/upstream-sync.md for the conflict policy.
set -euo pipefail

tag="${1:?usage: $0 <upstream tag, e.g. 1.1.17>}"
remote="${UPSTREAM_REMOTE:-upstream}"
base="${BASE_BRANCH:-$(git rev-parse --abbrev-ref HEAD)}"
branch="sync/upstream-${tag}"

cd "$(git rev-parse --show-toplevel)"
[ -z "$(git status --porcelain --untracked-files=no)" ] || { echo "working tree is not clean" >&2; exit 1; }
git remote get-url "$remote" >/dev/null 2>&1 || git remote add "$remote" https://github.com/rustdesk/rustdesk-server.git

git fetch "$remote" "+refs/tags/*:refs/upstream-tags/*"
git rev-parse --verify --quiet "refs/upstream-tags/${tag}^{commit}" >/dev/null \
    || { echo "upstream tag ${tag} not found" >&2; exit 1; }

git switch -c "$branch" "$base"
if ! git merge --no-ff --no-commit "refs/upstream-tags/${tag}"; then
    echo
    echo "Conflicts to resolve (policy: docs/upstream-sync.md):"
    git diff --name-only --diff-filter=U
    echo "Then: git add <files> && git commit, and re-run the checks below by hand."
    exit 2
fi

git submodule update --init
export DATABASE_URL="${DATABASE_URL:-sqlite://$PWD/db_v2.sqlite3}"
export SCTGDESK_SKIP_WEB_BUILD="${SCTGDESK_SKIP_WEB_BUILD:-1}"
cargo build --workspace --locked
cargo test --workspace --exclude oauth2 --locked --no-fail-fast -- --skip test_nat64

echo
echo "Build and tests pass. Review, then: git commit -m 'merge: upstream rustdesk-server ${tag}'"
