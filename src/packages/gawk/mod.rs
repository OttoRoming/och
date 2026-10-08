use super::*;

pub fn package() -> Package {
    Package::new("gawk", "5.4.1")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/gawk/gawk-5.4.1.tar.xz",
            hex!("07f6f7342b7febe4313fc2c2542ad93d64fe20ad8717200109f105a826f5fd37"),
        )
        .dependencies([
            bash::package,
            glibc::package,
            mpfr::package,
        ])
        .make_dependencies([
            binutils::package,
            coreutils::package,
            gcc::package,
            gettext::package,
            gmp::package,
            grep::package,
            make::package,
            patch::package,
            readline::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
