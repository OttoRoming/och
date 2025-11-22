use std::path::PathBuf;

/// Data for locally installed packages
pub mod local;

/// Data for packages available in remote sync repository
pub mod sync;

fn data_dir_path() -> PathBuf {
    PathBuf::from("/var/lib/och/")
}
