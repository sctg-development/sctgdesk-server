#!/bin/sh
# deb: configure; rpm: install or upgrade. A first install starts nothing: the server needs its options
# (see the unit rustdesk-hbbr.service and the documentation), then "systemctl enable --now rustdesk-hbbr".
set -e
mkdir -p /var/lib/rustdesk-server /var/log/rustdesk-server
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || true
    # an upgrade restarts the service if it runs
    systemctl try-restart rustdesk-hbbr.service || true
fi
exit 0
