use std::{
    convert, error, fmt,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process,
};

use crate::{details::Details, tar_utils, terminal::Progress};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TarFailed,
    OchBuildFailed(Option<i32>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::TarFailed => {
                write!(f, "Failed to archive package")
            }
            &Self::OchBuildFailed(code_option) => {
                let code = match code_option {
                    Some(v) => format!("{}", v),
                    None => "unknown".to_string(),
                };
                write!(f, "OCHBUILD exited with statuscode {}", code)
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
        if let Some(speed) = tar_utils::line_find_speed(&line) {
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

/// Executes a OCHBUILD script
pub fn run_ochbuild(ochbuild: &Path) -> Result<(), Error> {
    let ochbuild_status = process::Command::new("bash")
        .arg("-e")
        .arg(ochbuild.to_str().unwrap())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if ochbuild_status.success() {
        Ok(())
    } else {
        Err(Error::OchBuildFailed(ochbuild_status.code()))
    }
}
