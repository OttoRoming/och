use super::*;

pub fn package() -> Package {
    Package::new("util-linux", "2.42.2")
        .dependencies([
            glibc::package(),
            ncurses::package(),
            readline::package(),
            systemd::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            file::package(),
            findutils::package(),
            gawk::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            pkgconf::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-chfn-chsh")
                .opt("disable-login")
                .opt("disable-nologin")
                .opt("disable-su")
                .opt("disable-setpriv")
                .opt("disable-runuser")
                .opt("disable-pylibmount")
                .opt("disable-liblastlog2")
                .opt("disable-static")
                .opt("without-python")
                .var("bindir", "/usr/bin")
                .var("libdir", "/usr/lib")
                .var("runstatedir", "/run")
                .var("sbindir", "/usr/sbin")
                .var("ADJTIME_PATH", "/var/lib/hwclock/adjtime")
                .var("docdir", "/usr/share/doc/util-linux-2.42.2"),
        ))
}
