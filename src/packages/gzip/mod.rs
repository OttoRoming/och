use super::*;

pub fn package() -> Package {
    Package::new("gzip", "1.14")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/gzip/gzip-1.14.tar.xz",
            hex!("01a7b881bd220bfdf615f97b8718f80bdfd3f6add385b993dcf6efd14e8c0ac6"),
        )
        .dependencies([
            bash::package,
            glibc::package,
        ])
        .make_dependencies([
            binutils::package,
            coreutils::package,
            gcc::package,
            grep::package,
            make::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
