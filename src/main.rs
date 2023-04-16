use std::process;

use clap::Parser;
use abbr::config::Config;

fn main() {
    let command = Config::parse();

    if let Err(err) = abbr::run(command) {
        eprintln!("Something went wrong:\n{err}");
        process::exit(1);
    }
}

