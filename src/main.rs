use std::process;

use clap::Parser;
use abbr::config::Config;

fn main() {
    let config = Config::parse();

    if let Err(err) = abbr::run(config) {
        eprintln!("Something went wrong:\n{err}");
        process::exit(1);
    }
}

