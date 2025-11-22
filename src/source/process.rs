use regex::Regex;
use std::{
    convert, error, fmt, fs,
    io::{self, BufRead},
    path::Path,
    process,
};

use crate::{tar_utils, terminal::Progress};

#[derive(Debug)]
pub enum Error {
    FailedExtraction,
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::FailedExtraction => {
                write!(f, "Failed to extract archive")
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

pub fn extract_tar(path: &Path, destination: &Path) -> Result<(), Error> {
    let total_bytes = fs::metadata(path)?.len();
    let mut process = process::Command::new("tar")
        .arg("--force-local")
        .arg("--checkpoint")
        .arg("--checkpoint-action=totals")
        .arg("-C")
        .arg(destination)
        .arg("-xf")
        .arg(path)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    let stderr = process.stderr.take().expect("stdout piped");
    let reader = io::BufReader::new(stderr);

    let mut progress = Progress::default();

    progress.start()?;
    for line_result in reader.lines() {
        let line = line_result?;
        let speed = tar_utils::line_find_speed(&line).unwrap_or("");
        let total_bytes_read = tar_utils::line_find_bytes_processed(&line);

        if let Some(total_bytes_read) = total_bytes_read {
            progress.update(total_bytes_read as f64 / total_bytes as f64, speed)?;
        }
    }
    progress.finish()?;

    if process.wait()?.success() {
        Ok(())
    } else {
        Err(Error::FailedExtraction)
    }
}
