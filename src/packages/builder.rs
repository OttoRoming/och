//! Build packages

use crate::paths;
use deko::AnyDecoder;
use sha2::{Digest, Sha256};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};

use futures_util::StreamExt;
use tempfile::tempfile;
use url::Url;

use super::{Package, Source};

fn unpack(reader: impl BufRead) -> io::Result<()> {
    let mut decoder = AnyDecoder::new(reader);
    decoder.fail_on_unknown_format(true);

    let mut archive = tar::Archive::new(decoder);

    archive.unpack(&paths::work())?;

    Ok(())
}

async fn fetch_http(url: Url, hash: &[u8; 32]) {
    let response = reqwest::get(url).await.unwrap(); //.bytes().await.unwrap();
    assert!(response.status().is_success());

    let mut file = tempfile().unwrap();
    let mut hasher = Sha256::new();
    let mut byte_stream = response.bytes_stream();

    while let Some(chunk_result) = byte_stream.next().await {
        let chunk = chunk_result.unwrap();
        file.write_all(&chunk).unwrap();
        hasher.update(&chunk);
    }

    let fetched_hash: [u8; 32] = hasher.finalize().into();
    assert_eq!(hash, &fetched_hash);

    file.seek(SeekFrom::Start(0)).unwrap();
    let reader = BufReader::new(file);
    unpack(reader).unwrap();
}

pub struct Builder<'a> {
    package: &'a Package,
}

impl<'a> Builder<'a> {
    pub fn new(package: &'a Package) -> Self {
        Self { package }
    }

    pub async fn build(&self) {
        self.fetch_sources().await;
    }

    async fn fetch_sources(&self) {
        for source in self.package.iter_sources() {
            match &source {
                &Source::Http { url, hash_sha256 } => fetch_http(url.clone(), hash_sha256).await,
            }
        }
    }
}
