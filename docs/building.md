# Building from source

This page is a stop-gap until the developer manual (mdBook) lands.

## Requirements

- Rust (stable), a C toolchain (libsodium is built from source)
- Node.js 20 and npm, only to build the web console and the API documentation

## Build and test

```sh
# the sqlx macros check queries against a database at compile time
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3

cargo build --workspace --locked
cargo test  --workspace --exclude oauth2 --locked -- --skip test_nat64
```

`cargo build` also builds `web/console` and `web/rapidoc` with npm and embeds the result into the API server.
To build without Node.js, set `SCTGDESK_SKIP_WEB_BUILD=1`: the console and API docs are then placeholders.

## Layout

| Path | Content |
|---|---|
| `src/`, `libs/hbb_common` | the RustDesk server (upstream code, kept close to upstream) |
| `crates/api-server` | REST API server, embedded in `hbbs` |
| `crates/api-{state,utils,oauth2,s3software}` | libraries of the API server |
| `web/console`, `web/rapidoc` | Vue web console, API documentation |

See [`docs/adr/`](adr/) for the reasoning behind this layout.

## Known gaps

- The unit tests of `crates/api-oauth2` do not compile; `hbb_common`'s `test_nat64` needs network access.
- `s3software`'s test that calls a live S3 bucket is ignored; run it explicitly with `--ignored`.
- `cargo audit` reports known advisories; the CI job is informational until they are triaged.
