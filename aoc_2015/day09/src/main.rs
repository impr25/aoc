use std::env;
use std::fs;
use std::collections::{HashSet, HashMap};

const MAX: u16 = u16::MAX;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Route<'a> {
    orig: &'a str,
    dest: &'a str,
}

impl <'a> Route<'a> {
    fn new(city_1: &'a str, city_2: &'a str) -> Self {
        Self {
                orig: city_1,
                dest: city_2,
        }
    }
}

fn get_permution(array: Vec<&str>) -> Vec<Vec<&str>> {
    match array.len() {
        0 | 1 => vec![array],
        2 => {
            let first = array[1];
            let second = array[0];
            vec![array, vec![first, second]]
        },
        _ => {
            let mut comb: Vec<Vec<&str>> = vec![];
            for i in 0..array.len() {
                let mut array_c = array.to_vec();
                let first = array_c[i];
                array_c.remove(i);
                for mut perm in get_permution(array_c) {
                    perm.insert(0, first);
                    comb.push(perm);
                }
            }  
            comb
        }, 
    }
}

fn result (routes: HashMap<Route, u16>, city_arr: Vec<&str>) -> (u16,u16) {
    let comb = get_permution(city_arr);
    let mut min_dist = MAX;
    let mut max_dist = 0;
    for route in comb {
        let mut dist: u16 = 0;
        for i in 0..route.len()-1 {
            let c1 = route[i];
            let c2 = route[i+1];
            let rt = Route::new(c1, c2);
            // println!("route ======> {rt:?},\n ");
            dist += routes.get(&rt).unwrap();
        }
        if dist<min_dist {min_dist = dist;}
        if dist>max_dist {max_dist = dist;}
    }
    return (min_dist,max_dist)
}
fn main() {

    let args: Vec<String> = env::args().collect();

    let contents: String = fs::read_to_string(&args[1]).expect("File not Found\n");

    let mut city_set: HashSet<&str> = HashSet::new();
  
    let mut routes: HashMap<Route, u16> = HashMap::new();

    for line in contents.split("\n") {
        let line_arr: Vec<&str> = line.trim().split(" ").collect();
        routes.insert(Route::new(line_arr[2], line_arr[0]), line_arr[4].parse::<u16>().unwrap());
        routes.insert(Route::new(line_arr[0], line_arr[2]), line_arr[4].parse::<u16>().unwrap());     
        city_set.insert(line_arr[0]);
        city_set.insert(line_arr[2]);        
    }

    let mut city_arr = vec![];
    for city in city_set {
        city_arr.push(city);
    }

    // println!("Hello, world! {routes:?},\n {city_arr:?}");
    let (res1,res2) = result(routes, city_arr);
    println!("answer is {res1} and 2nd answer is {res2}");
}