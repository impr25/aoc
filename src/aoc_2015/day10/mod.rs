use std::env;
use std::fs;


pub fn run(file_path: &str) {
    
    // let args: Vec<String> = env::args().collect();

    let content: String = fs::read_to_string(file_path).expect("File Not Found");
    
    println!("Hello, world!{content:?}");
}
