use super::*;

pub fn package() -> Package {
    Package::new("gperf", "3.3")
        .dependencies([
            gcc::package(),
            glibc::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            make::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/gperf-3.3"),
        ))
        .check(Check::Make)
}
