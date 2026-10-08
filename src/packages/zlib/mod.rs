use super::*;

pub fn package() -> Package {
    Package::new("zlib", "1.3.2")
        .bootstrap()
        .http_src(
            "https://zlib.net/fossils/zlib-1.3.2.tar.gz",
            hex!("bb329a0a2cd0274d05519d61c667c062e06990d72e125ee2dfa8de64f0119d16"),
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
