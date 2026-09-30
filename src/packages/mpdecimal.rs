use super::*;

pub fn package() -> Package {
    Package::new("mpdecimal", "4.0.1")
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
            make::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/mpdecimal-4.0.1"),
        ))
}
