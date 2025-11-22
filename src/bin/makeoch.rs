use std::{collections::HashMap, env, fs, path::PathBuf, process};

use anyhow::{Result, bail};
use och::{
    details::{self},
    packaging,
    terminal::log::*,
};

fn makeoch() -> Result<()> {
    unsafe {
        env::set_var("MAKEFLAGS", "-j8");
        env::set_var("NINJAJOBS", "8");
    }

    info("Parsing OCHBUILD");
    let root = env::current_dir()?;
    let mut ochbuild_path = root.clone();
    ochbuild_path.push("OCHBUILD");
    let contents = fs::read_to_string(&ochbuild_path).expect("Failed to read OCHBUILD file");
    let details = details::parse(contents)?;

    info(&format!(
        "Found package \"{}\" version {}",
        details.name, details.version,
    ));
    let work_path = PathBuf::from("work");
    if fs::metadata(&work_path).is_ok() {
        info("Clearing work directory");
        fs::remove_dir_all(&work_path)?;
    }

    fs::create_dir(&work_path)?;
    env::set_current_dir(&work_path)?;
    let work_path = env::current_dir()?;
    let mut destdir_dir = work_path.clone();
    destdir_dir.push("dest");
    fs::create_dir(&destdir_dir)?;
    unsafe {
        env::set_var("DESTDIR", destdir_dir.to_str().unwrap());
    }

    let mut source_paths = HashMap::new();
    info("Fetching sources");
    for source in details.sources.iter() {
        let path = source.fetch()?;
        source_paths.insert(source, path);
    }

    info("Checking sources");
    for source in details.sources.iter() {
        source.check(source_paths.get(source).unwrap())?;
    }

    info("Processing sources");
    for source in details.sources.iter() {
        source.process(source_paths.get(source).unwrap(), &work_path)?;
    }

    info("Running OCHBUILD");
    let ochbuild_status = process::Command::new("bash")
        .arg("-e")
        .arg(ochbuild_path.to_str().unwrap())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !ochbuild_status.success() {
        bail!("OCHBUILD Exited with statuscode {}", ochbuild_status)
    }

    info("Creating package archvie");
    packaging::archive_package(&details, &destdir_dir, &root)?;

    Ok(())
}

fn main() {
    match makeoch() {
        Err(e) => err(&format!("{}", e)),
        _ => {}
    };
}
