use curl::easy::{Easy, WriteError};
use std::{
    fs, io,
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::terminal::Progress;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("curl: {0}")]
    Curl(#[from] curl::Error),
}

pub fn url(url: &str, path: &Path) -> Result<(), Error> {
    let mut handle = Easy::new();
    handle.url(url)?;
    handle.progress(true)?;

    // Add these common curl settings that often fix transfer issues
    handle.follow_location(true)?; // Follow redirects
    handle.useragent("Mozilla/5.0 (compatible; curl)")?; // Set a user agent

    let file = fs::File::create(path)?;
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

    Ok(())
}
