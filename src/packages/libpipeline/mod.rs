use super::*;

pub fn package() -> Package {
    Package::new("libpipeline", "1.5.8")
        .http_src(
            "https://download.savannah.gnu.org/releases/libpipeline/libpipeline-1.5.8.tar.gz",
            hex!("1b1203ca152ccd63983c3f2112f7fe6fa5afd453218ede5153d1b31e11bb8405"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
