#!/usr/bin/env bash
# Builds the signed apt, yum/dnf and apk repositories from a folder of packages.
#
#   packaging/repos/build-repos.sh <packages-dir> <site-dir>
#
# <packages-dir> holds the *.deb, *.rpm and *.apk files (all versions and architectures mixed); <site-dir> receives
#   apt/   deb repository     (dists/stable/main/binary-<arch>/, pool/main/, InRelease, sctgdesk.gpg, sctgdesk.asc)
#   rpm/   yum/dnf repository (repodata/ with a signed repomd.xml, sctgdesk.asc; the packages are not signed one by one)
#   apk/   OpenWrt/Alpine apk repository, one folder per architecture (<arch>/APKINDEX.tar.gz) and sctgdesk.rsa.pub
#
# Environment (the signing keys; see docs/repositories.md to create them):
#   GPG_PRIVATE_KEY   armored private key, signs apt and rpm            (required)
#   GPG_PASSPHRASE    its passphrase                                     (optional)
#   APK_PRIVATE_KEY   RSA private key in PEM, signs the apk index and packages (required)
#
# The tools run in containers (Debian, Fedora, Alpine) so that the runner or the laptop needs only docker.
set -euo pipefail

packages="$(cd "${1:?usage: $0 <packages-dir> <site-dir>}" && pwd)"
mkdir -p "${2:?usage: $0 <packages-dir> <site-dir>}"
site="$(cd "$2" && pwd)"
: "${GPG_PRIVATE_KEY:?GPG_PRIVATE_KEY is required}"
: "${APK_PRIVATE_KEY:?APK_PRIVATE_KEY is required}"
GPG_PASSPHRASE="${GPG_PASSPHRASE:-}"

mkdir -p "$site/apt/pool/main" "$site/rpm" "$site/apk"
cp "$packages"/*.deb "$site/apt/pool/main/" 2>/dev/null || echo "no deb package"
cp "$packages"/*.rpm "$site/rpm/" 2>/dev/null || echo "no rpm package"
cp "$packages"/*.apk "$site/apk/" 2>/dev/null || echo "no apk package"

# ---- apt (Debian container) ---------------------------------------------------------------------------------------
docker run --rm -e GPG_PRIVATE_KEY -e GPG_PASSPHRASE -v "$site/apt:/repo" debian:12 bash -euo pipefail -c '
  apt-get update -qq && apt-get install -y -qq apt-utils gnupg >/dev/null
  export GNUPGHOME=$(mktemp -d); chmod 700 "$GNUPGHOME"
  echo "$GPG_PRIVATE_KEY" | gpg --batch --quiet --import
  key=$(gpg --list-secret-keys --with-colons | awk -F: "/^fpr/ {print \$10; exit}")
  sign() { gpg --batch --yes --pinentry-mode loopback --passphrase "$GPG_PASSPHRASE" --default-key "$key" "$@"; }
  cd /repo
  archs=$(ls pool/main/*.deb 2>/dev/null | sed -E "s/.*_([a-z0-9]+)\.deb$/\1/" | sort -u)
  for arch in $archs; do
    mkdir -p dists/stable/main/binary-$arch
    apt-ftparchive --arch $arch packages pool > dists/stable/main/binary-$arch/Packages
    gzip -9kf dists/stable/main/binary-$arch/Packages
  done
  apt-ftparchive \
    -o APT::FTPArchive::Release::Origin=SCTGDesk -o APT::FTPArchive::Release::Label=SCTGDesk \
    -o APT::FTPArchive::Release::Suite=stable -o APT::FTPArchive::Release::Codename=stable \
    -o APT::FTPArchive::Release::Components=main -o "APT::FTPArchive::Release::Architectures=$(echo $archs)" \
    release dists/stable > dists/stable/Release
  sign --clearsign -o dists/stable/InRelease dists/stable/Release
  sign --armor --detach-sign -o dists/stable/Release.gpg dists/stable/Release
  gpg --export "$key" > sctgdesk.gpg
  gpg --export --armor "$key" > sctgdesk.asc
  chmod -R a+rX /repo'

# ---- rpm (Fedora container) ---------------------------------------------------------------------------------------
# The metadata of the repository is signed (repomd.xml.asc, checked with repo_gpgcheck=1), not each package: the signature
# of a package goes through rpmsign, whose way of calling gpg changes between rpm versions.
docker run --rm -e GPG_PRIVATE_KEY -e GPG_PASSPHRASE -v "$site/rpm:/repo" fedora:latest bash -euo pipefail -c '
  dnf install -y -q createrepo_c gnupg2 >/dev/null
  export GNUPGHOME=$(mktemp -d); chmod 700 "$GNUPGHOME"
  echo "$GPG_PRIVATE_KEY" | gpg --batch --quiet --import
  key=$(gpg --list-secret-keys --with-colons | awk -F: "/^fpr/ {print \$10; exit}")
  cd /repo
  createrepo_c --quiet .
  gpg --batch --yes --pinentry-mode loopback --passphrase "$GPG_PASSPHRASE" --default-key "$key" --armor --detach-sign -o repodata/repomd.xml.asc repodata/repomd.xml
  gpg --export --armor "$key" > sctgdesk.asc
  chmod -R a+rX /repo'

# ---- apk (Alpine container) ---------------------------------------------------------------------------------------
# One folder per architecture, as apk expects (<repository>/<arch>/APKINDEX.tar.gz). The architecture is in the
# name of each file: <name>_<version>_<arch>.apk (OpenWrt names such as mips_24kc or aarch64_cortex-a53).
docker run --rm -e APK_PRIVATE_KEY -v "$site/apk:/repo" alpine:3.22 sh -eu -c '
  apk add --quiet abuild openssl >/dev/null
  printf "%s\n" "$APK_PRIVATE_KEY" > /tmp/sctgdesk.rsa
  openssl rsa -in /tmp/sctgdesk.rsa -pubout -out /repo/sctgdesk.rsa.pub 2>/dev/null
  cd /repo
  for f in *.apk; do
    [ -e "$f" ] || continue
    # name and version have no "_", the architecture may (x86_64, mips_24kc): drop the first two fields
    rest=${f#*_}; arch=${rest#*_}; arch=${arch%.apk}
    mkdir -p "$arch"
    # inside a repository a package is called <name>-<version>.apk (the file name is how apk finds it)
    name=${f%%_*}; version=${rest%%_*}
    mv "$f" "$arch/$name-$version.apk"
  done
  for dir in */; do
    dir=${dir%/}
    ( cd "$dir"
      # Only the index is signed: apk (2 and 3, the one of OpenWrt) checks a package against the checksum that the signed
      # index holds for it, so the packages themselves stay as they were built
      apk index --allow-untrusted -q -o APKINDEX.tar.gz *.apk
      abuild-sign -q -k /tmp/sctgdesk.rsa APKINDEX.tar.gz )
  done
  chmod -R a+rX /repo'
echo "repositories written to $site"
