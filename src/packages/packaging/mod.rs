use super::*;

pub fn package() -> Package {
    Package::new("packaging", "26.3")
        .dependencies([python::package])
        .make_dependencies([flit_core::package])
        .build(Build::Python("packaging"))
}
