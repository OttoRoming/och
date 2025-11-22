use std::{collections::HashMap, env, fs, io, path::PathBuf};

use och::{
    details::{self},
    packaging, source,
    terminal::log::*,
};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(
        "Failed to read OCHBUILD, hint: are you sure that you are in a directory with an OCHBUILD file?"
    )]
    FailedToReadOchBuild,
    #[error("failed to parse OCHBUILD: {0}")]
    Details(#[from] details::Error),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("source fetch error: {0}")]
    SourceFetch(#[from] source::FetchError),
    #[error("source check error: {0}")]
    SourceCheck(#[from] source::CheckError),
    #[error("source processing error: {0}")]
    SourceProcess(#[from] source::ProcesError),
    #[error("packaging error: {0}")]
    Packaging(#[from] packaging::Error),
}

fn makeoch() -> Result<(), Error> {
    unsafe {
        env::set_var("MAKEFLAGS", "-j8");
        env::set_var("NINJAJOBS", "8");
    }

    info("Parsing OCHBUILD");
    let root = env::current_dir()?;
    let mut ochbuild_path = root.clone();
    ochbuild_path.push("OCHBUILD");
    let contents = fs::read_to_string(&ochbuild_path).map_err(|_| Error::FailedToReadOchBuild)?;
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
    packaging::run_ochbuild(&ochbuild_path)?;

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
