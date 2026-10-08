use super::*;

pub fn package() -> Package {
    Package::new("zstd", "1.5.7")
        .http_src(
            "https://github.com/facebook/zstd/releases/download/v1.5.7/zstd-1.5.7.tar.gz",
            hex!("eb33e51f49a15e023950cd7825ca74a4a2b43db8354825ac24fc1b7ee09e6fa3"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            gzip::package(),
            lz4::package(),
            make::package(),
            xz::package(),
            zlib::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
