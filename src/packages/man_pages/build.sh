#!/usr/bin/env brush

rm -v man3/crypt*
make -R GIT=false prefix=/usr install
