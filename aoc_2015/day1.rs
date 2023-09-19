use std::io;

fn main() {
    let mut puzzle_in = String::new();
    io::stdin().read_line(&mut puzzle_in).expect("failed to get input");
    println!("{}",puzzle_in);
}