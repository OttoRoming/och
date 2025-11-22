use std::{
    env, fmt, io,
    path::{Path, PathBuf},
};

mod check;
mod fetch;
mod process;

pub use check::Error as CheckError;
pub use fetch::Error as FetchError;
pub use process::Error as ProcesError;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Source {
    Tar { url: String, hash: Option<String> },
    Get { url: String, hash: Option<String> },
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Get { url, hash: _ } => {
                write!(f, "get({})", url)
            }
            Self::Tar { url, hash: _ } => {
                write!(f, "tar({})", url)
            }
        }
    }
}

fn url_filename(url: &str) -> io::Result<PathBuf> {
    let mut path = env::current_dir()?;

    match PathBuf::from(url).file_name() {
        Some(s) => path.push(s),
        None => path.push("file"),
    }

    Ok(path)
}

impl Source {
    pub fn fetch(&self) -> Result<PathBuf, fetch::Error> {
        match self {
            Source::Get { url, hash: _ } | Source::Tar { url, hash: _ } => {
                let filename = url_filename(url)?;
                fetch::url(url, &filename)?;

                Ok(filename)
            }
        }
    }

    pub fn check(&self, path: &Path) -> Result<(), check::Error> {
        match self {
            Source::Tar { url: _, hash } | Source::Get { url: _, hash } => {
                let file_hash = check::sha256(path)?;

                if file_hash
                    != hash.clone().ok_or(check::Error::MissingHash {
                        found: file_hash.clone(),
                    })?
                {
                    return Err(check::Error::WrongHash {
                        expected: hash.clone().unwrap(),
                        found: file_hash,
                    }
                    .into());
                }
            }
        }

        Ok(())
    }

    pub fn process(&self, path: &Path, destination: &Path) -> Result<(), process::Error> {
        match self {
            Source::Tar { url: _, hash: _ } => {
                process::extract_tar(path, destination)?;
            }
            Source::Get { url: _, hash: _ } => {}
        }

        Ok(())
    }
}
