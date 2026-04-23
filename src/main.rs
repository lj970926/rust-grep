use std::env;
use std::fs;


struct GrepConfig {
    query: String,
    file_path: String,
}

impl GrepConfig {
    fn new(args: &[String]) -> GrepConfig {
        let query = args[1].clone();
        let file_path = args[2].clone();
        GrepConfig {query, file_path}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = GrepConfig::new(&args);
    println!("Search: {}", config.query);
    println!("File path: {}", config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Failed to open file.");
    println!("Read content:\n {contents}");
}
