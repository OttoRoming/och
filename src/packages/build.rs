/// The shell script that builds and installs a package.
///
/// The script is run from the root of the extracted source tree with the
/// staging directory exported as `DESTDIR`, and is expected to configure,
/// compile and install the package. It is embedded at compile time with
/// `include_str!`, so recipes live next to the package that defines them.
#[derive(Debug, Clone, Copy)]
pub struct Build {
    script: &'static str,
}

impl Build {
    pub const fn new(script: &'static str) -> Self {
        Self { script }
    }

    /// The shell script to execute.
    pub const fn script(&self) -> &'static str {
        self.script
    }
}
