use super::*;

pub fn package() -> Package {
    Package::new("attr", "2.6.0")
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            m4::package(),
            make::package(),
            perl::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .var("prefix", "/usr")
                .var("sysconfdir", "/etc")
                .var("docdir", "/usr/share/doc/attr-2.6.0"),
        ))
        .check(Check::Make)
}
