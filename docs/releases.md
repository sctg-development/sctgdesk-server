# Versions and releases

## Version numbers

Server versions follow `<upstream version>-sctg.<n>`, for example `1.1.16-sctg.1`: the upstream release the code
is based on, then our own counter. The former `1.1.99-xx` numbers were chosen to sort above upstream and
caused collisions with upstream's tags (issue #13); they are not used any more. (Decision proposed in
[ADR 0002](adr/0002-upstream-sync-strategy.md); the package version in `Cargo.toml` still says `1.1.99-47` until
the first release under the new scheme.)

## Tags

- Our release tags are prefixed `sctg-v`, for example `sctg-v1.1.16-sctg.1`.
- Upstream tags are never fetched into the normal tag namespace. They live in `refs/upstream-tags/*`
  (`scripts/sync-upstream.sh` does this), so they cannot collide with ours.

## Releasing

A release is a decision of the maintainer: tag, binaries, container images and announcement are done by a person,
never by a merge. Before tagging: the `main` build is green, the [key exchange check](testing.md) was run, and the
release notes say which upstream version it is based on.

The release workflows inherited from the former repositories are **not verified** since the consolidation; check
`.github/workflows/` before relying on them.
