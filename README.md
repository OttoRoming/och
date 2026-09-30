# OCH - Package manager for Elsie Linux

/ɔkː/

## LFS 13.1 — Chapter 8 packaging status

Chapter 8 of the LFS book (*Installing Basic System Software*) builds 80
packages. The ones that are built with a plain

```sh
./configure [options]
make
make install
```

— optionally with a simple `make check` in between — are implemented as
modules under `src/packages/`. Everything else is listed below, grouped by
whatever stops it from being described with the current `Package` builder, so
that the missing pieces are easy to spot later.

Recipes live in `lfs/13.1/chapter08/`; the dependency lists come from
`lfs/13.1/appendices/dependencies.html`, where

- `Required at runtime` becomes `.dependencies(...)`, and
- `Installation depends on` becomes `.make_dependencies(...)`, minus
  anything already listed as a runtime dependency.

### Packaged

| Package | Version | Module |
| --- | --- | --- |
| Acl | 2.4.0 | [acl.rs](src/packages/acl.rs) |
| Attr | 2.6.0 | [attr.rs](src/packages/attr.rs) |
| Autoconf | 2.73 | [autoconf.rs](src/packages/autoconf.rs) |
| Automake | 1.18.1 | [automake.rs](src/packages/automake.rs) |
| Bash | 5.3 | [bash.rs](src/packages/bash.rs) |
| Bison | 3.8.2 | [bison.rs](src/packages/bison.rs) |
| Diffutils | 3.12 | [diffutils.rs](src/packages/diffutils.rs) |
| Expat | 2.8.3 | [expat.rs](src/packages/expat.rs) |
| File | 5.48 | [file.rs](src/packages/file.rs) |
| Findutils | 4.11.0 | [findutils.rs](src/packages/findutils.rs) |
| GDBM | 1.26 | [gdbm.rs](src/packages/gdbm.rs) |
| Gperf | 3.3 | [gperf.rs](src/packages/gperf.rs) |
| Gzip | 1.14 | [gzip.rs](src/packages/gzip.rs) |
| Less | 704 | [less.rs](src/packages/less.rs) |
| Libffi | 3.8.0 | [libffi.rs](src/packages/libffi.rs) |
| Libpipeline | 1.5.8 | [libpipeline.rs](src/packages/libpipeline.rs) |
| M4 | 1.4.21 | [m4.rs](src/packages/m4.rs) |
| Make | 4.4.1 | [make.rs](src/packages/make.rs) |
| Man-DB | 2.13.1 | [man_db.rs](src/packages/man_db.rs) |
| mpdecimal | 4.0.1 | [mpdecimal.rs](src/packages/mpdecimal.rs) |
| Patch | 2.8 | [patch.rs](src/packages/patch.rs) |
| Pcre2 | 10.47 | [pcre2.rs](src/packages/pcre2.rs) |
| Procps-ng | 4.0.7 | [procps_ng.rs](src/packages/procps_ng.rs) |
| Psmisc | 23.7 | [psmisc.rs](src/packages/psmisc.rs) |
| Texinfo | 7.3 | [texinfo.rs](src/packages/texinfo.rs) |
| Util-linux | 2.42.2 | [util_linux.rs](src/packages/util_linux.rs) |
| Xz | 5.8.3 | [xz.rs](src/packages/xz.rs) |

## Not packaged (categorised for future work)

### Data only — no build system

Files are copied straight into place.

- **Iana-Etc-20260805** — `cp -v services protocols /etc`

### Plain `make` — no `configure` script

- **Man-pages-6.18** — installs with `make -R GIT=false prefix=/usr install`
- **Bzip2-1.0.8** — plus a patch and two `sed` edits on the `Makefile`
- **Lz4-1.10.0** — `make BUILD_STATIC=no PREFIX=/usr`
- **Zstd-1.5.7** — `make prefix=/usr`

### Different build system

#### Meson + Ninja

- **Pkgconf-3.0.5** — bootstraps Meson from its source tarball first
- **Kmod-34.2**
- **Systemd-261.2**
- **D-Bus-1.16.2**

#### Python (`pip3 wheel` / `pip3 install`)

- **Flit-Core-4.0.2**
- **Packaging-26.3**
- **Wheel-0.48.0**
- **Setuptools-84.0.0**
- **Meson-1.12.0**
- **MarkupSafe-3.0.3**
- **Jinja2-3.1.6**

### Custom / multi-stage builds

Not a single configure → make → install pass at all.

- **Glibc-2.44** — two patches, separate build directory, `nsswitch.conf`,
  time zone data and locale generation
- **GCC-16.2.0** — `sed` on the target files, build directory, plugin and
  start-file symlinks, toolchain sanity checks
- **Python-3.14.7** — patch, then a custom preformatted-documentation install
- **Ninja-1.13.2** — bootstrapped with `python3 configure.py --bootstrap`
- **Vim-9.2.1025** — `feature.h` edit, `vi`/man-page symlinks, `/etc/vimrc`
- **Sqlite-3530400** — documentation unzip, `make LDFLAGS.rpath=""`
- **Ncurses-6.6** — `DESTDIR` staging install plus a second ABI-5 build
- **Libelf from Elfutils-0.195** — builds and installs only `lib`/`libelf`
  targets by hand
- **Libxcrypt-4.5.2** — configure/build/install twice with different options
- **Shadow-4.20.2** — `make exec_prefix=/usr install` plus
  `make -C man install-man`
- **Tcl-8.6.18** — built from `unix/`, generated config files rewritten with
  `sed`, private headers installed separately

### Non-standard `configure` invocation

The configure step needs environment variables or arguments the builder
cannot express yet.

- **OpenSSL-4.0.1** — uses `./config`, not `./configure`
- **Perl-5.44.0** — `sh Configure -des -D ...`
- **Bc-7.0.3** — `CC='gcc -std=c99' ./configure -G -O3 -r`
- **Groff-1.24.1** — `PAGE=<paper_size> ./configure`

### Out-of-tree build directory

- **Binutils-2.47** — also builds/installs with `make tooldir=/usr`
- **DejaGNU-1.6.3** — also generates documentation with `makeinfo`
- **E2fsprogs-1.47.4**

### Patch or `sed` fixups before configuring

- **Readline-8.3**
- **Expect-5.45.4**
- **GMP-6.3.0**
- **Libcap-2.78**
- **Gawk-5.4.1**
- **Grep-3.12**
- **Kbd-2.10.0**
- **Tar-1.35**
- **Coreutils-9.11** — also needs `autoreconf`/`automake`
- **Inetutils-2.8**
- **GRUB-2.14** — also configured three times (BIOS, 64-bit UEFI, 32-bit UEFI)
- **IPRoute2-7.1.0**

### Extra or non-standard `make` targets

- **MPFR-4.2.2** — `make html` + `make install-html`
- **MPC-1.4.1** — `make html` + `make install-html`
- **Sed-4.10** — `make html` plus a manual documentation install

### Commands after `make install`

- **Zlib-1.3.2** — `rm -fv /usr/lib/libz.a`
- **Flex-2.6.4** — `lex` and `lex.1` symlinks
- **Gettext-1.0** — `chmod -v 0755 /usr/lib/preloadable_libintl.so`
- **Libtool-2.6.2** — `rm -fv /usr/lib/libltdl.a`
