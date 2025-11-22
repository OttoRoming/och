use regex::Regex;
use std::{
    convert, error, fmt,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process,
};

use crate::{details::Details, terminal::Progress};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TarFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::TarFailed => {
                write!(f, "Failed to archive package")
            }
            &Self::Io(err) => {
                write!(f, "io: {}", err)
            }
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl error::Error for Error {}

// FIXME: code duplication of src/source/process.rs
fn line_find_speed(line: &str) -> Option<&str> {
    let re = Regex::new(r"\s\(\d*\w*, (\d+\w+/s)\)$").unwrap();
    let find = re.captures(line)?;
    let result = find.get(0)?.as_str();

    Some(result)
}

pub fn archive_package(
    details: &Details,
    source_dir: &Path,
    destination_dir: &Path,
) -> Result<PathBuf, Error> {
    let mut package_archive_path = destination_dir.to_path_buf();
    package_archive_path.push(format!("{}-{}.tar.lz", details.name, details.version));

    // Create the tar.lz archive using the `tar` command
    let mut process = std::process::Command::new("tar")
        .arg("--owner=0")
        .arg("--group=0")
        .arg("--lzip")
        .arg("--force-local")
        .arg("--checkpoint")
        .arg("--checkpoint-action=totals")
        .arg("-cf")
        .arg(package_archive_path.to_str().unwrap())
        .arg("-C")
        .arg(source_dir)
        .arg(".")
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    let stderr = process.stderr.take().expect("stdout piped");
    let reader = io::BufReader::new(stderr);

    let mut progress = Progress::default();

    progress.start()?;
    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(speed) = line_find_speed(&line) {
            progress.add(0.1, speed)?;
        }
    }
    progress.finish()?;

    if process.wait()?.success() {
        Ok(package_archive_path)
    } else {
        Err(Error::TarFailed)
    }
}
