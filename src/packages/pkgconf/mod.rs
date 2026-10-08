use super::*;

pub fn package() -> Package {
    Package::new("pkgconf", "3.0.5")
        .http_src(
            "https://distfiles.ariadne.space/pkgconf/pkgconf-3.0.5.tar.xz",
            hex!("3acd3a8a3cce65a8d620321855d92fb602e026cbe8e13ee36bdec58483b59ace"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            binutils::package(),
            gcc::package(),
            ninja::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
