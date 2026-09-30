use super::*;

pub fn package() -> Package {
    Package::new("procps-ng", "4.0.7")
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
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .opt("disable-kill")
                .opt("enable-watch8bit")
                .opt("with-systemd")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/procps-ng-4.0.7"),
        ))
}
