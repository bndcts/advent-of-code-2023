use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    //let split_contents: Vec<Vec<char>> = contents.split("\n").collect::<Vec<Vec<char>>>();
    let prepared_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    first_puzzle(prepared_data.clone());
}

fn first_puzzle(data: Vec<Vec<char>>) {
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
                    }
                    cur_string.clear()
                }
            } else {
                if !cur_string.is_empty() {
                    let num = cur_string.parse::<i32>().unwrap();
                    total += num;
                    cur_string.clear();
                }
            }
        }
        if !cur_string.is_empty() {
            if check_neighbors(data.clone(), cur_left_x, data[y].len()-1, cur_y) {
                let num = cur_string.parse::<i32>().unwrap();
                total += num;
            }
            cur_string.clear();
        }
    }
    println!("Total: {}", total);
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
}