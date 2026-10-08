/// The shell script that builds and installs a package.
///
/// The script is run from the root of the extracted source tree with the
/// staging directory exported as `DESTDIR`, and is expected to configure,
/// compile and install the package. It is embedded at compile time with
/// `include_str!`, so recipes live next to the package that defines them.
#[derive(Debug, Clone, Copy)]
pub enum Build {
    Python(&'static str),
    Script(&'static str),
}
