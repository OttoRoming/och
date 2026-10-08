use super::*;

pub fn package() -> Package {
    Package::new("jinja2", "3.1.6")
        .dependencies([
            markupsafe::package,
            python::package,
        ])
        .make_dependencies([
            setuptools::package,
            wheel::package,
        ])
        .build(Build::Python("Jinja2"))
}
