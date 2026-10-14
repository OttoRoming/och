//! Builds one package, or every package.

use clap::Parser;
use och::packages::{self, Package};
use och::terminal::log;
use std::process::ExitCode;

/// Builds packages with the recipes in `packages/`.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The package to build.
    #[arg(required_unless_present = "all", conflicts_with = "all")]
    package: Option<String>,

    /// Build every package.
    #[arg(long)]
    all: bool,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            log::err(&error.to_string());
            ExitCode::FAILURE
        }
    }
}

/// Builds what the command line asked for.
async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let packages = packages::Packages::load()?;

    match &args.package {
        Some(name) => {
            let package = packages
                .get(name)
                .ok_or_else(|| format!("no package named `{name}`"))?;

            build(packages::builder::Builder::new(package).build().await, name)
        }
        None => build_all(&packages).await,
    }
}

/// Builds every package, in name order so runs are reproducible.
///
/// The build order is currently just alphabetical; dependencies are not
/// resolved yet, so a package can be built before the packages it needs.
async fn build_all(packages: &packages::Packages) -> Result<(), Box<dyn std::error::Error>> {
    let mut all: Vec<&Package> = packages.iter().collect();
    all.sort_by_key(|package| package.name());

    for package in all {
        build(
            packages::builder::Builder::new(package).build().await,
            package.name(),
        )?;
    }

    Ok(())
}

/// Reports a build result against the package it belongs to.
fn build(
    result: Result<(), packages::builder::Error>,
    package: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    result.map_err(|error| format!("failed to build `{package}`: {error}").into())
}
