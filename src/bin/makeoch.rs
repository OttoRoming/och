use futures_util::StreamExt;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fmt::Write, io::Read, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt, process};

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

fn url_filename(url: &str) -> Result<PathBuf> {
    let mut path = env::current_dir()?;
    match PathBuf::from(url).file_name() {
        Some(s) => path.push(s),
        None => path.push("download.bin"),
    }

    Ok(path)
}

async fn fetch_url(url: &str, path: impl AsRef<Path>) -> Result<()> {
    let response = reqwest::get(url).await?.error_for_status()?;

    let total_size = response.content_length().unwrap_or(u64::MAX);
    let pb = ProgressBar::new(total_size);
    pb.set_style(PROGRESS_STYLE.clone());
    let mut stream = response.bytes_stream();

    let mut file = fs::File::create(&path).await?;
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

async fn fetch_source(source: &Source) -> Result<PathBuf> {
    match source {
        Source::Tar { url, hash: _ } => {
            let filename = url_filename(url)?;
            fetch_url(url, &filename).await?;

            Ok(filename)
        }
    }
}

async fn sha256_file(path: impl AsRef<Path>) -> Result<String> {
    let bytes = fs::read(path).await?;
    let hash = sha256::digest(&bytes);

    Ok(hash)
}

async fn check_source(source: &Source, path: impl AsRef<Path>) -> Result<()> {
    match source {
        Source::Tar { url, hash } => {
            let file_hash = sha256_file(path).await?;

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

async fn tar_extract_file(path: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
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
    archive.unpack(destination)?;
    pb.finish();

    Ok(())
}

async fn process_source(
    source: &Source,
    path: impl AsRef<Path>,
    work_path: impl AsRef<Path>,
) -> Result<()> {
    match source {
        Source::Tar { url: _, hash: _ } => {
            tar_extract_file(path, work_path).await?;
        }
    }

    Ok(())
}

async fn makeoch() -> Result<()> {
    unsafe {
        env::set_var("MAKEFLAGS", "-j8");
    }

    info!("Parsing OCHBUILD");
    let root = env::current_dir()?;
    let mut ochbuild_path = root.clone();
    ochbuild_path.push("OCHBUILD");
    let contents = fs::read_to_string(&ochbuild_path)
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
    let work_path = env::current_dir()?;
    let mut destination_dir = work_path.clone();
    destination_dir.push("dest");
    fs::create_dir(&destination_dir).await?;
    unsafe {
        env::set_var("DESTDIR", destination_dir.to_str().unwrap());
    }

    let mut source_paths = HashMap::new();
    info!("Fetching sources");
    for source in details.sources.iter() {
        let path = fetch_source(source).await?;
        source_paths.insert(source, path);
    }

    info!("Checking sources");
    for source in details.sources.iter() {
        check_source(source, source_paths.get(source).unwrap()).await?;
    }

    info!("Processing sources");
    for source in details.sources.iter() {
        process_source(source, source_paths.get(source).unwrap(), &work_path).await?;
    }

    info!("Running OCHBUILD");
    let ochbuild_status = process::Command::new("bash")
        .arg("-e")
        .arg(ochbuild_path.to_str().unwrap())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await?;

    if !ochbuild_status.success() {
        bail!("OCHBUILD Exited with statuscode {}", ochbuild_status)
    }

    info!("Creating package archvie");
    let mut pacakge_archive_path = root.clone();
    pacakge_archive_path.push(format!("{}-{}.tar.gz", details.name, details.version));
    let package_archive = std::fs::File::create(&pacakge_archive_path)?;
    let buffer = std::io::BufWriter::new(package_archive);
    let encoder = flate2::write::GzEncoder::new(buffer, flate2::Compression::best());
    let mut tar = tar::Builder::new(encoder);
    tar.append_dir_all(".", destination_dir)?;
    tar.finish()?;

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
