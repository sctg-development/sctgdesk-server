<p align="center">
  <a href="#quick-start">Quick start</a> •
  <a href="#docker">Docker</a> •
  <a href="#how-to-create-a-keypair">Keypair</a> •
  <a href="#building-from-source">Build</a> •
  <a href="#env-variables">Variables</a> •
  <a href="https://sctg-development.github.io/sctgdesk-server/">Developer manual</a><br>
  [<a href="README-FR.md">French</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-TW.md">繁體中文</a>] | [<a href="README-ZH.md">简体中文</a>] | [<a href="README-RU.md">Русский</a>]<br>
</p>

# SCTGDesk

![Docker Pulls](https://img.shields.io/docker/pulls/sctg/sctgdesk-server)
[![CI](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/ci.yml)

[**Binary Download**](https://github.com/sctg-development/sctgdesk-server/releases) •
[**Developer manual**](https://sctg-development.github.io/sctgdesk-server/) •
[**Contributing**](CONTRIBUTING.md)

SCTGDesk is a self-hosted remote-desktop server (and a client) derived from [RustDesk](https://github.com/rustdesk/rustdesk), free and open source under the **AGPL-3.0**.

> **SCTGDesk is an independent project. It is not affiliated with, endorsed by or sponsored by the RustDesk project or Purslane Ltd.**
> "RustDesk" is the name of that project; it is used here only to say what this software is derived from and compatible with.

What it adds to the RustDesk server (`hbbs`, `hbbr`):

* The server side of the **encrypted TCP/WebSocket connection** (key exchange versions 0 and 1). Upstream's open-source server does not implement it.
* An **API server** compatible with the RustDesk clients, embedded in `hbbs` (still a preliminary implementation):
  * Support for personal address book
  * Support for shared address book at group level
    * read-only, read-write, admin
  * Support for shared address book at user level
    * read-only, read-write, admin
* A simple **web console**.

The web console is at `http://<server-ip>:21114/` (see [Default admin user](#default-admin-user)).
The API documentation is built into the server at `http://<server-ip>:21114/api/doc/`.

The client (`client/`) is upstream RustDesk plus a small overlay. The server is meant to keep working with the official RustDesk clients: compatibility comes first.

## License and source code

SCTGDesk is distributed under the [GNU Affero General Public License v3](LICENSE). The source code of every release is in this repository, at the tag of that release. If you modify this software and let other people use it over a network, section 13 of the license requires you to offer them the corresponding source code of your modified version.

## Star the project

**If you appreciate my work, please consider giving it a star! 🤩 or a** [![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/sctg-development)

## Quick start

With Docker, save this as `docker-compose.yml`:

```yaml
version: '3'

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

Replace `sctgdesk.example.com` with the address of your server, then:

```bash
mkdir -p data
docker-compose up
```

Open the ports `21114` to `21119` (TCP, and `21116` also in UDP) on your firewall.

## Binaries

Binaries are available for the following platforms:  
- Linux x86_64 statically linked
- Linux arm64 statically linked
- Linux armv7 statically linked
- MacOS Intel
- MacOS Apple Silicon
- Windows x86_64

Get them from the [releases](https://github.com/sctg-development/sctgdesk-server/releases) page. Released binaries after v1.1.99-40 are attested with GitHub Actions: you can check the attestation by looking up the sha256sum of the binary on `https://search.sigstore.dev/?hash=<sha256>`, for example.

### Default admin user

The default admin user is created with the username `admin` and the password `Hello,world!`. **Change the password at the first login** on the web console: it is public.

## API server

The API server and the web console are part of `hbbs` and of this repository (`crates/api-server`, `web/`). The standalone repository [sctgdesk-api-server](https://github.com/sctg-development/sctgdesk-api-server) is replaced by this one: file API and web console issues here.

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

### Use in the client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Updates of the client

The client can look for new versions on your API server instead of GitHub: the server answers `POST /api/software/version/latest` (the check made by current RustDesk clients) and `GET /api/software/releases/latest`. The client in `client/` is already set up this way (see its [README](client/README.md)). The latest version and the download page are read from `s3config.toml`.

# Security


The embedded API server is not secured nor protected agains DDOS attacks. A good practice is to use a reverse proxy in front of the API server. NGINX is a good choice for this purpose. HAProxy is also a good choice.  
We use HAProxy in front of the API server in our production environment.
This is our configuration file for HAProxy it is provided as an example only. You should adapt it to your own needs.:

```haproxy
global
    log /dev/log    local0
    log /dev/log    local1 notice
    chroot /var/lib/haproxy
    stats socket /run/haproxy/admin.sock mode 660 level admin expose-fd listeners
    stats timeout 30s
    user haproxy
    group haproxy
    daemon

defaults
    log global
    retries 2
    timeout connect 3000ms
    timeout server 5000ms
    timeout client 5000ms

frontend hbbs_wss
    bind 0.0.0.0:21120 ssl crt /etc/haproxy/hbb.pem
    default_backend hbbs_wss_backend

frontend hbbs_api
    mode http
    option forwardfor
    bind 0.0.0.0:21114 ssl crt /etc/haproxy/api.pem
    http-request set-header X-Forwarded-Proto https
    default_backend hbbs_api_backend

frontend hbbs_api_443
    mode http
    option forwardfor
    bind 0.0.0.0:443 ssl crt /etc/haproxy/api.pem
    http-request set-header X-Forwarded-Proto https
    filter compression
    compression algo gzip
    compression type text/css text/html text/javascript application/javascript text/plain text/xml application/json
    compression offload
    default_backend hbbs_api_backend

frontend hbbr_wss
    bind 0.0.0.0:21121 ssl crt /etc/haproxy/hbb.pem
    default_backend hbbr_wss_backend

backend hbbs_api_backend
    mode http
    server srv_main 127.0.0.1:21113

backend hbbs_wss_backend
    server srv_main 127.0.0.1:21118

backend hbbr_wss_backend
    server srv_main 127.0.0.1:21119
```

The hbbs server is launched with

```service
[Unit]
Description=SCTGDesk signal server

[Service]
Type=simple
LimitNOFILE=1000000
ExecStart=/usr/bin/hbbs --api-port=21113 -k <your-secret-key> --mask=10.10.0.235/24 -r rendez-vous.example.org -R rendez-vous.example.org
#Environment="RUST_LOG=debug"
Environment="ALWAYS_USE_RELAY=Y"
Environment="OAUTH2_CREATE_USER=1"
Environment="S3CONFIG_FILE=s3config.toml"
Environment="OAUTH2_CONFIG_FILE=oauth2.toml"
WorkingDirectory=/var/lib/rustdesk-server/
User=
Group=
Restart=always
StandardOutput=append:/var/log/rustdesk-server/hbbs.log
StandardError=append:/var/log/rustdesk-server/hbbs.error
# Restart service after 10 seconds if node service crashes
RestartSec=10

[Install]
WantedBy=multi-user.target
```

# Limit Unwanted Access

To restrict access to your server, you can use the `--logged-in-only` option or set the `LOGGED_IN_ONLY=Y` environment variable for the `hbbs` server. This will limit control to logged-in users only.

Even with this option enabled, users can still register on the Rendez-vous server, but they won't be able to control another user's peer. If someone tries to control a peer without being logged in, they'll receive an error message:

<img width="524" alt="Error message for unauthenticated control attempt" src="https://github.com/user-attachments/assets/cfa46504-39d8-46a7-9072-3ece6818b4a3">

By enabling this feature, you can add an extra layer of security to your server and prevent unauthorized access.

**Configuring `LOGGED_IN_ONLY`**

To enable this feature:

1. Set the `LOGGED_IN_ONLY=Y` environment variable for the `hbbs` server.
2. Alternatively, you can use the `--logged-in-only` option when running the `hbbs` server.

**Example**

To set the `LOGGED_IN_ONLY` environment variable, you can add the following line to your `~/.bashrc` file or equivalent:
```bash
export LOGGED_IN_ONLY=Y
```

## Building from source

You need a Rust toolchain and, for the web console, Node.js ≥ 20.

```bash
git submodule update --init                   # libs/hbb_common
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3
cargo build --release
```

On Windows with cmd.exe: `set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3" && cargo build --release`.

To build without Node.js (the web console is then a placeholder), set `SCTGDESK_SKIP_WEB_BUILD=1`.

Three executables are generated in `target/release`:

* hbbs - ID/Rendezvous server with the API server
* hbbr - relay server
* rustdesk-utils - command line utilities (key pairs, connectivity check)

See the [developer manual](https://sctg-development.github.io/sctgdesk-server/) and [docs/building.md](docs/building.md) for the details.

## Docker

Docker images are published on [Docker hub](https://hub.docker.com/r/sctg/sctgdesk-server/) (`sctg/sctgdesk-server:latest`, for amd64, arm64v8 and arm32v7). They are built on `ubuntu-22.04` with only the main binaries (`hbbr` and `hbbs`) added.

You can start them directly with `docker run`:

```bash
docker run --name hbbs --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]>
docker run --name hbbr --net=host -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr
```

or without `--net=host`, but direct P2P connections cannot work:

```bash
docker run --name hbbs -p 21114:21114 -p 21115:21115 -p 21116:21116 -p 21116:21116/udp -p 21118:21118 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbs -r <relay-server-ip[:port]>
docker run --name hbbr -p 21117:21117 -p 21119:21119 -v "$PWD/data:/usr/local/share/sctgdesk" -d sctg/sctgdesk-server:latest hbbr
```

The `relay-server-ip` parameter is the IP address (or DNS name) of the server running these containers. The **optional** `port` has to be used if you use a port different from **21117** for `hbbr`.

For systems using SELinux, replace `/usr/local/share/sctgdesk` by `/usr/local/share/sctgdesk:z` for the containers to run correctly, or disable SELinux container separation with `--security-opt label=disable`.

> [!NOTE]
> In China, the image may be replaced with a version number (for example `sctg/sctgdesk-server:1.1.99-37`): otherwise an old version may be pulled because of image acceleration.

## How to create a keypair

A keypair is needed for encryption; you can provide it, as explained before, but you need a way to create one.

You can use this command to generate a keypair:

```bash
/usr/bin/rustdesk-utils genkeypair
```

If you don't have (or don't want) the `rustdesk-utils` package installed on your system, you can invoke the same command with docker:

```bash
docker run --rm --entrypoint /usr/bin/rustdesk-utils  sctg/sctgdesk-server:latest genkeypair
```

The output will be something like this:

```text
Public Key:  8BLLhtzUBU/XKAH4mep3p+IX4DSApe7qbAwNH9nv4yA=
Secret Key:  egAVd44u33ZEUIDTtksGcHeVeAwywarEdHmf99KM5ajwEsuG3NQFT9coAfiZ6nen4hfgNICl7upsDA0f2e/jIA==
```

## Env variables

`hbbs` and `hbbr` can be configured using these environment variables. You can specify them as usual or use an `.env` file.

| variable | binary | description |
| --- | --- | --- |
| ALWAYS_USE_RELAY | hbbs | if set to **"Y"** disallows direct peer connection |
| DB_URL | hbbs | path of the SQLite database (default `./db_v2.sqlite3`) |
| LOGGED_IN_ONLY | hbbs | if set to **"Y"** only logged-in users can control a peer (same as `--logged-in-only`) |
| REQUIRE_KX_V1 | hbbs | if set to **"Y"** refuses clients that use the legacy (version 0) TCP key exchange (same as `--require-kx-v1`) |
| DOWNGRADE_START_CHECK | hbbr | delay (in seconds) before downgrade check |
| DOWNGRADE_THRESHOLD | hbbr | threshold of downgrade check (bit/ms) |
| KEY | hbbs/hbbr | if set force the use of a specific key, if set to **"_"** force the use of any key |
| LIMIT_SPEED | hbbr | speed limit (in Mb/s) |
| OAUTH2_CONFIG_FILE | hbbs | path for oauth2 config file |
| OAUTH2_CREATE_USER | hbbs | if set to **"1"** create a user if it doesn't exist |
| PORT | hbbs/hbbr | listening port (21116 for hbbs - 21117 for hbbr) |
| RELAY | hbbs | IP address/DNS name of the machines running hbbr (separated by comma) |
| RUST_LOG | all | set debug level (error\|warn\|info\|debug\|trace) |
| S3CONFIG_FILE | hbbs | path for s3 config file |
| SINGLE_BANDWIDTH | hbbr | max bandwidth for a single connection (in Mb/s) |
| TOTAL_BANDWIDTH | hbbr | max total bandwidth (in Mb/s) |

## Documentation and contributing

* [Developer manual](https://sctg-development.github.io/sctgdesk-server/): architecture, key exchange, API, building, upstream sync, roadmap.
* [CONTRIBUTING.md](CONTRIBUTING.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), [SECURITY.md](SECURITY.md).
* For general self-hosting questions about the RustDesk server, the [RustDesk documentation](https://rustdesk.com/docs/en/self-host/) also applies.
