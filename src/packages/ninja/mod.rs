use super::*;

pub fn package() -> Package {
    Package::new("ninja", "1.13.2")
        .http_src(
            "https://github.com/ninja-build/ninja/archive/v1.13.2/ninja-1.13.2.tar.gz",
            hex!("974d6b2f4eeefa25625d34da3cb36bdcebe7fbce40f4c16ac0835fd1c0cbae17"),
        )
        .dependencies([
            gcc::package(),
            glibc::package(),
        ])
        .make_dependencies([
            binutils::package(),
            coreutils::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
