use super::*;

pub fn package() -> Package {
    Package::new("texinfo", "7.3")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/texinfo/texinfo-7.3.tar.xz",
            hex!("51f74eb0f51cfa9873b85264dfdd5d46e8957ec95b88f0fb762f63d9e164c72e"),
        )
        .dependencies([
            glibc::package,
            ncurses::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            gcc::package,
            gettext::package,
            grep::package,
            make::package,
            patch::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
