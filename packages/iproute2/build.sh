#!/usr/bin/env brush

sed -i /ARPD/d Makefile
rm -fv man/man8/arpd.8
make NETNS_RUN_DIR=/run/netns
make DESTDIR="$DESTDIR" SBINDIR=/usr/sbin install
install -vDm644 COPYING README* -t "$DESTDIR"/usr/share/doc/iproute2-7.1.0
