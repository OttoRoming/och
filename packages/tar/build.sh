#!/usr/bin/env brush

patch -Np1 -i ../tar-1.35-acl_fix-1.patch
FORCE_UNSAFE_CONFIGURE=1  \
./configure --prefix=/usr
make
make DESTDIR="$DESTDIR" install
make -C doc DESTDIR="$DESTDIR" install-html docdir=/usr/share/doc/tar-1.35
