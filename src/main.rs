use std::process;

use clap::Parser;
use abbr::config::Config;

fn main() {
    let command = Config::parse();

    if let Err(err) = abbr::run(command) {
        eprintln!("Application error:\n{err}");
        process::exit(1);
    }
}

