use super::*;

pub fn package() -> Package {
    Package::new("xz", "5.8.3")
        .http_src(
            "https://github.com//tukaani-project/xz/releases/download/v5.8.3/xz-5.8.3.tar.xz",
            hex!("fff1ffcf2b0da84d308a14de513a1aa23d4e9aa3464d17e64b9714bfdd0bbfb6"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            make::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/xz-5.8.3"),
        ))
        .check(Check::Make)
}
