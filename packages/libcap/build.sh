#!/usr/bin/env brush

sed -i '/install -m.*STA/d' libcap/Makefile
make prefix=/usr lib=lib
make DESTDIR="$DESTDIR" prefix=/usr lib=lib install
