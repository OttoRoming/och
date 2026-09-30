use super::*;

pub fn package() -> Package {
    Package::new("patch", "2.8")
        .http_src(
            "https://ftpmirror.gnu.org/patch/patch-2.8.tar.xz",
            hex!("f87cee69eec2b4fcbf60a396b030ad6aa3415f192aa5f7ee84cad5e11f7f5ae3"),
        )
        .dependencies([
            attr::package(),
            glibc::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
            sed::package(),
        ])
        .build(Build::Configure(
            Configure::new()
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
