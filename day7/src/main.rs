use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    part_one(&prepared_data);
    //second_part(&prepared_data);
}
fn part_one(raw_data : &Vec<&str>) {
    let hands_bids = get_hands_and_bids(raw_data.clone());
    let hands_types = get_hands_types_map(raw_data.clone());
    let types_bucket = get_types_bucket(hands_types.clone());
    let mut sorted_types_bucket : HashMap<usize, Vec<&str>> = HashMap::new();
    for (k, mut v) in types_bucket.clone() {
        v = custom_sort(v);
        v.reverse();
        sorted_types_bucket.insert(k, v);
    }
    let mut rank = 1;
    let mut total = 0;
    for i in 1..8 {
        let hands = sorted_types_bucket.get(&i).unwrap();
        println!("i: {}", i);
        println!("Hands: {:?}", hands);
        for j in hands.iter() {
            total += rank * hands_bids.get(j).unwrap();
            rank += 1;
        }
    }
    println!("Total: {}", total);
}

fn custom_sort(mut vec: Vec<&str>) -> Vec<&str> {
    let order = "AKQJT98765432";

    let mut rank_map = std::collections::HashMap::new();
    for (rank, ch) in order.chars().enumerate() {
        rank_map.insert(ch, rank);
    }

    vec.sort_by(|a, b| {
        let mut rank_a = 0;
        let mut rank_b = 0;
        let mut mul = 14*14*14*14;
        for i in 0..5 {
            rank_a += mul * rank_map.get(&a.chars().nth(i).unwrap()).unwrap();
            rank_b += mul * rank_map.get(&b.chars().nth(i).unwrap()).unwrap();
            mul = mul / 14;
        }
        rank_a.cmp(&rank_b)
    });

    vec
}

fn get_types_bucket(hands_types: HashMap<&str, usize>) -> HashMap<usize, Vec<&str>> {
    let mut types_bucket : HashMap<usize, Vec<&str>> = HashMap::new();
    for (hand, hand_type) in hands_types {
        let hands = types_bucket.entry(hand_type).or_insert(Vec::new());
        hands.push(hand);
    }
    types_bucket
}

fn get_hands_types_map(raw_data: Vec<&str>) -> HashMap<&str, usize>{
    let mut hands_types : HashMap<&str, usize> = HashMap::new();
    for line in raw_data {
        let split_line: Vec<&str> = line.split(" ").collect();
        let hand = split_line[0];
        let hand_type = classify_hand(hand);
        hands_types.insert(hand, hand_type);
    }
    hands_types
}


fn get_hands_and_bids(raw_data: Vec<&str>) -> HashMap<&str, usize>{
    let mut hands_bids : HashMap<&str, usize> = HashMap::new();
    for line in raw_data {
        let split_line: Vec<&str> = line.split(" ").collect();
        let name = split_line[0];
        let bid = split_line[1].parse::<usize>().unwrap();
        hands_bids.insert(name, bid);
    }
    hands_bids
}

fn classify_hand(hand: &str) -> usize {
    let mut counts = HashMap::new();

    for ch in hand.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let mut count_values: Vec<_> = counts.values().collect();
    count_values.sort_unstable();

    let hand_type = match count_values.as_slice() {
        [5] => 7,
        [1, 4] => 6,
        [2, 3] => 5,
        [1, 1, 3] => 4,
        [1, 2, 2] => 3,
        [1, 1, 1, 2] => 2,
        [1, 1, 1, 1, 1] => 1,
        _ => 0,
    };
    if hand_type == 0 {
        println!("Unknown hand type: {}", hand);
    }
    hand_type
}
