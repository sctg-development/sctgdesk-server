# Architecture

## Components

| Binary | Role | Default ports |
|---|---|---|
| `hbbs` | ID / rendezvous server, with the REST API and web console embedded | 21116 TCP+UDP (rendezvous), 21115 TCP (NAT test), 21118 TCP (WebSocket), 21114 TCP (API, console) |
| `hbbr` | relay server | 21117 TCP, 21119 TCP (WebSocket) |
| `rustdesk-utils` | key pair generation and validation, connectivity check (`doctor`) | – |

The ports are defined in `libs/hbb_common/src/config.rs` and `src/main.rs`; `rustdesk-utils doctor` checks them.

`hbbs` stores peers and the API data in a SQLite database (`db_v2.sqlite3`, or the file given by `DB_URL`).
Its key pair is `id_ed25519` / `id_ed25519.pub` in the working directory; clients are given the public key.

## Repository layout

| Path | Content | Origin |
|---|---|---|
| `src/` | `hbbs`, `hbbr`, `rustdesk-utils` | upstream, with our changes in `main.rs` and `rendezvous_server.rs` |
| `libs/hbb_common` | shared protocol library | git submodule on `rustdesk/hbb_common`, unmodified |
| `crates/api-server` | REST API, embedded into `hbbs` | ours |
| `crates/api-{state,utils,oauth2,s3software}` | libraries of the API server | ours |
| `web/console`, `web/rapidoc` | Vue web console, API documentation | ours |
| `client/` | patches, overlay and branding for the client | ours, on top of upstream |
| `deploy/helm` | Helm chart (to be refactored) | ours |
| `docs/` | this manual | ours |

All the crates live in one Cargo workspace; see [ADR 0005](adr/0005-workspace-structure.md).

## What is ours in upstream files

Kept deliberately small, see [Syncing with upstream](upstream-sync.md):

- `src/main.rs`: starts the API server, defines `API_PORT`;
- `src/rendezvous_server.rs`: the TCP [key exchange](key-exchange.md) and its tests;
- `src/common.rs`: the `--require-kx-v1` flag.

## Transports

`hbbs` speaks UDP, TCP and WebSocket natively (upstream). On TCP and WebSocket, the connection is encrypted by a
key exchange that our server implements and upstream's server does not.
