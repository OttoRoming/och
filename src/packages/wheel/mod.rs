use super::*;

pub fn package() -> Package {
    Package::new("wheel", "0.48.0")
        .dependencies([python::package()])
        .make_dependencies([
            flit_core::package(),
            packaging::package(),
        ])
        .build(Build::Python("wheel"))
}
