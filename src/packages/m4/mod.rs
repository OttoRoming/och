use super::*;

pub fn package() -> Package {
    Package::new("m4", "1.4.21")
        .http_src(
            "https://ftpmirror.gnu.org/m4/m4-1.4.21.tar.xz",
            hex!("f25c6ab51548a73a75558742fb031e0625d6485fe5f9155949d6486a2408ab66"),
        )
        .dependencies([
            bash::package(),
            glibc::package(),
        ])
        .make_dependencies([
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
