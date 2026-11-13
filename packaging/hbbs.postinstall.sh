#!/bin/sh
# deb: configure ($2 is the previous version, empty on a first install); rpm: $1 is 1 on an install, 2 on an upgrade.
# A first install on an empty data directory enables and starts the service with the defaults of the unit (a random
# key pair, see packaging/default-hbbs). A migration from the former rustdesk-server-* packages, or any install that finds
# data, starts nothing: the options of the old unit are not carried over, and they may differ (ports, key).
# An upgrade restarts the service if it runs.
set -e
DATA=/var/lib/rustdesk-server
UNIT=rustdesk-hbbs.service
FRESH=/run/sctgdesk-server-fresh

first=0
case "$1" in
    configure) [ -z "$2" ] && first=1 ;;
    1) first=1 ;;
esac

mkdir -p "$DATA" /var/log/rustdesk-server
# the key pair, the secrets and the database are for root (or the user of the unit) only
chmod 700 "$DATA"
for f in id_ed25519 secret_key.txt oauth2.toml s3config.toml db_v2.sqlite3 db_v2.sqlite3-wal db_v2.sqlite3-shm; do
    [ -e "$DATA/$f" ] && chmod 600 "$DATA/$f"
done

# the postrm of the former rustdesk-server-hbbs package masks the unit of the same name when it is removed by the
# migration to this package: the mask would hide the unit installed here
if [ "$(readlink /etc/systemd/system/$UNIT 2>/dev/null)" = /dev/null ]; then
    rm -f "/etc/systemd/system/$UNIT"
fi

if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || true
    if [ "$first" = 1 ] && [ -n "$(find "$FRESH" -mmin -10 2>/dev/null)" ]; then
        systemctl enable --now "$UNIT" || echo "sctgdesk-server: $UNIT did not start (port already used?): see journalctl -u $UNIT" >&2
    elif [ "$first" = 1 ]; then
        echo "sctgdesk-server: data found in $DATA, $UNIT is not started. Set the options in /etc/default/sctgdesk-hbbs, then: systemctl enable --now $UNIT" >&2
    else
        systemctl try-restart "$UNIT" || true
    fi
fi
exit 0
