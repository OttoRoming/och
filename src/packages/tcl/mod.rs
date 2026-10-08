use super::*;

pub fn package() -> Package {
    Package::new("tcl", "8.6.18")
        .http_src(
            "https://downloads.sourceforge.net/tcl/tcl8.6.18-src.tar.gz",
            hex!("14f9af32b1767ff718477a8f974ad03c34341097e6b43f4ce54644ee974e268e"),
        )
        .http_src(
            "https://downloads.sourceforge.net/tcl/tcl8.6.18-html.tar.gz",
            hex!("bee2d83a19ce50624eeae951be29fe5f654b171357a6391123caf18107cec543"),
        )
        .dependencies([
            glibc::package,
            zlib::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gcc::package,
            grep::package,
            make::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
