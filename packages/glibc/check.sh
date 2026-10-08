#!/usr/bin/env brush

cd build
make check
grep "Timed out" $(find -name \*.out)
