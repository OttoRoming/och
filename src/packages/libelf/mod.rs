use super::*;

pub fn package() -> Package {
    Package::new("libelf", "0.195")
        .http_src(
            "https://sourceware.org/ftp/elfutils/0.195/elfutils-0.195.tar.bz2",
            hex!("37629fdf7f1f3dc2818e138fca2b8094177d6c2d0f701d3bb650a561218dc026"),
        )
        .dependencies([
            bzip2::package(),
            glibc::package(),
            xz::package(),
            zlib::package(),
            zstd::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            make::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
