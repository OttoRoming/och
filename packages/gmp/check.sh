#!/usr/bin/env brush

make check
cat $(find -name '*.log') | grep -c ^PASS
