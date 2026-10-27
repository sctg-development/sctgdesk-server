# History

## Origin

SCTGDesk started as a RustDesk server fork (`sctgdesk-server`) with a separate API server (`sctgdesk-api-server`) and
a client fork (`sctgdesk`). They were consolidated into this repository, keeping the commit history of the API
server (imported under `crates/` and `web/`).

## Tags of the former API server repository

The former `sctgdesk-api-server` repository had three tags. The monorepo has tags of the same names for the
server, so these were **not** copied: they stay in the archived repository, and the commits they pointed to are in
this repository's history.

| Former tag | Commit (in this repository) | Date | What it is |
|---|---|---|---|
| `1.1.99-38` | `2c33baf` | 2024-06-15 | first numbered release ("add openapi.json to release") |
| `1.1.99-39` | `ef3d926` | 2024-08-27 | generic OAuth2 provider |
| `nightly` | `a4eca2e` | 2024-09-07 | moving tag (not meaningful after that date) |

`git show <commit>` works for all of them. The hashes differ from the ones in the former repository because the history was rewritten to move the files under `crates/` and `web/`.
