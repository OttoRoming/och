use super::*;

pub fn package() -> Package {
    Package::new("bash", "5.3")
        .http_src(
            "https://ftpmirror.gnu.org/bash/bash-5.3.tar.gz",
            hex!("0d5cd86965f869a26cf64f4b71be7b96f90a3ba8b3d74e27e8e9d9d5550f31ba"),
        )
        .dependencies([
            glibc::package(),
            ncurses::package(),
            readline::package(),
        ])
        .make_dependencies([
            binutils::package(),
            bison::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            patch::package(),
            sed::package(),
            texinfo::package(),
        ])
        .self_dependant()
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
