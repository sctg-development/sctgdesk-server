# SCTGDesk client

The client is **upstream RustDesk plus a small overlay**, not a fork of its source: this directory holds only what
is ours (see [ADR 0001](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/adr/0001-client-placement.md) and the upstream-compatibility rule: the client stays
as close to the official one as possible).

| Path | Content |
|---|---|
| `UPSTREAM_TAG` | the upstream release the overlay is built on (`1.5.0`) |
| `patches/` | source changes, as `git format-patch` files applied on the pristine tag |
| `overlay/` | files copied over the tree: logos and icons (our visual signature) |
| `branding.py` | name, servers, key, URLs: string replacements, strict (a rule that matches nothing fails the build) |
| `build-sctg.py` | entry point: brands the tree, then runs upstream's `build.py` with the remaining arguments |
| `../.github/workflows/client-build.yaml` | the client build (from the former client repository): checks out this repository, runs `apply.sh`, then builds. **Never run**: toolchain versions were moved to upstream 1.5.0's, nothing else was changed |
| `apply.sh` | clones the tag, applies `patches/` and `overlay/` |

## Build

```sh
client/apply.sh                      # -> client/work (git-ignored)
cd client/work
export APP_NAME=SCTGDesk ORG_NAME=sctg RENDEZVOUS_SERVER=... RS_PUB_KEY=... API_SERVER=...
python3 branding.py --check          # which rules match, writes nothing
./build-sctg.py --flutter            # upstream build.py options follow
```

The client needs the upstream toolchain (Flutter, vcpkg, ...): see upstream's documentation. `branding.py` needs only Python 3.

## What differs from upstream

- **Update check.** Upstream skips it for custom clients and posts to `api.rustdesk.com`. Patch `0001` posts the
  same request to our API server (`POST /api/software/version/latest`).
- **TCP only.** The former `enable-tcp-mode` option is gone: use upstream's `disable-udp`. Not yet tested against
  our server (upstream's tip says the OSS server does not support it).
- **Branding** at build time (`branding.py`), including the default servers and key.
- The web client is built separately, from an older upstream tag: see below.

## Web client and landing page

Upstream removed the sources of its web client (`flutter/web/v1/js`) after 1.4.0, so the web client is built
from `UPSTREAM_TAG_WEB` (`1.4.0`), not from `UPSTREAM_TAG`. It does not follow upstream any more.

```sh
client/apply.sh /tmp/web web          # upstream 1.4.0 + web-overlay/, no patches
cd /tmp/web && python3 branding.py --web
```

`web-overlay/flutter/web/` holds our own files: `client.html`, the landing page with the download links of the
desktop clients (it asks `API_SERVER` for them), and its `images/`. The build renames it to `index.html`; the web
client itself becomes `sctgdesk.html`. `branding.py --web` points the browser at our servers (WebSocket over TLS,
ports 21120 and 21121 of a reverse proxy in front of `hbbs` 21118 and `hbbr` 21119), replaces the default servers and
key of upstream, and fails if one is left. The workflow is `.github/workflows/client-web.yml`.

## Known gaps

- The web client was branded on a clean `1.4.0` checkout, not built yet. The upstream README of its sources says
  "v1 is not compatible with current Flutter source code", a line that was already there in 1.2.7.
- The client was **not compiled** while preparing this layout (it needs Flutter and vcpkg): only the patch
  application and the branding rules were checked against a clean `1.5.0` checkout.
- Icons: only `res/` and the macOS icon are overlaid. Windows, Android and iOS icons are upstream's.
- The macOS signing and dmg steps of `build-sctg.py`, the flatpak rule and the client workflow come from the former script and are unverified on `1.5.0`.
- `deploy/helm/` (the Helm chart of the former client repository) deploys the server and was written for a TCP test:
  to be refactored.
