use super::*;

pub fn package() -> Package {
    Package::new("vim", "9.2.1025")
        .http_src(
            "https://github.com/vim/vim/archive/v9.2.1025/vim-9.2.1025.tar.gz",
            hex!("cb0dbf701f8550bb54940304fa4d8b875e078db84967e5b36a0eb7ebe8a6d0cc"),
        )
        .dependencies([
            acl::package(),
            attr::package(),
            glibc::package(),
            ncurses::package(),
            tcl::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
