use super::*;

pub fn package() -> Package {
    Package::new("man-pages", "6.18")
        .http_src(
            "https://www.kernel.org/pub/linux/docs/man-pages/man-pages-6.18.tar.xz",
            hex!("c934fadc8b59748c68227a34f6581d2ddf8282b73cdcd52546c8cd88b74b24d1"),
        )
        .make_dependencies([
            bash::package,
            coreutils::package,
            make::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
}
