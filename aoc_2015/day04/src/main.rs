use std::env;
use std::fs;
use md5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("File Not Found\n");
    
    println!("Hello, world!\n{:?}", &args[1]);

    for line in content.split('\n'){
        let mut i:u64 = 0;
        loop {
            let mut code: String = line.trim().to_owned();
            code.push_str(&i.to_string());
            // println!("{}",code);
            let hash = md5::compute(code);
            let hash_str = format!("{:x}", hash);
            if &hash_str[..6] == "000000"{
                println!("answer = {}", i);
                break
            }
            if std::u64::MAX == i{
                println!("max reached = {}", i);
                break
            }
            // println!("{}",&hash_str[..5]);
            i+=1;
        }

    }
}
