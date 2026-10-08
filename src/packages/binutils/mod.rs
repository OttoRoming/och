use super::*;

pub fn package() -> Package {
    Package::new("binutils", "2.47")
        .http_src(
            "https://sourceware.org/pub/binutils/releases/binutils-2.47.tar.xz",
            hex!("154ab23b60070e8f27013c22977f1129425d67d1e8acd6e13010e617811e4cff"),
        )
        .dependencies([
            glibc::package(),
            zlib::package(),
            zstd::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            file::package(),
            flex::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            perl::package(),
            pkgconf::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
