use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::error::Error;
use std::io;

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    data: HashMap<String, Vec<String>>,
}

impl Storage {

    pub fn open(path: &Path) -> BoxResult<Self> {
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true);

        let mut contents = String::new();
        let mut file = options.open(path)?;
        file.read_to_string(&mut contents)?;

        Ok(Self::parse(&contents)?)
    }

    pub fn parse(contents: &str) -> serde_json::Result<Self> {
        match contents.is_empty() {
            true => Ok(Self::new()),
            false => serde_json::from_str(&contents),
        }
    }

    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn store(&mut self, abbr: &str, full: &str) {
        self.data.entry(abbr.to_owned())
            .and_modify(|v| v.push(full.to_owned()))
            .or_insert(vec![full.to_owned()]);
    }

    fn stringify_matches(matches: &Vec<String>) -> String {
        match matches.len().cmp(&1) {
            Ordering::Less => "No matches :(".to_owned(),
            Ordering::Equal => matches[0].to_owned(),
            Ordering::Greater => {
                let mut s = String::new();
                s.push_str("Matches\n");
                for matching in matches.iter() {
                    s.push_str(&format!(" * {matching}\n"));
                }
                s.pop();
                s
            }
        }
    }

    pub fn get_as_str(&self, abbr: &str) -> String {
        Self::stringify_matches(self.data.get(abbr).unwrap_or(&Vec::new()))
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let mut options = OpenOptions::new();
        options.write(true).create(true);

        let mut file = options.open(&path)?;
        self.save_to_file(&mut file)?;
        Ok(())
    }

    pub fn save_to_file(&self, file: &mut File) -> io::Result<()> {
        let json = serde_json::to_string(self).expect("Failed to serialize storage");
        file.write(json.as_bytes())?;
        Ok(())
    }
}
