#!/bin/sh
# Enables and (re)starts the service after an install or an upgrade. Skipped when the package is installed into an
# image (IPKG_INSTROOT) because there is no running system to start it on.
[ -n "$IPKG_INSTROOT" ] && exit 0
mkdir -p /etc/sctgdesk-server
chmod 700 /etc/sctgdesk-server
if [ -x /etc/init.d/sctgdesk-hbbr ]; then
	/etc/init.d/sctgdesk-hbbr enable
	/etc/init.d/sctgdesk-hbbr restart || echo "sctgdesk-server: hbbr did not start (port already used?): see logread -e hbbr" >&2
fi
exit 0
