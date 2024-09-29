use std::env;
use std::fs;
use regex::Regex;

fn part1 (contents: String){
    let mut grid: [[bool;1000];1000] = [[false;1000];1000];
    let mut lights: i32 = 0; 

    for line in contents.split('\n'){
        let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
        if let Some(captures) = re.captures(line){
            let x1: usize = captures[1].parse().unwrap();
            let y1: usize = captures[2].parse().unwrap();
            let x2: usize = captures[3].parse().unwrap();
            let y2: usize = captures[4].parse().unwrap();
        
        if line.starts_with("turn on"){
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    if grid[i][j] {
                        continue;
                    }
                    else {
                        grid[i][j] = true;
                        lights += 1;
                    }
                }
            }
            // println!("light = {}", lights);
        }
        else if line.starts_with("turn off") {
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    if grid[i][j] {
                        grid[i][j] = false;
                        lights -= 1;
                    }
                    else {
                        continue;
                    }
                }
            }
            // println!("light = {}", lights);
        }
        else if line.trim().starts_with("toggle"){
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    if grid[i][j] {
                        grid[i][j] = false;
                        lights -= 1;
                    }
                    else {
                        grid[i][j] = true;
                        lights += 1;
                    }
                }
            }
            // println!("light = {}", lights);
        }
        else {
            continue
        }
        }
    }
    println!("Hello, world! the answer is {}", lights);
}

fn part2 (contents : String){
    // let mut grid: [[u16;1000];1000] = [[0;1000];1000]; // You can not use such large array here because its overflow the stack memory 
    let mut grid: Vec<Vec<u16>> =  vec![vec![0;1000];1000];

    for line in contents.split('\n'){
        let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
        if let Some(captures) = re.captures(line){
            let x1: usize = captures[1].parse().unwrap();
            let y1: usize = captures[2].parse().unwrap();
            let x2: usize = captures[3].parse().unwrap();
            let y2: usize = captures[4].parse().unwrap();
        
        if line.starts_with("turn on"){
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    grid[i][j] += 1;
                }
            }
            // println!("ran");
        }
        else if line.starts_with("turn off") {
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    if grid[i][j] == 0 {
                        continue;
                    }
                    else {
                        grid[i][j] -= 1
                    }
                }
            }
            // println!("ran");
        }
        else if line.trim().starts_with("toggle"){
            for i in x1..x2+1 {
                for j in y1..y2+1 {
                    grid[i][j] += 2;
                }
            }
            // println!("ran");
        }
        else {
            continue
        }
        }
    }
    let mut brightness: i64 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            let value = grid[i][j];
            brightness += value as i64;
        }
    }
    println!("Hello, world! the answer is {}", brightness);
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let contents: String = fs::read_to_string(file_path).expect("file not found");

    // part1(contents);
    part2(contents);
    
}
