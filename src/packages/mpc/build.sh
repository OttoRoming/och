#!/usr/bin/env brush

./configure --prefix=/usr    \
            --disable-static \
            --docdir=/usr/share/doc/mpc-1.4.1
make
make html
make install
make install-html
