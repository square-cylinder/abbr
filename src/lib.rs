pub mod config;
pub mod storage;

use std::error;
use std::fs;
use std::path::{PathBuf, Path};
use std::result;

use config::{Config, Mode, GetConfig, PutConfig, ModConfig};

use storage::{Storage, StorageModification, StorageError};

type BoxResult<T> = result::Result<T, Box<dyn error::Error>>;

pub fn run(config: Config) -> BoxResult<()> {
    let file = config.file.unwrap_or(get_path()?);
    match config.mode {
        Mode::Get(cfg) => run_get(cfg, &file),
        Mode::Put(cfg) => run_put(cfg, &file),
        Mode::Mod(cfg) => run_mod(cfg, &file),
    }
}


pub fn run_get(cfg: GetConfig, file: &Path) -> BoxResult<()> {
    let storage = match Storage::load(file) {
        Ok(v) => v,
        Err(StorageError::NoSuchFile) => {
            println!("You have yet to store any abbreviations, do so with `abbr put`");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };
    let abbr = cfg.abbr.to_uppercase();
    println!("{}", storage.get(&abbr));
    Ok(())
}

pub fn run_put(cfg: PutConfig, file: &Path) -> BoxResult<()> {
    let mut storage = match Storage::load(file) {
        Ok(v) => v,
        Err(StorageError::NoSuchFile) => Storage::new(),
        Err(e) => return Err(e.into()),
    };
    let PutConfig { abbr, full, description } = cfg;
    let abbr = abbr.to_uppercase();
    storage.put(abbr.clone(), full.clone(), description)?;
    storage.write(file)?;
    println!("Successfully added: {} - {}", abbr, full);
    Ok(())
}

pub fn run_mod(cfg: ModConfig, file: &Path) -> BoxResult<()> {
    let mut storage = Storage::load(file)?;
    let ModConfig { abbr, id, meaning, description } = cfg;
    let modification = StorageModification::new(abbr.to_uppercase(), id.map(|num| num - 1));
    let modification = if let Some(name) = meaning {
        modification.name(name)
    } else {
        modification
    };
    let modification = match description.as_deref() {
        None => modification,
        Some("") => modification.description(None),
        Some(_) => modification.description(Some(description.unwrap())),
    };
    storage.modify(modification)?;
    storage.write(file)?;
    let id = id.unwrap_or(1);
    println!("Successfully modified: {} ({})", abbr, id);
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

#[cfg(test)]
mod tests {

}
