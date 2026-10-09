#!/usr/bin/env brush

./configure --prefix=/usr     \
            --disable-static  \
            --sysconfdir=/etc \
            --docdir=/usr/share/doc/attr-2.6.0

make
make DESTDIR="$DESTDIR" install
