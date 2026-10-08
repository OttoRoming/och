use super::*;

pub fn package() -> Package {
    Package::new("e2fsprogs", "1.47.4")
        .http_src(
            "https://downloads.sourceforge.net/project/e2fsprogs/e2fsprogs/v1.47.4/e2fsprogs-1.47.4.tar.gz",
            hex!("2cec05f39c20ee621f14926195664e66e6017190ac8e4bbdb16d86082e43c5da"),
        )
        .dependencies([
            glibc::package,
            util_linux::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gawk::package,
            gcc::package,
            grep::package,
            gzip::package,
            make::package,
            pkgconf::package,
            sed::package,
            systemd::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
