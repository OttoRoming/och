use std::{
    fmt,
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
    Git { url: String, commit_hash: String },
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
            Self::Git { url, commit_hash } => {
                write!(f, "git({}, {})", url, commit_hash)
            }
        }
    }
}

impl Source {
    pub fn fetch(&self) -> Result<PathBuf, fetch::Error> {
        match self {
            Source::Get { url, hash: _ } | Source::Tar { url, hash: _ } => Ok(fetch::url(url)?),
            Source::Git {
                url,
                commit_hash: _,
            } => Ok(fetch::git(url)?),
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
            Source::Git {
                url: _,
                commit_hash: _,
            } => { /* No check for git sources */ }
        }

        Ok(())
    }

    pub fn process(&self, path: &Path, destination: &Path) -> Result<(), process::Error> {
        match self {
            Source::Tar { url: _, hash: _ } => {
                process::extract_tar(path, destination)?;
            }
            Source::Git {
                url: _,
                commit_hash,
            } => process::checkout_git_commit(path, &commit_hash)?,
            Source::Get { url: _, hash: _ } => {}
        }

        Ok(())
    }
}
