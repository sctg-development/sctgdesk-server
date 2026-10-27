# Contributing to SCTGDesk

Thanks for helping. This page is the short version; the developer manual
(sources in `docs/`, built with mdBook) has the details.

## Before you start

- **Compatibility first.** The client and server must keep working with official RustDesk. Keep changes small and
  isolated; files that come from upstream (`src/`, `libs/hbb_common`) are changed as little as possible, because every
  change is paid for again at each [upstream sync](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/upstream-sync.md).
- Not sure where a change belongs? Open an issue first. Changes to upstream-derived files, the wire protocol or the
  key exchange are worth discussing before you write them.
- Read the [code of conduct](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/CODE_OF_CONDUCT.md).

## Build and test

```sh
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3
export SCTGDESK_SKIP_WEB_BUILD=1              # skips Node.js
git submodule update --init
cargo build --workspace --locked
cargo test --workspace --exclude oauth2 --locked -- --skip test_nat64
```

See [Building from source](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/building.md) and [Testing](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/testing.md). The CI also checks `rustfmt` and
`clippy` on `crates/*`: run `cargo fmt --manifest-path crates/<name>/Cargo.toml` before you push.

## Commits and pull requests

- One logical change per commit, in English, following [Conventional Commits](https://www.conventionalcommits.org/):
  `fix(rendezvous): ...`, `feat(api-server): ...`, `docs: ...`, `build:`, `ci:`, `test:`, `chore:`.
- Sign your commits off (`git commit -s`): by adding `Signed-off-by`, you certify the
  [Developer Certificate of Origin](https://developercertificate.org/). There is no CLA.
- Branch from the default branch (`tcpserver-master-build` for now), open the pull request there, and describe
  what you tested and what you could not test.
- Do not mix formatting changes with other changes, and do not reformat upstream-derived files.
- Tests: add one for a bug you fix when it can be tested without the network. Do not commit databases, generated
  configuration or credentials.

## License and names

The project is AGPL-3.0; your contribution is under the same license. "RustDesk" is a third-party name: use it to
describe compatibility or origin only, not as a product name ([ADR 0004](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/adr/0004-license-and-trademark.md)).

## Security

Do not report vulnerabilities in a public issue. See [SECURITY.md](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/SECURITY.md).

## Where to start

Issues labelled `good first issue` are meant for newcomers. The [roadmap](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/roadmap.md) lists what is open.
