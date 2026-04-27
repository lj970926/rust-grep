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
    fn build(args: &[String]) -> Result<GrepConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(GrepConfig {query, file_path, ignore_case})
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
    let args: Vec<String> = env::args().collect();
    let config = GrepConfig::build(&args).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {err}");
        process::exit(1);
    });
    // println!("Search: {}", config.query);
    // println!("File path: {}", config.file_path);
    if let Err(e) = run(config) {
        println!("Run error: {e}");
    }
}
