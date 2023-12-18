use std::fs;
use std::collections::HashSet;
use std::cmp::{min, max};

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

fn part_one(data : &Vec<&str>) {
    let universe = expand_universe(&data);
    let mut pair_set : HashSet<((usize, usize), (usize,usize))> = HashSet::new();
    let mut all_galaxies = Vec::new();
    for (y, line) in universe.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                all_galaxies.push((x, y));
            }
        }
    }
    // create pairs
    for (i, galaxy) in all_galaxies.iter().enumerate() {
        for j in i+1..all_galaxies.len() {
            pair_set.insert((*galaxy, all_galaxies[j]));
        }
    }
    let mut total = 0;
    for pair in pair_set {
        let distance = ((pair.0).0 as i32 - (pair.1).0 as i32).abs() + ((pair.0).1 as i32 - (pair.1).1 as i32).abs();
        total += distance;
    }
    println!("Total : {}", total);
}

fn part_two(data: &Vec<&str>) {
    let to_expand = find_columns_to_expand(&data);
    let mut pair_set : HashSet<((usize, usize), (usize,usize))> = HashSet::new();
    let mut all_galaxies = Vec::new();
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                all_galaxies.push((x, y));
            }
        }
    }
    for (i, galaxy) in all_galaxies.iter().enumerate() {
        for j in i+1..all_galaxies.len() {
            pair_set.insert((*galaxy, all_galaxies[j]));
        }
    }
    let mut total: usize = 0;
    let mut adds = 0;
    for pair in pair_set {
        let mut distance = ((pair.0).0 as i32 - (pair.1).0 as i32).abs() + ((pair.0).1 as i32 - (pair.1).1 as i32).abs();
        let x_values = ((min(pair.0.0, pair.1.0)+1)..max(pair.0.0, pair.1.0)).collect::<Vec<_>>();
        let y_values = ((min(pair.0.1, pair.1.1)+1)..max(pair.0.1, pair.1.1)).collect::<Vec<_>>();
        let x_adds = x_values.iter().filter(|x| to_expand.0.contains(&x)).count();
        let y_adds = y_values.iter().filter(|y| to_expand.1.contains(&y)).count();
        total += (x_adds + y_adds)*999999;
        total += distance as usize;
    }
    println!("Total : {}", total);
    println!("Adds : {}", adds);
}

fn expand_universe(data : &Vec<&str>) -> Vec<String>{
    let find_columns_to_expand = find_columns_to_expand(&data);
    let mut new_universe = Vec::new();
    for (y, line) in data.iter().enumerate() {
        let mut new_line = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if find_columns_to_expand.0.contains(&x){
                new_line.push(c);
                new_line.push('.');
            } else {
                new_line.push(c);
            }
        }
        new_universe.push(new_line);
        if find_columns_to_expand.1.contains(&y) {
            new_universe.push(vec!['.'; new_universe[0].len()]);
        }
    }
    let new_universe_str = new_universe.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<_>>();
    new_universe_str
}

fn find_columns_to_expand(data : &Vec<&str>) -> (Vec<usize>, Vec<usize>) {
    let mut x_values = (0..data[0].len()).collect::<Vec<_>>();
    let mut y_values = Vec::new();
    for (y, line) in data.iter().enumerate() {
        if !line.contains('#') {
            y_values.push(y);
        }
        let temp_x = line.chars().enumerate().filter(|(_, c)| *c == '#').map(|(i, _)| i).collect::<Vec<_>>();
        x_values = x_values.into_iter().filter(|x| !temp_x.contains(x)).collect::<Vec<_>>();

    }
    (x_values, y_values)
}

// test expand
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns() {
        let data = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#....."
        ];
        let expected = (vec![2,5,8], vec![3,7]);
        assert_eq!(find_columns_to_expand(&data), expected);
    }
    #[test]
    fn test_expand() {
        let data = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#....."
        ];
        let expected = vec![
                "....#........",
                ".........#...",
                "#............",
                ".............",
                ".............",
                "........#....",
                ".#...........",
                "............#",
                ".............",
                ".............",
                ".........#...",
                "#....#......."
        ];
        // expected as lines of str

        assert_eq!(expand_universe(&data), expected);
    }
}