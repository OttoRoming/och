use super::*;

pub fn package() -> Package {
    Package::new("libcap", "2.78")
        .http_src(
            "https://www.kernel.org/pub/linux/libs/security/linux-privs/libcap2/libcap-2.78.tar.xz",
            hex!("0d621e562fd932ccf67b9660fb018e468a683d7b827541df27813228c996bb11"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            attr::package(),
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            make::package(),
            perl::package(),
            sed::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
