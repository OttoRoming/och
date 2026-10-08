/// The shell script that checks a package.
///
/// It is run from the root of the extracted source tree, after the package has
/// been configured and compiled. Recipes live next to the package definition
/// that refers to them.
#[derive(Debug, Clone)]
pub struct Check {
    script: String,
}

impl Check {
    pub fn new(script: impl Into<String>) -> Self {
        Self {
            script: script.into(),
        }
    }

    /// The shell script to execute.
    pub fn script(&self) -> &str {
        &self.script
    }
}
