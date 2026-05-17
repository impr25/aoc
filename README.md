# Advent of Code (AoC) Solutions in Rust

This repository contains my solutions for Advent of Code, structured as a single Cargo project.

## Running existing solutions

You can run any solution by passing the year, day, and an optional mode (`test` or `final`/`input`):

```bash
cargo run -- <year> <day> [mode]
```

- `<year>`: The 4-digit year (e.g., `2015`, `2024`)
- `<day>`: The day number (e.g., `1` for day 1, `25` for day 25)
- `[mode]`: (Optional) The name of the input file to run against, without the `.txt` extension. Defaults to `test`. Common modes are `test` and `final` (or `input`).

**Examples**:
```bash
# Runs year 2015, day 1 using test.txt
cargo run -- 2015 1 

# Runs year 2015, day 1 using test.txt
cargo run -- 2015 1 test 

# Runs year 2015, day 1 using final.txt (or input.txt)
cargo run -- 2015 1 final 
```

---

## Adding a New Solution for an Existing Year

To add a solution for a new day in a year that already exists (e.g., Day 12 of 2015):

1. **Create the Day Directory**: Navigate to the specific year's folder (`src/aoc_YYYY`) and create a new folder named `dayXX` (padded with a leading zero, e.g., `day12`).
2. **Create Required Files**: Inside the `dayXX` folder, create:
   - `mod.rs`
   - `test.txt` (for example inputs)
   - `final.txt` or `input.txt` (for the real puzzle input)
3. **Implement the Solution**: In `dayXX/mod.rs`, write your solution logic. It must expose a `run(file_path: &str)` function:
   ```rust
   use std::fs;

   pub fn run(file_path: &str) {
       let contents = fs::read_to_string(file_path).expect("File not found");
       
       // Add your solution logic here

       println!("Part 1 Answer:");
       println!("Part 2 Answer:");
   }
   ```
4. **Register the Day**: Open the year's module file (e.g., `src/aoc_YYYY/mod.rs`) and do two things:
   - Declare the module at the top: `pub mod dayXX;`
   - Add it to the `run` method's match statement:
     ```rust
     "XX" => dayXX::run(path),
     ```
     *(Make sure to match the padded string, e.g., `"12"`, `"09"`)*

---

## Adding a New Year

To add the first solution for an entirely new year (e.g., 2025):

1. **Create the Year Directory**: Under `src/`, create a new directory named `aoc_YYYY` (e.g., `aoc_2025`).
2. **Create the Year Module**: Give this directory a `mod.rs` file (`src/aoc_YYYY/mod.rs`). Set it up to route days:
   ```rust
   pub mod day01;

   pub fn run(day: &str, path: &str) {
       match day {
           "01" => day01::run(path),
           _ => println!("waiting for the solution"),
       }
   }
   ```
3. **Create the First Day**: Inside the new `aoc_YYYY` directory, create a `day01` folder and follow the steps from "Adding a New Solution for an Existing Year" to add `mod.rs`, `test.txt`, etc.
4. **Register the Year**: Open `src/main.rs`:
   - Declare the new year module at the top of the file: `mod aoc_YYYY;`
   - Update the `match year.as_str()` block inside the `main` function to route to your new year:
     ```rust
     "YYYY" => aoc_YYYY::run(&day_padded, &file_path),
     ```
