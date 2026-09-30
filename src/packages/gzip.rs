use super::*;

pub fn package() -> Package {
    Package::new("gzip", "1.14")
        .dependencies([
            bash::package(),
            glibc::package(),
        ])
        .make_dependencies([
            binutils::package(),
            coreutils::package(),
            gcc::package(),
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
