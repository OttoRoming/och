use super::*;

pub fn package() -> Package {
    Package::new("sqlite", "3530400")
        .http_src(
            "https://sqlite.org/2026/sqlite-autoconf-3530400.tar.gz",
            hex!("0e9483900e92cd5de8fd48d16bf9200145a61f7fd5be542a5ac81d8a9516eb9c"),
        )
        .http_src(
            "https://sqlite.org/2026/sqlite-doc-3530400.zip",
            hex!("a1d0f5de57485d062796ed7e67daff0758b50d00001a0f233a2c15aaf40bbdc8"),
        )
        .dependencies([glibc::package()])
        .make_dependencies([
            bash::package(),
            binutils::package(),
            gcc::package(),
            gzip::package(),
            make::package(),
            ncurses::package(),
            readline::package(),
            zlib::package(),
        ])
        .build(Build::Script(include_str!("build.sh")))
}
