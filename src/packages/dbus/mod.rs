use super::*;

pub fn package() -> Package {
    Package::new("dbus", "1.16.2")
        .http_src(
            "https://dbus.freedesktop.org/releases/dbus/dbus-1.16.2.tar.xz",
            hex!("0ba2a1a4b16afe7bceb2c07e9ce99a8c2c3508e5dec290dbb643384bd6beb7e2"),
        )
        .dependencies([
            glibc::package(),
            systemd::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            pkgconf::package(),
            sed::package(),
            util_linux::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
