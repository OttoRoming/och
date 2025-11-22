use std::{collections::HashMap, convert, env, error, fmt, fs, io, path::PathBuf};

use och::{
    details::{self},
    packaging, source,
    terminal::log::*,
};

#[derive(Debug)]
enum Error {
    FailedToReadOchBuild,
    Details(details::Error),
    Io(io::Error),
    SourceFetch(source::FetchError),
    SourceCheck(source::CheckError),
    SourceProcess(source::ProcesError),
    Packaging(packaging::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToReadOchBuild => {
                write!(
                    f,
                    "Failed to read OCHBUILD, hint: are you sure that you are in a directory with an OCHBUILD file?"
                )
            }
            Self::Details(err) => {
                write!(f, "failed to parse OCHBUILD: {}", err)
            }
            Self::Io(err) => {
                write!(f, "io error: {}", err)
            }
            Self::SourceFetch(err) => {
                write!(f, "source fetch error: {}", err)
            }
            Self::SourceCheck(err) => {
                write!(f, "source check error: {}", err)
            }
            Self::SourceProcess(err) => {
                write!(f, "source processing error: {}", err)
            }
            Self::Packaging(err) => {
                write!(f, "packaging error: {}", err)
            }
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl convert::From<details::Error> for Error {
    fn from(value: details::Error) -> Self {
        Error::Details(value)
    }
}

impl convert::From<source::FetchError> for Error {
    fn from(value: source::FetchError) -> Self {
        Error::SourceFetch(value)
    }
}

impl convert::From<source::CheckError> for Error {
    fn from(value: source::CheckError) -> Self {
        Error::SourceCheck(value)
    }
}

impl convert::From<source::ProcesError> for Error {
    fn from(value: source::ProcesError) -> Self {
        Error::SourceProcess(value)
    }
}

impl convert::From<packaging::Error> for Error {
    fn from(value: packaging::Error) -> Self {
        Error::Packaging(value)
    }
}

impl error::Error for Error {}

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
