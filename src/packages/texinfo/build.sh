#!/usr/bin/env brush

./configure --prefix=/usr

make
make install

# Optionally install the components belonging in a TeX installation
make TEXMF=/usr/share/texmf install-tex
