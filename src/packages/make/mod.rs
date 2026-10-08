use super::*;

pub fn package() -> Package {
    Package::new("make", "4.4.1")
        .http_src(
            "https://ftpmirror.gnu.org/make/make-4.4.1.tar.gz",
            hex!("dd16fb1d67bfab79a72f5e8390735c49e3e8e70b4945a15ab1f81ddb78658fb3"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
