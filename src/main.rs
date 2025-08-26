//main.rs
use std::env;
use std::process;
use core::result::Result::Err;
use minigrep::{Config,run,Parse};

fn main() {
    let config = Config::parse_config(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    }

}
