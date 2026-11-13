# Deploying a server

This guide takes a server from nothing to a production setup: install, key, ports, TLS, login, updates, backup and
rollback. The [README](https://github.com/sctg-development/sctgdesk-server#readme) has the short version.

## What runs where

| Binary | Role | Ports |
|---|---|---|
| `hbbs` | ID / rendezvous server, with the REST API and the web console | 21116 TCP+UDP, 21115, 21118 (WebSocket), 21114 (API) |
| `hbbr` | relay | 21117, 21119 (WebSocket) |

One `hbbs` and one `hbbr` are enough for a team. `hbbs` is told where the relay is with `-r <host[:port]>` (or
`RELAY`). Both keep their files in the **working directory**: the key pair `id_ed25519` / `id_ed25519.pub`, and for
`hbbs` the SQLite database `db_v2.sqlite3` (or the file of `DB_URL`).

## Install

### Packages (Linux, amd64 and arm64)

```sh
sudo apt install ./sctgdesk-server-hbbs_*.deb ./sctgdesk-server-hbbr_*.deb ./sctgdesk-server-utils_*.deb
# Fedora, RHEL: sudo dnf install ./sctgdesk-server-*.rpm
```

The packages contain static musl binaries (`/usr/bin/hbbs`, `hbbr`, `rustdesk-utils`) and the systemd units
`rustdesk-hbbs.service` and `rustdesk-hbbr.service`. They replace the former `rustdesk-server-*` packages, and use the
same directories: `/var/lib/rustdesk-server` (data, mode 700) and `/var/log/rustdesk-server` (logs).

- **A first install on an empty data directory works at once**: the services are enabled and started with the
  defaults, and `hbbs` creates a random key pair (`id_ed25519`, see [the key pair](#the-key-pair)). `hbbr` is started
  with `-k _`, which makes it use the same key.
- **An install that finds data starts nothing** (a migration from `rustdesk-server-*`, or a reinstall): the options of
  the old unit are not carried over, and a default `hbbs` takes the API port 21114, which a reverse proxy may hold.
  Set the options, then `systemctl enable --now rustdesk-hbbs rustdesk-hbbr`.
- An upgrade restarts the services that run; a removal stops and disables them and keeps the data.
- **Before every install, upgrade or migration the packages copy the data directory** to
  `/var/backups/sctgdesk-server/<date>/` (mode 700, the 3 latest are kept; the database with SQLite's backup).

#### Migrating from the former `rustdesk-server-*` packages

Installing `sctgdesk-server-*` removes the former packages. **Do not purge them** (`dpkg -P`, `apt purge`): their
`postrm` deletes the whole `/var/lib/rustdesk-server`, the database and the key pair included. If you do, restore from
`/var/backups/sctgdesk-server/`. A removal without purge leaves the data and the old units' drop-ins in place, and
`apt` keeps a `rc` entry for them, which is harmless.

### Options

The units read `/etc/default/sctgdesk-hbbs` and `/etc/default/sctgdesk-hbbr` (mode 600, kept by the package when you
edit them). `HBBS_ARGS` and `HBBR_ARGS` replace the arguments of the program; the other lines are environment
variables:

```sh
# /etc/default/sctgdesk-hbbs
HBBS_ARGS=-r relay.example.com -k <your-key> --logged-in-only --api-port=21113
OAUTH2_CREATE_USER=1
```

then `sudo systemctl restart rustdesk-hbbs`. The key is in this file, not in the unit, so keep it at mode 600: the
units and their drop-ins in `/etc/systemd/system` are readable by every user of the machine. For more than options
(a dedicated user, other limits) use `sudo systemctl edit rustdesk-hbbs`, which survives upgrades. The units run as
root, like the former packages; to run as a dedicated user, create it, give it the two directories and set `User=`
and `Group=`. The configuration files named in `Environment=` (`oauth2.toml`, `s3config.toml`) are relative to
`WorkingDirectory=/var/lib/rustdesk-server/`.

### Archives (Windows, macOS, Linux)

Download the zip of your platform from the release, check it against `SHA256SUMS-*`, unpack it in the directory that
will hold the data and run `hbbr` and `hbbs -r <relay>` from there. On Windows, run them as a service with the tool
of your choice (NSSM, the Task Scheduler); open the ports in the Windows firewall. The macOS binaries are not signed
or notarized: Gatekeeper asks for confirmation (`xattr -d com.apple.quarantine <file>`).

### Docker and Kubernetes

See the README for `docker compose`. The image keeps its data in `/usr/local/share/sctgdesk`: mount a volume there or
the key pair is lost with the container. `kubernetes/example.yaml` and `deploy/helm/sctgdesk` are starting points,
not tested recipes.

## The key pair

Clients encrypt their traffic to the key of `hbbs`. On the first start, `hbbs` creates `id_ed25519` and
`id_ed25519.pub` if they are missing; the content of the `.pub` file is the key you give to clients. To prepare a
pair yourself:

```sh
rustdesk-utils genkeypair
rustdesk-utils validatekeypair <public key> <secret key>
```

Write the two lines in `id_ed25519.pub` and `id_ed25519` (mode 600) in the data directory of **both** `hbbs` and
`hbbr`, or pass `-k <public key>` to both. **Back up `id_ed25519`**: if it changes, every client has to be configured
again. Never publish it, nor put it in a repository or an image.

## Ports and firewall

Open 21114-21119 (TCP) and 21116 (UDP) towards the Internet, or only 21115-21117 and 21116/UDP if you serve the
console and the web client through a reverse proxy on 443 (next section). Direct peer-to-peer connections need UDP;
`ALWAYS_USE_RELAY=Y` disables them if you prefer every session to go through your relay.

Check from another machine: `rustdesk-utils doctor <your server>`.

## TLS reverse proxy

The embedded API server speaks plain HTTP and has no protection against abuse. Browsers need **TLS** for the web
console and for the web client's WebSockets. Put a reverse proxy in front: it terminates TLS and forwards to
`hbbs` (API, 21114 or the `--api-port` you chose), 21118 (rendezvous WebSocket) and 21119 (relay WebSocket).

The web client (built by [custom builds](custom-builds.md#the-web-client)) talks to `wss://<rendezvous>:21120`
and `wss://<rendezvous>:21121`, so the proxy listens there. This HAProxy configuration is the one used in production,
as an example; adapt names, certificates and addresses:

```haproxy
global
    log /dev/log    local0
    chroot /var/lib/haproxy
    stats socket /run/haproxy/admin.sock mode 660 level admin expose-fd listeners
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
    server srv_main 127.0.0.1:21113     # hbbs --api-port=21113

backend hbbs_wss_backend
    server srv_main 127.0.0.1:21118

backend hbbr_wss_backend
    server srv_main 127.0.0.1:21119
```

Here `hbbs` is started with `--api-port=21113` so that the proxy can own 21114 and 443; the API is then not reachable
without TLS from outside, as long as the firewall does not open 21113. The timeouts above are short: raise
`timeout client` and `timeout server` if WebSocket sessions are cut. `hbbs` does not log the WebSocket rendezvous
requests, so look at the proxy's log when a web client cannot connect.

OIDC login builds its callback from the host the client used (`<host>/api/oidc/callback`), so the proxy has to pass
the `Host` header and `X-Forwarded-Proto` unchanged.

## Only logged-in users can control

`--logged-in-only` (`LOGGED_IN_ONLY=Y`) makes `hbbs` call `POST /api/currentUser` of its own API with the token the
client sends in its connection request. Without a token the client gets "The connection is not allowed. You have not
logged in."; with a token the API refuses, "Your session expired."; and "login server is unreachable" if the API does
not answer. The API must therefore be reachable by `hbbs` on the `--api-port` it was given.

Sessions are stored in the database (a hash of the token, 90 days): restarting `hbbs` does not log anybody out.

## Users, groups and the console

Open `http://<server>:21114/` (or your TLS address), log in as `admin` with the default password `Hello,world!` and
**change it at once**: it is public. The console manages users, groups, devices and address books (read-only,
read-write or admin rules per user or group). The API reference is at `/api/doc/` and in
[REST API](api.md).

## Login with OpenID Connect and OAuth2

Providers are listed in a TOML file (`oauth2.toml` is a commented example) named by `OAUTH2_CONFIG_FILE`:

```toml
[[provider]]
provider = "Github"                       # Github, Gitlab, Google, Facebook, or Custom for any OIDC server (Dex...)
authorization_url = "https://github.com/login/oauth/authorize"
token_exchange_url = "https://github.com/login/oauth/access_token"
app_id = "<client id>"
app_secret = "<client secret>"
scope = "read:user"
op_auth_string = "oidc/github"            # what the client shows and sends
op = "github"
```

Register an application with the provider; its **redirect (callback) URL** is
`https://<your API host>/api/oidc/callback`. With `OAUTH2_CREATE_USER=1` the first login of an unknown user creates
it, otherwise only users that already exist in the console can log in. The file holds secrets: keep it out of
repositories and readable by the service user only. The native client and the web client both use this flow (the
client opens the provider's page, then polls `/api/oidc/auth-query`).

## Software updates of your clients

The client can ask your server for new versions. `hbbs` reads `S3CONFIG_FILE` (`s3config.toml`: S3-compatible
bucket, keys, and the object keys of the installers, with `{version}` as a placeholder) and answers the update check
and the download links. Details and the format are in [REST API](api.md#software-update-endpoints). **The file holds
the credentials of your bucket**: give it a read-only key, never commit the real one, and note that the copy in this
repository is an example only.

### Health probes

- `hbbs`: `GET /livez` (the process answers) and `GET /readyz` (200 when the database answers, 503 otherwise) on the API
  port (21114), without authentication.
- `hbbr`: set `HEALTH_PORT` to a free port; `GET /livez` and `GET /readyz` (200 when the relay port accepts a TCP
  connection) are then answered there. Off by default.

In Kubernetes: `livenessProbe` on `/livez`, `readinessProbe` on `/readyz`. The Helm chart sets them with
`--set probes.enabled=true` (off by default: they exist in the images of this repository, not in the upstream image the
chart still defaults to). Keep `HEALTH_PORT` and the API port off the
internet.

## Backup, upgrade, rollback

Back up `id_ed25519`, `id_ed25519.pub`, `secret_key.txt`, `oauth2.toml`, `s3config.toml` and the database (the packages also copy them to `/var/backups/sctgdesk-server/` at each install, but that is on the same disk). Use SQLite's own backup, which
is safe while `hbbs` runs:

```sh
sqlite3 /var/lib/rustdesk-server/db_v2.sqlite3 ".backup '/root/backup/db_v2.sqlite3'"
cp -p /usr/bin/hbbs /usr/bin/hbbr /root/backup/
```

### Download a backup

With `--allow-remote-backup` (or `ALLOW_REMOTE_BACKUP=Y`, in `/etc/default/sctgdesk-hbbs` for the packages) `hbbs`
lets an **administrator** download a backup, from the console (page **Backup**) or with the API:

```sh
TOKEN=$(curl -s https://api.example.com/api/login -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"...","id":"","uuid":""}' | jq -r .access_token)
curl -fo backup.tar.gz -H "Authorization: Bearer $TOKEN" https://api.example.com/api/backup
```

The archive holds a consistent copy of the database (taken while `hbbs` runs, no `-wal` file to add), `oauth2.toml`,
`s3config.toml`, `id_ed25519`, `id_ed25519.pub`, `secret_key.txt` (the files that exist) and a `MANIFEST.txt`. To
restore, stop `hbbs` and `hbbr`, unpack the files in the data directory (mode 600, and no old `db_v2.sqlite3-wal`
or `-shm` next to the database) and start them.

- It is **off by default**: without the flag the endpoint answers 404, even to an administrator. A plain user, or
  no token, gets 401.
- **The archive holds the private key and the secrets of the server**: an administrator who downloads it can
  impersonate the server. Only enable it if you trust every administrator, serve the API over TLS (a reverse proxy),
  and keep the archives encrypted. Each download is logged at warning level with the name of the administrator.
- The files are those of the working directory of `hbbs` (`OAUTH2_CONFIG_FILE` and `S3CONFIG_FILE` are followed).
  With `ROCKET_SECRET_KEY` set there is no `secret_key.txt`. It does not include `hbbr`'s own files.

Upgrade with the packages, or replace the binaries and restart:

```sh
install -m755 hbbs-new /usr/bin/hbbs && systemctl restart rustdesk-hbbs
```

The schema changes of the database are migrations applied by `hbbs` at start; a new table stays in place if you go
back, so **rolling back the binary is enough** unless a release note says otherwise:

```sh
install -m755 /root/backup/hbbs /usr/bin/hbbs && systemctl restart rustdesk-hbbs
```

After an upgrade: `hbbs --version`, the console loads, an official client connects, and with `--logged-in-only` a
logged-in client connects and a logged-out one is refused. The [key exchange check](testing.md) lists what to try
with a released official client.

## Troubleshooting

| Symptom | Look at |
|---|---|
| clients say "key mismatch" or never connect | the key in the client is not the content of `id_ed25519.pub`; `hbbs` and `hbbr` use different keys or `-k` values |
| only the relayed connection works | UDP 21116 is not open, or `ALWAYS_USE_RELAY=Y` |
| "You have not logged in." | the client has no token: it must log in (a web client needs the account of the API server of its own build) |
| "Your session expired." | the token is unknown to the API server: log in again; check that `--api-port` matches the API |
| web client cannot connect | the proxy's log; certificates; 21120 and 21121 open; the `Host` header |
| nothing in `hbbs`'s log | set `RUST_LOG=debug`; the WebSocket rendezvous requests are not logged by `hbbs` |
