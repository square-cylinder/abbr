pub mod config;
pub mod storage;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use config::{Config, Mode};
use storage::Storage;


type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> BoxResult<()> {
    match config.mode {
        Mode::Get { abbr } => run_get(&abbr)?,
        Mode::Put { abbr, full } => run_put(&abbr, &full)?,
    };
    Ok(())
}


pub fn run_get(abbr: &str) -> BoxResult<()> {
    let abbr = abbr.to_uppercase();

    let path = get_path()?;

    let storage = Storage::open(&path)?;

    let matching = storage.get_as_str(&abbr);
    println!("{}", matching);

    Ok(())
}

pub fn run_put(abbr: &str, full: &str) -> BoxResult<()> {
    let abbr = abbr.to_uppercase();

    let path = get_path()?;
    let mut storage = Storage::open(&path)?;
    storage.store(&abbr, full)?;
    storage.save(&path)?;
    println!("{} is now stored!", abbr);

    Ok(())
}

fn get_path() -> BoxResult<PathBuf> {
    let mut path = get_dir()?;
    path.push("storage.json");

    Ok(path)
}

fn get_dir() -> BoxResult<PathBuf> {
    let error_msg: &'static str = "Could not fetch path to storage";
    let mut dir = directories::UserDirs::new()
        .ok_or(error_msg)?
        .document_dir()
        .ok_or(error_msg)?
        .to_owned();

    dir.push("abbr");

    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    Ok(dir)
}
