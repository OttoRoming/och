use hex_literal::hex;
use url::Url;

mod build;
mod check;

pub use build::Build;
pub use check::Check;

mod acl;
mod attr;
mod autoconf;
mod automake;
mod bash;
mod bc;
mod binutils;
mod bison;
mod bzip2;
mod coreutils;
mod dbus;
mod dejagnu;
mod diffutils;
mod e2fsprogs;
mod expat;
mod expect;
mod file;
mod findutils;
mod flex;
mod gawk;
mod gcc;
mod gdbm;
mod gettext;
mod glibc;
mod gmp;
mod gperf;
mod grep;
mod groff;
mod grub;
mod gzip;
mod iana_etc;
mod inetutils;
mod iproute2;
mod kbd;
mod kmod;
mod less;
mod libcap;
mod libelf;
mod libffi;
mod libpipeline;
mod libtool;
mod libxcrypt;
mod lz4;
mod m4;
mod make;
mod man_db;
mod man_pages;
mod mpc;
mod mpdecimal;
mod mpfr;
mod ncurses;
mod ninja;
mod openssl;
mod patch;
mod pcre2;
mod perl;
mod pkgconf;
mod procps_ng;
mod psmisc;
mod readline;
mod sed;
mod shadow;
mod sqlite;
mod systemd;
mod tar;
mod tcl;
mod texinfo;
mod util_linux;
mod vim;
mod xz;
mod zlib;
mod zstd;

enum Source {
    Http { url: Url, hash_sha256: [u8; 32] },
}

struct Package {
    name: String,
    version: String,
    sources: Vec<Source>,
    dependencies: Box<Vec<Package>>,
    make_dependencies: Box<Vec<Package>>,
    self_dependent: bool,
    build: Option<Build>,
    check: Option<Check>,
}

impl Package {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            sources: Vec::new(),
            dependencies: Box::new(Vec::new()),
            make_dependencies: Box::new(Vec::new()),
            self_dependent: false,
            build: None,
            check: None,
        }
    }

    pub fn http_src(mut self, url: &str, hash_sha256: [u8; 32]) -> Self {
        self.sources.push(Source::Http {
            url: Url::parse(url).unwrap(),
            hash_sha256,
        });
        self
    }

    pub fn dependencies<T: IntoIterator<Item = Package>>(mut self, iter: T) -> Self {
        self.dependencies.extend(iter);
        self
    }

    pub fn make_dependencies<T: IntoIterator<Item = Package>>(mut self, iter: T) -> Self {
        self.make_dependencies.extend(iter);
        self
    }

    pub fn self_dependant(mut self) -> Self {
        self.self_dependent = true;
        self
    }

    pub fn build(mut self, build: Build) -> Self {
        self.build = Some(build);
        self
    }

    pub fn check(mut self, check: Check) -> Self {
        self.check = Some(check);
        self
    }
}
