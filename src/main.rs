use std::env;
use std::error::Error;
use std::fs;
use std::process;

use rust_grep::search;


struct GrepConfig {
    query: String,
    file_path: String,
}

impl GrepConfig {
    fn build(args: &[String]) -> Result<GrepConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(GrepConfig {query, file_path})
    }
}

fn run(config: GrepConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
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
