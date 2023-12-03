use std::fs;
use std::collections::HashMap;
use substring::Substring;
use std::io::prelude::*;

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    first_puzzle(split_contents.clone());
    let parsed_contents = parse_strings_to_numbers(split_contents);
    // write parsed contents to file
    let mut file = fs::File::create("src/parsed_input.txt").expect("Unable to create file");
    for line in parsed_contents.clone() {
        file.write_all(line.as_bytes()).expect("Unable to write data");
        file.write_all("\n".as_bytes()).expect("Unable to write data");
    }
    let alter_parsed_contents = parsed_contents.iter().map(|s| &**s).collect();
    first_puzzle(alter_parsed_contents);
}

fn first_puzzle(split_contents: Vec<&str>) {
    let mut total: usize = 0;
    for line in split_contents {
        let mut first: char= 'a';
        let mut second: char = 'b';
        for c in line.chars() {
            if c.is_numeric() {
                if first == 'a' {
                    first = c;
                    second = c;
                } else {
                    second = c;
                }
            }
        }
        let mut res_string = String::new();
        res_string.push(first);
        res_string.push(second);
        let num: usize = res_string.parse().expect("Should have been able to parse");
        total += num;
    }
    println!("Total:\n{total}");
}

fn parse_strings_to_numbers(split_contents: Vec<&str>) -> Vec<String>{
    let map_of_numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut result: Vec<String> = Vec::new();
    for line in split_contents{
        let mut current_line = String::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_line.push(c);
                continue;
            }
            for possible_number in map_of_numbers.keys() {
                let split_string = line.substring(i, i+5);
                if split_string.contains(possible_number) {
                    if split_string.substring(1, 5).contains(possible_number) {
                        continue;
                    }
                    current_line.push(*map_of_numbers.get(possible_number).expect("Should have been able to get"));
                }
            }
        }
        result.push(current_line);
    }
    return result;
}
