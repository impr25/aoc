use std::env;
use std::fs;

const X_UNI: u8 = 120;
const SLASH_UNI: u8 = 92;
const QUOTE_UNI: u8 = 34;

fn part1 (content: &String, method: bool) -> u16 {

// One way to solve the problem
    if method{
        let mut total_char: u16 = 0;
        let mut total_rd_char: u16 = 0;

        for line in content.split("\n") {
            total_char += line.trim().len() as u16;

            let mut escape: bool = false;
            let mut hexa: u8 = 0;
            for char in line.trim().chars() {
                // println!("char = {} and total_rd_char={}",char,total_rd_char);
                if hexa != 0 {
                    hexa -= 1;
                    if hexa == 0 {
                        escape = false;
                        total_rd_char += 1;
                    }
                    continue;
                }
                match char {
                    '"' => {if escape==true{total_rd_char += 1;escape = false;} else {continue}},
                    '\\' => {if escape==true{total_rd_char += 1; escape = false;} else {escape = true;}},
                    'x' => {if escape==true{hexa = 2;}else {total_rd_char += 1;}},
                    ' ' => continue,
                    _ => {if escape==false{total_rd_char += 1;}else {escape = false;}},
                }
            }
        }
        return total_char-total_rd_char;
    }
    // An alternate way to solve using fold in RUST
    else {
        let mut total_char: u16 = 0;
        for line in content.split("\n") {
            let (_,result) = line.trim().bytes().fold((false,0), |(escape,count),b| match (escape,b){
                (true, X_UNI) => (false, count + 3),
                (true, _) => (false,count+1),
                (false,QUOTE_UNI) => (false,count+1),
                (false,SLASH_UNI) => (true,count),
                _ => (false,count),
            });
            total_char += result as u16;
        }
        return total_char;
    }
}

fn part2 (content: &String, method: bool) -> u16 {
    if method {
        let mut total_char: u16 = 0;
        let mut total_rd_char: u16 = 0;

        for line in content.split("\n") {
            total_char += line.trim().len() as u16;
            let new_line = format!("{:?}",line.trim());
            // println!("new line is => {}", new_line);
            total_rd_char += new_line.len() as u16;
        }
        // println!(" total_rd_char {},total_char {} ", total_rd_char,total_char);
        return total_rd_char - total_char;
    }
    else {
        let mut total_char: u16 = 0;
        for line in content.split("\n") {
            // println!("{line}");
            let result: i32 = line.trim().bytes().map(|b| match b{
                QUOTE_UNI => 1,
                SLASH_UNI => 1,
                _ => 0,
            }).sum();
            // println!("result {}", result);
            total_char = total_char + result as u16 + 2;
        }

        return total_char;
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let content: String = fs::read_to_string(&args[1]).expect("File Not Found");

    let method: bool = false;
    println!("Hello, world! answer part1  is => {}", part1(&content,method));
    println!("Hello, world! answer part2 is => {}", part2(&content,method));

    

}
