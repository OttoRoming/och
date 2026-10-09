#!/usr/bin/env brush

./configure --prefix=/usr

make
make DESTDIR="$DESTDIR" install

# Optionally install the components belonging in a TeX installation
make DESTDIR="$DESTDIR" TEXMF=/usr/share/texmf install-tex
