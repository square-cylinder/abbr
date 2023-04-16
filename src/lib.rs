pub mod config;

use std::error::Error;
use std::path::PathBuf;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
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

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    data: HashMap<String, Vec<String>>,
}

impl Storage {

    fn get_path() -> BoxResult<PathBuf> {
        let mut path = Self::get_dir()?;
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

    fn open() -> BoxResult<Self> {
        let path = Self::get_path()?;

        let mut options = OpenOptions::new();
        options.read(true);

        Self::new(&mut options.open(&path)?)
    }

    fn new(file: &mut File) -> BoxResult<Self>{
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let storage: Storage = if contents.is_empty() {
            Storage { data: HashMap::new() }
        } else {
            serde_json::from_str(&contents)?
        };

        Ok(storage)
    }

    fn store(&mut self, abbr: &str, full: &str) {
        self.data.entry(abbr.to_owned())
            .and_modify(|v| v.push(full.to_owned()))
            .or_insert(vec![full.to_owned()]);
    }

    fn get_as_str(&self, abbr: &str) -> String {
        let mut result = String::new();
        match self.data.get(abbr) {
            Some(matches) => {
                if matches.len() == 1 {
                    result.push_str(&matches[0]);
                } else if matches.len() > 1 {
                    result.push_str("Matches\n");
                    for matching in matches {
                        result.push_str(&format!(" * {matching}\n"));
                    }
                    result.pop();
                } else {
                    result.push_str("No matches :(");
                }
            },
            None => {
                result.push_str("No matches :(");
            }
        };
        result
    }

    fn save(&self) -> BoxResult<()> {
        let path = Self::get_path()?;

        let mut options = OpenOptions::new();
        options.write(true).create(true);

        self.save_to_file(&mut options.open(&path)?)
    }

    fn save_to_file(&self, file: &mut File) -> BoxResult<()> {
        file.write(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }
}

pub fn run_get(abbr: &str) -> BoxResult<()> {
    let abbr = abbr.to_uppercase();

    let storage = Storage::open()?;

    let matching = storage.get_as_str(&abbr);
    println!("{}", matching);

    Ok(())
}

pub fn run_put(abbr: &str, full: &str) -> BoxResult<()> {
    let abbr = abbr.to_uppercase();

    let mut storage = Storage::open()?;

    storage.store(&abbr, full);

    storage.save()?;

    Ok(())
}
