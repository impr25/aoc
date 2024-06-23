use std::env;
use std::fs;

fn calculate_area(dim: &mut Vec<i64>) -> (i64,i64) {
    dim[..].sort();
    // println!("{}", dim[0]);
    return (3*dim[0]*dim[1] + 2*dim[1]*dim[2] + 2*dim[2]*dim[0], 2*(dim[0]+dim[1])+dim[0]*dim[1]*dim[2]);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path= &args[1];
    
    let contents = fs::read_to_string(file_path).expect("File not found");

    let mut area: i64 = 0;
    let mut length: i64 = 0;


    for line in contents.split('\n') {
        // println!("{}", line);

        // let dim: Vec<&[u8]> = line.split('x').map(|s| s.as_bytes()).collect();
        // println!("Print is ASCII: {:?},{:?},{:?}",dim[0],dim[1],dim[2]);

        let mut dim: Vec<i64> = line.split('x').map(|s| s.trim().parse::<i64>().unwrap()).collect();
        area += calculate_area(&mut dim).0;
        length += calculate_area(&mut dim).1;
        // println!("{:?},{:?},{:?}",dim[0],dim[1],dim[2])
        
    }
    println!("Total Giftwrap needed: {}",area);
    println!("Total ribbon needed: {}",length);
}
