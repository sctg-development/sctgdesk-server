# ADR 0005: Cargo workspace and repository layout

Status: Proposed

## Context

- Upstream's root `Cargo.toml` is the `hbbs` package with `[workspace] members = ["libs/hbb_common"]` and `exclude = ["ui"]`. Any change we make to those files conflicts at every sync.
- Today the server depends on the API server with `git = ".../sctgdesk-api-server.git", branch = "main"`. Consequence found in the audit: `cargo build --locked` **fails** because the lock no longer matches, and two builds on different days yield different binaries.
- Build problems in the API server: `libs/state/build.rs` opens a SQLite database and inserts a test row at compile time, and requires `DATABASE_URL` as an absolute path (known only to CI); the root `build.rs` runs `npm run build` and rewrites the tracked `webconsole/package.json`.
- Neither project runs meaningful tests today (0 executed by `cargo test`).

## Options

1. Move everything to `crates/*` and make the root a virtual workspace. Cleanest, but moves upstream's files: every sync becomes a rename-conflict exercise.
2. **Keep upstream's layout at the root**, add our code as separate workspace members under `crates/` and `web/`.
3. Keep the git dependency. Rejected: reproducibility problem above.

## Recommendation

Option 2:

```
Cargo.toml              # upstream root package (hbbs); only [workspace] and one dependency line differ
src/ ui/ libs/hbb_common  # upstream layout (hbb_common becomes a submodule, ADR 0002)
crates/
  api-server/           # was sctgdesk-api-server (history preserved)
  api-oauth2/ api-s3software/ api-state/ api-utils/   # its former libs/*
web/console/            # Vue console (was webconsole/), own package.json, own build
client/                 # overlay, ADR 0001
docs/                   # mdBook + ADRs
scripts/                # sync-upstream.sh, dev helpers
```

- The server depends on `crates/api-server` by **path**: reproducible, `--locked` works.
- Fix the build before the import is considered done: replace the compile-time SQLite access with sqlx offline mode (`.sqlx/` query metadata) and move the test row into a test fixture; run the console build from a `xtask`/Makefile target (`cargo xtask console`) with the built assets either committed or embedded through an explicit feature, and never mutate tracked files.
- `[workspace] members = [".", "libs/hbb_common", "crates/*"]`; the one-line `Cargo.toml` edit is the entire diff to an upstream file.
- CI minimum (Phase 3): `cargo fmt --check`, `cargo clippy`, `cargo build --locked`, `cargo test --workspace`, `cargo audit`, `cargo deny`, console build, and the key-exchange round-trip test from ADR 0003.

## Consequences

- The import in Phase 3 renames paths inside the API server history (using `git filter-repo --to-subdirectory-filter` or `git subtree`; the choice is made and justified in Phase 3 after looking at the noisy "wip" history).
- Known blocker: audit R1 (33-41 RUSTSEC advisories). The workspace lets us run one `cargo update` and one audit for everything; advisories that need real work (for example `sodiumoxide`, unmaintained) are tracked as separate issues.
