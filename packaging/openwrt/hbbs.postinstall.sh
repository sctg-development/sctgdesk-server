#!/bin/sh
# Enables and (re)starts the service after an install or an upgrade. Skipped when the package is installed into an
# image (IPKG_INSTROOT) because there is no running system to start it on.
[ -n "$IPKG_INSTROOT" ] && exit 0
mkdir -p /etc/sctgdesk-server
chmod 700 /etc/sctgdesk-server
if [ -x /etc/init.d/sctgdesk-hbbs ]; then
	/etc/init.d/sctgdesk-hbbs enable
	/etc/init.d/sctgdesk-hbbs restart || echo "sctgdesk-server: hbbs did not start (port already used?): see logread -e hbbs" >&2
fi
exit 0
