use super::*;

pub fn package() -> Package {
    Package::new("file", "5.48")
        .http_src(
            "https://astron.com/pub/file/file-5.48.tar.gz",
            hex!("ed14656883b23a364b4057c05595d93252da9bc473d30106519519d0da141283"),
        )
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
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
