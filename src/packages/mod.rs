use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use url::Url;

use serde::{Deserialize, Deserializer};

mod build;
pub mod builder;
mod check;

pub use build::Build;
pub use check::Check;

/// Where the package definitions live, relative to the root of the project.
pub const PACKAGE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/packages");

/// The TOML file that every package directory contains.
const PACKAGE_FILE: &str = "package.toml";

/// Errors that can occur while loading package definitions.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read the package directory `{path}`: {source}")]
    ReadDir {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read `{path}`: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse `{path}`: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("`{0}` is defined by more than one package directory")]
    Duplicate(String),
    #[error("`{package}` depends on `{dependency}`, which no package defines")]
    UnknownDependency { package: String, dependency: String },
}

/// Something that is downloaded before a package is built.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    Http {
        #[serde(deserialize_with = "deserialize_url")]
        url: Url,
        /// Hex-encoded SHA256 of the downloaded file.
        #[serde(rename = "sha256", deserialize_with = "deserialize_sha256")]
        hash_sha256: [u8; 32],
    },
}

/// A package definition, loaded from `packages/<name>/package.toml`.
///
/// Dependencies are stored as package names rather than eagerly constructed
/// [`Package`] values, because the dependency graph contains cycles (a package
/// can be needed to build itself, and the chapters 6 and 7 bootstrap packages
/// depend on each other). [`Packages`] resolves the names on demand.
#[derive(Debug, Clone)]
pub struct Package {
    name: String,
    version: String,
    bootstrap: bool,
    sources: Vec<Source>,
    dependencies: Vec<String>,
    make_dependencies: Vec<String>,
    patches: Vec<String>,
    build: Option<Build>,
    check: Option<Check>,
}

impl Package {
    /// Reads a package definition and the scripts it refers to.
    fn load(definition: &Path) -> Result<Self, Error> {
        let dir = definition.parent().unwrap_or_else(|| Path::new("."));

        let contents = fs::read_to_string(definition).map_err(|source| Error::Read {
            path: definition.to_path_buf(),
            source,
        })?;

        let raw: RawPackage = toml::from_str(&contents).map_err(|source| Error::Parse {
            path: definition.to_path_buf(),
            source,
        })?;

        let build = match raw.build {
            Some(RawBuild::Script { script }) => Some(Build::Script(read_script(dir, &script)?)),
            Some(RawBuild::Python { module }) => Some(Build::Python(module)),
            None => None,
        };

        let check = raw
            .check
            .map(|check| read_script(dir, &check.script).map(Check::new))
            .transpose()?;

        Ok(Self {
            name: raw.name,
            version: raw.version,
            bootstrap: raw.bootstrap,
            sources: raw.sources,
            dependencies: raw.dependencies,
            make_dependencies: raw.make_dependencies,
            patches: raw.patches,
            build,
            check,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    /// Whether this is one of the packages built in chapters 6 and 7.
    ///
    /// The resolver can treat these as already available when ordering builds,
    /// instead of every dependency list having to decide how to reference them.
    pub fn is_bootstrap(&self) -> bool {
        self.bootstrap
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn make_dependencies(&self) -> &[String] {
        &self.make_dependencies
    }

    pub fn patches(&self) -> &[String] {
        &self.patches
    }

    pub fn build(&self) -> Option<&Build> {
        self.build.as_ref()
    }

    pub fn check(&self) -> Option<&Check> {
        self.check.as_ref()
    }

    pub fn iter_sources(&self) -> impl Iterator<Item = &Source> {
        self.sources.iter()
    }
}

/// Every package definition, keyed by name.
#[derive(Debug, Default)]
pub struct Packages {
    packages: HashMap<String, Package>,
}

impl Packages {
    /// Loads every package defined in [`PACKAGE_DIR`].
    pub fn load() -> Result<Self, Error> {
        Self::load_from(PACKAGE_DIR)
    }

    /// Loads every `<dir>/<package>/package.toml`.
    pub fn load_from(dir: impl AsRef<Path>) -> Result<Self, Error> {
        let dir = dir.as_ref();

        let entries = fs::read_dir(dir).map_err(|source| Error::ReadDir {
            path: dir.to_path_buf(),
            source,
        })?;

        let mut packages = HashMap::new();
        for entry in entries {
            let entry = entry.map_err(|source| Error::ReadDir {
                path: dir.to_path_buf(),
                source,
            })?;

            let definition = entry.path().join(PACKAGE_FILE);
            if !definition.is_file() {
                continue;
            }

            let package = Package::load(&definition)?;
            let name = package.name.clone();

            if packages.insert(name.clone(), package).is_some() {
                return Err(Error::Duplicate(name));
            }
        }

        let packages = Self { packages };
        packages.check_dependencies()?;

        Ok(packages)
    }

    /// Fails if a dependency list names a package that does not exist.
    fn check_dependencies(&self) -> Result<(), Error> {
        for package in self.packages.values() {
            for dependency in package
                .dependencies
                .iter()
                .chain(&package.make_dependencies)
            {
                if !self.packages.contains_key(dependency) {
                    return Err(Error::UnknownDependency {
                        package: package.name.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Package> {
        self.packages.values()
    }

    /// The packages `package` needs at runtime.
    pub fn dependencies<'a>(&'a self, package: &'a Package) -> impl Iterator<Item = &'a Package> {
        resolve(&self.packages, &package.dependencies)
    }

    /// The packages `package` needs to build.
    pub fn make_dependencies<'a>(
        &'a self,
        package: &'a Package,
    ) -> impl Iterator<Item = &'a Package> {
        resolve(&self.packages, &package.make_dependencies)
    }
}

fn resolve<'a>(
    packages: &'a HashMap<String, Package>,
    names: &'a [String],
) -> impl Iterator<Item = &'a Package> {
    names.iter().filter_map(|name| packages.get(name))
}

/// Reads a script named by a package definition, relative to its directory.
fn read_script(dir: &Path, script: &Path) -> Result<String, Error> {
    let path = dir.join(script);

    fs::read_to_string(&path).map_err(|source| Error::Read { path, source })
}

/// The package definition as written in TOML, before scripts are read.
#[derive(Deserialize)]
struct RawPackage {
    name: String,
    version: String,
    #[serde(default)]
    bootstrap: bool,
    #[serde(default)]
    sources: Vec<Source>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    make_dependencies: Vec<String>,
    #[serde(default)]
    patches: Vec<String>,
    #[serde(default)]
    build: Option<RawBuild>,
    #[serde(default)]
    check: Option<RawCheck>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum RawBuild {
    Script { script: PathBuf },
    Python { module: String },
}

#[derive(Deserialize)]
struct RawCheck {
    script: PathBuf,
}

fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let url = String::deserialize(deserializer)?;

    url.parse().map_err(serde::de::Error::custom)
}

fn deserialize_sha256<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    let hex = String::deserialize(deserializer)?;
    let digits = hex.as_bytes();

    if digits.len() != 64 {
        return Err(serde::de::Error::custom(format!(
            "expected 64 hexadecimal characters, found {}",
            digits.len()
        )));
    }

    let mut hash = [0u8; 32];
    for (index, byte) in hash.iter_mut().enumerate() {
        let pair = std::str::from_utf8(&digits[index * 2..index * 2 + 2])
            .map_err(serde::de::Error::custom)?;

        *byte = u8::from_str_radix(pair, 16).map_err(serde::de::Error::custom)?;
    }

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every definition must parse, name dependencies that exist, and refer to
    /// scripts that are present on disk.
    #[test]
    fn every_package_loads() {
        let packages = Packages::load().expect("package definitions should load");

        let packages: Vec<_> = packages.iter().collect();
        assert!(!packages.is_empty());

        for package in packages {
            assert!(
                package.build().is_some(),
                "{} has no build recipe",
                package.name()
            );
        }
    }
}
