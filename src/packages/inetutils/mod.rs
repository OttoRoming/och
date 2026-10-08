use super::*;

pub fn package() -> Package {
    Package::new("inetutils", "2.8")
        .http_src(
            "https://ftpmirror.gnu.org/inetutils/inetutils-2.8.tar.gz",
            hex!("57b3cf4f77555992881e5ba2a09a63b05aa2c56342a60ed4305b5f45938390b5"),
        )
        .dependencies([
            gcc::package,
            glibc::package,
            ncurses::package,
            readline::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            grep::package,
            make::package,
            patch::package,
            sed::package,
            texinfo::package,
            zlib::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
