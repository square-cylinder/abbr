use std::{
    fmt,
    path::Path,
    collections::HashMap,
    error,
    fs::OpenOptions,
    result,
    io::{self, Read, Write},
};

use serde::{Serialize, Deserialize};

type Error = StorageError;
type Result<T> = result::Result<T, Error>;

pub struct StorageModification {
    acronym: String,
    id: Option<usize>,
    new_name: Option<String>,
    new_description: Option<Option<String>>,
}

impl StorageModification {
    pub fn new(acronym: String, id: Option<usize>) -> Self {
        Self {
            acronym,
            id,
            new_name: None,
            new_description: None,
        }
    }

    pub fn name(mut self, new_name: String) -> Self {
        self.new_name = Some(new_name);
        self
    }

    pub fn description(mut self, new_description: Option<String>) -> Self {
        self.new_description = Some(new_description);
        self
    }
}

/// Struct for representing acronym database
#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    data: HashMap<String, Entry>
}

impl Storage {

    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn delete(&mut self, acronym: &str, id: Option<usize>) -> Result<()> {
        let entry = self.data.get_mut(acronym)
            .ok_or(StorageError::NoSuchItem)?;
        let id = match id {
            Some(val) => val,
            None => {
                if entry.items.len() > 1 {
                    return Err(StorageError::AmbigousItem);
                }
                0
            }
        };
        if id < entry.items.len() {
            entry.items.remove(id);
        } else {
            return Err(StorageError::NoSuchItem);
        }
        if entry.items.is_empty(){
            self.data.remove(acronym);
        }
        Ok(())
    }

    pub fn modify(
        &mut self,
        StorageModification { acronym, id, new_name, new_description }: StorageModification)
        -> Result<()>
    {
        let entry = self.data.get_mut(&acronym)
            .ok_or(StorageError::NoSuchItem)?;
        let id = match id {
            Some(val) => val,
            None => {
                if entry.items.len() > 1 {
                    return Err(StorageError::AmbigousItem);
                }
                0
            }
        };
        let mut item = entry.items.get_mut(id)
            .ok_or(StorageError::NoSuchItem)?;
        if let Some(new_name) = new_name {
            item.name = new_name;
        }
        if let Some(new_description) = new_description {
            item.description = new_description;
        }
        Ok(())
    }

    /// Query data for acronym, returns string representation
    pub fn get(&self, acronym: &str) -> String {
        match self.data.get(acronym) {
            Some(entry) => entry.to_string(),
            None => format!("{} is not stored", acronym),
        }
    }

    /// Add an acronym to database, can fail if aconym is already present, i.e.
    /// the short form and the long form are found
    pub fn put(
        &mut self,
        acronym: String,
        name: String,
        description: Option<String>,
    ) -> Result<()> {
        match self.data.get_mut(&acronym) {
            Some(entry) => entry.add_item(name, description)?,
            None => {
                self.data.insert(acronym.clone(), 
                    Entry::new(acronym, vec![Item::new(name, description)])
                );
            },
        }
        Ok(())
    }

    /// Loads a json encoded storage struct from a file
    pub fn load(path: &Path) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str(&contents)?)
    }

    /// Writes a json enncoded storage struct to a file
    pub fn write(&self, path: &Path) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let contents = serde_json::to_string_pretty(self)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entry {
    acronym: String,
    items: Vec<Item>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.acronym)?;
        for (i, item) in self.items.iter().enumerate() {
            write!(f, "\n {}) {}", i+1, item)?;
        }
        Ok(())
    }
}

impl Entry {
    fn new(acronym: String, items: Vec<Item>) -> Self {
        Self {
            acronym,
            items,
        }
    }

    fn add_item(&mut self, name: String, description: Option<String>) -> Result<()> {
        if self.items.iter().fold(false, |acc, item| acc || &item.name == &name) {
            return Err(StorageError::DuplicatePut(name));
        }
        self.items.push(Item::new(name, description));
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    name: String,
    description: Option<String>,
}

impl Item {
    fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = self.description.as_ref() {
            write!(f, "{}: {}", self.name, description)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Debug)]
pub enum StorageError {
    NoSuchFile,
    NoSuchItem,
    AmbigousItem,
    ParsingProblem(String),
    DuplicatePut(String),
    Other(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSuchFile => write!(f, "Tried to load a file which doesn't exist"),
            Self::NoSuchItem => write!(f, "Item does not exist in storage"),
            Self::AmbigousItem => write!(f, "Not enough information to decide which item, please provide an id"),
            Self::ParsingProblem(s) => write!(f, "Failed to parse storage: {}", s),
            Self::DuplicatePut(s) => write!(f, "{} already exists in storage", s),
            Self::Other(s) => write!(f, "Unexpected storage error: {}", s),
        }
    }
}

impl From<io::Error> for StorageError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => StorageError::NoSuchFile,
            _ => StorageError::Other(value.to_string()),
        }
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParsingProblem(value.to_string())
    }
}

impl error::Error for StorageError { }

#[cfg(test)]
mod tests {
    use super::*;

}
