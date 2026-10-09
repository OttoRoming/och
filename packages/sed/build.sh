#!/usr/bin/env brush

./configure --prefix=/usr
make
make html
make DESTDIR="$DESTDIR" install
install -vDm644 doc/sed.html -t "$DESTDIR"/usr/share/doc/sed-4.10
