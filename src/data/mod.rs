use std::{
    default::Default,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

fn data_dir_path() -> PathBuf {
    PathBuf::from("/var/lib/och/")
}
fn data_file_path() -> PathBuf {
    PathBuf::from("/var/lib/och/data.ron")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub is_explicit: bool,
    pub version: String,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub installed: Vec<Package>,
}

impl Default for Data {
    fn default() -> Self {
        Self { installed: vec![] }
    }
}

pub fn read() -> Result<Data> {
    let mut data = Data::default();
    let file = fs::File::open(data_file_path());
    if let Ok(mut file) = file {
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        data = ron::from_str(&source)?;
    };

    Ok(data)
}

pub fn write(data: &Data) -> Result<()> {
    let mut file = match fs::File::create(data_file_path()) {
        Ok(file) => file,
        Err(_) => {
            if fs::metadata(data_dir_path()).is_err() {
                fs::create_dir_all(data_dir_path())?;
            }
            fs::File::create_new(data_file_path())?
        }
    };

    let data_string = ron::ser::to_string_pretty(data, PrettyConfig::default())?;
    file.write_all(data_string.as_bytes())?;
    Ok(())
}
