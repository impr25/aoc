use std::env;
use std::fs;


fn main() {
    
    let args: Vec<String> = env::args().collect();

    let content: String = fs::read_to_string(&args[1]).expect("File Not Found");
    
    println!("Hello, world!{content:?}");
}
