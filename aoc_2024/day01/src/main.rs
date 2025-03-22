use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect(); 
    
    let content: String = fs::read_to_string(&args[1]).unwrap();

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in content.split("\n") {
        let arr: Vec<&str> = line.trim().split("   ").collect();
        left.push(arr[0].parse::<u32>().unwrap());
        right.push(arr[1].parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();
    
    let mut total:u32 = 0;
    let len = left.len();
    for i in 0..len {
        // println!("total = {total}");
        total += left[i].abs_diff(right[i]);
    }
    
    println!("Hello, world! {total:?} ");
}
