use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path= &args[1];
    
    let contents = fs::read_to_string(file_path).expect("File not found");
    let mut answer: i32 = 0;
  
    // for ind in 0..contents.chars().count(){
    //     if answer == -1{
    //         println!("negative ind {}",ind);
    //     }
    //     if contents.chars().nth(ind).unwrap() =='(' {
    //         answer += 1;
    //     }
    //     else if contents.chars().nth(ind).unwrap() == ')' {
    //         answer -= 1;            
    //     }
    //     else {
    //         println!("answer {}", answer)
    //     }
    // }

    for (ind,char) in contents.chars().enumerate(){
        if answer == -1{
            println!("Floor Goes to -1 at: {}", ind);
        }
        if char == '('{
            // print!("{}",char);
            answer += 1;
        }
        else if char == ')'{
            // print!("{}",char);
            answer -= 1;
        }
        else {
            println!("New line occured, Previous Answer: {}", answer);
        }
    }

    println!("New line occured, Previous Answer: {}", answer);
    println!("Current file path {}", file_path);
    // println!("File contains \n{}", contents);
}
