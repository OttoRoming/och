use super::*;

pub fn package() -> Package {
    Package::new("perl", "5.44.0")
        .http_src(
            "https://www.cpan.org/src/5.0/perl-5.44.0.tar.xz",
            hex!("505cf43912e9480495c344c70260452e32aa2a73c546a026b3f100053b23ce91"),
        )
        .dependencies([
            gdbm::package(),
            glibc::package(),
            libxcrypt::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
            zlib::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
