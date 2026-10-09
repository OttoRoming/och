#!/usr/bin/env brush

./configure --prefix=/usr --docdir=/usr/share/doc/automake-1.18.1

make
make DESTDIR="$DESTDIR" install
