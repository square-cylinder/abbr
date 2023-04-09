use std::{path::{Path, PathBuf}, fs};

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

fn get_documents() -> Result<PathBuf, String> {
    if let Some(user_dirs) = directories::UserDirs::new() {
        if let Some(documents) = user_dirs.document_dir() {
            return Ok(documents.to_path_buf());
        }
    }
    Err("Could not retrieve users document folder".to_owned())
}

fn get_program_directory() -> Result<PathBuf, String> {
    let mut path = get_documents()?;
    path.push("abbr");
    match fs::create_dir_all(&path) {
        Ok(()) => Ok(path),
        Err(e) => Err(e.to_string())
    }
}

fn get_storage() -> Result<PathBuf, String> {
    let mut path = get_program_directory()?;
    path.push("storage.txt");
    Ok(path)
}

fn main() {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::Get { abbr } => {
            todo!();
        },
        SubCommand::Put { abbr, full } => {
            todo!();
        }
    }
}
