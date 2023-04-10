use std::fmt;
use std::{path::{Path, PathBuf}, fs};
use std::io::{self, Read};
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
        fs::File::create(storage)
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

fn get_matches(folder: &ProgramFolder, abbr: &str) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut f = folder.read_storage()?;
    let mut content = String::new();
    let mut matches = Vec::new();
    let abbr = abbr.to_uppercase();
    f.read_to_string(&mut content)?;
    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return Err(StorageCorrupted.into());
        }
        let (cmp, res) = (parts[0].to_uppercase(), parts[1]);
        if abbr == cmp {
            matches.push(res.to_owned());
        }
    }
    Ok(matches)
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
    } else {
        println!("Possibilities:");
        for v in m {
            println!(" * {v}");
        }
    }
}

fn main() {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::Get { abbr } => {
            print_possibilities(&abbr);
        },
        SubCommand::Put { abbr, full } => {
            todo!()
        }
    }
}
