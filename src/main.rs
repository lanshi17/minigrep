//main.rs
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    println!("with text:\n{contents}");
}


trait Parse {
    fn parse_config(args: &[String]) -> Self;
}
struct Config {
    query: String,
    file_path: String,
}
impl Parse for Config {
    fn parse_config(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Self { query, file_path }
    }
}