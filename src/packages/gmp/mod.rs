use super::*;

pub fn package() -> Package {
    Package::new("gmp", "6.3.0")
        .http_src(
            "https://ftpmirror.gnu.org/gmp/gmp-6.3.0.tar.xz",
            hex!("a3c2b80201b89e68616f4ad30bc66aee4927c3ce50e33929ca819d5c43538898"),
        )
        .dependencies([
            gcc::package(),
            glibc::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            grep::package(),
            m4::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
