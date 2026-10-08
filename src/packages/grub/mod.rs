use super::*;

pub fn package() -> Package {
    Package::new("grub", "2.14")
        .http_src(
            "https://ftpmirror.gnu.org/grub/grub-2.14.tar.xz",
            hex!("bc8d3c73535b8838d8c8e2654d73edc4e6ae8c8acdb45d5df5dc9a1547446d43"),
        )
        .dependencies([
            bash::package(),
            gcc::package(),
            gettext::package(),
            glibc::package(),
            sed::package(),
            xz::package(),
        ])
        .make_dependencies([
            binutils::package(),
            bison::package(),
            coreutils::package(),
            diffutils::package(),
            grep::package(),
            make::package(),
            ncurses::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
