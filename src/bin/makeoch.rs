// use std::{collections::HashMap, env, fs, io, path::PathBuf};
//
// use och::{packages, packaging, source, terminal::log::*};

use och::packages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // info("Starting och packaging process...");
    // packaging::package(packages::bash::package());
    let packages = packages::Packages::load().unwrap();
    let package = packages.iter().find(|p| p.name() == "autoconf").unwrap();

    dbg!("run");
    let builder = packages::builder::Builder::new(package);
    builder.build().await?;

    Ok(())
}
