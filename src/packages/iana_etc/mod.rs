use super::*;

pub fn package() -> Package {
    Package::new("iana-etc", "20260805")
        .http_src(
            "https://github.com/Mic92/iana-etc/releases/download/20260805/iana-etc-20260805.tar.gz",
            hex!("29270860664e324107537f32ea476a333ca52d71b59c74bc06ecc3b0fa9cf490"),
        )
        .make_dependencies([coreutils::package()])
        .build(Build::new(include_str!("build.sh")))
}
