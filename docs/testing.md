# Testing

```sh
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3
export SCTGDESK_SKIP_WEB_BUILD=1        # no Node.js needed
cargo test --workspace --exclude oauth2 --locked --no-fail-fast -- --skip test_nat64
```

This is what the CI runs (`.github/workflows/ci.yml`), together with `rustfmt` and `clippy` on `crates/*`, a
`cargo build --workspace --locked`, the web console build and an informational `cargo audit`.

## What is excluded, and why

| Test | Reason |
|---|---|
| `crates/api-oauth2` unit tests | they do not compile (they call functions that no longer exist); open issue |
| `hbb_common` `test_nat64` | needs DNS64 and network access |
| `s3software` bucket test | calls the live S3 bucket of `s3config.toml`; ignored, run with `--ignored` |

## Protocol compatibility

Unit tests cannot say whether a real client still works. After an upstream sync or a change in
`rendezvous_server.rs`, check the [TCP key exchange](key-exchange.md) with a client's own code against a running
`hbbs`, and when possible with a released official client.

## Writing tests

- Prefer small tests next to the code, like `key_exchange_tests` in `src/rendezvous_server.rs`.
- Tests must not need the network or your production database: use a temporary SQLite file.
- Do not commit generated files (`s3config.toml` credentials, databases touched by a build).
