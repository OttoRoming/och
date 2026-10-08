use super::*;

pub fn package() -> Package {
    Package::new("attr", "2.6.0")
        .http_src(
            "https://download.savannah.gnu.org/releases/attr/attr-2.6.0.tar.gz",
            hex!("d42fa374513180bb48cb11a46696f488240e5124ff1e6ad88b0abff706985612"),
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
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
