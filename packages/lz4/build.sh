#!/usr/bin/env brush

make BUILD_STATIC=no PREFIX=/usr
make DESTDIR="$DESTDIR" BUILD_STATIC=no PREFIX=/usr install
