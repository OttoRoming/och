use std::{default::Default, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

static DATA_DIR_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/var/lib/och/"));
static DATA_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/var/lib/och/data.ron"));

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub is_explicit: bool,
    pub version: String,
    pub files: Vec<String>,
}

impl Package {
    pub fn new(name: String, is_explicit: bool) -> Self {
        Self { name, is_explicit }
    }
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

pub async fn read() -> Result<Data> {
    let mut data = Data::default();
    let file = fs::File::open(&DATA_FILE_PATH.clone()).await;
    if let Ok(mut file) = file {
        let mut source = String::new();
        file.read_to_string(&mut source).await?;
        data = ron::from_str(&source)?;
    };

    Ok(data)
}

pub async fn write(data: &Data) -> Result<()> {
    let mut file = match fs::File::create(&DATA_FILE_PATH.clone()).await {
        Ok(file) => file,
        Err(_) => {
            if fs::metadata(&DATA_DIR_PATH.clone()).await.is_err() {
                fs::create_dir_all(&DATA_DIR_PATH.clone()).await?;
            }
            fs::File::create_new(&DATA_FILE_PATH.clone()).await?
        }
    };

    let data_string = ron::ser::to_string_pretty(data, PrettyConfig::default())?;
    file.write_all(data_string.as_bytes()).await?;
    Ok(())
}
