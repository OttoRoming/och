use super::*;

pub fn package() -> Package {
    Package::new("bison", "3.8.2")
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            m4::package(),
            make::package(),
            perl::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/bison-3.8.2"),
        ))
        .check(Check::Make)
}
