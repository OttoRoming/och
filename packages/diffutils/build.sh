#!/usr/bin/env brush

./configure --prefix=/usr

make
make DESTDIR="$DESTDIR" install
