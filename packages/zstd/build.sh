#!/usr/bin/env brush

make prefix=/usr
make DESTDIR="$DESTDIR" prefix=/usr install
rm -v /usr/lib/libzstd.a
