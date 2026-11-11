# SCTGDesk client

The client is **upstream RustDesk plus a small overlay**, not a fork of its source: this directory holds only what
is ours (see [ADR 0001](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/docs/adr/0001-client-placement.md) and the upstream-compatibility rule: the client stays
as close to the official one as possible).

| Path | Content |
|---|---|
| `UPSTREAM_TAG` | the upstream release the overlay is built on (`1.5.3`) |
| `patches/` | source changes, as `git format-patch` files applied on the pristine tag |
| `overlay/` | files copied over the tree: logos and icons (our visual signature) |
| `branding.py` | name, servers, key, URLs: string replacements, strict (a rule that matches nothing fails the build) |
| `build-sctg.py` | entry point: brands the tree, then runs upstream's `build.py` with the remaining arguments |
| `../.github/workflows/client-{windows,macos,web}.yml` | the client builds: check out this repository, run `apply.sh`, brand, build, sign and (when `publish` is on, or from `release.yml` for a full release) copy to S3 and Cloudflare Pages. The iOS, Android and Linux builds of the former `client-build.yaml` were removed, see the git history |
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
  same request to our API server (`POST /api/software/version/latest`). Since 1.5.3 upstream only accepts the GitHub
  release of upstream in the answer (`release_id_from_update_url`): the patch takes the version from the release page of
  our API server instead. Patch `0002` makes the update card open the download of the platform, and patch `0003`
  turns the in-app update off for our client (it only downloads the GitHub releases of upstream and would fail at
  every check).
- **TCP only.** The former `enable-tcp-mode` option is gone: use upstream's `disable-udp`. Not yet tested against
  our server (upstream's tip says the OSS server does not support it).
- **Branding** at build time (`branding.py`), including the default servers and key.
- The web client is built separately, from an older upstream tag: see below.

## Web client and landing page

Upstream removed the sources of its web client (kept in `flutter/web/v1`, which `apply.sh web` moves up to
`flutter/web` where its tooling and `flutter build web` expect them) after 1.4.0, so the web client is built
from `UPSTREAM_TAG_WEB` (`1.4.0`), not from `UPSTREAM_TAG`. It does not follow upstream any more.

```sh
client/apply.sh /tmp/web web          # upstream 1.4.0 + web-overlay/, no patches
cd /tmp/web && python3 branding.py --web
```

`web-overlay/flutter/web/` holds our own files: `client.html`, the landing page with the download links of the
desktop clients (it asks `API_SERVER` for them), and its `images/`. The page has no text: a click on the Windows or the
Apple icon opens a fan under it, two thin lines at 120° (`--fan-angle`) leading to two icons, 32 and 64 bits for
Windows, Intel and Apple Silicon for the Mac (a bare chip, and a chip with the apple). The links come from
`/api/software/client-download-link/{w32,w64,osx,osx-arm64}`; an icon without a link stays dimmed. `page#windows` and
`page#apple` open a fan on load. The build renames it to `index.html`; the web
client itself becomes `sctgdesk.html`. `branding.py --web` points the browser at our servers (WebSocket over TLS,
ports 21120 and 21121 of a reverse proxy in front of `hbbs` 21118 and `hbbr` 21119), replaces the default servers and
key of upstream, and fails if one is left. The workflow is `.github/workflows/client-web.yml`.

## Known gaps

- The web client was branded on a clean `1.4.0` checkout, not built yet. The upstream README of its sources says
  "v1 is not compatible with current Flutter source code", a line that was already there in 1.2.7.
- The client was **not compiled** while preparing this layout (it needs Flutter and vcpkg): only the patch
  application and the branding rules were checked against a clean `1.5.3` checkout (`git am` of the patches, `branding.py --check`, and `branding.py` with
  the submodule of `hbb_common`: every rule applies; the flatpak rule is optional and matches nothing).
- Icons: only `res/` and the macOS icon are overlaid. Windows, Android and iOS icons are upstream's.
- The macOS signing and dmg steps of `build-sctg.py`, the flatpak rule and the client workflow come from the former script and are unverified on `1.5.3`.
- `deploy/helm/` (the Helm chart of the former client repository) deploys the server and was written for a TCP test:
  to be refactored.
- Like upstream 1.5.3, `client-macos.yml` builds the Apple Silicon client with **Flutter 3.44.9** (Flutter 3.24.5
  crashes in the text input, rustdesk issue 16817) and the Intel client with 3.24.5. For arm64 the sources are
  patched on the fly by upstream's `.github/patches/apply_flutter_3.44_source_patches.sh` and `bridge.yml` generates a
  second bridge (`bridge-artifact-flutter-3.44`). Not built yet: the first run of the workflow is the test. The Windows
  arm64 client, also on Flutter 3.44 upstream, is not built here.
