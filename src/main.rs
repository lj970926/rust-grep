use std::env;
use std::error::Error;
use std::fs;
use std::process;

use rust_grep::{search, search_case_insensitive};

struct GrepConfig {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl GrepConfig {
    fn build(mut args: impl Iterator<Item = String>) -> Result<GrepConfig, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Fail to get query.")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Fail to get file_path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(GrepConfig {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: GrepConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    // println!("Read content:\n{contents}");
    Ok(())
}

fn main() {
    let args = env::args();
    let config = GrepConfig::build(args).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {err}");
        process::exit(1);
    });
    // println!("Search: {}", config.query);
    // println!("File path: {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Run error: {e}");
    }
}
