use super::*;

pub fn package() -> Package {
    Package::new("acl", "2.4.0")
        .http_src(
            "https://download.savannah.gnu.org/releases/acl/acl-2.4.0.tar.xz",
            hex!("e661131456d2708a01c614a0f400e11d7d1bfaeb6f3e74b75bb980b72f0161a3"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            m4::package(),
            make::package(),
            perl::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
