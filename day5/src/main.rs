use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    first_part(&prepared_data);
    //second_part(&prepared_data);
}

fn first_part(raw_data: &Vec<&str>) {
    let data = prepare_map_data(&raw_data);
    let seeds = find_seeds_part_one(&raw_data);
    find_lowest_location(seeds, data);
}

fn second_part(raw_data: &Vec<&str>) {
    let data = prepare_map_data(&raw_data);
    let seeds = find_seeds_part_two(&raw_data);
    find_lowest_location(seeds, data);
}

fn find_lowest_location (seeds: Vec<usize>, data: Vec<Vec<[usize; 3]>>) {
    //println!("The seeds are: {:?}", seeds);
    let mut min_location: usize = usize::MAX;
    for seed in seeds {
        let mut cur_num = seed;
        for maps in &data {
            let mut new_num = cur_num;
            for map in maps {
                let destination = map[0];
                let source = map[1];
                let range = map[2];
                if cur_num >= source && cur_num < source + range {
                    new_num = destination + (cur_num - source);
                    break;
                }
            }
            cur_num = new_num;
        }
        if cur_num < min_location {
            min_location = cur_num;
        }
    }
    println!("The minimum location is: {}", min_location);
}

fn find_seeds_part_two(raw_data : &Vec<&str>) -> Vec<usize>{
    let raw_line = raw_data[0];
    let mut seeds: Vec<usize> = Vec::new();
    let mut current_source_seed = usize::MAX;
    for word in raw_line.split_whitespace() {
        if word.contains("seeds") {
            continue;
        }
        if current_source_seed == usize::MAX {
            current_source_seed = word.parse::<usize>().unwrap();
            continue;
        } else {
            seeds.extend(current_source_seed..(current_source_seed+ word.parse::<usize>().unwrap()));
        }
    }
    return seeds;
}

fn find_seeds_part_one(raw_data : &Vec<&str>) -> Vec<usize>{
    let raw_line = raw_data[0];
    let mut seeds: Vec<usize> = Vec::new();
    for word in raw_line.split_whitespace() {
        if word.contains("seeds") {
            continue;
        }
        seeds.push(word.parse::<usize>().unwrap());
    }
    return seeds;
}

fn prepare_map_data(raw_data: &Vec<&str>) -> Vec<Vec<[usize; 3]>>{
    let mut data: Vec<Vec<[usize; 3]>> = Vec::new();
    let mut cur_map_data: Vec<[usize; 3]> = Vec::new();
    for line in raw_data[1..].iter() {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            if cur_map_data.is_empty() {
                continue;
            }
            data.push(cur_map_data);
            cur_map_data = Vec::new();
            continue;
        }
        let mut line_data: [usize; 3] = [0; 3];
        for (i, word) in line.split_whitespace().enumerate() {
            line_data[i] = word.parse::<usize>().unwrap();
        }
        cur_map_data.push(line_data);
    }
    data.push(cur_map_data);
    return data;
}