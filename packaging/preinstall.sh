#!/bin/sh
# Before every install, upgrade or migration (deb: preinst, rpm: %pre), shared by hbbs and hbbr.
#  - copies the data directory to /var/backups/sctgdesk-server/<date> (the 3 latest copies are kept), so that a
#    migration from the former rustdesk-server-* packages, whose purge deletes /var/lib/rustdesk-server, or a bad
#    upgrade never loses the database, the key pair or the configuration;
#  - notes in /run when the directory is empty, so that the postinstall starts the services of a fresh install.
# A failed copy warns and never stops the installation.
DATA=/var/lib/rustdesk-server
BACKUPS=/var/backups/sctgdesk-server
FRESH=/run/sctgdesk-server-fresh

has_data() {
    [ -n "$(find "$DATA" -maxdepth 1 \( -name 'db_v2.sqlite3*' -o -name 'id_ed25519*' -o -name '*.toml' \) 2>/dev/null | head -n 1)" ]
}

if has_data; then
    # hbbs and hbbr are installed one after the other: one copy per run is enough
    if [ -z "$(find "$BACKUPS" -maxdepth 1 -type d -name '20*' -mmin -10 2>/dev/null | head -n 1)" ]; then
        dest="$BACKUPS/$(date +%Y%m%d-%H%M%S)"
        if mkdir -p "$BACKUPS" "$dest" && chmod 700 "$BACKUPS" "$dest"; then
            ok=1
            for f in "$DATA"/*; do
                [ -e "$f" ] || continue
                case "$f" in
                    */db_v2.sqlite3-wal|*/db_v2.sqlite3-shm) ;; # taken with the database
                    */db_v2.sqlite3)
                        if command -v sqlite3 >/dev/null 2>&1; then
                            sqlite3 "$f" ".backup '$dest/db_v2.sqlite3'" || ok=0
                        else
                            # no sqlite3: the three files, which is consistent while hbbs is stopped
                            cp -p "$f" "$f-wal" "$f-shm" "$dest"/ 2>/dev/null || cp -p "$f" "$dest"/ || ok=0
                        fi
                        ;;
                    *) cp -pR "$f" "$dest"/ || ok=0 ;;
                esac
            done
            if [ "$ok" = 1 ]; then
                echo "sctgdesk-server: $DATA copied to $dest"
            else
                echo "sctgdesk-server: WARNING: the copy of $DATA to $dest is incomplete" >&2
            fi
            # keep the 3 latest
            ls -1d "$BACKUPS"/20* 2>/dev/null | head -n -3 | while read -r old; do rm -rf "$old"; done
        else
            echo "sctgdesk-server: WARNING: cannot create $BACKUPS, $DATA is not copied" >&2
        fi
    fi
    # a marker of the same transaction (hbbs and hbbr) stays; an old one goes
    if [ -z "$(find "$FRESH" -mmin -10 2>/dev/null)" ]; then rm -f "$FRESH"; fi
else
    touch "$FRESH" 2>/dev/null || true
fi
exit 0
