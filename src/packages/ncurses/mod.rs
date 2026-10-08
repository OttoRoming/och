use super::*;

pub fn package() -> Package {
    Package::new("ncurses", "6.6")
        .bootstrap()
        .http_src(
            "https://invisible-mirror.net/archives/ncurses/ncurses-6.6.tar.gz",
            hex!("355b4cbbed880b0381a04c46617b7656e362585d52e9cf84a67e2009b749ff11"),
        )
        .dependencies([glibc::package])
        .make_dependencies([
            bash::package,
            binutils::package,
            coreutils::package,
            diffutils::package,
            gawk::package,
            gcc::package,
            grep::package,
            make::package,
            patch::package,
            sed::package,
        ])
        .build(Build::Script(include_str!("build.sh")))
}
