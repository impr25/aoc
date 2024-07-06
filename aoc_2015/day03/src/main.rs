use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("First Argument of program :{}", &args[0]);

    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("File Not Found \n");
    for line in content.split('\n'){
        // let mut houses: i32 = 1;
        let mut x_dir_s: i32 = 0;
        let mut y_dir_s: i32 = 0;
        let mut x_dir_r: i32 = 0;
        let mut y_dir_r: i32 = 0;
        let mut coord: HashSet<(i32,i32)> = HashSet::new();
        coord.insert((x_dir_r,y_dir_r));
        coord.insert((x_dir_s,y_dir_s));
        for (ind,char) in line.chars().enumerate(){
            if ind % 2 == 0 {
                match char {
                    '^' => y_dir_s += 1,
                    'v' => y_dir_s -= 1,
                    '>' => x_dir_s += 1,
                    '<' => x_dir_s -= 1,
                    _ => continue,
                }
                coord.insert((x_dir_s,y_dir_s));
            }
            else {
                match char {
                    '^' => y_dir_r += 1,
                    'v' => y_dir_r -= 1,
                    '>' => x_dir_r += 1,
                    '<' => x_dir_r -= 1,
                    _ => continue,
                }
                coord.insert((x_dir_r,y_dir_r));
            }
            
        }
        // println!("{}, {}",coord_s.len(), coord_r.len());
        let houses= coord.len();
        println!("{}", houses);
    }
    println!("{}",file_path);
    println!("Hello, world!");
}
