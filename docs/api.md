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

The latest client version and the download page come from `s3config.toml` (see `crates/api-s3software`).

## Where the code is

`crates/api-server/src/lib.rs` holds the routes; the database layer and its migrations are in `crates/api-state`
(`db_v2/migrations`, checked by `sqlx` at compile time, hence `DATABASE_URL` when building).
