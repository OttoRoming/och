use super::*;

pub fn package() -> Package {
    Package::new("autoconf", "2.73")
        .http_src(
            "https://ftpmirror.gnu.org/autoconf/autoconf-2.73.tar.xz",
            hex!("9fd672b1c8425fac2fa67fa0477b990987268b90ff36d5f016dae57be0d6b52e"),
        )
        .dependencies([
            bash::package(),
            coreutils::package(),
            grep::package(),
            m4::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .make_dependencies([perl::package()])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
