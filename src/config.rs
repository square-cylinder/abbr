use clap::{Subcommand, Parser};

#[derive(Subcommand, Debug)]
pub enum Mode {
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
pub struct Config {
    #[clap(subcommand)]
    pub mode: Mode,
}

