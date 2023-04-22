use std::path::PathBuf;

use clap::{Subcommand, Parser, Args};

#[derive(Subcommand, Debug)]
pub enum Mode {
    Put(PutConfig),
    Get(GetConfig),
}

#[derive(Args, Debug)]
pub struct GetConfig {
    /// The abbreviation you want to look up
    pub abbr: String,
}

#[derive(Args, Debug)]
pub struct PutConfig {
    /// The abbreviation you want add
    pub abbr: String,
    /// What it means
    pub full: String,
    /// Optional description
    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub mode: Mode,
    /// The file to use as storage for abbreviations
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

