use super::*;

pub fn package() -> Package {
    Package::new("tar", "1.35")
        .http_src(
            "https://ftpmirror.gnu.org/tar/tar-1.35.tar.xz",
            hex!("4d62ff37342ec7aed748535323930c7cf94acf71c3591882b26a7ea50f3edc16"),
        )
        .http_src(
            "https://www.linuxfromscratch.org/patches/lfs/13.1/tar-1.35-acl_fix-1.patch",
            hex!("51cadccfcb2f29cfdb4ec18015360100aa167179a72ece040a6a8612aeec5081"),
        )
        .dependencies([
            acl::package(),
            attr::package(),
            bzip2::package(),
            glibc::package(),
            gzip::package(),
            xz::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            bison::package(),
            coreutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            inetutils::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
