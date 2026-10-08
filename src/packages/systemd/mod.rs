use super::*;

pub fn package() -> Package {
    Package::new("systemd", "261.2")
        .http_src(
            "https://github.com/systemd/systemd/archive/v261.2/systemd-261.2.tar.gz",
            hex!("ed1059ff964f5df35b6056434cc17cc83f86dc913f10489948a0b19b6081c5ec"),
        )
        .http_src(
            "https://anduin.linuxfromscratch.org/LFS/systemd-man-pages-261.2.tar.xz",
            hex!("9be13865c9c7f9fecba7b86fc5185aecef00164c87ed68b4f6884283aefebe50"),
        )
        .dependencies([
            acl::package,
            glibc::package,
            libxcrypt::package,
            openssl::package,
            util_linux::package,
            xz::package,
            zlib::package,
            zstd::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gawk::package,
            gcc::package,
            gperf::package,
            grep::package,
            jinja2::package,
            lz4::package,
            meson::package,
            pcre2::package,
            pkgconf::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
