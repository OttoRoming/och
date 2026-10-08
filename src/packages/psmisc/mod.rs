use super::*;

pub fn package() -> Package {
    Package::new("psmisc", "23.7")
        .http_src(
            "https://sourceforge.net/projects/psmisc/files/psmisc/psmisc-23.7.tar.xz",
            hex!("58c55d9c1402474065adae669511c191de374b0871eec781239ab400b907c327"),
        )
        .dependencies([
            glibc::package(),
            ncurses::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
