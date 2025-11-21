pub mod day01;

pub fn run(day: &str, path: &str) {
    match day {
        "01" => day01::run(path),
        _ => println!("waiting for the solution"),
    }
}
