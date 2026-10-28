# Roadmap

Open work, roughly in priority order. Nothing here is a promise; each item is a candidate for an issue.

## Make the consolidation trustworthy

- **Run the CI on GitHub.** The workflows (`ci.yml`, `client-build.yaml`) were written and checked locally only.
- **Fix `crates/api-oauth2` tests**, which do not compile, and re-enable them in CI.
- **Triage `cargo audit`** (dozens of advisories, informational for now) and add `cargo-deny` for licenses.
- **Adapt or remove** what was inherited and never checked: `docker/`, `docker-classic/`, `kubernetes/`, `update-api-server.sh`,
  the legacy macOS and Windows build workflows (now manual).
- **Verify the Docker image build** (`Dockerfile`, `multiarch-docker-hub.yml`, rewritten for amd64 and arm64 on the shared
  multi-arch action; 32-bit ARM and the `.deb` packages are dropped). It has not been run.
- **Translated READMEs** (`README-*.md`) are stale.

## Protocol

- **Put the key-exchange check in CI**: a test that runs a client's `key_exchange()` against `hbbs` and sends one
  encrypted request ([ADR 0003](adr/0003-tcp-and-key-exchange.md)).
- Test a **released official client** against the server, including WebSocket and `hbbr`.
- A test for `--require-kx-v1`.
- Decide whether `--require-kx-v1` becomes the default once old clients are gone.

## Client

- **Compile the client** from `client/` on CI, then fix what the first runs show.
- Test upstream's `disable-udp` against our server (upstream's tip says the OSS server does not support it).
- Icons for Windows, Android and iOS (only `res/` and the macOS icon are overlaid).
- Refactor `deploy/helm` (written for a TCP test, deploys the server).
- Web client: absent from upstream 1.5.0; decide whether to bring one back.

## Compliance

- AGPL section 13: a source link in the web console footer and in `--version`, and exact sources per release.
- Replace the credentials-looking values in `s3config.toml` with examples (the values in the repository are fake).
- Non-affiliation notice in the README ([ADR 0004](adr/0004-license-and-trademark.md)).

## Releases

- First release under the `<upstream>-sctg.<n>` scheme, with `sctg-v` tags ([Versions and releases](releases.md)).
- Publish this manual.

## Web console

The Vue console (`web/console`) is on old dependencies and built with `npm install --force`: a modernisation
would be a good, well-bounded contribution.
