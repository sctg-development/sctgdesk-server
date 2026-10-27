# ADR 0003: Fate of the TCP code

Status: Proposed

## Context

The initial assumption was that the TCP work (2024-04) was a failed attempt and dead code. The audit and a live test show otherwise.

Verified facts (2026-10-27):

- Upstream `hbbs` already speaks **TCP and WebSocket** natively (`Sink::TcpStream`, `Sink::Ws`, WebSocket on port+2, reverse-proxy `X-Real-IP` handling).
- Upstream `hbbs` has **no** `KeyExchange` handler in `src/`. Our fork implements it (`key_exchange_phase1` and the phase-2 handler in `src/rendezvous_server.rs`).
- The upstream client has `secure_tcp`, which tolerates a server without the exchange, and `secure_tcp_required`, which **refuses** such a server so that nothing is sent in clear.
- **Live test**: our `hbbs` against the upstream client's `key_exchange()` (copied verbatim, with the client's exact `hbb_common`): the exchange succeeds at version 0, an encrypted `TestNatRequest` round-trips, and a wrong public key is rejected. Not tested: the full Flutter client, and upstream `hbbs`.
- The client's current scheme is **version 1** (signed `KxParams`, transcript binding). Our server only speaks version 0 and offers no signed parameters.
- Client side, our `RendezvousMediator::is_udp_disabled()` reads `enable-tcp-mode == "N"`. Upstream now has its own `is_udp_disabled()` reading `disable-udp == "Y"`. The two switches are different options with different meanings.
- Both our client (`src/common.rs`) and our server (`rendezvous_server.rs:570`) log the session symmetric key at debug level.
- Issue #16 shows users believe our "TCP mode" lets them cross an HTTPS proxy without UDP; that is a Pro-server feature.

## Options

1. **Delete** all TCP code. Breaks the `secure_tcp_required` path against our server; removes the only OSS server-side key exchange we know of.
2. **Isolate behind a Cargo feature flag.** Keeps the code but ships it off by default; a feature that protocol clients require should not be optional.
3. **Keep, modernise, and trim.**

## Recommendation

Option 3:

- **Keep** the server key exchange. Make it default-on.
- **Upgrade to version 1** (signed `KxParams`, transcript binding), answering version-0 clients as the legacy scheme. Depends on the `hbb_common` bump from ADR 0002 and must be re-tested with the same harness against a recent client.
- **Remove** the symmetric-key debug logs in client and server. This is independent of everything else and can ship first.
- **Remove** our client-side `is_udp_disabled` and `enable-tcp-mode` in favour of upstream's `disable-udp`.
- Add a regression test that runs the key-exchange round-trip in CI (the harness from the audit is the seed).
- Document what "TCP mode" does and does not do, to close issue #16.

## Consequences

- No behaviour is removed from servers that exist today.
- The server pins a newer `hbb_common` than upstream's server does; this is the cost of staying compatible with current clients.
- Open: whether upstream plans its own server-side exchange. If it lands, our implementation becomes a candidate for removal; the CI round-trip test tells us when.
