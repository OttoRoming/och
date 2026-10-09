#!/usr/bin/env brush

./configure --prefix=/usr    \
            --disable-static \
            --docdir=/usr/share/doc/gettext-1.0
make
make DESTDIR="$DESTDIR" install
chmod -v 0755 /usr/lib/preloadable_libintl.so
