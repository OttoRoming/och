#!/usr/bin/env brush

./configure --prefix=/usr    \
            --disable-static \
            --enable-libgdbm-compat

make
make install
