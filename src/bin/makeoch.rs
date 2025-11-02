use futures_util::StreamExt;
use std::{env, error::Error, fmt::Write, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

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

async fn fetch_url(url: &str) -> Result<(), Box<dyn Error>> {
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

async fn fetch_source(source: Source) -> Result<(), Box<dyn Error>> {
    match source {
        Source::Tar { url, hash: _ } => {
            fetch_url(&url).await?;
        }
    }

    Ok(())
}

async fn makeoch() -> Result<(), Box<dyn Error>> {
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

    for source in details.sources {
        fetch_source(source).await?;
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
