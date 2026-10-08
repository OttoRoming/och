use super::*;

pub fn package() -> Package {
    Package::new("mpfr", "4.2.2")
        .http_src(
            "https://ftpmirror.gnu.org/mpfr/mpfr-4.2.2.tar.xz",
            hex!("b67ba0383ef7e8a8563734e2e889ef5ec3c3b898a01d00fa0a6869ad81c6ce01"),
        )
        .dependencies([
            glibc::package(),
            gmp::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
