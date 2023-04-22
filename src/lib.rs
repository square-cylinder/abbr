pub mod config;
pub mod storage;

use std::error::Error;
use std::fs;
use std::path::{PathBuf, Path};

use config::{Config, Mode, GetConfig, PutConfig};
use storage::Storage;


type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> BoxResult<()> {
    let file = config.file.unwrap_or(get_path()?);
    match config.mode {
        Mode::Get(cfg) => run_get(cfg, &file)?,
        Mode::Put(cfg) => run_put(cfg, &file)?,
    };
    Ok(())
}


pub fn run_get(cfg: GetConfig, file: &Path) -> BoxResult<()> {
    let abbr = cfg.abbr.to_uppercase();

    let storage = Storage::open(file)?;

    let matching = storage.get_as_str(&abbr);
    println!("{}", matching);

    Ok(())
}

pub fn run_put(cfg: PutConfig, file: &Path) -> BoxResult<()> {
    let abbr = cfg.abbr.to_uppercase();
    let description = cfg.description.as_deref();

    let mut storage = Storage::open(file)?;
    storage.store(&abbr, &cfg.full, description)?;
    storage.save(file)?;
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
