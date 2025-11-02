use futures_util::StreamExt;
use std::path::Path;
use std::{env, error::Error, fmt::Write, io::Read, path::PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use anyhow::{Result, anyhow, bail};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use och::details::Source;
use och::{details, err, info};
use once_cell::sync::Lazy;

pub static PROGRESS_STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template(
        "[{elapsed_precise:.cyan/blue}] [{wide_bar:.yellow}] {bytes}/{total_bytes} ({eta})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    .progress_chars("oc ")
});

fn url_filename(url: &str) -> String {
    PathBuf::from(url)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download.bin")
        .to_string()
}

async fn fetch_url(url: &str) -> Result<()> {
    let filename = url_filename(url);
    let response = reqwest::get(url).await?.error_for_status()?;

    let total_size = response.content_length().unwrap_or(u64::MAX);
    let pb = ProgressBar::new(total_size);
    pb.set_style(PROGRESS_STYLE.clone());
    let mut stream = response.bytes_stream();

    let mut file = fs::File::create(&filename).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    pb.finish();

    Ok(())
}

async fn fetch_source(source: &Source) -> Result<()> {
    match source {
        Source::Tar { url, hash: _ } => {
            fetch_url(url).await?;
        }
    }

    Ok(())
}

async fn sha256_file(path: impl AsRef<Path>) -> Result<String> {
    let bytes = fs::read(path).await?;
    let hash = sha256::digest(&bytes);

    Ok(hash)
}

async fn check_source(source: &Source) -> Result<()> {
    match source {
        Source::Tar { url, hash } => {
            let filename = url_filename(url);
            let file_hash = sha256_file(&filename).await?;

            if file_hash
                != hash
                    .clone()
                    .ok_or(anyhow!("missing hash {} ({})", url, file_hash))?
            {
                bail!("Incorrect hash for {}", url)
            }
        }
    }

    Ok(())
}

async fn tar_extract_file(path: impl AsRef<Path>) -> Result<()> {
    let file = std::fs::File::open(&path)?;
    let total = file.metadata()?.len();
    let pb = ProgressBar::new(total);
    pb.set_style(PROGRESS_STYLE.clone());

    let pb_reader = pb.wrap_read(file);

    let reader: Box<dyn Read> = {
        let name = path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_lowercase();
        if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            Box::new(flate2::read::GzDecoder::new(pb_reader))
        } else if name.ends_with(".tar.xz") || name.ends_with(".txz") {
            Box::new(xz2::read::XzDecoder::new(pb_reader))
        } else if name.ends_with(".tar.bz2") || name.ends_with(".tbz") || name.ends_with(".tbz2") {
            Box::new(bzip2::read::BzDecoder::new(pb_reader))
        } else if name.ends_with(".tar") {
            // plain tar
            Box::new(pb_reader)
        } else {
            // fallback: try to auto-detect by magic or assume plain tar
            // (optionally replace this with deko/autocompress for magic detection)
            Box::new(pb_reader)
        }
    };

    let mut archive = tar::Archive::new(reader);
    archive.unpack(".")?;

    Ok(())
}

async fn process_source(source: &Source) -> Result<()> {
    match source {
        Source::Tar { url, hash: _ } => {
            let filename = url_filename(url);
            tar_extract_file(filename).await?;
        }
    }

    Ok(())
}

async fn makeoch() -> Result<()> {
    info!("Parsing OCHBUILD");
    let contents = fs::read_to_string("OCHBUILD")
        .await
        .expect("Failed to read OCHBUILD file");
    let details = details::parse(contents)?;

    info!(
        "Found package \"{}\" version {}",
        details.name, details.version
    );
    let work_path = PathBuf::from("work");
    if fs::metadata(&work_path).await.is_ok() {
        info!("Clearing work directory");
        fs::remove_dir_all(&work_path).await?;
    }

    fs::create_dir(&work_path).await?;
    env::set_current_dir(&work_path)?;

    info!("Fetching sources");
    for source in &details.sources {
        fetch_source(source).await?;
    }

    info!("Checking sources");
    for source in &details.sources {
        check_source(source).await?;
    }

    info!("Processing sources");
    for source in &details.sources {
        process_source(source).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    match makeoch().await {
        Err(e) => {
            err!("{}", e)
        }
        _ => {}
    };
}
