use super::*;

pub fn package() -> Package {
    Package::new("python", "3.14.7")
        .bootstrap()
        .dependencies([
            bzip2::package,
            expat::package,
            gdbm::package,
            glibc::package,
            libffi::package,
            libxcrypt::package,
            mpdecimal::package,
            ncurses::package,
            openssl::package,
            zlib::package,
        ])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            gcc::package,
            gettext::package,
            grep::package,
            make::package,
            pkgconf::package,
            sed::package,
            util_linux::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
