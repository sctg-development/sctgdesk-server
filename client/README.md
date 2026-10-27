# SCTGDesk client

The client is **upstream RustDesk plus a small overlay**, not a fork of its source: this directory holds only what
is ours (see [ADR 0001](../docs/adr/README.md) (on the `docs/adr` branch until it is merged) and the upstream-compatibility rule: the client stays
as close to the official one as possible).

| Path | Content |
|---|---|
| `UPSTREAM_TAG` | the upstream release the overlay is built on (`1.5.0`) |
| `patches/` | source changes, as `git format-patch` files applied on the pristine tag |
| `overlay/` | files copied over the tree: logos and icons (our visual signature) |
| `branding.py` | name, servers, key, URLs: string replacements, strict (a rule that matches nothing fails the build) |
| `build-sctg.py` | entry point: brands the tree, then runs upstream's `build.py` with the remaining arguments |
| `ci/` | the former client workflows, kept for reference; **not adapted** to this layout yet |
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
- Not carried over: the web client (`flutter/web`), which does not exist in upstream 1.5.0.

## Known gaps

- The client was **not compiled** while preparing this layout (it needs Flutter and vcpkg): only the patch
  application and the branding rules were checked against a clean `1.5.0` checkout.
- Icons: only `res/` and the macOS icon are overlaid. Windows, Android and iOS icons are upstream's.
- The macOS signing and dmg steps of `build-sctg.py`, the flatpak rule and `ci/` come from the former script and are unverified on `1.5.0`.
- `deploy/helm/` (the Helm chart of the former client repository) deploys the server and was written for a TCP test:
  to be refactored.
