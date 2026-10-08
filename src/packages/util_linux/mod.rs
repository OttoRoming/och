use super::*;

pub fn package() -> Package {
    Package::new("util-linux", "2.42.2")
        .http_src(
            "https://www.kernel.org/pub/linux/utils/util-linux/v2.42/util-linux-2.42.2.tar.xz",
            hex!("03a05d3adf9602ef128f2da05b84b3205ce60c351e5737c0370f74000679ce8a"),
        )
        .dependencies([
            glibc::package(),
            ncurses::package(),
            readline::package(),
            systemd::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            file::package(),
            findutils::package(),
            gawk::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            make::package(),
            pkgconf::package(),
            sed::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
