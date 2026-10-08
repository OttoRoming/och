#!/usr/bin/env brush

cd build
echo 'NAME="Linux From Scratch"' > /etc/os-release
unshare -m ninja test
