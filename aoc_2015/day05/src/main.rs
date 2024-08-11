use std::env;
use std::fs;


fn vowel_check(str_b: &str) -> bool{
    let mut num_vowel: u16 = 0;
    for char in str_b.chars(){
        match char {
            'a' => num_vowel+=1,
            'e' => num_vowel+=1,
            'i' => num_vowel+=1,
            'o' => num_vowel+=1,
            'u' => num_vowel+=1,
            _ => continue,
        }
    }
    if num_vowel>2{
        return true;
    }
    else {
        return false;
    }
}

fn double_check(str_b: &str) -> bool{
    let mut prev: char = ' ';
    for char in str_b.chars(){
        if prev==char{
            return true;
        }
        else {
            prev = char;
            continue;
        }
    }
    return false;
}

fn restricted_set_check(str_b: &str) -> bool{
    if str_b.contains("ab") | str_b.contains("cd") 
    | str_b.contains("pq") | str_b.contains("xy"){
        return true;
    }
    else {
        return false;
    }
}

fn part2_check1(str_b: &str) -> bool{

    for (ind,char) in str_b.chars().enumerate() {
        if ind == str_b.len()-2{
            break;
        }
        else {
            let slice = &str_b[ind..ind+2];
            if str_b[ind+2..].contains(slice){
                return true;
            }
            else {
                continue;
            }
        }
    }
    return false;
}

fn part2_check2(str_b: &str) -> bool{
    let mut prev1:char = ' ';
    let mut prev2:char = ' ';
    for char in str_b.chars() {
        if char == prev1{
            return true;
        }
        else {
            prev1 = prev2;
            prev2 = char;
        }
    }
    return false
}
fn main() {

    let args:Vec<String> = env::args().collect();

    let file_path: &String = &args[1];
    let content:String = fs::read_to_string(file_path).expect("File not Found\n");

    let mut answer_1:u32 = 0;
    let mut answer_2:u32 = 0;

    for line in content.split("\n"){
        if restricted_set_check(line.trim()){
            // println!("Naughty restricted {}", line);
            continue;
        }
        else if !double_check(line.trim()) {
            // println!("Naughty double {}",line);
            continue;            
        }
        else if !vowel_check(line.trim()) {
            // println!("Naughty vowel {}", line);
            continue;
        }
        else {
            // println!("Nice");
            answer_1+=1;
        }
    }
    println!("Hello, world! and answer of part 1 is {}",answer_1);

    for line in content.split("\n"){
        if !part2_check1(line.trim()) {
            // println!("Naughty check 1 {}",line);
            continue;            
        }
        else if !part2_check2(line.trim()) {
            // println!("Naughty check 2 {}", line);
            continue;
        }
        else {
            // println!("Nice");
            answer_2+=1;
        }
    }
    println!("Hello, world! and answer is {}",answer_2);
}
