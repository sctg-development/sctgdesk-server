#!/bin/sh
# Run as root in a clean Debian container (packages.yml, or by hand):
#   docker run --rm -v "$PWD/dist:/p:ro" -v "$PWD/packaging:/t:ro" debian:12 sh /t/test-migration.sh /p
# Installs fake former packages rustdesk-server-hbbs/hbbr 1.1.10-3 (with the maintainer scripts of the real ones: the
# postrm masks the unit on a removal and deletes the data on a purge), with data and a configuration, then the
# sctgdesk-server-* packages from the directory given, and checks that the migration loses nothing.
set -eu
DIST=${1:?directory of the deb files}
DATA=/var/lib/rustdesk-server
fail() { echo "FAIL: $*" >&2; exit 1; }

apt-get update -qq >/dev/null
apt-get install -y -qq init-system-helpers >/dev/null

for s in hbbs hbbr; do
    d=$(mktemp -d)
    mkdir -p "$d/DEBIAN" "$d/usr/bin" "$d/usr/lib/systemd/system"
    printf 'Package: rustdesk-server-%s\nVersion: 1.1.10-3\nArchitecture: all\nMaintainer: test <t@example.org>\nDescription: former package\n' "$s" > "$d/DEBIAN/control"
    printf '#!/bin/sh\nexit 0\n' > "$d/usr/bin/$s"; chmod 755 "$d/usr/bin/$s"
    printf '[Service]\nExecStart=/usr/bin/%s\n' "$s" > "$d/usr/lib/systemd/system/rustdesk-$s.service"
    cat > "$d/DEBIAN/postrm" <<EOT
#!/bin/sh
set -e
SERVICE=rustdesk-$s.service
if [ "\$1" = "purge" ]; then
	rm -rf $DATA/ /var/log/rustdesk-server/rustdesk-$s.*
	deb-systemd-helper purge "\${SERVICE}" >/dev/null || true
	deb-systemd-helper unmask "\${SERVICE}" >/dev/null || true
fi
if [ "\$1" = "remove" ]; then
	deb-systemd-helper mask "\${SERVICE}" >/dev/null || true
fi
exit 0
EOT
    chmod 755 "$d/DEBIAN/postrm"
    dpkg-deb --build "$d" "/tmp/rustdesk-server-$s.deb" >/dev/null
done
dpkg -i /tmp/rustdesk-server-hbbs.deb /tmp/rustdesk-server-hbbr.deb >/dev/null

mkdir -p "$DATA"
echo "old database" > "$DATA/db_v2.sqlite3"
echo "old key" > "$DATA/id_ed25519"
echo "old oauth" > "$DATA/oauth2.toml"

apt-get install -y -qq "$DIST"/sctgdesk-server-hbbs_*.deb "$DIST"/sctgdesk-server-hbbr_*.deb >/dev/null

for s in hbbs hbbr; do
    [ -f /usr/lib/systemd/system/rustdesk-$s.service ] || fail "the unit rustdesk-$s.service is missing"
    [ "$(readlink /etc/systemd/system/rustdesk-$s.service 2>/dev/null || true)" != /dev/null ] || fail "rustdesk-$s.service is masked"
    [ -f /etc/default/sctgdesk-$s ] || fail "/etc/default/sctgdesk-$s is missing"
    grep -q 'ExecStart=/usr/bin/'$s /usr/lib/systemd/system/rustdesk-$s.service || fail "ExecStart of $s"
done
dpkg -l rustdesk-server-hbbs 2>/dev/null | grep -q '^ii' && fail "the former package is still installed"
for f in db_v2.sqlite3 id_ed25519 oauth2.toml; do
    [ -f "$DATA/$f" ] || fail "$f is missing from $DATA"
    b=$(ls -d /var/backups/sctgdesk-server/20*/ 2>/dev/null | head -n 1)
    [ -n "$b" ] && [ -f "$b/$f" ] || fail "$f is not in the copy of $DATA"
done
[ "$(stat -c %a "$DATA")" = 700 ] || fail "the mode of $DATA is not 700"
[ "$(stat -c %a "$DATA/id_ed25519")" = 600 ] || fail "the mode of id_ed25519 is not 600"
[ -e /run/sctgdesk-server-fresh ] && fail "a migration is not a fresh install: nothing must start"

# the purge of the former packages, as an administrator does after a migration, leaves a copy
dpkg -P rustdesk-server-hbbs rustdesk-server-hbbr >/dev/null 2>&1 || true
[ -f "$(ls -d /var/backups/sctgdesk-server/20*/ | head -n 1)/db_v2.sqlite3" ] || fail "no copy after the purge"
echo "migration: OK"
