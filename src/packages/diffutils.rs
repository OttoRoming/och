use super::*;

pub fn package() -> Package {
    Package::new("diffutils", "3.12")
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gawk::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
