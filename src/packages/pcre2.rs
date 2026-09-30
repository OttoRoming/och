use super::*;

pub fn package() -> Package {
    Package::new("pcre2", "10.47")
        .http_src(
            "https://github.com/PCRE2Project/pcre2/releases/download/pcre2-10.47/pcre2-10.47.tar.bz2",
            hex!("47fe8c99461250d42f89e6e8fdaeba9da057855d06eb7fc08d9ca03fd08d7bc7"),
        )
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
