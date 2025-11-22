use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

fn local_data_file_path() -> PathBuf {
    PathBuf::from("/var/lib/och/local")
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

pub fn read() -> anyhow::Result<Data> {
    let mut data = Data::default();
    let file = fs::File::open(local_data_file_path());
    if let Ok(mut file) = file {
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        data = serde_json::from_str(&source)?;
    };

    Ok(data)
}

pub fn write(data: &Data) -> anyhow::Result<()> {
    let mut file = match fs::File::create(local_data_file_path()) {
        Ok(file) => file,
        Err(_) => {
            if fs::metadata(super::data_dir_path()).is_err() {
                fs::create_dir_all(super::data_dir_path())?;
            }
            fs::File::create_new(local_data_file_path())?
        }
    };

    let data_string = serde_json::to_string(data)?;
    file.write_all(data_string.as_bytes())?;
    Ok(())
}
