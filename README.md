# OCH - Package manager for Elsie Linux

/ɔkː/

## LFS 13.1 — Chapter 8 packaging status

Chapter 8 of the LFS book (*Installing Basic System Software*) builds 80
packages. 72 of them are implemented as modules under `src/packages/`; each
module keeps its metadata in `mod.rs` and its recipes in a sibling `build.sh`
and `check.sh`, embedded with `include_str!`. The remaining 8 are the Python
packages, listed under *Not packaged* below.

Recipes live in `lfs/13.1/chapter08/`; the dependency lists come from
`lfs/13.1/appendices/dependencies.html`, where

- `Required at runtime` becomes `.dependencies(...)`, and
- `Installation depends on` becomes `.make_dependencies(...)`, minus
  anything already listed as a runtime dependency.

Dependencies on packages this crate does not define (the excluded Python
packages, plus the chapter 5 Linux API headers) are omitted from the module
metadata so that the `packages` module compiles on its own.

### Packaged

| Package | Version | Module |
| --- | --- | --- |
| Acl | 2.4.0 | [acl](src/packages/acl/mod.rs) |
| Attr | 2.6.0 | [attr](src/packages/attr/mod.rs) |
| Autoconf | 2.73 | [autoconf](src/packages/autoconf/mod.rs) |
| Automake | 1.18.1 | [automake](src/packages/automake/mod.rs) |
| Bash | 5.3 | [bash](src/packages/bash/mod.rs) |
| Bc | 7.0.3 | [bc](src/packages/bc/mod.rs) |
| Binutils | 2.47 | [binutils](src/packages/binutils/mod.rs) |
| Bison | 3.8.2 | [bison](src/packages/bison/mod.rs) |
| Bzip2 | 1.0.8 | [bzip2](src/packages/bzip2/mod.rs) |
| Coreutils | 9.11 | [coreutils](src/packages/coreutils/mod.rs) |
| D-Bus | 1.16.2 | [dbus](src/packages/dbus/mod.rs) |
| DejaGNU | 1.6.3 | [dejagnu](src/packages/dejagnu/mod.rs) |
| Diffutils | 3.12 | [diffutils](src/packages/diffutils/mod.rs) |
| E2fsprogs | 1.47.4 | [e2fsprogs](src/packages/e2fsprogs/mod.rs) |
| Expat | 2.8.3 | [expat](src/packages/expat/mod.rs) |
| Expect | 5.45.4 | [expect](src/packages/expect/mod.rs) |
| File | 5.48 | [file](src/packages/file/mod.rs) |
| Findutils | 4.11.0 | [findutils](src/packages/findutils/mod.rs) |
| Flex | 2.6.4 | [flex](src/packages/flex/mod.rs) |
| Gawk | 5.4.1 | [gawk](src/packages/gawk/mod.rs) |
| GCC | 16.2.0 | [gcc](src/packages/gcc/mod.rs) |
| GDBM | 1.26 | [gdbm](src/packages/gdbm/mod.rs) |
| Gettext | 1.0 | [gettext](src/packages/gettext/mod.rs) |
| Glibc | 2.44 | [glibc](src/packages/glibc/mod.rs) |
| GMP | 6.3.0 | [gmp](src/packages/gmp/mod.rs) |
| Gperf | 3.3 | [gperf](src/packages/gperf/mod.rs) |
| Grep | 3.12 | [grep](src/packages/grep/mod.rs) |
| Groff | 1.24.1 | [groff](src/packages/groff/mod.rs) |
| GRUB | 2.14 | [grub](src/packages/grub/mod.rs) |
| Gzip | 1.14 | [gzip](src/packages/gzip/mod.rs) |
| Iana-Etc | 20260805 | [iana_etc](src/packages/iana_etc/mod.rs) |
| Inetutils | 2.8 | [inetutils](src/packages/inetutils/mod.rs) |
| IPRoute2 | 7.1.0 | [iproute2](src/packages/iproute2/mod.rs) |
| Kbd | 2.10.0 | [kbd](src/packages/kbd/mod.rs) |
| Kmod | 34.2 | [kmod](src/packages/kmod/mod.rs) |
| Less | 704 | [less](src/packages/less/mod.rs) |
| Libcap | 2.78 | [libcap](src/packages/libcap/mod.rs) |
| Libelf | 0.195 | [libelf](src/packages/libelf/mod.rs) |
| Libffi | 3.8.0 | [libffi](src/packages/libffi/mod.rs) |
| Libpipeline | 1.5.8 | [libpipeline](src/packages/libpipeline/mod.rs) |
| Libtool | 2.6.2 | [libtool](src/packages/libtool/mod.rs) |
| Libxcrypt | 4.5.2 | [libxcrypt](src/packages/libxcrypt/mod.rs) |
| Lz4 | 1.10.0 | [lz4](src/packages/lz4/mod.rs) |
| M4 | 1.4.21 | [m4](src/packages/m4/mod.rs) |
| Make | 4.4.1 | [make](src/packages/make/mod.rs) |
| Man-DB | 2.13.1 | [man_db](src/packages/man_db/mod.rs) |
| Man-pages | 6.18 | [man_pages](src/packages/man_pages/mod.rs) |
| MPC | 1.4.1 | [mpc](src/packages/mpc/mod.rs) |
| mpdecimal | 4.0.1 | [mpdecimal](src/packages/mpdecimal/mod.rs) |
| MPFR | 4.2.2 | [mpfr](src/packages/mpfr/mod.rs) |
| Ncurses | 6.6 | [ncurses](src/packages/ncurses/mod.rs) |
| Ninja | 1.13.2 | [ninja](src/packages/ninja/mod.rs) |
| OpenSSL | 4.0.1 | [openssl](src/packages/openssl/mod.rs) |
| Patch | 2.8 | [patch](src/packages/patch/mod.rs) |
| Pcre2 | 10.47 | [pcre2](src/packages/pcre2/mod.rs) |
| Perl | 5.44.0 | [perl](src/packages/perl/mod.rs) |
| Pkgconf | 3.0.5 | [pkgconf](src/packages/pkgconf/mod.rs) |
| Procps-ng | 4.0.7 | [procps_ng](src/packages/procps_ng/mod.rs) |
| Psmisc | 23.7 | [psmisc](src/packages/psmisc/mod.rs) |
| Readline | 8.3 | [readline](src/packages/readline/mod.rs) |
| Sed | 4.10 | [sed](src/packages/sed/mod.rs) |
| Shadow | 4.20.2 | [shadow](src/packages/shadow/mod.rs) |
| Sqlite | 3530400 | [sqlite](src/packages/sqlite/mod.rs) |
| Systemd | 261.2 | [systemd](src/packages/systemd/mod.rs) |
| Tar | 1.35 | [tar](src/packages/tar/mod.rs) |
| Tcl | 8.6.18 | [tcl](src/packages/tcl/mod.rs) |
| Texinfo | 7.3 | [texinfo](src/packages/texinfo/mod.rs) |
| Util-linux | 2.42.2 | [util_linux](src/packages/util_linux/mod.rs) |
| Vim | 9.2.1025 | [vim](src/packages/vim/mod.rs) |
| Xz | 5.8.3 | [xz](src/packages/xz/mod.rs) |
| Zlib | 1.3.2 | [zlib](src/packages/zlib/mod.rs) |
| Zstd | 1.5.7 | [zstd](src/packages/zstd/mod.rs) |

## Not packaged

The Python packages, excluded from this pass:

- **Flit-Core-4.0.2**
- **Jinja2-3.1.6**
- **MarkupSafe-3.0.3**
- **Meson-1.12.0**
- **Packaging-26.3**
- **Python-3.14.7** — patch, then a custom preformatted-documentation install
- **Setuptools-84.0.0**
- **Wheel-0.48.0**

## Post-install configuration

Chapter 8 gives the following packages a dedicated *Configuring …* section
whose commands must run after `make install`. They are packaged now, but those
commands are deliberately **not** part of `build.sh`, because they write to
the live system (`/etc`, the dynamic loader, the root password) rather than
`$DESTDIR`, and will need a separate configuration step:

- **E2fsprogs-1.47.4** — `sed -i 's/metadata_csum_seed,//' /etc/mke2fs.conf`
- **Glibc-2.44** — `/etc/nsswitch.conf`, time zone data (`tzselect`,
  `/etc/localtime`) and the dynamic loader (`/etc/ld.so.conf`, `ld.so.conf.d`)
- **Shadow-4.20.2** — `pwconv`/`grpconv`, `useradd -D` defaults,
  `/etc/sub{u,g}id` and the root password
- **Vim-9.2.1025** — `/etc/vimrc`

## Notes

- The recipes are faithful transcriptions of chapter 8. Commands that install
  to absolute paths (`/usr`, `/etc`) do not honour `$DESTDIR`; this matters
  for the non-autotools packages (Bzip2, Iana-Etc, IPRoute2, Kbd, Lz4,
  Man-pages, Ninja, Shadow, Sqlite, Tcl, Zstd, …) once package building is
  implemented.
- `groff`'s `build.sh` resolves chapter 8's `<paper_size>` placeholder as
  `PAGE="${PAGE:-letter}"`.
- `check.sh` starts with `cd build` (or `cd unix` for Tcl, `cd build` for the
  Meson packages) where the tests must run in a subdirectory.
- The `tester` user handling was removed from the check recipes, as och does
  not assume a root install.
- GCC's toolchain sanity greps and Glibc's `systemctl disable --now nscd` are
  transcribed as-is and may return non-zero in a minimal environment.
