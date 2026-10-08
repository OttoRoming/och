use super::*;

pub fn package() -> Package {
    Package::new("markupsafe", "3.0.3")
        .dependencies([python::package()])
        .make_dependencies([
            setuptools::package(),
            wheel::package(),
        ])
        .build(Build::Python("Markupsafe"))
}
