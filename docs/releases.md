# Versions and releases

## Version numbers

Server versions follow `<upstream version>-sctg.<n>`, for example `1.1.16-sctg.1`: the upstream release the code
is based on, then our own counter. The former `1.1.99-xx` numbers were chosen to sort above upstream and
caused collisions with upstream's tags (issue #13); they are not used any more. (Decision proposed in
[ADR 0002](adr/0002-upstream-sync-strategy.md).) The version is set in `Cargo.toml`, `crates/api-server/Cargo.toml`,
`Cargo.lock`, `web/console/package.json` (and its lock file) and `debian/changelog`, in one commit.

## Tags

- Our release tags are prefixed `sctg-v`, for example `sctg-v1.1.16-sctg.1`.
- Upstream tags are never fetched into the normal tag namespace. They live in `refs/upstream-tags/*`
  (`scripts/sync-upstream.sh` does this), so they cannot collide with ours.

## Releasing

A release is a decision of the maintainer, made by publishing a GitHub release; the pipeline does the rest (next
section). Before publishing: the default branch build is green, the [key exchange check](testing.md) was run, the
version is bumped everywhere (below), and the release notes say which upstream version it is based on.

1. Bump the version in a pull request and merge it (`Cargo.toml`, `crates/api-server/Cargo.toml`, `Cargo.lock`,
   `web/console/package.json` and its lock, `debian/changelog`).
2. Optionally run `packages.yml` and `binaries.yml` by hand without `publish`: they build and test, and publish nothing.
3. Create the release `sctg-v<version>` on the default branch. Tick **pre-release** to publish the server side only.
4. Check the assets, Docker Hub, and (full release) S3 and Cloudflare Pages.

## The release pipeline

Publishing a release runs `.github/workflows/release.yml`, which calls the other workflows with the tag of the
release (nothing to type) :

| Job | Workflow | Result |
|---|---|---|
| Docker image and Linux binaries | `multiarch-docker-hub.yml` | image `sctg/sctgdesk-server` (amd64, arm64) tagged with the release tag, `latest` for a full release only; static Linux zips on the release |
| deb and rpm | `packages.yml` | packages (amd64, arm64) and SHA-256 sums on the release |
| Windows and macOS binaries | `binaries.yml` | server zips (Windows amd64 and arm64, macOS Intel and Apple Silicon) and SHA-256 sums on the release |
| Windows client, macOS client | `client-windows.yml`, `client-macos.yml` | installers copied to S3: they become the downloads of the landing page and of the update check |
| Web client | `client-web.yml` | archives on S3, and the site on Cloudflare Pages (**replaces the live page**) |

A **pre-release** runs the server jobs and builds the clients, but publishes none of them: the clients on S3 and
the web client change what users get, and `latest` moves the Docker tag. The tag guard of `packages.yml` and
`binaries.yml` stops them if the tag does not match `Cargo.toml`. A failed job is re-run from the run's page in the
*Actions* tab (*Re-run failed jobs*): the called workflows read the release event, which a new manual run would not
have. Drafts, pushes and tags alone start nothing. The clients are versioned by the upstream tag they are built on
(`client/UPSTREAM_TAG`), not by the server version.

Secrets used (repository settings): `DOCKER_USERNAME`, `DOCKER_PASSWORD`; `APP_NAME`, `APP_NAME_LOW`, `ORG_NAME`,
`RS_PUB_KEY`, `RENDEZVOUS_SERVER`, `API_SERVER` (branding); `AWS_S3_ENDPOINT`, `AWS_ACCESS_KEY_ID`,
`AWS_SECRET_ACCESS_KEY`, `AWS_S3_BUCKET`; `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`,
`CLOUDFLARE_PROJECT_NAME`; `MACOS_P12_BASE64`, `MACOS_P12_PASSWORD`, `MACOS_CODESIGN_IDENTITY`,
`MACOS_NOTARIZE_JSON` (and `MACOS2_P12_BASE64`, `MACOS3_P12_BASE64` for the Windows signing certificates).

The former workflows (`main.yml`, `macos-intel-build.yml`, `windows.yml`, `ubuntu.yml`, `client-build.yaml`,
`client-release.yml`) were removed: they are in the git history. The iOS, Android and Linux client builds were part
of `client-build.yaml` and are not built any more.

## Linux packages

The `Linux packages (deb, rpm)` workflow (`.github/workflows/packages.yml`, run by the release pipeline or by hand) builds static musl binaries
of `hbbs`, `hbbr` and `rustdesk-utils` for amd64 and arm64 and packs them with [nfpm](https://nfpm.goreleaser.com/)
(`packaging/`): `sctgdesk-server-hbbs`, `sctgdesk-server-hbbr` and `sctgdesk-server-utils`, as deb and rpm. It checks
that the binaries are static and installs the packages in a Debian and a Fedora container.

- The tag input (`sctg-v<version>`) must match the version of `Cargo.toml`, or the workflow stops.
- Without `publish` nothing is published. With it, the packages and their SHA-256 sums go to the release of the
  tag, created as a **pre-release** when it does not exist.
- The packages replace the former `rustdesk-server-*` ones. A first install on an empty data directory starts
  the services with a random key pair; one that finds data (a migration) starts nothing. Options are in
  `/etc/default/sctgdesk-hbbs` and `-hbbr`. Data is copied to `/var/backups/sctgdesk-server/` before each install. An
  upgrade restarts the services that run.
- The units run as root, like the former packages; the data is in `/var/lib/rustdesk-server`, the logs in
  `/var/log/rustdesk-server`.

## Windows and macOS binaries

The `Windows and macOS binaries` workflow (`.github/workflows/binaries.yml`, run by the release pipeline or by hand, same `tag`
and `publish` inputs) builds `hbbs`, `hbbr` and `rustdesk-utils` on GitHub runners for Windows amd64 and arm64 (`windows-latest`,
`windows-11-arm`, C runtime linked statically) and macOS Intel and Apple Silicon (`macos-15-intel`, `macos-15`),
checks that they run and report the version of the tag, and zips them as
`sctgdesk-server-<version>-<windows|macos>-<x86_64|aarch64>.zip`. With `publish` the archives and
`SHA256SUMS-windows-macos` go to the release of the tag, created as a pre-release if missing. The binaries are not
signed (Windows) nor signed and notarized (macOS).

The release pipeline above runs both workflows with `publish` on.
