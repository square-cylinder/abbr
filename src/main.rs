use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::{path::{Path, PathBuf}, fs};
use std::io::{self, Read, Write};
use std::error;

use clap::{Subcommand, Parser};

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Resolve an abbreviation
    Get {
        #[clap(forbid_empty_values=true)]
        /// The abbreviation you want to look up
        abbr: String,
    },
    /// Add a new abbreviation
    Put {
        #[clap(forbid_empty_values=true)]
        /// The abbreviation you want to add
        abbr: String,
        /// What it means
        full: String,
    },
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Clone)]
struct NoDocsFolder;

impl fmt::Display for NoDocsFolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not find the documents folder")
    }
}

impl error::Error for NoDocsFolder {}

#[derive(Debug)]
struct ProgramFolder {
    path: PathBuf
}

impl ProgramFolder {
    const DIRECTORY_NAME: &str = "abbr";
    const STORAGE_FILENAME: &str = "storage.txt";

    fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut path = match directories::UserDirs::new() {
            Some(user_dir) => match user_dir.document_dir() {
                Some(docs) => docs.to_path_buf(),
                None => return Err(NoDocsFolder.into())
            },
            None => return Err(NoDocsFolder.into())
        };
        path.push(Self::DIRECTORY_NAME);
        if !path.exists() {
            match fs::create_dir(&path) {
                Ok(_) => println!("Created directory: {}", path.display()),
                Err(e) => return Err(e.into())
            }
        }
        Ok(Self { path })
    }

    fn read_storage(&self) -> io::Result<fs::File> {
        let mut storage = self.path.clone();
        storage.push(Self::STORAGE_FILENAME);
        fs::File::open(storage)
    }

    fn write_storage(&self) -> io::Result<fs::File> {
        let mut storage = self.path.clone();
        storage.push(Self::STORAGE_FILENAME);
        OpenOptions::new()
            .read(false)
            .write(false)
            .create(true)
            .append(true)
            .open(storage)
    }
}

#[derive(Debug, Clone)]
struct StorageCorrupted;

impl fmt::Display for StorageCorrupted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage file incorrectly formatted, could not read")
    }
}

impl error::Error for StorageCorrupted { }

fn parse_abbreviations(string: &str) -> Result<HashMap<String, Vec<String>>, ()> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in string.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let (key, val) = (parts[0].to_uppercase(), parts[1]);
        if let Some(vals) = map.get_mut(&key) {
            vals.push(val.to_owned());
        } else {
            map.insert(key, vec![val.to_owned()]);
        }
    }
    Ok(map)
}

fn get_matches(folder: &ProgramFolder, abbr: &str) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut f = folder.read_storage()?;
    let mut content = String::new();
    let abbr = abbr.to_uppercase();
    f.read_to_string(&mut content)?;
    if let Ok(data) = parse_abbreviations(&content) {
        Ok(match data.get(&abbr) {
            Some(matches) => matches.to_owned(),
            None => Vec::new(),
        })
    } else {
        Err(StorageCorrupted.into())
    }
}

fn print_possibilities(search: &str) {
    let p = match ProgramFolder::new() {
        Ok(val) => val,
        Err(e) => { 
            println!("Could not access program storage space: {e}");
            return;
        }
    };
    let m = match get_matches(&p, search) {
        Ok(val) => val,
        Err(e) => {
            println!("Could not read storage file: {e}");
            return;
        }
    };
    if m.is_empty() {
        println!("No matches found :(");
    } else if m.len() == 1 {
        println!("{}",  m[0]);
    } else {
        println!("Possibilities:");
        for v in m {
            println!(" * {v}");
        }
    }
}

fn store_abbr(abbr: &str, full: &str) -> Result<(), ()> {
    let p = match ProgramFolder::new() {
        Ok(val) => val,
        Err(_) => return Err(())
    };
    let mut f = match p.write_storage() {
        Ok(val) => val,
        Err(_) => return Err(())
    };
    match writeln!(f, "{}:{}", abbr, full) {
        Ok(_) => (),
        Err(_) => return Err(())
    };
    Ok(())
}

fn main() {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::Get { abbr } => {
            print_possibilities(&abbr);
        },
        SubCommand::Put { abbr, full } => {
            match store_abbr(&abbr, &full) {
                Ok(_) => println!("Success!"),
                Err(_) => println!("Failed...")
            };
        }
    }
}
