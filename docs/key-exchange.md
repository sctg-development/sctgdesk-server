# TCP key exchange

Clients that connect to `hbbs` over TCP or WebSocket expect an encrypted channel. Upstream's server has no
handler for it; ours does (`key_exchange_phase1` and the phase-2 handler in `src/rendezvous_server.rs`).

## The exchange

1. The server sends a `KeyExchange` message: its ephemeral X25519 public key signed with the server key
   (`id_ed25519`), the version it speaks, and signed parameters (`KxParams`).
2. The client verifies the signature with the public key it was given, generates a symmetric key and sends it
   sealed to the server's ephemeral key (`crypto_box`), with its own ephemeral key and the version it picked.
3. Both sides derive the stream keys.

## Versions

| Version | Stream keys | Notes |
|---|---|---|
| 0 | one key for both directions | the original scheme, what older clients speak |
| 1 | one key per direction, derived from the transcript | signed `KxParams`, defined in `hbb_common` (`KX_VERSION_LATEST = 1`) |

The server advertises version 1 and accepts the version the client picks (the highest both speak). Bit 255 of the
advertised public key is set: X25519 ignores it, so version-0 clients are unaffected, while version-1 clients
take it as "signed parameters are required" and refuse a downgrade by a man in the middle.

`--require-kx-v1` (or `REQUIRE_KX_V1=Y`) refuses clients that pick version 0. It is off by default.

## Tests

`cargo test -p hbbs key_exchange` runs the unit tests: the key sealed by a client is recovered, a corrupted or
foreign box is rejected without panicking, the advertised key still opens what the client sealed to it, and
client and server version-1 streams interoperate.

The real check, done after each upstream sync, is to run the **upstream client's own `key_exchange()` code**
against a running `hbbs` and send one encrypted request. It negotiated version 1 on the day it was written; the
harness is not in the repository yet (see the [roadmap](roadmap.md)).

## Not covered

A released official client binary, the WebSocket path and `hbbr` have not been tested with this exchange. The
`--require-kx-v1` refusal has no test. Reasoning behind the design: [ADR 0003](adr/0003-tcp-and-key-exchange.md).
