#!/usr/bin/env brush

patch -Np1 -i ../kbd-2.10.0-backspace-1.patch
sed -i '/RESIZECONS_PROGS=/s/yes/no/' configure
sed -i 's/resizecons.8 //' docs/man/man8/Makefile.in
./configure --prefix=/usr --disable-vlock
make
make DESTDIR="$DESTDIR" install
cp -R -v docs/doc -T /usr/share/doc/kbd-2.10.0
