use std::{
    default::Default,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub mod local;

fn data_dir_path() -> PathBuf {
    PathBuf::from("/var/lib/och/")
}
fn sync_data_file_path() -> PathBuf {
    PathBuf::from("/var/lib/och/sync")
}
