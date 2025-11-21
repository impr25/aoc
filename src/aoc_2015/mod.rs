pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub fn run(day: &str, path: &str) {
    match day {
        "01" => day01::run(path),
        "02" => day02::run(path),
        "03" => day03::run(path),
        "04" => day04::run(path),
        "05" => day05::run(path),
        "06" => day06::run(path),
        "07" => day07::run(path),
        "08" => day08::run(path),
        "09" => day09::run(path),
        "10" => day10::run(path),
        "11" => day11::run(path),
        _ => println!("waiting for the solution"),
    }
}
