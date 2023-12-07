use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    //first_part(&prepared_data);
    //second_part_bin_search(&prepared_data);
    part_two_dif(&prepared_data);
}

fn first_part(raw_data: &Vec<&str>) {
    let mut data: Vec<Vec<&str>> = Vec::new();
    for line in raw_data.iter() {
        let split_line: Vec<&str> = line.split(" ").collect();
        data.push(split_line[1..].to_vec());
    }
    let times : Vec<usize> = data[0].iter().filter_map(|x| x.parse::<usize>().ok()).collect();
    let distances : Vec<usize> = data[1].iter().filter_map(|x| x.parse::<usize>().ok()).collect();
    let mut total = 0;
    for i in 0..times.len() {
        let distance = distances[i];
        let time = times[i];
        let mut amount_of_possibilities = 0;
        let mut max_reached = false;
        for j in 1..time {
            let covered_distance = (time-j) * j;
            if covered_distance >= distance {
                amount_of_possibilities += 1;
                max_reached = true;
            }
            if max_reached && covered_distance < distance {
                break;
            }
        }
        if total == 0 {
            total = amount_of_possibilities;
        } else {
            total *= amount_of_possibilities;
        
        }
    }
    println!("Total: {}", total);
}

fn second_part_brute_force(raw_data: &Vec<&str>) {
    let time : usize = raw_data[0].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).product();
    let distance : usize = raw_data[1].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).product();
    println!("Time: {}, Distances: {}", time, distance);
    let mut left = 1;
    let mut right = time;
    let mut result = 0;
    while right > left {
        let left_distance = (time - left) * left;
        let right_distance = right * (time-right);
        if left_distance < distance {
            left += 1;
        }
        if right_distance < distance {
            right -= 1;
        }
    }
    let total = right - left;
    println!("Total: {}", total);
}

fn second_part_bin_search(raw_data: &Vec<&str>) {
    let time : usize = raw_data[0].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).product();
    let distance : usize = raw_data[1].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).product();
    let mut left = 1;
    let mut right = time-1;
    let mut left_mid = (left + right) / 2;
    let mut right_mid = (left + right) / 2;
    loop {
        if left_go_up(left, time, distance) {
            left = left + (left_mid-left)/2;
        } else {
            left_mid = left_mid - (left_mid-left)/2; 
        }
        if right_go_down(right, time, distance) {
            right = right - (right-right_mid)/2;
        } else {
            right_mid = right_mid + (right-right_mid)/2;
        }
        if left == left_mid - 1 && right == right_mid + 1 {
            let left_distance = (time - left) * left;
            let right_distance = right * (time-right);
            println!("time: {}, distance: {}", time, distance);
            println!("Left: {}, Right: {}, Left Mid: {}, Right Mid: {}", left, right, left_mid, right_mid);
            println!("Left Distance: {}, Right Distance: {}", left_distance, right_distance);
            break;
        }
    }
    let total = right - left;
    println!("Total: {}", total);
}

fn part_two_dif(raw_data: &Vec<&str>) {
    let time : usize = raw_data[0].split_whitespace().filter(|x| x.chars().all(char::is_numeric)).collect::<String>().parse().unwrap();
    let distance: usize = raw_data[1].split_whitespace().filter(|s| s.chars().all(char::is_numeric)).collect::<String>().parse().unwrap();
    let mut left = 1;
    let mut right = time-1;
    let mut mid = (left + right) / 2;
    while !(is_enough(mid, time, distance) && !is_enough(mid+1, time, distance)){
        if is_enough(mid, time, distance) {
            left = mid;
        } else {
            right = mid;
        }
        mid = (left + right) / 2;
    }
    let right_border = mid;
    right = right_border;
    left = 1;
    mid = (left + right) / 2;
    while !(is_enough(mid, time, distance) && !is_enough(mid-1, time, distance)){
        if is_enough(mid, time, distance) {
            right = mid;
        } else {
            left = mid;
        }
        mid = (left + right) / 2;
    }
    println!("Answer: {}", right_border - mid + 1);
}

fn is_enough(point : usize, time : usize, distance : usize) -> bool {
    if (time - point) * point >= distance {
        true
    } else {
        false
    }
}
fn left_go_up(left: usize, time:usize, distance:usize) -> bool {
    if (time - left) * left < distance {
        true
    } else {
        false
    }
}

fn right_go_down(right:usize, time : usize, distance : usize) -> bool{
    if right * (time - right) < distance {
        true
    } else {
        false
    }
}
