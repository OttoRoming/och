use super::*;

pub fn package() -> Package {
    Package::new("file", "5.48")
        .dependencies([
            glibc::package(),
            bzip2::package(),
            xz::package(),
            zlib::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
            zstd::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
