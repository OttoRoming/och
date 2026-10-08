#!/usr/bin/env brush

make -j$(($(nproc)>4?$(nproc):4)) check
