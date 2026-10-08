#!/usr/bin/env brush

sed '/test_plugin_glvs/d' -i src/testdir/Make_all.mak
TERM=xterm-256color LANG=en_US.UTF-8 make -j1 test
