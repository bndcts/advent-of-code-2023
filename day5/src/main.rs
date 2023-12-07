use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    //first_part(&prepared_data);
    second_part(&prepared_data);
}

fn first_part(raw_data: &Vec<&str>) {
    let data = prepare_map_data(&raw_data);
    let seeds = find_seeds_part_one(&raw_data);
    find_lowest_location(seeds, data);
}

fn second_part(raw_data: &Vec<&str>) {
    let mut min_location: usize = usize::MAX;
    let data = prepare_map_data(&raw_data);
    let seeds = find_seeds_part_two(&raw_data);
    let mut seeds_ranges: Vec<(usize, usize)> = seeds;
    for maps in &data {
        let mut new_seed_ranges : Vec<(usize, usize)> = Vec::new();
        while !seeds_ranges.is_empty() {
            let mut to_add : Vec<(usize, usize)> = Vec::new();
            for seed_range in &seeds_ranges {
                for map in maps {
                    let destination = map[0];
                    let source = map[1];
                    let range = map[2];
                    if source >= seed_range.0 && source < seed_range.0 + seed_range.1 {
                        let diff_source_seed1 = source - seed_range.0;
                        let diff_source_end_seed_end = source + range -1 - (seed_range.0 + seed_range.1 -1);
                        let new_seed_range = (destination+diff_source_seed_1, seed_range.1 + diff_source_end_seed_end);
                        let new_old_seed_range_1 = (seed_range.0, diff_source_seed1);
                        let new_old_seed_range_2 = (source + range-1, seed_range.1 - (source + range-1));
                        if new_old_seed_range_1.1 > 0 {
                            to_add.push(new_old_seed_range_1);
                        }
                        if new_old_seed_range_2.1 > 0 {
                            to_add.push(new_old_seed_range_2);
                        }
                        new_seed_ranges.push(new_seed_range);
                        to_add.push(new_seed_range);
                    
                    }
                }
            }
            seeds_ranges = to_add;
        }
        seeds_ranges = new_seed_ranges;
    }
    for seed_range in &seeds_ranges {
        if seed_range.0 < min_location {
            min_location = seed_range.0;
        }
    }
    println!("The minimum location is: {}", min_location);
}

fn find_seeds_part_two(raw_data: &Vec<&str>) -> Vec<(usize, usize)>{
    let raw_line = raw_data[0];
    let mut seeds: Vec<(usize, usize)> = Vec::new();
    let mut current_source_seed = usize::MAX;
    for word in raw_line.split_whitespace() {
        if word.contains("seeds") {
            continue;
        }
        if current_source_seed == usize::MAX {
            current_source_seed = word.parse::<usize>().unwrap();
            continue;
        } else {
            seeds.push((current_source_seed, word.parse::<usize>().unwrap()));
            current_source_seed = usize::MAX;
        }
    }
    return seeds;
}

fn second_part_brute_force(raw_data: &Vec<&str>) {
    let data = prepare_map_data(&raw_data);
    let seeds = find_seeds_part_two_brute_force(&raw_data);
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

fn find_seeds_part_two_brute_force(raw_data : &Vec<&str>) -> Vec<usize>{
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