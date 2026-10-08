use super::*;

pub fn package() -> Package {
    Package::new("readline", "8.3")
        .http_src(
            "https://ftpmirror.gnu.org/readline/readline-8.3.tar.gz",
            hex!("fe5383204467828cd495ee8d1d3c037a7eba1389c22bc6a041f627976f9061cc"),
        )
        .dependencies([
            glibc::package(),
            ncurses::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gawk::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            patch::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
