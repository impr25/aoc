use std::env;
use std::fs;
use std::collections::HashMap;
// use std::process::exit;

fn find_value<'a>(signal_map: &HashMap<&'a str,&'a str>, value_map: &mut HashMap<&'a str,u16>, signal: &'a str) -> u16 {
    if let Some(i) = value_map.get(signal) {
        return *i;
    }

    let expr: Vec<&str>= signal_map.get(signal).unwrap().split(" ").collect();
    // println!("Expression {} = {:?}",signal, expr);

    if expr.len() == 1 {
        if let Ok(val) = expr[0].parse::<u16>() {
            value_map.insert(signal, val);
            return val;
        }
        let val = find_value(signal_map, value_map, &expr[0]);
        value_map.insert(signal,val);
        return val;
    }

    if expr[0] == "NOT" {
        let val = !find_value(signal_map, value_map, &expr[1])  ;
        value_map.insert(signal,val);
        return val;
    }
    else if expr[1] == "AND" {
        let val_1: u16 = if let Ok(val) = expr[0].parse::<u16>() {
            val
        }
        else {
            find_value(signal_map, value_map, &expr[0])
        };

        let val_2: u16 = if let Ok(val) = expr[2].parse::<u16>() {
            val
        }
        else {
            find_value(signal_map, value_map, &expr[2])
        };
        
        let val = val_1 & val_2;
        value_map.insert(signal,val);
        return val;
    }
    else if expr[1] == "OR" {
        let val_1: u16 = if let Ok(val) = expr[0].parse::<u16>() {
            val
        }
        else {
            find_value(signal_map, value_map, &expr[0])
        };

        let val_2: u16 = if let Ok(val) = expr[2].parse::<u16>() {
            val
        }
        else {
            find_value(signal_map, value_map, &expr[2])
        };

        let val = val_1 | val_2;
        value_map.insert(signal,val);
        return val;
    }
    else if expr[1] == "RSHIFT" {
        let val_1 = find_value(signal_map, value_map, &expr[0]);
        let val = val_1 >> expr[2].parse::<u16>().unwrap();
        value_map.insert(signal,val);
        return val;
    }
    else if expr[1] == "LSHIFT" {
        let val_1 = find_value(signal_map, value_map, &expr[0]);
        let val = val_1 << expr[2].parse::<u16>().unwrap();
        value_map.insert(signal,val);
        return val;
    }
    else {
        println!("Error Occured somewhere {}", signal);
        return 0;
    }


    //SIMPLIFIED VERSION TRIAL

    // let result = match expr.as_slice() {
    //     // Direct assignment (either a number or another signal)
    //     [single] => single.parse::<u16>().unwrap_or_else(|_| find_value(signal_map, value_map, single)),

    //     // NOT operation
    //     ["NOT", operand] => !find_value(signal_map, value_map, operand) & 0xFFFF,

    //     // AND, OR, LSHIFT, RSHIFT operations
    //     [left, "AND", right] => resolve_binary_op(signal_map, value_map, left, right, |a, b| a & b),
    //     [left, "OR", right] => resolve_binary_op(signal_map, value_map, left, right, |a, b| a | b),
    //     [left, "LSHIFT", right] => resolve_binary_op(signal_map, value_map, left, right, |a, b| a << b),
    //     [left, "RSHIFT", right] => resolve_binary_op(signal_map, value_map, left, right, |a, b| a >> b),

    //     _ => panic!("Unknown expression for signal: {}", signal),
    // };

}


fn main() {
    let args: Vec<String> = env::args().collect();

    let contents: String = fs::read_to_string(&args[1]).expect("File Not Found");

    let mut signal_map: HashMap<&str, &str> = HashMap::new();
    let mut value_map: HashMap<&str, u16> = HashMap::new();

    for line in contents.split('\n'){
        let arr: Vec<&str> = line.trim().split(" -> ").collect();
        signal_map.insert(arr[1].trim(), arr[0].trim());
    }
    // let letter = &args[2];
    let answer = find_value(&signal_map, &mut value_map, &args[2]);

    // println!("Hello, world! {} {:?}", answer, signal_map);
    // println!("Hello, world! {} \n {:?}", answer, value_map);
    println!("Hello, world! first answer is {}", answer);
    let answer_str: &str = &answer.to_string();
    signal_map.insert("b", answer_str);
    value_map.clear();

    let answer2 = find_value(&signal_map, &mut value_map, &args[2]);
    println!("Hello, world! second answer is {}", answer2);

}
