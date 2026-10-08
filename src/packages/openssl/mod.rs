use super::*;

pub fn package() -> Package {
    Package::new("openssl", "4.0.1")
        .http_src(
            "https://github.com/openssl/openssl/releases/download/openssl-4.0.1/openssl-4.0.1.tar.gz",
            hex!("2db3f3a0d6ea4b59e1f094ace2c8cd536dffb87cdc39084c5afa1e6f7f37dd09"),
        )
        .dependencies([
            glibc::package,
            perl::package,
        ])
        .make_dependencies([
            binutils::package,
            coreutils::package,
            gcc::package,
            make::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
