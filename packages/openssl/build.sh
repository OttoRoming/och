#!/usr/bin/env brush

./config --prefix=/usr         \
         --openssldir=/etc/ssl \
         --libdir=lib          \
         shared                \
         zlib-dynamic
make
make DESTDIR="$DESTDIR" INSTALL_LIBS= MANSUFFIX=ssl install
mv -v /usr/share/doc/openssl /usr/share/doc/openssl-4.0.1
cp -vfr doc/* /usr/share/doc/openssl-4.0.1
