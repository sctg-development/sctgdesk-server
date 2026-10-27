# Syncing with upstream

We follow `rustdesk/rustdesk-server` by **release tag**, one tag at a time (see [ADR 0002](adr/0002-upstream-sync-strategy.md)).

```sh
scripts/sync-upstream.sh 1.1.17   # creates sync/upstream-1.1.17, merges, builds, tests
```

The script fetches upstream tags into `refs/upstream-tags/*` (our own tags are never overwritten), merges
without committing, and stops on conflicts. It never pushes.

## Conflict policy

| File | Resolution |
|---|---|
| `Cargo.toml` | keep our `version` and `authors`; take upstream's dependency changes. Upstream's target-specific `reqwest` replaces any plain one. |
| `Cargo.lock` | never hand-merge: take ours, then `cargo build` and commit the result |
| `debian/changelog`, `README*.md` | keep ours |
| `.github/workflows/*` deleted on our side | stay deleted (our CI is `ci.yml`) |
| `libs/hbb_common` | a submodule on `rustdesk/hbb_common`; take upstream's pin, or bump it deliberately (the key exchange needs at least `edefa08`) |
| `src/main.rs`, `src/rendezvous_server.rs` | our code (API server, TCP key exchange) on top of upstream's; keep the diff small |

## What is ours in upstream files

- `src/main.rs`: API server start-up, `API_PORT`; `check_software_update()` is deliberately not called (it queries RustDesk's version endpoint).
- `src/rendezvous_server.rs`: TCP key exchange (versions 0 and 1) and its tests.

After a sync, run the real check against a recent client: `hbbs` must complete the key exchange and answer an encrypted request.
