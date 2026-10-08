use std::{collections::HashMap, env, fs, io, path::PathBuf};

use och::{packages, packaging, source, terminal::log::*};

fn main() {
    info("Starting och packaging process...");
    packaging::package(packages::bash::package());
}
