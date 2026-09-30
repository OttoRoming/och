use super::*;

pub fn package() -> Package {
    Package::new("gperf", "3.3")
        .http_src(
            "https://ftpmirror.gnu.org/gperf/gperf-3.3.tar.gz",
            hex!("fd87e0aba7e43ae054837afd6cd4db03a3f2693deb3619085e6ed9d8d9604ad8"),
        )
        .dependencies([
            gcc::package(),
            glibc::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            make::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/gperf-3.3"),
        ))
        .check(Check::Make)
}
