use std::{fs, io, path::Path};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("missing hash, found {found}")]
    MissingHash { found: String },
    #[error("unexpected hash, expected {expected}, found {found}")]
    WrongHash { expected: String, found: String },
}

pub fn sha256(path: &Path) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let hash = sha256::digest(&bytes);

    Ok(hash)
}
