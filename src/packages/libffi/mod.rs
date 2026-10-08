use super::*;

pub fn package() -> Package {
    Package::new("libffi", "3.8.0")
        .http_src(
            "https://github.com/libffi/libffi/releases/download/v3.8.0/libffi-3.8.0.tar.gz",
            hex!("7da3e2d9a171eb0a038f592ecad3ff2bb2550f3496d87b3b29ad0cf4430c0db4"),
        )
        .dependencies([glibc::package])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            gcc::package,
            make::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
