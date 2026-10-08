use super::*;

pub fn package() -> Package {
    Package::new("expat", "2.8.3")
        .http_src(
            "https://github.com/libexpat/libexpat/releases/download/R_2_8_3/expat-2.8.3.tar.xz",
            hex!("f6256df90c906773d344da084402b7d3e4f22ed41b1a59c989098a83d3ea0c85"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
