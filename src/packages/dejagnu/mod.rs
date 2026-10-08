use super::*;

pub fn package() -> Package {
    Package::new("dejagnu", "1.6.3")
        .http_src(
            "https://ftpmirror.gnu.org/dejagnu/dejagnu-1.6.3.tar.gz",
            hex!("87daefacd7958b4a69f88c6856dbd1634261963c414079d0c371f589cd66a2e3"),
        )
        .dependencies([
            bash::package(),
            expect::package(),
        ])
        .make_dependencies([
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
