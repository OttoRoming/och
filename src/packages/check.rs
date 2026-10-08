/// The shell script that checks a package.
///
/// The script is run from the root of the extracted source tree, after the
/// package has been configured and compiled. It is embedded at compile time
/// with `include_str!`, so recipes live next to the package that defines
/// them.
#[derive(Debug, Clone, Copy)]
pub struct Check {
    script: &'static str,
}

impl Check {
    pub const fn new(script: &'static str) -> Self {
        Self { script }
    }

    /// The shell script to execute.
    pub const fn script(&self) -> &'static str {
        self.script
    }
}
