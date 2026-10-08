use super::*;

pub fn package() -> Package {
    Package::new("meson", "1.12.0")
        .dependencies([python::package()])
        .make_dependencies([
            ninja::package(),
            setuptools::package(),
            wheel::package(),
        ])
        .build(Build::Python("meson"))
}
