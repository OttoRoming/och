use super::*;

pub fn package() -> Package {
    Package::new("setuptools", "84.0.0")
        .dependencies([python::package])
        .make_dependencies([wheel::package])
        .build(Build::Python("setuptools"))
}
