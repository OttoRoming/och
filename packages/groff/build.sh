#!/usr/bin/env brush

PAGE="A4" ./configure --prefix=/usr
make -j1
make DESTDIR="$DESTDIR" install
