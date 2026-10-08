use super::*;

pub fn package() -> Package {
    Package::new("bc", "7.0.3")
        .http_src(
            "https://github.com/gavinhoward/bc/releases/download/7.0.3/bc-7.0.3.tar.xz",
            hex!("91eb74caed0ee6655b669711a4f350c25579778694df248e28363318e03c7fc4"),
        )
        .dependencies([
            glibc::package(),
            ncurses::package(),
            readline::package(),
        ])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            coreutils::package(),
            gcc::package(),
            grep::package(),
            make::package(),
        ])
        .build(Build::new(include_str!("build.sh")))
        .check(Check::new(include_str!("check.sh")))
}
