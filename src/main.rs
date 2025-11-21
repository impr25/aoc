mod aoc_2015;
mod aoc_2024;

use std::env;
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

    // Pad day with leading zero if needed
    let day_padded = if day.len() == 1 {
        format!("0{}", day)
    } else {
        day.to_string()
    };

    let file_path = format!("src/aoc_{}/day{}/{}.txt", year, day_padded, mode);

    println!("Running Year: {}, Day: {}, Mode: {}", year, day, mode);
    println!("Input file: {}", file_path);

    match year.as_str() {
        "2015" => match day_padded.as_str() {
            "01" => aoc_2015::day01::run(&file_path),
            "02" => aoc_2015::day02::run(&file_path),
            "03" => aoc_2015::day03::run(&file_path),
            "04" => aoc_2015::day04::run(&file_path),
            "05" => aoc_2015::day05::run(&file_path),
            "06" => aoc_2015::day06::run(&file_path),
            "07" => aoc_2015::day07::run(&file_path),
            "08" => aoc_2015::day08::run(&file_path),
            "09" => aoc_2015::day09::run(&file_path),
            "10" => aoc_2015::day10::run(&file_path),
            "11" => aoc_2015::day11::run(&file_path),
            _ => eprintln!("Day {} not implemented for 2015", day),
        },
        "2024" => match day_padded.as_str() {
            "01" => aoc_2024::day01::run(&file_path),
            _ => eprintln!("Day {} not implemented for 2024", day),
        },
        _ => eprintln!("Year {} not implemented", year),
    }
}
