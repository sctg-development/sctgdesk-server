<p align="center">
  <a href="#install">Install</a> •
  <a href="#first-run">First run</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#clients">Clients</a> •
  <a href="#build-it-yourself">Build</a> •
  <a href="https://sctg-development.github.io/sctgdesk-server/">Manual</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>] | [<a href="README-RU.md">Русский</a>]<br>
</p>

# SCTGDesk

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![CI](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml)

SCTGDesk is a self-hosted remote-desktop server, with its own clients, derived from
[RustDesk](https://github.com/rustdesk/rustdesk). It is free software under the **AGPL-3.0**.

> **SCTGDesk is an independent project. It is not affiliated with, endorsed by or sponsored by the RustDesk project
> or Purslane Ltd.** "RustDesk" is the name of that project; it is used here only to say what this software is
> derived from and compatible with.

This repository is the whole project: the server, the API server, the web console and the client overlay. It replaces
the former repositories `sctgdesk-api-server` and `sctgdesk`, which are archived.

## What you get

*   The RustDesk server: `hbbs` (ID and rendezvous server) and `hbbr` (relay), plus `rustdesk-utils` (key pairs,
    connectivity check). Compatible with the official RustDesk clients.
*   The server side of the **encrypted TCP and WebSocket connection** (key exchange versions 0 and 1), which the
    open-source upstream server does not implement. This is what makes the **web client** work.
*   A **REST API** embedded in `hbbs`, compatible with the clients: login (password, OpenID Connect / OAuth2:
    GitHub, GitLab, Google, Dex...), sessions that survive a restart, personal and shared address books (per user and
    per group, read-only / read-write / admin), users and groups, software-update endpoints.
*   A **web console** to manage all of that, and interactive API documentation.
*   A **connection history** with a note on each connection (what was done), searchable and exportable to CSV, filled by the RustDesk clients.
*   An option to refuse unauthenticated control (`--logged-in-only`).
*   A **client** (`client/`): upstream RustDesk plus a small overlay, branded at build time with your name, servers and
    key; and a **web client**.

Compatibility comes first: the server must keep working with official RustDesk clients, and our changes to upstream
files are kept small so that new upstream releases can be merged.

## Install

Pick one. Whatever you choose, you need a **key pair** and the ports below.

### Linux packages (deb, rpm)

Each release has `sctgdesk-server-hbbs`, `sctgdesk-server-hbbr` and `sctgdesk-server-utils` for **amd64** and
**arm64**, as `.deb` and `.rpm`, built from static musl binaries (no library dependency). They replace the former
`rustdesk-server-*` packages.

```bash
sudo apt install ./sctgdesk-server-hbbs_*_amd64.deb ./sctgdesk-server-hbbr_*_amd64.deb ./sctgdesk-server-utils_*_amd64.deb
# or: sudo dnf install ./sctgdesk-server-*.rpm
```

A first install starts nothing: edit the units (`systemctl edit --full rustdesk-hbbs`, see
[the deployment guide](docs/deployment.md)), then `sudo systemctl enable --now rustdesk-hbbs rustdesk-hbbr`. An
upgrade restarts the services that were running. Data is in `/var/lib/rustdesk-server`, logs in
`/var/log/rustdesk-server`; the units run as root.

### Binaries (Linux, Windows, macOS)

The [releases](https://github.com/sctg-development/sctgdesk-server/releases) page also has zip archives with `hbbs`,
`hbbr` and `rustdesk-utils` for Windows (amd64, arm64) and macOS (Intel, Apple Silicon), with `SHA256SUMS-*` files.
Unpack, then run them from the directory where the data should live (see [First run](#first-run)).

### Docker

Images are published on [Docker hub](https://hub.docker.com/r/sctg/sctgdesk-server/) as `sctg/sctgdesk-server`.
Save as `docker-compose.yml`, replacing `sctgdesk.example.com` with the address of your server:

```yaml
networks:
  sctgdesk-net:
    external: false

services:
  hbbs:
    container_name: hbbs
    ports:
      - 21114:21114
      - 21115:21115
      - 21116:21116
      - 21116:21116/udp
      - 21118:21118
    image: sctg/sctgdesk-server:latest
    command: hbbs -r sctgdesk.example.com:21117
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    depends_on:
      - hbbr
    restart: unless-stopped

  hbbr:
    container_name: hbbr
    ports:
      - 21117:21117
      - 21119:21119
    image: sctg/sctgdesk-server:latest
    command: hbbr
    volumes:
      - ./data:/usr/local/share/sctgdesk
    networks:
      - sctgdesk-net
    restart: unless-stopped
```

```bash
mkdir -p data && docker compose up -d
```

For SELinux, append `:z` to the volume, or use `--security-opt label=disable`. Without `--net=host` direct peer-to-peer
connections cannot work (everything goes through the relay). A Helm chart is in `deploy/helm` (to be refactored) and
an example manifest in `kubernetes/`.

### Ports

| Port | Protocol | Used by | Purpose |
|---|---|---|---|
| 21114 | TCP | `hbbs` | REST API, web console, API documentation (`--api-port`) |
| 21115 | TCP | `hbbs` | NAT type test |
| 21116 | TCP + **UDP** | `hbbs` | ID / rendezvous |
| 21117 | TCP | `hbbr` | relay |
| 21118 | TCP | `hbbs` | WebSocket (web client) |
| 21119 | TCP | `hbbr` | WebSocket (web client) |

Open them on your firewall. The API and WebSocket ports should be behind a TLS reverse proxy on the Internet: see
the [deployment guide](docs/deployment.md).

## First run

`hbbs` and `hbbr` keep their data (key pair, SQLite database) in the **current directory**.

```bash
mkdir -p /var/lib/sctgdesk && cd /var/lib/sctgdesk
hbbr &
hbbs -r relay.example.com &
```

On the first start `hbbs` creates `id_ed25519` (secret) and `id_ed25519.pub` (the **public key**, which you give to
your clients). To make a key pair yourself, or to check one:

```bash
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

To refuse clients that do not have the key, start both servers with `-k <key>` (or `KEY=<key>`). Check that a
server answers on all its ports with `rustdesk-utils doctor <server>`.

**Default admin.** The web console is at `http://<server>:21114/` and the API documentation at
`http://<server>:21114/api/doc/`. A new database has the user `admin` with the password `Hello,world!`, which is
public: **change it at the first login**.

## Configuration

`hbbs` and `hbbr` take options on the command line (`hbbs --help`) and from environment variables, or an `.env`
file.

| variable | binary | description |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | if set to **"Y"** disallows direct peer connection |
| DB_URL | hbbs | path of the SQLite database (default `./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | hbbs | if set to **"Y"** only logged-in users can control a peer (same as `--logged-in-only`) |
| REQUIRE_KX_V1 | hbbs | if set to **"Y"** refuses clients that use the legacy (version 0) TCP key exchange (same as `--require-kx-v1`) |
| DOWNGRADE_START_CHECK | hbbr | delay (in seconds) before downgrade check |
| DOWNGRADE_THRESHOLD | hbbr | threshold of downgrade check (bit/ms) |
| KEY | hbbs/hbbr | if set force the use of a specific key, if set to **"\_"** force the use of any key |
| LIMIT_SPEED | hbbr | speed limit (in Mb/s) |
| OAUTH2\_CONFIG_FILE | hbbs | path of the OAuth2 / OpenID Connect providers file (`oauth2.toml`) |
| OAUTH2\_CREATE_USER | hbbs | if set to **"1"** create a user at the first login with a provider |
| PORT | hbbs/hbbr | listening port (21116 for hbbs - 21117 for hbbr) |
| RELAY | hbbs | IP address/DNS name of the machines running hbbr (separated by comma) |
| RUST_LOG | all | set debug level (error|warn|info|debug|trace) |
| S3CONFIG_FILE | hbbs | path of the software-update file (`s3config.toml`) |
| SINGLE_BANDWIDTH | hbbr | max bandwidth for a single connection (in Mb/s) |
| TOTAL_BANDWIDTH | hbbr | max total bandwidth (in Mb/s) |

### Only logged-in users can control

`--logged-in-only` (or `LOGGED_IN_ONLY=Y`) makes `hbbs` ask the API server whether the token of the client is a live
session before it connects two peers. Anyone can still register their device, but nobody can control a device
without logging in. A client that is not logged in sees "The connection is not allowed. You have not logged in."; one
whose session ended sees "Your session expired."

### Login with GitHub, GitLab, Google, Dex...

Copy `oauth2.toml` and fill in the providers you use (each needs an application registered with the provider, whose
callback is your API server), then set `OAUTH2_CONFIG_FILE=oauth2.toml`, and `OAUTH2_CREATE_USER=1` to create the
user at the first login. See the [deployment guide](docs/deployment.md#login-with-openid-connect-and-oauth2).

### Updates of the client

The clients can look for new versions on your API server instead of GitHub: it answers
`POST /api/software/version/latest` (the check of current RustDesk clients) and `GET /api/software/releases/latest`.
The latest version and the download links are read from `s3config.toml` (S3-compatible storage), see
[the REST API](docs/api.md#software-update-endpoints).

## Clients

*   **Official RustDesk clients** work with this server: give them your server and your public key.
*   **The SCTGDesk client** is upstream RustDesk plus our overlay (name, icons, default servers and key). You build it
    for your own servers, see [custom builds](docs/custom-builds.md#the-client). The **web client** is built from
    the same overlay.

## Build it yourself

You need a Rust toolchain and, for the web console, Node.js 20.

```bash
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3     # the sqlx macros check queries at compile time
cargo build --release --locked                         # hbbs, hbbr, rustdesk-utils in target/release
```

Set `SCTGDESK_SKIP_WEB_BUILD=1` to build without Node.js (the console is then a placeholder). The
[developer manual](https://sctg-development.github.io/sctgdesk-server/) covers building, testing, custom server and
client builds, packaging, releases and syncing with upstream.

## Screenshots

### Webconsole

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">

### Api documentation

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### In the client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Security

*   The embedded API server is not protected against abuse: put a TLS reverse proxy (HAProxy, NGINX, Caddy...) in front
    of it, and of the WebSocket ports. A HAProxy example is in the [deployment guide](docs/deployment.md#tls-reverse-proxy).
*   Change the default admin password, keep `id_ed25519` secret and back it up: changing it makes every client reject
    your server.
*   Report vulnerabilities as described in [SECURITY.md](SECURITY.md).

## License and source code

SCTGDesk is distributed under the [GNU Affero General Public License v3](LICENSE). The source code of every release
is in this repository, at the tag of that release. If you modify this software and let other people use it over a
network, section 13 of the license requires you to offer them the corresponding source code of your modified version.

Check the SHA-256 sums published with each release before running a downloaded binary.

## Contributing

[Developer manual](https://sctg-development.github.io/sctgdesk-server/) · [CONTRIBUTING.md](CONTRIBUTING.md) ·
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) · [SECURITY.md](SECURITY.md). For general self-hosting questions, the
[RustDesk documentation](https://rustdesk.com/docs/en/self-host/) also applies.

If you like this work, consider giving it a star, or
[sponsoring it](https://github.com/sponsors/sctg-development).
