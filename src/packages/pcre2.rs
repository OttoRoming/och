use super::*;

pub fn package() -> Package {
    Package::new("pcre2", "10.47")
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            bzip2::package(),
            coreutils::package(),
            gcc::package(),
            gzip::package(),
            make::package(),
            readline::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("enable-unicode")
                .opt("enable-jit")
                .opt("enable-pcre2-16")
                .opt("enable-pcre2-32")
                .opt("enable-pcre2grep-libz")
                .opt("enable-pcre2grep-libbz2")
                .opt("enable-pcre2test-libreadline")
                .opt("disable-static")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/pcre2-10.47"),
        ))
        .check(Check::Make)
}
