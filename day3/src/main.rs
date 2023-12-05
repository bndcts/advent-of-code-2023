use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    //let split_contents: Vec<Vec<char>> = contents.split("\n").collect::<Vec<Vec<char>>>();
    let prepared_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let parts_map = first_puzzle(prepared_data.clone());
    part_two(prepared_data.clone(), parts_map);
}

fn first_puzzle(data: Vec<Vec<char>>) -> HashMap<(usize,usize), i32>{
    let mut parts_map: HashMap<(usize,usize), i32> = HashMap::new();
    let mut cur_string: String = String::new();
    let mut cur_left_x = 0;
    let mut cur_y = 0;
    let mut total = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let c = *c;
            if c.is_digit(10) {
                if cur_string.is_empty() {
                    cur_left_x = x;
                    cur_y = y;
                }
                cur_string.push(c);
            } else if c == '.' {
                if !cur_string.is_empty() {
                    let cur_right_x = x-1;
                    if check_neighbors(data.clone(), cur_left_x, cur_right_x, cur_y) {
                        let num = cur_string.parse::<i32>().unwrap();
                        total += num;
                        parts_map.insert((cur_left_x, cur_y), num);
                    }
                    cur_string.clear()
                }
            } else {
                if !cur_string.is_empty() {
                    let num = cur_string.parse::<i32>().unwrap();
                    total += num;
                    parts_map.insert((cur_left_x, cur_y), num);
                    cur_string.clear();
                }
            }
        }
        if !cur_string.is_empty() {
            if check_neighbors(data.clone(), cur_left_x, data[y].len()-1, cur_y) {
                let num = cur_string.parse::<i32>().unwrap();
                total += num;
                parts_map.insert((cur_left_x, cur_y), num);
            }
            cur_string.clear();
        }
    }
    println!("Total: {}", total);
    return parts_map;
}

fn part_two(data : Vec<Vec<char>>, parts_map: HashMap<(usize,usize), i32>) {
    let mut total = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let c = *c;
            if c == '*' {
                let adjacent_parts = adjacent_parts(&data, x, y);
                if adjacent_parts.len() == 2 {
                    let first_num = adjacent_parts[0];
                    let second_num = adjacent_parts[1];
                    total += parts_map[&first_num] * parts_map[&second_num];
                }
            }
        }
    } 
    println!("Total part 2: {}", total);
}

fn adjacent_parts(data: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut adjacent_parts: HashSet<(usize, usize)> = HashSet::new();
    for (add_x, add_y) in adjacent_neighbors.iter() {
        if x == 0 && *add_x == -1 {
            continue;
        }
        if y == 0 && *add_y == -1 {
            continue;
        }
        let new_x = (x as i32 + add_x) as usize;
        let new_y = (y as i32 + add_y) as usize;
        if is_point_in_bounds(data[0].len(), data.len(), new_x, new_y) {
            if data[new_y][new_x].is_digit(10) {
                let mut iter_x = new_x;
                while iter_x > 0 {
                    if data[new_y][iter_x-1].is_digit(10) {
                        iter_x -= 1;
                    } else {
                        break;
                    }
                }
                adjacent_parts.insert((iter_x, new_y));
                //if parts_map.contains_key(&(iter_x, new_y)) {
                 //   adjacent_parts.insert((iter_x, new_y));
                //}
            }
        }
    }
    // return adjacent parts ad Vec
    return adjacent_parts.into_iter().collect::<Vec<(usize,usize)>>();
}

fn is_point_in_bounds(max_x: usize, max_y: usize, x: usize, y: usize) -> bool {
    if y >= max_y{
        return false;
    }
    if x >= max_x{
        return false;
    }
    return true;
}
fn check_neighbors (data: Vec<Vec<char>>, left_x: usize, right_x : usize, y: usize) -> bool {
    fn is_symbol(c: char) -> bool {
        if ['*', '@', '/', '-', '=', '#', '%', '$', '&', '+'].contains(&c) {
            return true;
        }
        return false;
    }
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for x in left_x..right_x+1 {
        for (add_x, add_y) in adjacent_neighbors.iter() {
            if x == 0 && *add_x == -1 {
                continue;
            }
            if y == 0 && *add_y == -1 {
                continue;
            }
            let new_x = (x as i32 + add_x) as usize;
            let new_y = (y as i32 + add_y) as usize;
            if is_point_in_bounds(data[y].len(), data.len(), new_x, new_y) {
                if is_symbol(data[new_y][new_x]) {
                    return true;
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_check_neighbours() {
        let data1 = vec![
            vec!['1','.', '.'],
            vec!['.','.','.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(check_neighbors(data1, 0, 1, 0), false);
    }
    #[test]
    fn test_check_neighbours2() {
        let data1 = vec![
            vec!['/','.', '.'],
            vec!['.','1','.'],
            vec!['.', '.', '.'],
        ];
        assert_eq!(check_neighbors(data1, 1, 1, 1), true);
    }

    #[test]
    fn test_tuple_key() {
        let mut map : HashMap<(usize, usize), i32> = HashMap::new();
        map.insert((1,2), 1);
        assert_eq!(map.contains_key(&(1,2)), true);
    }
}