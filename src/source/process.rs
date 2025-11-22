use std::{
    fs,
    io::{self, BufRead},
    path::Path,
    process,
};

use crate::{tar_utils, terminal::Progress};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("failed to extract archive")]
    FailedExtraction,
    #[error("failed to checkout git commit")]
    FailedGitCheckout,
}

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

pub fn checkout_git_commit(path: &Path, commit_hash: &str) -> Result<(), Error> {
    let status = process::Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("checkout")
        .arg(commit_hash)
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::FailedGitCheckout)
    }
}
