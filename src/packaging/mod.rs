use std::io::{self, Read, Seek, SeekFrom, Write};

use sha2::{Digest, Sha256};
use tempfile::tempfile;

use crate::packages::{Package, Source};

/// Build the provided package to a tarball
pub fn package(pkg: Package) {
    for source in pkg.iter_sources() {
        match source {
            Source::Http { url, hash_sha256 } => {
                let body = ureq::get(url)
                    .call()
                    .unwrap_or_else(|error| panic!("Failed to fetch {url}: {error}"))
                    .into_body()
                    .into_reader();

                // Stream the archive to an anonymous temporary file while
                // hashing it. This keeps the whole tarball out of memory and
                // lets us verify it before anything is unpacked.
                let mut file = tempfile()
                    .unwrap_or_else(|error| panic!("Failed to create temporary file: {error}"));

                let hash = hash_while_writing(body, &mut file)
                    .unwrap_or_else(|error| panic!("Failed to download {url}: {error}"));

                assert_eq!(
                    &hash, hash_sha256,
                    "Hash mismatch for {}: expected {:x?}, got {:x?}",
                    url, hash_sha256, hash
                );

                file.seek(SeekFrom::Start(0))
                    .unwrap_or_else(|error| panic!("Failed to rewind {url}: {error}"));

                unpack(&mut file).unwrap_or_else(|error| panic!("Failed to unpack {url}: {error}"));
            }
        }
    }
    // pkg.iter_build_dependencies()
}

/// Streams `reader` into `writer`, returning the SHA256 of what was written.
fn hash_while_writing(reader: impl Read, writer: &mut impl Write) -> io::Result<[u8; 32]> {
    let mut reader = HashingReader::new(reader);

    io::copy(&mut reader, writer)?;
    writer.flush()?;

    Ok(reader.finalize())
}

/// A reader that hashes everything read through it.
struct HashingReader<R> {
    inner: R,
    hasher: Sha256,
}

impl<R: Read> HashingReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            hasher: Sha256::new(),
        }
    }

    fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }
}

impl<R: Read> Read for HashingReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let read = self.inner.read(buffer)?;

        self.hasher.update(&buffer[..read]);

        Ok(read)
    }
}

/// Unpacks a gzip-compressed tarball into the current directory.
fn unpack(reader: impl Read) -> io::Result<()> {
    let decoder = flate2::read::GzDecoder::new(io::BufReader::new(reader));
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(".")?;

    Ok(())
}
