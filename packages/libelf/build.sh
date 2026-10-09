#!/usr/bin/env brush

./configure --prefix=/usr        \
            --disable-debuginfod \
            --enable-libdebuginfod=dummy
make -C lib
make -C libelf
make -C libelf DESTDIR="$DESTDIR" install
install -vm644 config/libelf.pc "$DESTDIR"/usr/lib/pkgconfig
rm /usr/lib/libelf.a
