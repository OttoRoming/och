#!/usr/bin/env brush

cd build
make -k check
grep '^FAIL:' $(find -name '*.log')
