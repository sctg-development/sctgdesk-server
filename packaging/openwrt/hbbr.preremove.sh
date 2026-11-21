#!/bin/sh
# Stops the service before its files are removed. The data directory is never deleted.
[ -n "$IPKG_INSTROOT" ] && exit 0
if [ -x /etc/init.d/sctgdesk-hbbr ]; then
	/etc/init.d/sctgdesk-hbbr stop || true
	/etc/init.d/sctgdesk-hbbr disable || true
fi
exit 0
