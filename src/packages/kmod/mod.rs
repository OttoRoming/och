use super::*;

pub fn package() -> Package {
    Package::new("kmod", "34.2")
        .http_src(
            "https://www.kernel.org/pub/linux/utils/kernel/kmod/kmod-34.2.tar.xz",
            hex!("5a5d5073070cc7e0c7a7a3c6ec2a0e1780850c8b47b3e3892226b93ffcb9cb54"),
        )
        .dependencies([
            glibc::package(),
            xz::package(),
            zlib::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            bison::package(),
            coreutils::package(),
            flex::package(),
            gcc::package(),
            gettext::package(),
            gzip::package(),
            make::package(),
            openssl::package(),
            pkgconf::package(),
            sed::package(),
            zstd::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
