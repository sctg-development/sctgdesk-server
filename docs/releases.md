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

## Linux packages

The `Linux packages (deb, rpm)` workflow (`.github/workflows/packages.yml`, run by hand) builds static musl binaries
of `hbbs`, `hbbr` and `rustdesk-utils` for amd64 and arm64 and packs them with [nfpm](https://nfpm.goreleaser.com/)
(`packaging/`): `sctgdesk-server-hbbs`, `sctgdesk-server-hbbr` and `sctgdesk-server-utils`, as deb and rpm. It checks
that the binaries are static and installs the packages in a Debian and a Fedora container.

- The tag input (`sctg-v<version>`) must match the version of `Cargo.toml`, or the workflow stops.
- Without `publish` nothing is published. With it, the packages and their SHA-256 sums go to the release of the
  tag, created as a **pre-release** when it does not exist.
- The packages replace the former `rustdesk-server-*` ones. A first install starts nothing: configure the options of
  the unit, then `systemctl enable --now rustdesk-hbbs rustdesk-hbbr`. An upgrade restarts the services that run.
- The units run as root, like the former packages; the data is in `/var/lib/rustdesk-server`, the logs in
  `/var/log/rustdesk-server`.

The release workflows inherited from the former repositories are **not verified** since the consolidation; check
`.github/workflows/` before relying on them.
