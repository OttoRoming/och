use super::*;

pub fn package() -> Package {
    Package::new("kbd", "2.10.0")
        .http_src(
            "https://www.kernel.org/pub/linux/utils/kbd/kbd-2.10.0.tar.xz",
            hex!("6e5ca4f8d76ee9e3a8db700b667f13e12aac9933828a64e1aaad93d26be9b479"),
        )
        .http_src(
            "https://www.linuxfromscratch.org/patches/lfs/13.1/kbd-2.10.0-backspace-1.patch",
            hex!("8be28dcb11420624a500f2ea4fe975f771174bffee50e54ec8cd295a2dec104e"),
        )
        .dependencies([
            bash::package(),
            coreutils::package(),
            glibc::package(),
        ])
        .make_dependencies([
            binutils::package(),
            bison::package(),
            flex::package(),
            gcc::package(),
            gettext::package(),
            gzip::package(),
            make::package(),
            patch::package(),
            sed::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
