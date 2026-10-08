use super::*;

pub fn package() -> Package {
    Package::new("mpc", "1.4.1")
        .http_src(
            "https://ftpmirror.gnu.org/mpc/mpc-1.4.1.tar.xz",
            hex!("91204cd32f164bd3b7c992d4a6a8ce6519511aadab30f78b6982d0bf8d73e931"),
        )
        .dependencies([
            glibc::package,
            gmp::package,
            mpfr::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gawk::package,
            gcc::package,
            grep::package,
            make::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
