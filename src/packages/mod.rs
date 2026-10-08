use hex_literal::hex;
use http::Uri;

mod build;
mod check;

pub use build::Build;
pub use check::Check;

/// A package definition, used as a dependency edge.
///
/// Dependencies are stored as functions rather than eagerly constructed
/// [`Package`] values, so building a package does not recursively build its
/// whole dependency closure (which would not terminate, as the chapters 6 and
/// 7 bootstrap dependencies form cycles).
pub type PackageFn = fn() -> Package;

mod acl;
mod attr;
mod autoconf;
mod automake;
pub mod bash;
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
mod flit_core;
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
mod jinja2;
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
mod markupsafe;
mod meson;
mod mpc;
mod mpdecimal;
mod mpfr;
mod ncurses;
mod ninja;
mod openssl;
mod packaging;
mod patch;
mod pcre2;
mod perl;
mod pkgconf;
mod procps_ng;
mod psmisc;
mod python;
mod readline;
mod sed;
mod setuptools;
mod shadow;
mod sqlite;
mod systemd;
mod tar;
mod tcl;
mod texinfo;
mod util_linux;
mod vim;
mod wheel;
mod xz;
mod zlib;
mod zstd;

#[derive(Debug)]
pub enum Source {
    Http { url: Uri, hash_sha256: [u8; 32] },
}

#[derive(Debug)]
pub struct Package {
    name: String,
    version: String,
    sources: Vec<Source>,
    dependencies: Vec<PackageFn>,
    make_dependencies: Vec<PackageFn>,
    patches: Vec<&'static str>,
    bootstrap: bool,
    build: Option<Build>,
    check: Option<Check>,
}

impl Package {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            sources: Vec::new(),
            patches: Vec::new(),
            dependencies: Vec::new(),
            make_dependencies: Vec::new(),
            bootstrap: false,
            build: None,
            check: None,
        }
    }

    pub fn http_src(mut self, url: &str, hash_sha256: [u8; 32]) -> Self {
        self.sources.push(Source::Http {
            url: url
                .parse::<Uri>()
                .expect("Failed to parse package source URL"),
            hash_sha256,
        });
        self
    }

    pub fn dependencies<const N: usize>(mut self, iter: [PackageFn; N]) -> Self {
        self.dependencies.extend(iter);
        self
    }

    pub fn make_dependencies<const N: usize>(mut self, iter: [PackageFn; N]) -> Self {
        self.make_dependencies.extend(iter);
        self
    }

    pub fn patches<T: IntoIterator<Item = &'static str>>(mut self, iter: T) -> Self {
        self.patches.extend(iter);
        self
    }

    /// Marks the package as one built in chapters 6 and 7 of Linux from Scratch.
    ///
    /// The resolver can treat these as already available when ordering builds,
    /// instead of every dependency list having to decide how to reference
    /// them.
    pub fn bootstrap(mut self) -> Self {
        self.bootstrap = true;
        self
    }

    pub fn is_bootstrap(&self) -> bool {
        self.bootstrap
    }

    pub fn build(mut self, build: Build) -> Self {
        self.build = Some(build);
        self
    }

    pub fn check(mut self, check: Check) -> Self {
        self.check = Some(check);
        self
    }

    pub fn iter_sources(&self) -> impl Iterator<Item = &Source> {
        return self.sources.iter();
    }

    pub fn iter_build_dependencies(&self) -> impl Iterator<Item = PackageFn> + '_ {
        self.dependencies
            .iter()
            .chain(self.make_dependencies.iter())
            .copied()
    }
}
