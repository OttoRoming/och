use super::*;

pub fn package() -> Package {
    Package::new("mpdecimal", "4.0.1")
        .bootstrap()
        .http_src(
            "https://www.bytereef.org/software/mpdecimal/releases/mpdecimal-4.0.1.tar.gz",
            hex!("96d33abb4bb0070c7be0fed4246cd38416188325f820468214471938545b1ac8"),
        )
        .dependencies([
            gcc::package,
            glibc::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gawk::package,
            grep::package,
            make::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
