use super::*;

pub fn package() -> Package {
    Package::new("libffi", "3.8.0")
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .var("with-gcc-arch", "native")
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
