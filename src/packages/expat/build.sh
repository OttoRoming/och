#!/usr/bin/env brush

./configure --prefix=/usr    \
            --disable-static \
            --docdir=/usr/share/doc/expat-2.8.3

make
make install

# Optionally install the documentation
install -v -m644 doc/*.{html,css} "${DESTDIR}/usr/share/doc/expat-2.8.3"
