use curl::easy::{Easy, WriteError};
use regex::Regex;
use std::{
    env, fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
};

use crate::terminal::Progress;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("curl: {0}")]
    Curl(#[from] curl::Error),
    #[error("failed to clone git repository")]
    FailedGitClone,
    #[error("git clone did not output destination path")]
    MissingGitCloneDestination,
}

fn url_filename(url: &str) -> io::Result<PathBuf> {
    let mut path = env::current_dir()?;

    match PathBuf::from(url).file_name() {
        Some(s) => path.push(s),
        None => path.push("file"),
    }

    Ok(path)
}

pub fn url(url: &str) -> Result<PathBuf, Error> {
    let mut handle = Easy::new();
    handle.url(url)?;
    handle.progress(true)?;

    // Add these common curl settings that often fix transfer issues
    handle.follow_location(true)?; // Follow redirects
    handle.useragent("Mozilla/5.0 (compatible; curl)")?; // Set a user agent

    let path = url_filename(url)?;
    let file = fs::File::create(&path)?;
    let mut file = std::io::BufWriter::new(file); // Add buffering

    let mut transfer = handle.transfer();
    transfer.write_function(move |data| match file.write_all(data) {
        Ok(()) => Ok(data.len()),
        Err(_) => Err(WriteError::Pause),
    })?;

    let progress = Arc::new(Mutex::new(Progress::default()));
    let progress_for_callback = Arc::clone(&progress);
    transfer.progress_function(move |dltotal, dlnow, _ultotal, _ulnow| {
        let mut guard = match progress_for_callback.lock() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let progress = if dltotal > 0.0 { dlnow / dltotal } else { 0.0 };
        let result = guard.update(progress, &format!("{:.0}/{:.0}", dlnow, dltotal));
        result.is_ok()
    })?;

    progress.lock().unwrap().start().unwrap();

    // Make sure to keep the transfer alive until perform completes
    let result = transfer.perform();

    // Explicitly drop the transfer to ensure file is flushed
    drop(transfer);

    result?;
    progress.lock().unwrap().finish().unwrap();

    Ok(path)
}

pub fn find_git_clone_destination(line: &str) -> Option<PathBuf> {
    let re = Regex::new(r"^Cloning into '(\S+)'...$").unwrap();

    if let Some(captures) = re.captures(line) {
        if let Some(matched) = captures.get(1) {
            return Some(PathBuf::from(matched.as_str()));
        }
        None
    } else {
        None
    }
}

pub fn git(url: &str) -> Result<PathBuf, Error> {
    let mut process = process::Command::new("git")
        .arg("clone")
        .arg(url)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    let stderr = process.stderr.take().expect("stderr piped");
    let reader = io::BufReader::new(stderr);

    let mut path = Option::<PathBuf>::None;
    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(found_path) = find_git_clone_destination(&line) {
            path = Some(found_path);
        }
    }

    if process.wait()?.success() {
        if let Some(cloned_path) = path {
            Ok(cloned_path)
        } else {
            Err(Error::MissingGitCloneDestination)
        }
    } else {
        Err(Error::FailedGitClone)
    }
}
