# Introduction

SCTGDesk is a self-hosted remote-desktop server and client derived from [RustDesk](https://github.com/rustdesk/rustdesk).
This manual is for people who want to **build, change or maintain** it. For installing and running a server, see the
[README](https://github.com/sctg-development/sctgdesk-server#readme).

## What this repository contains

One repository, kept as close to upstream as possible so that new RustDesk releases can be merged:

- the **RustDesk server** (`hbbs`, `hbbr`, `rustdesk-utils`), taken from [`rustdesk/rustdesk-server`](https://github.com/rustdesk/rustdesk-server);
- our **REST API server** and **Vue web console**, embedded in `hbbs` (address books, users, groups, OIDC, software update endpoints);
- the **client**, as a small overlay applied on a pristine upstream RustDesk release (`client/`).

What is ours and what is upstream's is spelled out in [Architecture](architecture.md) and in the
[decision records](adr/).

## Compatibility is the first rule

The client must keep working with official RustDesk clients, and our server with official clients. When in doubt,
follow upstream's behaviour and keep our changes small and isolated: that is what makes the monthly
[upstream sync](upstream-sync.md) cheap.

## License and trademark

The project is **AGPL-3.0**, like upstream. A modified version offered over a network must offer its source to its
users (AGPL section 13). "RustDesk" is a third-party name: we use it only to describe compatibility and origin
("a fork of RustDesk"), never as the name of our product. SCTGDesk is not affiliated with or endorsed by the
RustDesk project or Purslane Ltd. See [ADR 0004](adr/0004-license-and-trademark.md).

## Status

The project is being consolidated. Anything marked *not verified* in this manual has not been exercised yet; the
[roadmap](roadmap.md) lists what is open.
