use super::*;

pub fn package() -> Package {
    Package::new("man-db", "2.13.1")
        .http_src(
            "https://download.savannah.gnu.org/releases/man-db/man-db-2.13.1.tar.xz",
            hex!("8afebb6f7eb6bb8542929458841f5c7e6f240e30c86358c1fbcefbea076c87d9"),
        )
        .dependencies([
            bash::package(),
            gdbm::package(),
            groff::package(),
            glibc::package(),
            gzip::package(),
            less::package(),
            libpipeline::package(),
            zlib::package(),
        ])
        .make_dependencies([
            binutils::package(),
            bzip2::package(),
            coreutils::package(),
            flex::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            pkgconf::package(),
            sed::package(),
            systemd::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
