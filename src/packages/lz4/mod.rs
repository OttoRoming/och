use super::*;

pub fn package() -> Package {
    Package::new("lz4", "1.10.0")
        .http_src(
            "https://github.com/lz4/lz4/releases/download/v1.10.0/lz4-1.10.0.tar.gz",
            hex!("537512904744b35e232912055ccf8ec66d768639ff3abe5788d90d792ec5f48b"),
        )
        .dependencies([glibc::package])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            gcc::package,
            make::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
