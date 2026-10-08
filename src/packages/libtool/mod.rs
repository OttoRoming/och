use super::*;

pub fn package() -> Package {
    Package::new("libtool", "2.6.2")
        .http_src(
            "https://ftpmirror.gnu.org/libtool/libtool-2.6.2.tar.xz",
            hex!("2ef1067c16c97db930fd740cc9bc3d3ba9a583804ae5ac42cc3e8719e49e191e"),
        )
        .dependencies([
            autoconf::package(),
            automake::package(),
            bash::package(),
            binutils::package(),
            coreutils::package(),
            file::package(),
            gcc::package(),
            glibc::package(),
            grep::package(),
            make::package(),
            sed::package(),
        ])
        .make_dependencies([
            diffutils::package(),
            gawk::package(),
            texinfo::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
