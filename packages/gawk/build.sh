#!/usr/bin/env brush

sed -i 's/extras//' Makefile.in
./configure --prefix=/usr
make
rm -f /usr/bin/gawk-5.4.1
make DESTDIR="$DESTDIR" install
ln -sv gawk.1 /usr/share/man/man1/awk.1
install -vDm644 doc/{awkforai.txt,*.{eps,pdf,jpg}} -t "$DESTDIR"/usr/share/doc/gawk-5.4.1
