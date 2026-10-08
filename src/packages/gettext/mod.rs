use super::*;

pub fn package() -> Package {
    Package::new("gettext", "1.0")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/gettext/gettext-1.0.tar.xz",
            hex!("71132a3fb71e68245b8f2ac4e9e97137d3e5c02f415636eb508ae607bc01add7"),
        )
        .dependencies([
            acl::package,
            bash::package,
            gcc::package,
            glibc::package,
        ])
        .make_dependencies([
            binutils::package,
            coreutils::package,
            gawk::package,
            grep::package,
            make::package,
            ncurses::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
