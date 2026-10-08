use super::*;

pub fn package() -> Package {
    Package::new("flit-core", "4.0.2")
        .dependencies([python::package()])
        .build(Build::Python("flit_core"))
}
