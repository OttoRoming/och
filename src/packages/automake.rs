use super::*;

pub fn package() -> Package {
    Package::new("automake", "1.18.1")
        .dependencies([
            bash::package(),
            coreutils::package(),
            grep::package(),
            m4::package(),
            sed::package(),
            texinfo::package(),
        ])
        .make_dependencies([
            autoconf::package(),
            gettext::package(),
            make::package(),
            perl::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/automake-1.18.1"),
        ))
        .check(Check::Make)
}
