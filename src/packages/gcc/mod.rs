use super::*;

pub fn package() -> Package {
    Package::new("gcc", "16.2.0")
        .http_src(
            "https://ftpmirror.gnu.org/gcc/gcc-16.2.0/gcc-16.2.0.tar.xz",
            hex!("e6738e29597f733270731aa90600f37ffdc045079dfc27ec7e8192cc81085c3e"),
        )
        .dependencies([
            bash::package(),
            binutils::package(),
            glibc::package(),
            mpc::package(),
        ])
        .make_dependencies([
            coreutils::package(),
            diffutils::package(),
            findutils::package(),
            gawk::package(),
            gcc::package(),
            gettext::package(),
            gmp::package(),
            grep::package(),
            m4::package(),
            make::package(),
            mpfr::package(),
            patch::package(),
            perl::package(),
            sed::package(),
            tar::package(),
            texinfo::package(),
            zstd::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
