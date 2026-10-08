use super::*;

pub fn package() -> Package {
    Package::new("bzip2", "1.0.8")
        .http_src(
            "https://www.sourceware.org/pub/bzip2/bzip2-1.0.8.tar.gz",
            hex!("ab5a03176ee106d3f0fa90e381da478ddae405918153cca248e682cd0c4a2269"),
        )
        .http_src(
            "https://www.linuxfromscratch.org/patches/lfs/13.1/bzip2-1.0.8-install_docs-1.patch",
            hex!("35e3bbd9642af51fef2a8a83afba040d272da42d7e3a251d8e43255a7b496702"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            diffutils::package(),
            gcc::package(),
            make::package(),
            patch::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
}
