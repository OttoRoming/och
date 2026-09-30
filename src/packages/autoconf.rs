use super::*;

pub fn package() -> Package {
    Package::new("autoconf", "2.73")
        .dependencies([
            bash::package(),
            coreutils::package(),
            grep::package(),
            m4::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .make_dependencies([perl::package()])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
