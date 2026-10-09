#!/usr/bin/env brush

./configure --prefix=/usr    \
            --disable-static \
            --with-gcc-arch=native

make
make DESTDIR="$DESTDIR" install
