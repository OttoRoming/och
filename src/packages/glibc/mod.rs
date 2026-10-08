use super::*;

pub fn package() -> Package {
    Package::new("glibc", "2.44")
        .http_src(
            "https://ftpmirror.gnu.org/glibc/glibc-2.44.tar.xz",
            hex!("37f600f2bef3c5e8300147059568b2a2e40a7ad6ccc65ce942556d49429cc667"),
        )
        .http_src(
            "https://www.iana.org/time-zones/repository/releases/tzdata2026c.tar.gz",
            hex!("e4a178a4477f3d0ea77cc31828ff72aa38feff8d61aa13e7e99e142e9d902be4"),
        )
        .http_src(
            "https://www.linuxfromscratch.org/patches/lfs/13.1/glibc-2.44-upstream_fixes-1.patch",
            hex!("500567ff0a22e295bfe3b55e9b397796f71a19f01e1ff7fb29c9189482ee1809"),
        )
        .http_src(
            "https://www.linuxfromscratch.org/patches/lfs/13.1/glibc-fhs-1.patch",
            hex!("643552db030e2f2d7ffde4f558e0f5f83d3fabf34a2e0e56ebdb49750ac27b0d"),
        )
        .make_dependencies([
            bash::package(),
            binutils::package(),
            bison::package(),
            coreutils::package(),
            diffutils::package(),
            gawk::package(),
            gcc::package(),
            gettext::package(),
            grep::package(),
            gzip::package(),
            make::package(),
            perl::package(),
            sed::package(),
            texinfo::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
