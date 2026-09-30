use super::*;

pub fn package() -> Package {
    Package::new("less", "704")
        .dependencies([
            glibc::package(),
            ncurses::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            pcre2::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("sysconfdir", "/etc"),
        ))
        .check(Check::Make)
}
