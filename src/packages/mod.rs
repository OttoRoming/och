use hex_literal::hex;
use std::collections::HashMap;
use std::ffi::OsString;
use url::Url;
mod acl;
mod attr;
mod autoconf;
mod automake;
mod bash;
mod bison;
mod diffutils;
mod expat;
mod file;
mod findutils;
mod gdbm;
mod gperf;
mod gzip;
mod less;
mod libffi;
mod libpipeline;
mod m4;
mod make;
mod man_db;
mod mpdecimal;
mod patch;
mod pcre2;
mod procps_ng;
mod psmisc;
mod texinfo;
mod util_linux;
mod xz;

struct Configure {
    options: HashMap<OsString, Option<OsString>>,
}

impl Configure {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
        }
    }

    pub fn opt(mut self, option: &str) -> Self {
        self.options.insert(OsString::from(option), None);
        self
    }

    pub fn var(mut self, var: &str, value: &str) -> Self {
        self.options
            .insert(OsString::from(var), Some(OsString::from(value)));
        self
    }
}

enum Build {
    Configure(Configure),
}

enum Check {
    Make, // only for packages with simple `make check`
}

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
