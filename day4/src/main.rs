use std::fs;
use std::collections::HashSet;


fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    first_part(&prepared_data);
    second_part(&prepared_data);
}

fn prepare_data(data: &Vec<&str>) -> Vec<(HashSet<isize>, HashSet<isize>)>  {
    let mut result : Vec<(HashSet<isize>, HashSet<isize>)> = Vec::new();
    for line in data {
        let first_split = line.split("|").collect::<Vec<&str>>();
        let first_part_only_cards_str = first_split[0].split(":").collect::<Vec<&str>>()[1];

        let first_part_only_cards_str_arr = first_part_only_cards_str.split(" ").collect::<Vec<&str>>();

        let first_part : HashSet<isize> = first_part_only_cards_str_arr
            .into_iter()
            .filter_map(|x| x.parse::<isize>().ok())
            .collect();
        
        let second_part_only_cards_str_arr = first_split[1].split(" ").collect::<Vec<&str>>();

        let second_part : HashSet<isize> = second_part_only_cards_str_arr
            .into_iter()
            .filter_map(|x| x.parse::<isize>().ok())
            .collect();

        result.push((first_part, second_part));
    }
    return result;
}

fn first_part(raw_data: &Vec<&str>) {
    let data = prepare_data(raw_data);
    let mut total : isize = 0;
    for (left, right) in data {
        let mut score: isize = 0;
        for i in left {
            if right.contains(&i) {
                if score == 0 {
                    score = 1;
                } else {
                    score = score * 2;
                }
            }
        }
        total +=score;
    }
    println!("First part: {}", total);
}

fn second_part(raw_data: &Vec<&str>) {
    let data = prepare_data(raw_data);
    let mut amount_of_cards : [usize; 205] = [1; 205];
    for (n, (left, right)) in data.iter().enumerate() {
        let mut score: isize = 0;
        for i in left {
            if right.contains(&i) {
                score += 1;
                }
        }
        for i in 1..score+1 {
            let proper_i = i as usize;
            amount_of_cards[n+proper_i] += amount_of_cards[n] * 1;
        }
    }
    let score : usize = amount_of_cards.iter().sum();
    println!("Second part: {}", score);
}