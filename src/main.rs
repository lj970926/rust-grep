use std::env;
use std::fs;


struct GrepArgs {
    query: String,
    file_path: String,
}


fn parse_args(args: &[String]) -> GrepArgs {
    let query = args[1].clone();
    let file_path = args[2].clone();
    GrepArgs {query, file_path}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);
    println!("Search: {}", config.query);
    println!("File path: {}", config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Failed to open file.");
    println!("Read content:\n {contents}");
}
