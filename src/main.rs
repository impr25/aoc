mod aoc_2015;
mod aoc_2024;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <year> <day> [mode]");
        process::exit(1);
    }

    let year = &args[1];
    let day = &args[2];
    let mode = if args.len() > 3 { &args[3] } else { "test" };

    // Validate day
    let day_num = match day.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid day");
            return;
        }
    };

    if day_num < 1 || day_num > 25 {
        println!("invalid day");
        return;
    }

    // Pad day with leading zero if needed
    let day_padded = if day_num < 10 {
        format!("{:02}", day_num)
    } else {
        day_num.to_string()
    };

    let file_path = format!("src/aoc_{}/day{}/{}.txt", year, day_padded, mode);
    let dir_path = format!("src/aoc_{}/day{}", year, day_padded);

    // Check if directory exists
    if !Path::new(&dir_path).exists() {
        println!("waiting for the solution");
        return;
    }

    println!("Running Year: {}, Day: {}, Mode: {}", year, day_num, mode);
    println!("Input file: {}", file_path);

    match year.as_str() {
        "2015" => aoc_2015::run(&day_padded, &file_path),
        "2024" => aoc_2024::run(&day_padded, &file_path),
        _ => eprintln!("Year {} not implemented", year),
    }
}
