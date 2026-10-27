# ADR 0002: Upstream synchronisation and `hbb_common`

Status: Accepted (maintainer decision, 2026-10-27: recommendation adopted)

## Context

- The server fork is **65 commits behind** `rustdesk/rustdesk-server` (base `b37033d`, 2024-09-28, between tags `1.1.11-1` and `1.1.12`). Upstream tagged `1.1.12` to `1.1.16` since.
- Our default branch is `tcpserver-master-build`; the old `master` is stale. Our tag `1.1.10-3` collides with an upstream tag of the same name at a different commit.
- `hbb_common`: upstream replaced the vendored copy with a **git submodule** on 2025-01-20 (`7a509f6`) and pins `69cea8d` (2026-07-26). Our server still carries a **vendored, modified copy** last synchronised in 2024-07, about 4,590 changed lines away from that pin. How much of that is our modification and how much is plain lag is **not yet established**.
- The current upstream client uses key-exchange version 1 (`KxParams`, `KX_PARAMS_DOMAIN`). That landed in `hbb_common` on 2026-09-22, **after** the pin used by upstream's server (see ADR 0003). The server therefore needs a newer `hbb_common` than upstream's own server pins.

## Options for the sync flow

1. Merge `upstream/master` into our branch periodically (what we did before).
2. Rebase our commits onto upstream master.
3. **Tag-by-tag merges** from a read-only mirror branch.

Rebasing 176 commits that include many "wip" ones rewrites public history and was ruled out by your rules (no force-push). Merging master commit-by-commit conflates unreleased upstream work with releases.

## Options for `hbb_common`

1. Keep vendoring. Every upstream bump is a manual three-way merge; the copies already diverged.
2. Submodule on upstream `rustdesk/hbb_common`, with our changes as patches.
3. **Submodule on a thin fork** `sctg-development/hbb_common` whose branches are `upstream` (mirror) and `sctg` (upstream + our few commits).

## Recommendation

**Flow**
- Add remote `upstream`; keep a local mirror branch `upstream/server` that only fast-forwards and receives no commits of ours.
- Merge **by release tag** into a sync branch (`sync/1.1.12`, `sync/1.1.13`, ... `sync/1.1.16`), one at a time: build, test, then merge into the integration branch. Five small steps instead of one 65-commit conflict.
- Upstream tags are fetched into `refs/upstream-tags/*` so they never collide with ours; our release tags use a different prefix (see below).
- A `scripts/sync-upstream.sh <tag>` does: fetch, create sync branch, merge, run build/test, print the conflict list. Phase 4 delivers it with its documentation.

**`hbb_common`**: option 3, with the explicit goal of shrinking the fork to zero commits over time. The server pins a `hbb_common` commit at least as new as the client's (`edefa08` at the time of writing) because of the key-exchange version. If an audit of our vendored copy shows no change worth keeping, we use upstream directly (option 2) and drop the fork.

**Versioning**: our `1.1.99-xx` numbers were chosen to sort above upstream and cause the tag clash and issue #13. I recommend `<upstream-version>-sctg.<n>` (for example `1.1.16-sctg.1`) with tags prefixed `sctg-v`. This is your call (Phase 1 decision), because it changes package versions and the update check in the client.

## Consequences

- One-time cost: isolating our changes in `hbb_common` (three-way diff from the 2024 base) before the submodule switch.
- `Cargo.lock` will conflict at every sync. Policy: never hand-merge it; take upstream's version, re-apply our dependency changes, run `cargo update -p <crate>` as needed.
- Because upstream CI files (`.github/workflows/build.yaml`, `main.yml`) overlap with ours, our workflows get distinct names to avoid permanent conflicts.
