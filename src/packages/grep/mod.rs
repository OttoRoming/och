use super::*;

pub fn package() -> Package {
    Package::new("grep", "3.12")
        .http_src(
            "https://ftpmirror.gnu.org/grep/grep-3.12.tar.xz",
            hex!("2649b27c0e90e632eadcd757be06c6e9a4f48d941de51e7c0f83ff76408a07b9"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            patch::package(),
            pcre2::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
