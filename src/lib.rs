pub mod config;

use std::error::Error;
use std::path::PathBuf;
use std::fmt::Display;
use std::fs::{self, File, OpenOptions};
use std::io::{Seek, Read, Write};
use std::collections::HashMap;

use config::{Config, Mode};

use serde::{Deserialize, Serialize};

type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> BoxResult<()> {
    match config.mode {
        Mode::Get { abbr } => run_get(&abbr)?,
        Mode::Put { abbr, full } => run_put(&abbr, &full)?,
    };
    Ok(())
}

pub fn get_storage_path() -> Result<PathBuf, AbbrError> {
    let mut dir = directories::UserDirs::new()
        .ok_or(AbbrError::new(ErrorKind::CannotGetStorage))?
        .document_dir()
        .ok_or(AbbrError::new(ErrorKind::CannotGetStorage))?
        .to_owned();

    dir.push("abbr");

    Ok(dir)
}

pub fn open_storage(options: &mut OpenOptions) -> BoxResult<File> {
    let mut path = get_storage_path()?;

    if !path.exists() {
        fs::create_dir(&path)?;
    }

    path.push("storage.json");

    Ok(options.open(&path)?)
}

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    data: HashMap<String, Vec<String>>,
}

pub fn run_get(abbr: &str) -> BoxResult<()> {
    let abbr = abbr.to_uppercase();

    let mut file = open_storage(OpenOptions::new().read(true))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let storage: Storage = if contents.is_empty() {
        Storage { data: HashMap::new() }
    } else {
        serde_json::from_str(&contents)?
    };

    match storage.data.get(&abbr) {
        Some(matches) => {
            if matches.len() == 1 {
                println!("{}", matches[0]);
            } else if matches.len() > 1 {
                println!("Matches:");
                for matching in matches {
                    println!(" * {matching}");
                }
            } else {
                println!("No matches :(");
            }
        },
        None => {
            println!("No matches :(");
        }
    };

    Ok(())
}

pub fn run_put(abbr: &str, full: &str) -> BoxResult<()> {
    let abbr = abbr.to_uppercase();

    let mut file = open_storage(OpenOptions::new().read(true).write(true).create(true))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut storage: Storage = if contents.is_empty() {
        Storage { data: HashMap::new() }
    } else {
        serde_json::from_str(&contents)?
    };

    storage.data.entry(abbr.to_owned())
        .and_modify(|v| v.push(full.to_owned()))
        .or_insert(vec![full.to_owned()]);

    file.rewind()?;
    file.write(serde_json::to_string(&storage)?.as_bytes())?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct AbbrError {
    pub kind: ErrorKind,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    CannotGetStorage,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::CannotGetStorage => write!(f, "Could not fetch path to program storage"),
        }
    }
}

impl Display for AbbrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Error for AbbrError { }

impl AbbrError {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
