#!/usr/bin/env brush

./configure --prefix=/usr --sysconfdir=/etc

make
make DESTDIR="$DESTDIR" install
