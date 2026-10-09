use std::env;
use std::path::PathBuf;

pub fn work() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("work");
    path
}
