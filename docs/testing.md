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

## End-to-end tests of `hbbs`

`tests/hbbs_e2e.rs` starts the real `hbbs` binary in a temporary directory and talks to it the way the official
client does: the key exchange of [TCP key exchange](key-exchange.md) is the client's own code, then a punch hole
request, and plain HTTP to the API. They exist because each of these failed in production and no unit test could see it:

| Test | What it prevents |
|---|---|
| `a_malformed_bearer_token_does_not_stop_the_server` | one request with `Authorization: Bearer x` aborted the whole process (the API server runs inside `hbbs`) |
| `logged_in_only_answers_when_the_api_server_is_unreachable` | with `--logged-in-only` the client waited forever for an answer |
| `logged_in_only_checks_tokens_on_the_configured_api_port` | the token check ignored `--api-port` and called port 21114 |
| `key_exchange_works_with_the_official_client_logic` | a change that breaks the official client's handshake |

`hbbs` listens on fixed ports (21115, 21116, 21118), so these tests run one after the other and need the ports free:
stop a local `hbbs` first.

They are checked to fail without the fixes: with the previous code the first three fail (the server aborts, or no
answer within 15 s). The last one is a guard and passes on both.

This does not replace a test with a released official client, which is still done by hand after an upstream sync or a
change in `rendezvous_server.rs`.

## Writing tests

- Prefer small tests next to the code, like `key_exchange_tests` in `src/rendezvous_server.rs`.
- Tests must not need the network or your production database: use a temporary SQLite file.
- Do not commit generated files (`s3config.toml` credentials, databases touched by a build).
