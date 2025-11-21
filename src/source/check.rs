use std::path::Path;
use std::{convert, error, fmt, fs, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MissingHash { found: String },
    WrongHash { expected: String, found: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::Io(err) => {
                write!(f, "io: {}", err)
            }
            &Self::MissingHash { found } => {
                write!(f, "missing hash, found {}", found)
            }
            &Self::WrongHash { expected, found } => {
                write!(f, "unexpected hash, expected {}, found {}", expected, found)
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

pub fn sha256(path: &Path) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let hash = sha256::digest(&bytes);

    Ok(hash)
}
