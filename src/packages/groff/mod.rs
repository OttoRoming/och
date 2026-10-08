use super::*;

pub fn package() -> Package {
    Package::new("groff", "1.24.1")
        .http_src(
            "https://ftpmirror.gnu.org/groff/groff-1.24.1.tar.gz",
            hex!("74e2819795b6aff431aeac983d63a9c8968eeaba2a2eba7df8ba4c7b41e7cfd8"),
        )
        .dependencies([
            gcc::package(),
            glibc::package(),
            perl::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            bison::package(),
            coreutils::package(),
            gawk::package(),
            grep::package(),
            make::package(),
            patch::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
