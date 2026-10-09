#!/usr/bin/env brush

rm -v man3/crypt*
make DESTDIR="$DESTDIR" -R GIT=false prefix=/usr install
