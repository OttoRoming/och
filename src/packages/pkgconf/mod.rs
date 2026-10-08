use super::*;

pub fn package() -> Package {
    Package::new("pkgconf", "3.0.5")
        .http_src(
            "https://github.com/mesonbuild/meson/releases/download/1.12.0/meson-1.12.0.tar.gz",
            hex!("88afe0c20e52030218924ac37d0c81c59b4b5f3ae3752c8c6d7470c7d365886c"),
        )
        .http_src(
            "https://distfiles.ariadne.space/pkgconf/pkgconf-3.0.5.tar.xz",
            hex!("3acd3a8a3cce65a8d620321855d92fb602e026cbe8e13ee36bdec58483b59ace"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            binutils::package(),
            gcc::package(),
            meson::package(),
            ninja::package(),
            python::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
