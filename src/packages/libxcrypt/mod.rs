use super::*;

pub fn package() -> Package {
    Package::new("libxcrypt", "4.5.2")
        .http_src(
            "https://github.com/besser82/libxcrypt/releases/download/v4.5.2/libxcrypt-4.5.2.tar.xz",
            hex!("71513a31c01a428bccd5367a32fd95f115d6dac50fb5b60c779d5c7942aec071"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            perl::package(),
            sed::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
