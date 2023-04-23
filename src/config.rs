use std::path::PathBuf;

use clap::{Subcommand, Parser, Args};

#[derive(Subcommand, Debug)]
pub enum Mode {
    Put(PutConfig),
    Get(GetConfig),
    Mod(ModConfig),
    Del(DelConfig),
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

fn parse_id(arg: &str) -> Result<usize, String> {
    let num: usize = arg.parse::<usize>()
        .map_err(|err| err.to_string())?;
    if num < 1 {
        return Err(String::from("id needs to be greater than or equal to 1"));
    }
    Ok(num)
}

#[derive(Args, Debug)]
pub struct ModConfig {
    /// The abbreviation you want to modify
    pub abbr: String,
    /// The associated id (may be excluded if there is only one item)
    #[arg(value_parser=parse_id)]
    pub id: Option<usize>,
    /// Optional new meaning
    #[arg(short, long)]
    pub meaning: Option<String>,
    /// Optional new description (use with empty string to remove current)
    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Args, Debug)]
pub struct DelConfig {
    /// The abbreviation you want to modify
    pub abbr: String,
    /// The associated id (may be excluded if there is only one item)
    #[arg(value_parser=parse_id)]
    pub id: Option<usize>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Config {
    #[command(subcommand)]
    pub mode: Mode,
    /// The file to use as storage for abbreviations
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}
