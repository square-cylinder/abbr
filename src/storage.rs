use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::fmt::Display;

type BoxResult<T> = Result<T, Box<dyn Error>>;

/// Datatype for representing an abbreviation, with all its possible matches
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    short_form: String,
    items: Vec<EntryItem>,
}

impl Entry {
    /// Gives the abbreviation name in the short form
    pub fn short(&self) -> &str {
        &self.short_form
    }

    /// Gives all matching meanings with description 
    pub fn items(&self) -> &[EntryItem] {
        &self.items
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.items.len().cmp(&1) {
            Ordering::Less => write!(f, "{} has no matches", self.short_form),
            Ordering::Equal => write!(f, "{}:\n{}", self.short_form, self.items()[0]),
            Ordering::Greater => {
                writeln!(f, "{} is one of the following:", self.short_form)?;
                for (index, item) in self.items().iter().enumerate() {
                    writeln!(f, "\nOption {}:", index + 1)?;
                    write!(f, "{}", item)?;
                }
                Ok(())
            }
        }
    }
}

/// Datatype for displaying a single match for an abbreviation. Does not
/// contain any information about the short form.
#[derive(Clone, Serialize, Deserialize)]
pub struct EntryItem {
    meaning: String,
    description: Option<String>,
    id: u32,
}

impl EntryItem {

    /// Returns the item description as an option, because it can be omitted
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|x| x.as_str())
    }

    /// Returns the meaning (long form) of the abbreviation it represents
    pub fn meaning(&self) -> &str {
        &self.meaning
    }

    /// Returns the entry id for the item
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Display for EntryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { meaning, description, id } = self;
        write!(f, "item: {} ({})", meaning, id)?;
        if let Some(descr) = description {
            write!(f, "\ndescription: {}", descr)?;
        }
        Ok(())
    }
}

/// Datatype for representing a collection of abbreviations
#[derive(Serialize, Deserialize)]
pub struct Storage {
    data: HashMap<String, Entry>,
    total_stored_items: u32,
}

impl Storage {

    /// Deprecated, don't use
    pub fn open(path: &Path) -> BoxResult<Self> {
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true);

        let mut contents = String::new();
        let mut file = options.open(path)?;
        file.read_to_string(&mut contents)?;

        Ok(Self::parse(&contents)?)
    }

    /// Converts a json representation of a storage object as a string into
    /// a storage object
    pub fn parse(contents: &str) -> serde_json::Result<Self> {
        match contents.is_empty() {
            true => Ok(Self::new()),
            false => serde_json::from_str(&contents),
        }
    }

    /// Creates a brand new, empty Storage object
    pub fn new() -> Self {
        Self { 
            data: HashMap::new(),
            total_stored_items: 0,
        }
    }

    /// Stores a new abbreviation without description for now...
    pub fn store(&mut self, abbr: &str, full: &str) {
        let full = full.to_owned();
        let item = EntryItem {
            meaning: full,
            description: None,
            id: self.total_stored_items,
        };
        self.total_stored_items += 1;
        match self.data.get_mut(abbr) {
            Some(entry) => entry.items.push(item), // TODO: some extra logic to avoid duplicates
            None => {
                let entry = Entry {
                    short_form: abbr.to_owned(),
                    items: vec![item],
                };
                self.data.insert(abbr.to_owned(), entry);
            },
        }
    }

    /// Returns an entry
    pub fn get(&self, abbr: &str) -> Entry {
        match self.data.get(abbr) {
            Some(entry) => entry.clone(),
            None => {
                Entry {
                    short_form: abbr.to_owned(),
                    items: Vec::new()
                }
            }
        }
    }

    /// Returns a string representation of the specified entry
    pub fn get_as_str(&self, abbr: &str) -> String {
        let entry = self.get(abbr);
        entry.to_string()
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_item_basic_functions_with_description() {
        let item = EntryItem {
            meaning: String::from("Central Processing Unit"), 
            description: Some(String::from("The brain of the computer")), 
            id: 1234
        };
        assert_eq!(item.to_string(), "\
item: Central Processing Unit (1234)
description: The brain of the computer");
        assert_eq!(item.meaning(), "Central Processing Unit");
        assert_eq!(item.description(), Some("The brain of the computer"));
        assert_eq!(item.id(), 1234);
    }

    #[test]
    fn entry_item_basic_functions_without_description() {
        let item = EntryItem {
            meaning: String::from("Central Processing Unit"), 
            description: None,
            id: 1234
        };
        assert_eq!(item.to_string(), "item: Central Processing Unit (1234)");
        assert_eq!(item.meaning(), "Central Processing Unit");
        assert_eq!(item.description(), None);
        assert_eq!(item.id(), 1234);
    }

}
