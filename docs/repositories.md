# Package repositories

The releases are also published as signed package repositories on GitHub Pages, next to this manual:

| Repository | Address | For |
|---|---|---|
| apt | `https://sctg-development.github.io/sctgdesk-server/apt` | Debian, Ubuntu (amd64, arm64, armhf) |
| rpm | `https://sctg-development.github.io/sctgdesk-server/rpm` | Fedora, RHEL and clones, openSUSE (x86_64, aarch64, armv7hl) |
| apk | `https://sctg-development.github.io/sctgdesk-server/apk/<architecture>` | OpenWrt and Alpine |

Only the latest full releases are kept (the pre-releases are not).

## Debian, Ubuntu

```sh
sudo mkdir -p /etc/apt/keyrings
sudo curl -fsSL https://sctg-development.github.io/sctgdesk-server/apt/sctgdesk.gpg -o /etc/apt/keyrings/sctgdesk.gpg
echo "deb [signed-by=/etc/apt/keyrings/sctgdesk.gpg] https://sctg-development.github.io/sctgdesk-server/apt stable main" |
  sudo tee /etc/apt/sources.list.d/sctgdesk.list
sudo apt update
sudo apt install sctgdesk-server-hbbs sctgdesk-server-hbbr
```

## Fedora, RHEL, openSUSE

```sh
sudo tee /etc/yum.repos.d/sctgdesk.repo <<'EOF'
[sctgdesk]
name=SCTGDesk
baseurl=https://sctg-development.github.io/sctgdesk-server/rpm/
enabled=1
gpgcheck=0
repo_gpgcheck=1
gpgkey=https://sctg-development.github.io/sctgdesk-server/rpm/sctgdesk.asc
EOF
sudo dnf install sctgdesk-server-hbbs sctgdesk-server-hbbr
```

The signature covers the metadata of the repository (`repo_gpgcheck`), and through it the checksum of every package; the
packages are not signed one by one, hence `gpgcheck=0`.

## OpenWrt

For OpenWrt 25.12 and later, which use `apk`. Older releases (`opkg`) are not supported: use the static binaries of the
[releases](https://github.com/sctg-development/sctgdesk-server/releases) there.

```sh
wget -O /etc/apk/keys/sctgdesk.rsa.pub https://sctg-development.github.io/sctgdesk-server/apk/sctgdesk.rsa.pub
echo "https://sctg-development.github.io/sctgdesk-server/apk/$(apk --print-arch)/APKINDEX.tar.gz" > /etc/apk/repositories.d/sctgdesk.list
apk update
apk add sctgdesk-server-hbbs sctgdesk-server-hbbr
```

`apk --print-arch` is the architecture of the router. These are published:

| OpenWrt architecture | Devices (examples) |
|---|---|
| `x86_64` | PCs, virtual machines |
| `aarch64_generic`, `aarch64_cortex-a53`, `aarch64_cortex-a72`, `aarch64_cortex-a76` | Raspberry Pi 3 and later, MediaTek Filogic, Rockchip |
| `arm_cortex-a7_neon-vfpv4`, `arm_cortex-a7_vfpv4`, `arm_cortex-a9`, `arm_cortex-a15_neon-vfpv4` | Raspberry Pi 2, many IPQ40xx and Marvell routers |
| `mips_24kc`, `mips_4kec` | big endian MIPS: Atheros / Qualcomm **ath79** |
| `mipsel_24kc`, `mipsel_74kc` | little endian MIPS: MediaTek / Ralink **ramips** (mt7621, mt76x8, rt305x) |

If your router is not listed, use the static binary of the release.

The packages install a `procd` service for each program and a UCI file to configure it (`/etc/config/sctgdesk-hbbs`,
`/etc/config/sctgdesk-hbbr`):

```sh
uci set sctgdesk-hbbs.main.args='-r relay.example.com -k <your-key>'
uci commit sctgdesk-hbbs
/etc/init.d/sctgdesk-hbbs restart
logread -e hbbs           # the log goes to the system log
```

Things to know about a router:

- **Storage.** `hbbs` is about 30 MB. Install it on a USB disk or on an extroot, not on a small flash. The key pair and
  the database go to `datadir` (`/etc/sctgdesk-server` by default, on the flash): put it on the disk too, a database
  writes a lot.
- **MIPS.** The MIPS builds are a tier 3 target of Rust, compiled with zig. They run (the CI starts `hbbs` and `hbbr` under
  QEMU and queries the API), but they have not been tried on a real router by the maintainers: please report what you
  see.
- **Ports.** Open 21115, 21116 (TCP and UDP), 21117, 21118 and 21119, and 21114 for the console, in the firewall of the
  router.

## For the maintainer: signing keys

The repositories are built by `.github/workflows/pages.yml` (`packaging/repos/build-repos.sh`) when a release is published
as a full release, or by hand (Actions, Pages, "Run workflow", tick *Deploy*). It needs two keys, stored as secrets of the
repository. Create them once, keep a copy of the private ones somewhere safe: users trust the public part, so losing
the private key means asking everybody to import a new one.

```sh
# apt and rpm: a GPG key (add a passphrase if you want one, and the secret REPO_GPG_PASSPHRASE)
gpg --quick-generate-key "SCTGDesk packages <packages@example.org>" rsa4096 sign 5y
gpg --armor --export-secret-keys "SCTGDesk packages"        # -> secret REPO_GPG_PRIVATE_KEY

# apk: an RSA key, in PEM
openssl genrsa -out sctgdesk.rsa 4096
cat sctgdesk.rsa                                            # -> secret REPO_APK_PRIVATE_KEY
```

The public keys are derived from them and published in the repositories. To try the script without the CI:
`GPG_PRIVATE_KEY=... APK_PRIVATE_KEY=... packaging/repos/build-repos.sh <folder of packages> <output folder>` (docker is
needed).
