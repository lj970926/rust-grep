use std::env;
use std::fs;
use std::process;


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

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = GrepConfig::build(&args).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {err}");
        process::exit(1);
    });
    println!("Search: {}", config.query);
    println!("File path: {}", config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Failed to open file.");
    println!("Read content:\n {contents}");
}
