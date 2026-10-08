use super::*;

pub fn package() -> Package {
    Package::new("shadow", "4.20.2")
        .http_src(
            "https://github.com/shadow-maint/shadow/releases/download/4.20.2/shadow-4.20.2.tar.xz",
            hex!("b89f432b75ae55a2d852b446d0365484795fdf533ab97d550e325fa15594a224"),
        )
        .dependencies([
            glibc::package(),
            libxcrypt::package(),
        ])
        .make_dependencies([
            acl::package(),
            attr::package(),
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            findutils::package(),
            gawk::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            libcap::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
}
