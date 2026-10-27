# Architecture Decision Records

Decisions that shape the consolidation of SCTGDesk (server, API server, client) into one repository.
Evidence comes from the Phase 0 audit; every upstream claim was checked against the real repositories on 2026-10-27, and what could not be verified is stated as such.

| ADR | Topic | Status | Recommendation |
|---|---|---|---|
| [0001](0001-client-placement.md) | Where the client lives | Proposed | `client/` overlay (patches + assets + CI) in the monorepo, not a full fork copy |
| [0002](0002-upstream-sync-strategy.md) | Upstream synchronisation, `hbb_common` | Proposed | Tag-by-tag merges into a mirror-based flow; `hbb_common` as a submodule pointing at a thin fork |
| [0003](0003-tcp-and-key-exchange.md) | Fate of the TCP code | Proposed | Keep and modernise the key exchange (v1); remove the duplicated client switch and key logging |
| [0004](0004-license-and-trademark.md) | AGPL compliance and the "RustDesk" mark | Proposed | Stay AGPL-3.0, add source offer, nominative use of the mark only |
| [0005](0005-workspace-structure.md) | Cargo workspace layout | Proposed | Upstream layout untouched at the root; our code in `crates/` and `web/` |

Status values: Proposed, Accepted, Superseded. Nothing here is Accepted until the maintainer says so.
