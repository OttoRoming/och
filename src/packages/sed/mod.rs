use super::*;

pub fn package() -> Package {
    Package::new("sed", "4.10")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/sed/sed-4.10.tar.xz",
            hex!("b8e72182b2ec96a3574e2998c47b7aaa64cc20ce000d8e9ac313cc07cecf28c7"),
        )
        .dependencies([
            acl::package,
            attr::package,
            glibc::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            gcc::package,
            gettext::package,
            grep::package,
            make::package,
            sed::package,
            texinfo::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
