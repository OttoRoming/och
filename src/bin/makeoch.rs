// use std::{collections::HashMap, env, fs, io, path::PathBuf};
//
// use och::{packages, packaging, source, terminal::log::*};

use och::packages;

#[tokio::main]
async fn main() {
    // info("Starting och packaging process...");
    // packaging::package(packages::bash::package());
    let packages = packages::Packages::load().unwrap();
    let package = packages.iter().nth(0).unwrap();
    let builder = packages::builder::Builder::new(package);
    builder.build().await;
}
