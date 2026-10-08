/// How a package is built and installed.
///
/// A build script is run from the root of the extracted source tree with the
/// staging directory exported as `DESTDIR`, and is expected to configure,
/// compile and install the package. Python modules are installed with `pip3`
/// instead, and are named by their import name.
#[derive(Debug, Clone)]
pub enum Build {
    Script(String),
    Python(String),
}

impl Build {
    /// The shell script to execute, for [`Build::Script`].
    pub fn script(&self) -> Option<&str> {
        match self {
            Self::Script(script) => Some(script),
            Self::Python(_) => None,
        }
    }

    /// The Python module to install, for [`Build::Python`].
    pub fn python(&self) -> Option<&str> {
        match self {
            Self::Python(module) => Some(module),
            Self::Script(_) => None,
        }
    }
}
