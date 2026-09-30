use super::*;

pub fn package() -> Package {
    Package::new("findutils", "4.11.0")
        .http_src(
            "https://ftpmirror.gnu.org/findutils/findutils-4.11.0.tar.xz",
            hex!("bfd19cb06cc71f3352d567e90284d8cdac02ac89774bbeadf0b533b0c11432fd"),
        )
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
