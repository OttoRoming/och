use super::*;

pub fn package() -> Package {
    Package::new("man-db", "2.13.1")
        .dependencies([
            bash::package(),
            gdbm::package(),
            groff::package(),
            glibc::package(),
            gzip::package(),
            less::package(),
            libpipeline::package(),
            zlib::package(),
        ])
        .make_dependencies([
            binutils::package(),
            bzip2::package(),
            coreutils::package(),
            flex::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            pkgconf::package(),
            sed::package(),
            systemd::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-setuid")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/man-db-2.13.1")
                .var("sysconfdir", "/etc")
                .var("enable-cache-owner", "bin")
                .var("with-browser", "/usr/bin/lynx")
                .var("with-vgrind", "/usr/bin/vgrind")
                .var("with-grap", "/usr/bin/grap"),
        ))
        .check(Check::Make)
}
