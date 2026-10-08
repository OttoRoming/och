use super::*;

pub fn package() -> Package {
    Package::new("diffutils", "3.12")
        .bootstrap()
        .http_src(
            "https://ftpmirror.gnu.org/diffutils/diffutils-3.12.tar.xz",
            hex!("7c8b7f9fc8609141fdea9cece85249d308624391ff61dedaf528fcb337727dfd"),
        )
        .dependencies([glibc::package])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            gawk::package,
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
