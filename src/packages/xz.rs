use super::*;

pub fn package() -> Package {
    Package::new("xz", "5.8.3")
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            make::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/xz-5.8.3"),
        ))
        .check(Check::Make)
}
