use std::fs;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    //part_one(&prepared_data);
    part_two(&prepared_data);
}

fn part_one(raw_data : &Vec<&str> ) {
    let map = get_map(raw_data.clone());
    let instructions = raw_data[0];
    //println!("map: {:?}", map);
    let mut cur_str : &str = "AAA";
    let mut cur_index : usize = 0;
    let mut counter = 0;
    while cur_str != "ZZZ" {
        let (left, right) = map.get(cur_str).unwrap();
        if cur_index == instructions.len() {
            cur_index = 0;
        }
        if instructions.chars().nth(cur_index).unwrap() == 'R'  {
            cur_str = right;
        } else {
            cur_str = left;
        } 
        counter += 1;
        cur_index += 1;
    }
    println!("counter: {}", counter);
}

fn calculate_path(map : &HashMap<&str, (&str, &str)>, instructions : &str, cur_string: &str, part_two: bool) -> usize {
    let mut cur_index : usize = 0;
    let mut counter = 0;
    let mut cur_str = cur_string;

    while cur_str.chars().nth(2).unwrap() != 'Z' && (part_two || cur_str == "AAA") {
        let (left, right) = map.get(cur_str).unwrap();
        if cur_index == instructions.len() {
            cur_index = 0;
        }
        if instructions.chars().nth(cur_index).unwrap() == 'R'  {
            cur_str = right;
        } else {
            cur_str = left;
        } 
        counter += 1;
        cur_index += 1;
    }
    counter
}

fn part_two(raw_data: &Vec<&str>) {
    let map = get_map(raw_data.clone());
    let instructions = raw_data[0];
    let mut cur_strings : Vec<&str> = Vec::new();
    for (k, _) in map.iter() {
        if k.chars().nth(2).unwrap() == 'A' {
            cur_strings.push(k);
        }
    }
    let mut counts = Vec::new();
    for i in cur_strings.iter() {
        counts.push(calculate_path(&map, instructions, i, true));
    }
    let res = counts.iter().fold(1, |a, &b| lcm(a, b));
    println!("res: {}", res);
}

fn get_map(raw_data : Vec<&str>) -> HashMap<&str, (&str, &str)> {
    let mut map : HashMap<&str, (&str, &str)> = HashMap::new();
    for line in raw_data.iter() {
        if line.contains("=") {
            let split_line = line.split(" = ").collect::<Vec<&str>>();
            let key = split_line[0];
            let left_val = &split_line[1][1..4];
            let right_val = &split_line[1][6..9];
            map.insert(key, (&left_val, &right_val));
        }
    }
    map
}