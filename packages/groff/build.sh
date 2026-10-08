#!/usr/bin/env brush

PAGE="${PAGE:-letter}" ./configure --prefix=/usr
make -j1
make install
