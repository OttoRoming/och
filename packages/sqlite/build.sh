#!/usr/bin/env brush

python3 -m zipfile -e ../sqlite-doc-3530400.zip .
./configure --prefix=/usr     \
            --disable-static  \
            --enable-fts{4,5} \
            CPPFLAGS="-D SQLITE_ENABLE_COLUMN_METADATA=1 \
                      -D SQLITE_ENABLE_UNLOCK_NOTIFY=1   \
                      -D SQLITE_ENABLE_DBSTAT_VTAB=1     \
                      -D SQLITE_SECURE_DELETE=1"
make LDFLAGS.rpath=""
make DESTDIR="$DESTDIR" install
cp -v -R sqlite-doc-3530400 -T /usr/share/doc/sqlite-3.53.4
