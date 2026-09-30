use super::*;

pub fn package() -> Package {
    Package::new("bison", "3.8.2")
        .http_src(
            "https://ftpmirror.gnu.org/bison/bison-3.8.2.tar.xz",
            hex!("9bba0214ccf7f1079c5d59210045227bcf619519840ebfa80cd3849cff5a5bf2"),
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
            m4::package(),
            make::package(),
            perl::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/bison-3.8.2"),
        ))
        .check(Check::Make)
}
