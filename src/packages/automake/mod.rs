use super::*;

pub fn package() -> Package {
    Package::new("automake", "1.18.1")
        .http_src(
            "https://ftpmirror.gnu.org/automake/automake-1.18.1.tar.xz",
            hex!("168aa363278351b89af56684448f525a5bce5079d0b6842bd910fdd3f1646887"),
        )
        .dependencies([
            bash::package,
            coreutils::package,
            grep::package,
            m4::package,
            sed::package,
            texinfo::package,
        ])
        .make_dependencies([
            autoconf::package,
            gettext::package,
            make::package,
            perl::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
