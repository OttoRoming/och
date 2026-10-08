use super::*;

pub fn package() -> Package {
    Package::new("iproute2", "7.1.0")
        .http_src(
            "https://www.kernel.org/pub/linux/utils/net/iproute2/iproute2-7.1.0.tar.xz",
            hex!("fd9fa1b95809417157ca83dd72957e3261bdbce896353cb936f80af0b33a4b5c"),
        )
        .dependencies([
            bash::package(),
            coreutils::package(),
            glibc::package(),
            libcap::package(),
            libelf::package(),
            zlib::package(),
        ])
        .make_dependencies([
            bison::package(),
            flex::package(),
            gcc::package(),
            make::package(),
            pkgconf::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
