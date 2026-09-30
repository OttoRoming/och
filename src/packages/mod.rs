use clap::builder::OsStr;

enum Method {
    Configure {
        configure: IntoIterator<(&str, &str)>,
    },
}

struct Package {
    method: Method,
}

impl Package {
    pub fn new() -> Package {}
}
