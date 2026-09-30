use super::*;

pub fn package() -> Package {
    Package::new("findutils", "4.11.0")
        .dependencies([
            bash::package(),
            glibc::package(),
        ])
        .make_dependencies([
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr")
                .var("localstatedir", "/var/lib/locate"),
        ))
}
