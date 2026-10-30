# REST API

The API server (`crates/api-server`, Rocket) runs inside `hbbs` on port 21114. It serves the endpoints used by
RustDesk clients (login, address books, heartbeat, system info, audit), the ones used by the web console
(users, groups, peers, shared address books and their rules, OIDC settings) and a few for software updates.

- **Interactive documentation**: `http://<server>:21114/api/doc/` (RapiDoc, generated from the code with
  `rocket_okapi`); the specification is `openapi.json` next to it. This is the reference: the list below is a map.
- **Web console**: `http://<server>:21114/` (Vue app in `web/console`, embedded at build time).
- **Authentication**: `POST /api/login` returns an access token; routes guarded by `AuthenticatedUser` or
  `AuthenticatedAdmin` expect it as a bearer token. A fresh database contains an `admin` user: change its
  password at first login (see the README).

## Endpoint groups

| Prefix | Purpose |
|---|---|
| `/api/login`, `/api/logout`, `/api/currentUser`, `/api/login-options` | sessions |
| `/api/oidc/*` | OpenID Connect login and settings |
| `/api/ab*` | address books: personal, shared, peers, tags, rules |
| `/api/user*`, `/api/enable-users`, `/api/group*` | users and groups |
| `/api/peers*`, `/api/heartbeat`, `/api/sysinfo`, `/api/audit`, `/api/stategies` | peers and their reports |
| `/api/software/*` | version and download endpoints, below |

## Software update endpoints

| Route | Used by |
|---|---|
| `POST /api/software/version/latest` | current RustDesk clients: same request and response as `api.rustdesk.com/version/latest`; the body, device fingerprint included, is ignored and not logged |
| `GET /api/software/releases/latest` | redirects to `/api/software/releases/tag/<version>`; imitates GitHub's "latest release" |
| `GET /api/software/releases/tag/<version>` | the target of the redirect |
| `GET /api/software/download` | redirects to the download page |
| `GET /api/software/version/server` | version of the server and of the latest client |

The client download links, the latest client version and the download page come from `s3config.toml` (see
`crates/api-s3software`).

A key of that file (`Windows64Key`, `OSXKey`, `IOSKey`, ...) is either the object itself, or contains `{version}`:

```toml
Windows64Key = "master/sctgdesk-releases/sctgdesk-{version}-x86_64.exe"
OSXKey       = "master/sctgdesk-releases/sctgdesk-{version}-x86_64.dmg"
OSXArm64Key  = "master/sctgdesk-releases/sctgdesk-{version}-aarch64.dmg"
IOSKey       = "master/sctgdesk-releases/sctgdesk-{version}.ipa"
ListBucket   = "master"   # Oracle Cloud only, see below
```

The API then lists the bucket for the prefix before `{version}` and picks the key with the highest `x.y.z`: the
whole key has to match, so `sctgdesk-{version}-x86_64.dmg` does not pick `sctgdesk-1.4.0-web-basic.zip`, and `1.10.0` is
higher than `1.9.3`. The answer is kept for five minutes. `GET /api/software/version/latest` reports the version
found that way. A key without `{version}` works as before and does not call the bucket. If the bucket cannot be
listed, the link answers 404 and the reason is in the log.

`GET /api/software/download/<key>` (`w64`, `w32`, `osx`, `osx-arm64`, `ios`) redirects to a freshly signed link of the
newest client, so its address does not expire: the update card of the clients opens it for their platform.

On Oracle Cloud, `Bucket` is the namespace and the keys start with the real bucket (`master/...`): set
`ListBucket` to that bucket, otherwise the listing goes to the wrong place and finds nothing.

## Where the code is

`crates/api-server/src/lib.rs` holds the routes; the database layer and its migrations are in `crates/api-state`
(`db_v2/migrations`, checked by `sqlx` at compile time, hence `DATABASE_URL` when building).
