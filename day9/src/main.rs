use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let prepared_data: Vec<&str> = contents
        .lines()
        .collect();
    part_one(&prepared_data);
    //part_two(&prepared_data);
}

fn part_one(raw_data : &Vec<&str> ) {
    let mut total = 0;
    for line in raw_data {
        let pyramid = build_pyramid(line);
        if pyramid.len() < 2 {
            println!("line is empty: {}", line);
            continue;
        }
        total += calculate_history(pyramid);
    }
    println!("total: {}", total);
}

fn build_pyramid(line: &str) -> Vec<Vec<isize>> {
    let mut pyramid = Vec::new();
    let nums = line.split(" ").collect::<Vec<_>>().iter().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let mut diffs = nums;
    pyramid.push(diffs.clone());
    let mut temp_diffs = Vec::new();
    while *diffs.iter().max().unwrap() != 0 || *diffs.iter().min().unwrap() != 0{
        temp_diffs.clear();
        for (n,i) in diffs.iter().enumerate() {
            if n == diffs.len()-1 {
                break;
            }
            temp_diffs.push(diffs[n+1]-i);
        }
        diffs = temp_diffs.clone();
        pyramid.push(diffs.clone());
    }
    pyramid
}

fn calculate_history(pyramid : Vec<Vec<isize>>) -> isize{
    let length = pyramid.len();
    let mut pyramid = pyramid[..length-1].to_vec();
    pyramid.reverse();
    let mut prior_add = 0;
    for line in pyramid.iter_mut() {
        let curr_len = line.len() -1;
        line.push(prior_add + line[curr_len]);
        prior_add = line[curr_len+1];
    }
    let tmp = pyramid[pyramid.len()-1].len();
    let res = pyramid[pyramid.len()-1][tmp-1];
    res
}

// test build pzramid
#[test]
fn test_build_pyramid() {
    let line = "0 3 6 9 12 15";
    let pyramid = build_pyramid(line);
    println!("pyramid: {:?}", pyramid);
    assert_eq!(pyramid, vec![vec![0,3,6,9,12,15], vec![3,3,3,3,3], vec![0,0,0,0]]);
}
