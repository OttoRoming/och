use super::*;

pub fn package() -> Package {
    Package::new("coreutils", "9.11")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/coreutils/coreutils-9.11.tar.xz",
            hex!("394024eda0a5955217ceda9cd1201e65dc8fa3aa29c2951135a49521d57c3cc3"),
        )
        .http_src(
            "https://www.linuxfromscratch.org/patches/lfs/13.1/coreutils-9.11-i18n-1.patch",
            hex!("7c2aecf9ae24b87b019595e382b9f41d2c96de7f95abe95ad8cacc7e2ce1bbcc"),
        )
        .dependencies([glibc::package])
        .make_dependencies([
            autoconf::package,
            automake::package,
            bash::package,
            binutils::package,
            coreutils::package,
            gcc::package,
            gettext::package,
            gmp::package,
            grep::package,
            libcap::package,
            make::package,
            openssl::package,
            patch::package,
            perl::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
