use anyhow::Result;
use clap::Parser;
use och::data::{self};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    package: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut db = data::read()?;
    dbg!(&db);
    db.installed.push(data::Package {
        name: args.package,
        is_explicit: true,
        version: "0.1.0".to_string(),
        files: vec![],
    });
    dbg!(&db);
    data::write(&db)?;

    Ok(())
}
