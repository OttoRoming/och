use super::*;

pub fn package() -> Package {
    Package::new("flex", "2.6.4")
        .http_src(
            "https://github.com/westes/flex/releases/download/v2.6.4/flex-2.6.4.tar.gz",
            hex!("e87aae032bf07c26f85ac0ed3250998c37621d95f8bd748b31f15b33c45ee995"),
        )
        .dependencies([
            bash::package,
            glibc::package,
            m4::package,
        ])
        .make_dependencies([
            binutils::package,
            coreutils::package,
            gcc::package,
            gettext::package,
            grep::package,
            make::package,
            patch::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
