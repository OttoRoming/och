use super::*;

pub fn package() -> Package {
    Package::new("gdbm", "1.26")
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
        .build(Build::Configure(
            Configure::new()
                .opt("disable-static")
                .opt("enable-libgdbm-compat")
                .var("prefix", "/usr"),
        ))
        .check(Check::Make)
}
