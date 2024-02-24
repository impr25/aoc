// use std::io;

// fn main() {
//     let mut puzzle_in = String::new();
//     io::stdin().read_line(&mut puzzle_in).expect("failed to get input");
//     println!("{}",puzzle_in);
// }

fn main() {
    let width1 = 30;
    let height1 = 50;
    let mut puzzle_in = Sting::new();

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

