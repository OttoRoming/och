use clap::Parser;
use och::data::{self};
use std::error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    package: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let mut db = data::local::read()?;
    dbg!(&db);
    db.installed.push(data::local::Package {
        name: args.package,
        is_explicit: true,
        version: "0.1.0".to_string(),
        files: vec![],
    });
    dbg!(&db);
    data::local::write(&db)?;

    Ok(())
}
