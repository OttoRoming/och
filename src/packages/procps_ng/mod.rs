use super::*;

pub fn package() -> Package {
    Package::new("procps-ng", "4.0.7")
        .http_src(
            "https://sourceforge.net/projects/procps-ng/files/Production/procps-ng-4.0.7.tar.xz",
            hex!("9d2021f47a4501c667862c9942a92d1953694b21d11bcd1702e83eb594e3d67d"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            make::package(),
            ncurses::package(),
            pkgconf::package(),
            systemd::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
