#!/usr/bin/env brush

./configure --prefix=/usr
make
make html
make install
install -vDm644 doc/sed.html -t /usr/share/doc/sed-4.10
