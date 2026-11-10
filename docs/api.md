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
| `/api/login`, `/api/logout`, `/api/currentUser`, `/api/login-options` | sessions: kept in the database (`api_session`, a hash of the token), so they survive a restart of `hbbs` for 90 days or until the logout |
| `/api/oidc/*` | OpenID Connect login and settings |
| `/api/ab*` | address books: personal, shared, peers, tags, rules |
| `/api/user*`, `/api/enable-users`, `/api/group*` | users and groups |
| `/api/device-group/accessible` | the groups, by name, for the "accessible devices" tab of the clients |
| `/api/peers*`, `/api/heartbeat`, `/api/sysinfo`, `/api/audit`, `/api/stategies` | peers and their reports |
| `/api/software/*` | version and download endpoints, below |

## Access control

A bearer token only says who the user is; what the user may see and do depends on the user's rights and group.

**Address books.** The right of a user on an address book is 0 (none), 1 (read), 2 (read and write) or 3 (everything,
including the rules), the highest of the rules given to the user and to the group of the user. The owner and the
administrators have 3; a personal address book is for its owner only. `POST /api/ab/shared/profiles` tells the client
the right of the rule, and the server enforces it on every route that takes an address book: peers and tags need 1 to
be read and 2 to be changed, and the rules need 3. A refusal answers `403`, not `401`, which the clients take for an
ended session.

**Groups.** A group lists the groups its users may connect to (`allowed_outgoings`) and the groups that may connect
to it (`allowed_incomings`), by uuid or by name; the API gives them back as `access_to` and `accessed_from`. The two
lists are the two ends of the same links: a group always reaches itself. Users that are not administrators only get:

| Route | What they get |
|---|---|
| `GET /api/users` | the users of their group and of the groups their group may connect to |
| `GET /api/device-group/accessible` | those groups |
| `GET /api/peers` | the devices of those users: a device belongs to the user who last logged in on it |

`/api/peers/count/*` and `/api/peers/cpus` are for administrators, and a user cannot change their own rights or status
(`PUT /api/user` ignores `is_admin` and `status` unless the caller is an administrator). Administrators see everything.
A device on which nobody logged in belongs to nobody: only administrators see it.

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
