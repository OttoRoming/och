use super::*;

pub fn package() -> Package {
    Package::new("less", "704")
        .http_src(
            "https://www.greenwoodsoftware.com/less/less-704.tar.gz",
            hex!("20a0b0a2bb2525fa53c7eee9beb854b4c9cf172eabb209af7020743547bfe9fb"),
        )
        .dependencies([
            glibc::package,
            ncurses::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gcc::package,
            grep::package,
            make::package,
            pcre2::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
