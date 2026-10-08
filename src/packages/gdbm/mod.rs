use super::*;

pub fn package() -> Package {
    Package::new("gdbm", "1.26")
        .http_src(
            "https://ftpmirror.gnu.org/gdbm/gdbm-1.26.tar.gz",
            hex!("6a24504a14de4a744103dcb936be976df6fbe88ccff26065e54c1c47946f4a5e"),
        )
        .dependencies([
            bash::package(),
            glibc::package(),
            readline::package(),
        ])
        .make_dependencies([
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
