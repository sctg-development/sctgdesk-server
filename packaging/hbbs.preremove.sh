#!/bin/sh
# deb: "remove" or "upgrade"; rpm: 0 (last version removed) or 1 (upgrade). Only a removal stops the service.
set -e
case "$1" in
    remove|0)
        if [ -d /run/systemd/system ]; then
            systemctl stop rustdesk-hbbs.service || true
            systemctl disable rustdesk-hbbs.service || true
        fi
        ;;
esac
exit 0
