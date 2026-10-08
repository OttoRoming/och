#!/usr/bin/env brush

tar -xf ../meson-1.12.0.tar.gz
mkdir build
cd    build

python3 ../meson-1.12.0/meson.py setup --prefix=/usr --buildtype=release ..
ninja
ninja install
mv /usr/share/doc/pkgconf{,-3.0.5}
ln -sv pkgconf   /usr/bin/pkg-config
ln -sv pkgconf.1 /usr/share/man/man1/pkg-config.1
