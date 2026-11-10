# Custom builds

How to produce your own binaries: the server (with your changes, your version string, your packages) and the client
(your name, servers and key). Read [Building from source](building.md) first.

## The server

### Build

```sh
git clone --recurse-submodules https://github.com/sctg-development/sctgdesk-server
cd sctgdesk-server
export DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3
cargo build --release --locked --bins
```

`target/release` then holds `hbbs`, `hbbr` and `rustdesk-utils`. `--locked` makes the build use the versions of
`Cargo.lock`. The web console and the API documentation are built with npm and embedded in `hbbs`;
`SCTGDESK_SKIP_WEB_BUILD=1` skips that (placeholders instead).

### Your own version number

The version printed by `hbbs --version` is the `version` of `Cargo.toml` (and of `crates/api-server/Cargo.toml`, the
web console `package.json` and `Cargo.lock`, which a release changes together). For a build of your own, follow
[Versions and releases](releases.md): `<upstream version>-<yourname>.<n>`. Keep the upstream part, so that it is clear
what the code is based on.

### Static binaries (Linux)

Static musl binaries run on any Linux, which is why the packages use them:

```sh
sudo apt install musl-tools pkg-config            # Debian, Ubuntu
rustup target add x86_64-unknown-linux-musl       # or aarch64-unknown-linux-musl
cargo build --release --locked --bins --target x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/hbbs   # "statically linked" / "not a dynamic executable"
```

For arm64 build on an arm64 machine (the CI uses an arm64 runner); cross-compiling needs a cross C toolchain for
libsodium, which this repository does not set up.

### Windows and macOS

The same `cargo build --release --locked --bins` works on the platform itself. On Windows the release workflow links
the C runtime statically (`RUSTFLAGS="-C target-feature=+crt-static"`) so that the Visual C++ redistributable is not
needed; in `cmd.exe` the database variable is `set "DATABASE_URL=sqlite://%CD%/db_v2.sqlite3"`.

### deb and rpm packages

`packaging/` holds the [nfpm](https://nfpm.goreleaser.com/) configurations (`nfpm-hbbs.yaml`, `nfpm-hbbr.yaml`,
`nfpm-utils.yaml`), the systemd units are in `systemd/`, and the maintainer scripts next to the configurations. After
building the static binaries for an architecture:

```sh
export VERSION=1.1.16-sctg.3 NFPM_ARCH=amd64 BIN_DIR=target/x86_64-unknown-linux-musl/release   # arm64 for aarch64
mkdir dist
for p in hbbs hbbr utils; do for f in deb rpm; do nfpm package --config packaging/nfpm-$p.yaml --packager $f --target dist/; done; done
```

nfpm turns `1.1.16-sctg.3` into `1.1.16~sctg.3` (deb) and `1.1.16~sctg.3-1` (rpm), so that a prerelease sorts below the
release. Change the maintainer, vendor and name in the three YAML files for a package of your own; keep
`replaces`/`conflicts` consistent with the names you use.

### Releases from GitHub Actions

Publishing a GitHub release runs `release.yml`, which builds and publishes everything for the tag of the release:
the Docker image, the Linux packages, the Windows and macOS server binaries, and (full release) the clients on S3
and Cloudflare Pages. The jobs, the secrets and the pre-release rule are in [Versions and releases](releases.md#the-release-pipeline).

`packages.yml` and `binaries.yml` can also be run by hand from the *Actions* tab. They ask for the **tag**
(`sctg-v<version>`, which must match `Cargo.toml` or the run stops) and for `publish` (off: build and test only; on:
upload to the release, created as a pre-release if missing):

| Workflow | Produces |
|---|---|
| `Linux packages (deb, rpm)` (`packages.yml`) | deb and rpm of hbbs, hbbr and utils for amd64 and arm64, installed in Debian and Fedora containers as a test |
| `Windows and macOS binaries` (`binaries.yml`) | zip archives with the three binaries for Windows (amd64, arm64) and macOS (Intel, Apple Silicon) |

To use them in a fork, fork the repository (submodules included), change the version and the names and run the two
workflows by hand on a branch: they need no secret. The Docker and client jobs of `release.yml` need the secrets of
the list in [releases](releases.md#the-release-pipeline): set your own, or remove those jobs from your copy.

### Docker image

The image is built from `Dockerfile` (and `docker-classic/`); the multi-architecture workflow publishes to Docker hub
with credentials of the maintainer. For your own: `docker build -t myrepo/sctgdesk-server .` from a checkout.
This path is not part of the verified release process.

## The client

The client is upstream RustDesk **plus an overlay** (`client/`): patches, logos, and a branding script that replaces
names, servers and the key in a clean checkout of the upstream tag. You never fork the upstream source.

### Settings

`branding.py` reads its settings from the environment:

| Variable | Meaning |
|---|---|
| `APP_NAME` | application name, for example `MyDesk` |
| `ORG_NAME` | organisation, used for the website and the bundle ids |
| `RENDEZVOUS_SERVER` | host of your `hbbs`, the default server of the client |
| `RS_PUB_KEY` | the **public** key: the content of your `id_ed25519.pub` |
| `API_SERVER` | `host[:port]` of your API server |
| `API_SERVER_PROTOCOL` | `http` or `https` (default `https`) |
| `COPYRIGHT_HOLDER` | optional, added to upstream's in the copyright |
| `PROJECT_URL` | optional, where "set up your own server" leads (default this repository) |
| `MACOS_CODESIGN_IDENTITY` | optional, `Name (TEAMID)` |

The rules are strict: a rule that matches nothing fails the build, so that a client never ships with RustDesk's
servers by accident. The script can be run twice.

### Desktop and mobile

```sh
client/apply.sh                       # clones the upstream tag of client/UPSTREAM_TAG, applies patches and overlay -> client/work
cd client/work
export APP_NAME=MyDesk ORG_NAME=myorg RENDEZVOUS_SERVER=rdv.example.com \
       RS_PUB_KEY="$(cat /path/to/id_ed25519.pub)" API_SERVER=api.example.com
python3 branding.py --check           # what would change, writes nothing
./build-sctg.py --flutter             # brands the tree, then runs upstream's build.py with the remaining arguments
```

The build needs the upstream toolchain (Flutter, vcpkg, a C++ toolchain, Xcode on macOS, ...): follow RustDesk's own
build documentation for the tag in `client/UPSTREAM_TAG`. `client/README.md` lists what differs from upstream and what
is **not verified**: the client was prepared against a clean checkout of upstream 1.5.0, and the iOS,
Android and Linux clients have no workflow any more. Treat a first build of those as work to do, not as a recipe.

Your icons and logos go in `client/overlay/` (copied over the tree). Windows, Android and iOS icons are still
upstream's.

### The web client

It is built from an older upstream tag (`client/UPSTREAM_TAG_WEB`) because upstream removed the sources of its web
client; it does not follow upstream any more.

```sh
client/apply.sh /tmp/web web          # upstream web client + client/web-overlay, no patches
cd /tmp/web
export APP_NAME=MyDesk RENDEZVOUS_SERVER=rdv.example.com RS_PUB_KEY="$(cat /path/to/id_ed25519.pub)" API_SERVER=api.example.com
python3 branding.py --web
```

Then build it as `client-web.yml` does (it uses a Flutter web build and the WebSocket ports of your reverse proxy,
21120 and 21121). The branding makes it talk to your servers, adds the login (including OAuth2 through your API
server) and the device identity. After deploying a new build, browsers can keep the old one: its
service worker caches the application, and `index.js` is served with a long `cache-control`. Serve
`flutter_service_worker.js` and `index.js` with `no-cache`, or ask users to reload with the cache emptied.

### Checking a custom client

1. `python3 branding.py --check` finds every rule.
2. After the build, search the output for the upstream values (`rs-ny.rustdesk.com`, upstream's key): branding fails if
   one is left in a tree, but check the artifact too.
3. Connect to your server with it **and** with an official client; compatibility with official clients is the first
   rule of this project.
4. With `--logged-in-only`: logged-in connects, logged-out is refused with the message of
   [the deployment guide](deployment.md#only-logged-in-users-can-control).
