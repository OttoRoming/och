#!/usr/bin/env brush

cd build
ulimit -s -H unlimited
make -k check
../contrib/test_summary -t
